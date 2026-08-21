//! Runtime-owned scroll state for node-backed components.
//!
//! The render vocabulary describes content and controls. GPUI owns offsets,
//! wheel physics, and imperative movement, so those mechanics stay here rather
//! than leaking into `poodle-node` or a component spec.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use gpui::{
    div, px, AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement,
    ScrollHandle, SharedString, StatefulInteractiveElement, Styled, Window,
};
use poodle_node::Node;

/// Persistent state for a tracked vertical viewport.
///
/// Hosts retain one value for the lifetime of the component instance. Clones
/// share the same GPUI handle and pin state.
#[derive(Clone, Debug)]
pub struct TrackedScrollState {
    handle: ScrollHandle,
    pinned: Rc<Cell<bool>>,
    jump_requested: Arc<AtomicBool>,
}

impl Default for TrackedScrollState {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackedScrollState {
    pub fn new() -> Self {
        Self {
            handle: ScrollHandle::new(),
            pinned: Rc::new(Cell::new(true)),
            jump_requested: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_pinned(&self) -> bool {
        self.pinned.get()
    }

    pub fn offset_y(&self) -> f32 {
        self.handle.offset().y.into()
    }

    pub fn max_offset_y(&self) -> f32 {
        self.handle.max_offset().y.into()
    }

    pub fn remaining_to_bottom(&self) -> f32 {
        (self.max_offset_y() + self.offset_y()).max(0.0)
    }

    /// Move the real viewport and re-arm following.
    pub fn jump_to_bottom(&self) {
        self.pinned.set(true);
        self.handle.scroll_to_bottom();
    }

    /// Send-safe component handler for a renderer-owned jump control.
    ///
    /// The handler records intent only. The next GPUI build consumes it on
    /// the runtime thread, where the non-Send `ScrollHandle` belongs.
    pub fn jump_handler(&self) -> Arc<dyn Fn() + Send + Sync> {
        let requested = Arc::clone(&self.jump_requested);
        Arc::new(move || {
            requested.store(true, Ordering::Release);
        })
    }

    fn consume_jump_request(&self) {
        if self.jump_requested.swap(false, Ordering::AcqRel) {
            self.jump_to_bottom();
        }
    }

    fn sync_pin_state(&self, threshold: f32) -> bool {
        let pinned = is_pinned(self.remaining_to_bottom(), threshold);
        let changed = pinned != self.pinned.get();
        self.pinned.set(pinned);
        changed
    }
}

fn is_pinned(remaining_to_bottom: f32, threshold: f32) -> bool {
    remaining_to_bottom.max(0.0) <= threshold.max(0.0)
}

/// Runtime behavior for [`tracked_vertical_scroll`].
#[derive(Clone, Copy, Debug)]
pub struct TrackedScrollOptions<'a> {
    pub viewport_id: &'a str,
    pub jump_id: &'a str,
    pub pin_threshold: f32,
    pub auto_follow: bool,
    pub is_empty: bool,
}

/// Render node content in one real GPUI scroll viewport.
///
/// `jump_control` remains renderer-owned node composition. This adapter only
/// decides when it is present and wires it to GPUI's real scroll handle.
pub fn tracked_vertical_scroll(
    content: &Node,
    jump_control: &Node,
    state: &TrackedScrollState,
    options: TrackedScrollOptions<'_>,
) -> AnyElement {
    state.consume_jump_request();
    if options.auto_follow && state.is_pinned() {
        state.handle.scroll_to_bottom();
    }

    let observed = state.clone();
    let pin_threshold = options.pin_threshold;
    let viewport = div()
        .id(ElementId::Name(SharedString::from(
            options.viewport_id.to_owned(),
        )))
        .size_full()
        .overflow_y_scroll()
        .track_scroll(&state.handle)
        .on_scroll_wheel(move |_event, window: &mut Window, cx: &mut App| {
            // GPUI's built-in wheel listener updates the handle later in the
            // same dispatch. Observe after dispatch, then rebuild only if the
            // derived pin posture changed.
            let observed = observed.clone();
            window.defer(cx, move |_window, cx| {
                if observed.sync_pin_state(pin_threshold) {
                    cx.refresh_windows();
                }
            });
        })
        .child(crate::to_gpui(content));

    let mut root = div().relative().size_full().child(viewport);
    if !options.is_empty && !state.is_pinned() {
        root = root.child(
            div()
                .id(ElementId::Name(SharedString::from(
                    options.jump_id.to_owned(),
                )))
                .absolute()
                .left(px(0.0))
                .right(px(0.0))
                .bottom(px(8.0))
                .flex()
                .justify_center()
                .child(crate::to_gpui(jump_control)),
        );
    }

    root.into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_state_uses_remaining_distance_and_clamps_negative_thresholds() {
        assert!(is_pinned(32.0, 32.0));
        assert!(!is_pinned(33.0, 32.0));
        assert!(is_pinned(-0.5, 0.0));
        assert!(!is_pinned(0.5, -1.0));
    }
}
