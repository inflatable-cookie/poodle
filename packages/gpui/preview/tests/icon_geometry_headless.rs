//! Focused headless probes for the private icon-geometry GPUI path.
//!
//! No native pixels, no windowed capture. Candidate geometry is fixture input.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use gpui::TestAppContext;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::motion_policy::MotionPolicy;
use poodle_node::NodeKind;
use poodle_render::context::RenderContext;
use poodle_render::icon_geometry::{
    activate_icon_geometry, compact_frame_point_caps, compact_frame_point_ptrs,
    create_icon_geometry_runtime, planned_candidate_fixture, resolved_frame_point_caps,
    resolved_icon_geometry, sample_icon_geometry, write_resolved_frame, GeometryEndpoint,
    GeometryRuntimeIntent, ICON_GEOMETRY_DURATION_MS,
};
use poodle_specs::IconSpec;

#[path = "../src/headless_driver.rs"]
mod headless_driver;
#[path = "../src/icon_geometry_host.rs"]
mod icon_geometry_host;

use headless_driver::HeadlessDriver;
use icon_geometry_host::{IconGeometryHost, ScheduledTickProbe};

struct CountingAllocator;

thread_local! {
    static COUNT_THIS_THREAD: Cell<bool> = const { Cell::new(false) };
}
static TICK_ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static TICK_PROBE_ARMED: AtomicBool = AtomicBool::new(false);

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        COUNT_THIS_THREAD.with(|active| {
            if active.get() {
                TICK_ALLOCATIONS.fetch_add(1, Ordering::SeqCst);
            }
        });
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, new_size) };
        COUNT_THIS_THREAD.with(|active| {
            if active.get() {
                TICK_ALLOCATIONS.fetch_add(1, Ordering::SeqCst);
            }
        });
        pointer
    }
}

fn begin_tick_allocation_probe() {
    if TICK_PROBE_ARMED.load(Ordering::SeqCst) {
        TICK_ALLOCATIONS.store(0, Ordering::SeqCst);
        COUNT_THIS_THREAD.with(|active| active.set(true));
    }
}

fn end_tick_allocation_probe() {
    COUNT_THIS_THREAD.with(|active| active.set(false));
}

fn run_headless(body: impl FnOnce(&mut TestAppContext)) {
    poodle_gpui_node_backend::reset_focus_registry();
    let mut cx = TestAppContext::single();
    body(&mut cx);
    cx.dispatcher.run_until_parked();
    cx.background_executor.forbid_parking();
    cx.quit();
    cx.dispatcher.run_until_parked();
}

fn theme() -> GpuiThemeProvider {
    GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE)
}

fn intent(owner: &str, pair_id: &str, target: GeometryEndpoint) -> GeometryRuntimeIntent {
    GeometryRuntimeIntent {
        owner: String::from(owner),
        pair_id: String::from(pair_id),
        target,
        initial: false,
    }
}

fn p95_millis(samples: &mut [Duration]) -> f64 {
    samples.sort_unstable();
    let idx = ((samples.len() as f64) * 0.95).ceil() as usize - 1;
    samples[idx.min(samples.len().saturating_sub(1))].as_secs_f64() * 1000.0
}

#[test]
fn resolved_geometry_paints_without_pair_lookup_and_tears_down() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let static_icon = poodle_render::icon(&IconSpec::new("plus"), &ctx);
        assert!(matches!(&static_icon.kind, NodeKind::Icon { name, .. } if name == "plus"));

        let mut host = IconGeometryHost::new(MotionPolicy::Full, 16.0, &ctx);
        let live = host.node();
        poodle_gpui_node_backend::begin_probe_capture();
        let mut driver = HeadlessDriver::new(cx, live);
        driver.with_window(|window, cx| {
            host.activate(
                intent(
                    "geometry-owner",
                    "chevron-left-to-chevron-right",
                    GeometryEndpoint::To,
                ),
                window,
                cx,
                &ctx,
            );
        });
        driver.draw_frame();
        {
            let node = host.node();
            let guard = node.lock().expect("node lock");
            match &guard.kind {
                NodeKind::ResolvedIconGeometry { frame, .. } => {
                    assert!(!frame.contours.is_empty());
                }
                _ => panic!("expected resolved geometry, got {guard:?}"),
            }
            assert!(!guard.has_text("chevron-left-to-chevron-right"));
        }

        poodle_gpui_node_backend::begin_probe_capture();
        driver.advance_clock(Duration::from_millis(16));
        driver.drain();
        driver.draw_if_invalidated();
        assert!(
            host.scheduled_wakeups() >= 1,
            "production scheduler must advance the 180ms clock, duration={ICON_GEOMETRY_DURATION_MS}"
        );
        assert_eq!(host.live_clocks(), 1);
        let scheduled_paint = poodle_gpui_node_backend::take_probe_capture();
        assert!(
            scheduled_paint.contains(&"content.text-icon.resolved-geometry"),
            "scheduler invalidation must repaint the mounted geometry: {scheduled_paint:?}"
        );

        let channels = poodle_gpui_node_backend::take_probe_capture();
        assert!(
            channels
                .iter()
                .any(|channel| *channel == "content.text-icon.resolved-geometry"),
            "missing resolved-geometry probe, got {channels:?}"
        );
        assert!(
            !channels.iter().any(|channel| channel.contains("pair")),
            "backend must not record pair lookup, got {channels:?}"
        );

        let wakeups_at_teardown = host.scheduled_wakeups();
        host.teardown();
        driver.draw_frame();
        assert!(
            host.scheduled_task_dropped(),
            "teardown must drop the scheduled native task"
        );
        driver.advance_clock(Duration::from_millis(u64::from(ICON_GEOMETRY_DURATION_MS)));
        driver.drain();
        driver.draw_frame();
        assert_eq!(
            host.scheduled_wakeups(),
            wakeups_at_teardown,
            "teardown must cancel the scheduled native frame"
        );
        assert_eq!(host.live_clocks(), 0);
        {
            let node = host.node();
            assert!(matches!(
                node.lock().expect("node lock").kind,
                NodeKind::Container
            ));
        }
        drop(driver);
    });
}

#[test]
fn host_keeps_inert_clock_uses_reverse_duration_and_cancels_on_policy_tightening() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut host = IconGeometryHost::new(MotionPolicy::Full, 16.0, &ctx);
        let mut driver = HeadlessDriver::new(cx, host.node());
        let first = driver.with_window(|window, cx| {
            host.activate(
                intent(
                    "lifecycle",
                    "chevron-left-to-chevron-right",
                    GeometryEndpoint::To,
                ),
                window,
                cx,
                &ctx,
            )
        });
        driver.draw_frame();
        driver.advance_clock(Duration::from_millis(72));
        driver.draw_frame();
        let repeated = driver.with_window(|window, cx| {
            host.activate(
                intent(
                    "lifecycle",
                    "chevron-left-to-chevron-right",
                    GeometryEndpoint::To,
                ),
                window,
                cx,
                &ctx,
            )
        });
        assert_eq!(
            repeated.interruption,
            poodle_headless::motion_policy::MotionInterruption::Inert
        );
        assert_eq!(host.live_key().as_deref(), Some(first.key.as_str()));
        assert!(
            !host.scheduled_task_dropped(),
            "inert repeat must keep the task alive"
        );

        let reverse = driver.with_window(|window, cx| {
            host.activate(
                intent(
                    "lifecycle",
                    "chevron-left-to-chevron-right",
                    GeometryEndpoint::From,
                ),
                window,
                cx,
                &ctx,
            )
        });
        assert_eq!(
            reverse.interruption,
            poodle_headless::motion_policy::MotionInterruption::Reverse
        );
        let reverse_ms = host.scheduled_duration_ms().expect("reverse duration");
        assert!(reverse_ms > 0 && reverse_ms < u64::from(ICON_GEOMETRY_DURATION_MS));

        driver.with_window(|window, _cx| {
            let decisions = host.set_policy(MotionPolicy::Frozen, window, &ctx);
            assert_eq!(decisions.len(), 1);
        });
        assert_eq!(host.live_clocks(), 0);
        driver.draw_frame();
        assert!(host.scheduled_task_dropped());
        drop(driver);
    });
}

#[test]
fn scheduled_tick_allocates_nothing_after_plan_creation() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut host = IconGeometryHost::new(MotionPolicy::Full, 16.0, &ctx);
        host.set_tick_probe(ScheduledTickProbe {
            before: begin_tick_allocation_probe,
            after: end_tick_allocation_probe,
        });
        let mut driver = HeadlessDriver::new(cx, host.node());
        driver.with_window(|window, cx| {
            host.activate(
                intent(
                    "allocation",
                    "chevron-left-to-chevron-right",
                    GeometryEndpoint::To,
                ),
                window,
                cx,
                &ctx,
            );
        });
        driver.draw_frame();
        TICK_PROBE_ARMED.store(true, Ordering::SeqCst);
        TICK_ALLOCATIONS.store(usize::MAX, Ordering::SeqCst);
        driver.advance_clock(Duration::from_millis(16));
        driver.drain();
        driver.draw_if_invalidated();
        TICK_PROBE_ARMED.store(false, Ordering::SeqCst);
        assert_eq!(
            TICK_ALLOCATIONS.load(Ordering::SeqCst),
            0,
            "scheduled tick allocated"
        );
        host.teardown();
        drop(driver);
    });
}

#[test]
fn missing_shared_lookup_cannot_recover_pair_meaning() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut host = IconGeometryHost::new(MotionPolicy::Full, 16.0, &ctx);
        let live = host.node();
        let mut driver = HeadlessDriver::new(cx, live);
        driver.with_window(|window, cx| {
            host.activate(
                intent("geometry-owner", "menu-to-ellipsis", GeometryEndpoint::To),
                window,
                cx,
                &ctx,
            );
        });
        driver.draw_frame();
        let node = host.node();
        let guard = node.lock().expect("node lock");
        match &guard.kind {
            NodeKind::ResolvedIconGeometry { frame, .. } => {
                assert!(frame.contours.is_empty());
            }
            _ => panic!("expected empty resolved geometry, got {guard:?}"),
        }
        drop(driver);
    });
}

#[test]
fn second_owner_on_one_host_retargets_and_two_hosts_stay_independent() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut host_a = IconGeometryHost::new(MotionPolicy::Full, 16.0, &ctx);
        let mut host_b = IconGeometryHost::new(MotionPolicy::Full, 16.0, &ctx);
        let live = host_a.node();
        let mut driver = HeadlessDriver::new(cx, live);
        let mut owner_a_key = String::new();
        let mut host_a_key = String::new();
        let mut host_b_key = String::new();
        driver.with_window(|window, cx| {
            let first = host_a.activate(
                intent(
                    "owner-a",
                    "chevron-left-to-chevron-right",
                    GeometryEndpoint::To,
                ),
                window,
                cx,
                &ctx,
            );
            owner_a_key = first.key;
            host_b_key = host_b
                .activate(
                    intent("owner-b", "circle-to-dot", GeometryEndpoint::To),
                    window,
                    cx,
                    &ctx,
                )
                .key;
            host_a_key = host_a
                .activate(
                    intent("owner-b", "circle-to-dot", GeometryEndpoint::To),
                    window,
                    cx,
                    &ctx,
                )
                .key;
        });
        assert_eq!(host_a.live_clocks(), 1);
        assert_eq!(host_b.live_clocks(), 1);
        host_a.with_runtime(|runtime| {
            assert!(
                sample_icon_geometry(runtime, &owner_a_key, 0.5).is_none(),
                "replaced owner must not keep a live clock"
            );
            assert!(sample_icon_geometry(runtime, &host_a_key, 0.5).is_some());
        });
        let a_points = host_a.with_runtime(|runtime| {
            sample_icon_geometry(runtime, &host_a_key, 0.5)
                .expect("host a frame")
                .contours[0]
                .points
                .clone()
        });
        let b_points = host_b.with_runtime(|runtime| {
            sample_icon_geometry(runtime, &host_b_key, 0.5)
                .expect("host b frame")
                .contours[0]
                .points
                .clone()
        });
        assert_eq!(
            a_points, b_points,
            "two hosts on the same pair remain independent clocks, not one global plan"
        );
        // Re-sample host B after mutating A: B must not pick up A's later axis.
        host_a.with_runtime(|runtime| {
            sample_icon_geometry(runtime, &host_a_key, 0.9);
        });
        let b_again = host_b.with_runtime(|runtime| {
            sample_icon_geometry(runtime, &host_b_key, 0.5)
                .expect("host b frame")
                .contours[0]
                .points
                .clone()
        });
        assert_eq!(b_points, b_again);
        host_a.teardown();
        host_b.teardown();
        drop(driver);
    });
}

#[test]
fn allocation_and_p95_budgets_match_the_card() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut host = IconGeometryHost::new(MotionPolicy::Full, 16.0, &ctx);
        let live = host.node();
        let mut driver = HeadlessDriver::new(cx, live);
        driver.with_window(|window, cx| {
            host.activate(
                intent(
                    "budget-owner",
                    "chevron-left-to-chevron-right",
                    GeometryEndpoint::To,
                ),
                window,
                cx,
                &ctx,
            );
        });
        let key = host.live_key().expect("scheduled key");

        let mut one_ms = Vec::with_capacity(40);
        host.with_runtime(|runtime| {
            sample_icon_geometry(runtime, &key, 0.2);
            let node = host.node();
            let mut guard = node.lock().expect("node lock");
            write_resolved_frame(runtime, &mut guard);
            let compact_caps = compact_frame_point_caps(runtime);
            let compact_ptrs = compact_frame_point_ptrs(runtime);
            let node_caps = resolved_frame_point_caps(&guard);
            for i in 0..40 {
                let started = Instant::now();
                black_box(sample_icon_geometry(runtime, &key, 0.2 + (i as f32) * 0.01));
                write_resolved_frame(runtime, &mut guard);
                one_ms.push(started.elapsed());
            }
            assert_eq!(compact_frame_point_caps(runtime), compact_caps);
            assert_eq!(compact_frame_point_ptrs(runtime), compact_ptrs);
            assert_eq!(resolved_frame_point_caps(&guard), node_caps);
        });
        let one_p95 = p95_millis(&mut one_ms);
        assert!(
            one_p95 <= 1.0,
            "p95 geometry update exceeded 1ms/instance: {one_p95}ms"
        );

        let mut four = [
            create_icon_geometry_runtime(MotionPolicy::Full),
            create_icon_geometry_runtime(MotionPolicy::Full),
            create_icon_geometry_runtime(MotionPolicy::Full),
            create_icon_geometry_runtime(MotionPolicy::Full),
        ];
        let mut four_nodes: Vec<_> = four
            .iter()
            .map(|runtime| resolved_icon_geometry(runtime, 16.0, &ctx))
            .collect();
        let keys: Vec<String> = (0..4)
            .map(|index| {
                let start = activate_icon_geometry(
                    &mut four[index],
                    intent(
                        &format!("budget-{index}"),
                        "chevron-left-to-chevron-right",
                        GeometryEndpoint::To,
                    ),
                );
                four_nodes[index] = resolved_icon_geometry(&four[index], 16.0, &ctx);
                start.key
            })
            .collect();
        let mut four_ms = Vec::with_capacity(40);
        for i in 0..40 {
            let started = Instant::now();
            for index in 0..4 {
                black_box(sample_icon_geometry(
                    &mut four[index],
                    &keys[index],
                    0.2 + (i as f32) * 0.01,
                ));
                write_resolved_frame(&four[index], &mut four_nodes[index]);
            }
            four_ms.push(started.elapsed());
        }
        let four_p95 = p95_millis(&mut four_ms);
        assert!(
            four_p95 <= 4.0,
            "p95 geometry update exceeded 4ms for four instances: {four_p95}ms"
        );

        let mut cold = Vec::with_capacity(40);
        for _ in 0..40 {
            let started = Instant::now();
            black_box(planned_candidate_fixture("chevron-left-to-chevron-right"));
            let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
            activate_icon_geometry(
                &mut runtime,
                GeometryRuntimeIntent {
                    owner: String::from("cold-plan"),
                    pair_id: String::from("chevron-left-to-chevron-right"),
                    target: GeometryEndpoint::From,
                    initial: true,
                },
            );
            black_box(resolved_icon_geometry(&runtime, 16.0, &ctx));
            cold.push(started.elapsed());
        }
        let cold_p95 = p95_millis(&mut cold);
        assert!(cold_p95 <= 2.0, "p95 cold plan exceeded 2ms: {cold_p95}ms");

        host.teardown();
        drop(driver);
    });
}
