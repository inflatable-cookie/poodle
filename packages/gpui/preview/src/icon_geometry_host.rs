//! GPUI-owned scheduler for one icon-geometry runtime.
//!
//! Shared composition still owns pair lookup and the compact frame. This host
//! owns the 180 ms clock, invalidation, and teardown. Dropping the scheduled
//! task is the cancellation path; a late timer must not write.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use gpui::{App, Task, Window};
use poodle_headless::motion_policy::MotionPolicy;
use poodle_node::Node;
use poodle_render::context::RenderContext;
use poodle_render::icon_geometry::{
    activate_icon_geometry, complete_icon_geometry, create_icon_geometry_runtime,
    icon_geometry_clock_timing, live_geometry_clock_count, sample_icon_geometry,
    set_icon_geometry_policy, teardown_icon_geometry, write_resolved_frame,
    GeometryRuntimeDecision, GeometryRuntimeIntent, IconGeometryRuntime, ICON_GEOMETRY_DURATION_MS,
};

const FRAME_INTERVAL: Duration = Duration::from_millis(16);

#[derive(Clone, Copy)]
pub struct ScheduledTickProbe {
    pub before: fn(),
    pub after: fn(),
}

struct HostInner {
    runtime: IconGeometryRuntime,
    key: Option<String>,
    started_at: Option<Instant>,
    duration: Duration,
    size: f32,
}

pub struct IconGeometryHost {
    inner: Arc<Mutex<HostInner>>,
    node: Arc<Mutex<Node>>,
    task: Option<Task<()>>,
    wakeups: Arc<AtomicUsize>,
    task_dropped: Arc<AtomicBool>,
    tick_probe: Option<ScheduledTickProbe>,
}

impl IconGeometryHost {
    pub fn new(policy: MotionPolicy, size: f32, ctx: &RenderContext<'_>) -> Self {
        let runtime = create_icon_geometry_runtime(policy);
        let node = poodle_render::icon_geometry::resolved_icon_geometry(&runtime, size, ctx);
        Self {
            inner: Arc::new(Mutex::new(HostInner {
                runtime,
                key: None,
                started_at: None,
                duration: Duration::from_millis(u64::from(ICON_GEOMETRY_DURATION_MS)),
                size,
            })),
            node: Arc::new(Mutex::new(node)),
            task: None,
            wakeups: Arc::new(AtomicUsize::new(0)),
            task_dropped: Arc::new(AtomicBool::new(false)),
            tick_probe: None,
        }
    }

    pub fn node(&self) -> Arc<Mutex<Node>> {
        Arc::clone(&self.node)
    }

    pub fn live_clocks(&self) -> usize {
        live_geometry_clock_count(&self.inner.lock().expect("geometry host").runtime)
    }

    pub fn scheduled_wakeups(&self) -> usize {
        self.wakeups.load(Ordering::SeqCst)
    }

    pub fn scheduled_task_dropped(&self) -> bool {
        self.task_dropped.load(Ordering::SeqCst)
    }

    pub fn live_key(&self) -> Option<String> {
        self.inner.lock().expect("geometry host").key.clone()
    }

    pub fn scheduled_duration_ms(&self) -> Option<u64> {
        let inner = self.inner.lock().expect("geometry host");
        inner.started_at.map(|_| inner.duration.as_millis() as u64)
    }

    pub fn set_tick_probe(&mut self, probe: ScheduledTickProbe) {
        self.tick_probe = Some(probe);
    }

    pub fn with_runtime<R>(&self, body: impl FnOnce(&mut IconGeometryRuntime) -> R) -> R {
        let mut inner = self.inner.lock().expect("geometry host");
        body(&mut inner.runtime)
    }

    pub fn activate(
        &mut self,
        intent: GeometryRuntimeIntent,
        window: &mut Window,
        cx: &mut App,
        ctx: &RenderContext<'_>,
    ) -> GeometryRuntimeDecision {
        let decision = {
            let mut inner = self.inner.lock().expect("geometry host");
            let decision = activate_icon_geometry(&mut inner.runtime, intent);
            let mut node = self.node.lock().expect("geometry node");
            *node = poodle_render::icon_geometry::resolved_icon_geometry(
                &inner.runtime,
                inner.size,
                ctx,
            );
            if decision.schedule {
                let (_, duration_ms) = icon_geometry_clock_timing(&inner.runtime, &decision.key)
                    .expect("scheduled geometry timing");
                inner.key = Some(decision.key.clone());
                inner.started_at = Some(cx.background_executor().now());
                inner.duration = Duration::from_millis(u64::from(duration_ms));
            } else if !decision.live_clock {
                inner.key = None;
                inner.started_at = None;
            }
            decision
        };
        if decision.schedule {
            self.cancel_task();
            self.spawn(window, cx);
        } else if !decision.live_clock {
            self.cancel_task();
        }
        window.refresh();
        decision
    }

    pub fn set_policy(
        &mut self,
        policy: MotionPolicy,
        window: &mut Window,
        ctx: &RenderContext<'_>,
    ) -> Vec<GeometryRuntimeDecision> {
        let decisions = {
            let mut inner = self.inner.lock().expect("geometry host");
            let decisions = set_icon_geometry_policy(&mut inner.runtime, policy);
            let mut node = self.node.lock().expect("geometry node");
            *node = poodle_render::icon_geometry::resolved_icon_geometry(
                &inner.runtime,
                inner.size,
                ctx,
            );
            if !decisions.iter().any(|decision| decision.live_clock) {
                inner.key = None;
                inner.started_at = None;
            }
            decisions
        };
        if !decisions.iter().any(|decision| decision.live_clock) {
            self.cancel_task();
        }
        window.refresh();
        decisions
    }

    pub fn teardown(&mut self) {
        self.cancel_task();
        let mut inner = self.inner.lock().expect("geometry host");
        teardown_icon_geometry(&mut inner.runtime, None);
        inner.key = None;
        inner.started_at = None;
        let mut node = self.node.lock().expect("geometry node");
        *node = Node::container();
    }

    fn cancel_task(&mut self) {
        drop(self.task.take());
    }

    fn spawn(&mut self, window: &mut Window, cx: &mut App) {
        let inner = Arc::clone(&self.inner);
        let node = Arc::clone(&self.node);
        let wakeups = Arc::clone(&self.wakeups);
        let task_dropped = Arc::clone(&self.task_dropped);
        let tick_probe = self.tick_probe;
        self.task_dropped.store(false, Ordering::SeqCst);
        self.task = Some(window.spawn(cx, async move |cx| {
            let _probe = TaskDropProbe(task_dropped);
            loop {
                cx.background_executor().timer(FRAME_INTERVAL).await;
                wakeups.fetch_add(1, Ordering::SeqCst);
                let keep = cx
                    .update(|window, cx| {
                        let now = cx.background_executor().now();
                        if let Some(probe) = tick_probe {
                            (probe.before)();
                        }
                        let keep = tick_scheduled_frame(&inner, &node, now);
                        window.refresh();
                        if let Some(probe) = tick_probe {
                            (probe.after)();
                        }
                        keep
                    })
                    .unwrap_or(false);
                if !keep {
                    break;
                }
            }
        }));
    }
}

struct TaskDropProbe(Arc<AtomicBool>);

impl Drop for TaskDropProbe {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

fn tick_scheduled_frame(
    inner: &Arc<Mutex<HostInner>>,
    node: &Arc<Mutex<Node>>,
    now: Instant,
) -> bool {
    let mut inner = inner.lock().expect("geometry host");
    let HostInner {
        runtime,
        key: key_slot,
        started_at,
        duration,
        ..
    } = &mut *inner;
    let (Some(key), Some(started)) = (key_slot.as_deref(), *started_at) else {
        return false;
    };
    let progress = (now.saturating_duration_since(started).as_secs_f32() / duration.as_secs_f32())
        .clamp(0.0, 1.0);
    if progress >= 1.0 {
        complete_icon_geometry(runtime, key);
        *key_slot = None;
        *started_at = None;
        let mut node = node.lock().expect("geometry node");
        write_resolved_frame(runtime, &mut node);
        return false;
    }
    sample_icon_geometry(runtime, key, progress);
    let mut node = node.lock().expect("geometry node");
    write_resolved_frame(runtime, &mut node);
    true
}

impl Drop for IconGeometryHost {
    fn drop(&mut self) {
        self.cancel_task();
    }
}
