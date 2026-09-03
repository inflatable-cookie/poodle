//! Window-owned tooltip lifecycle runtime (spec 013, card g16.066).
//!
//! Replaces GPUI 0.2.2's hardcoded 500ms hover-only `.tooltip()` with one
//! Poodle-owned backend runtime.
//!
//! # Lifecycle Contract
//! - 300ms open delay on hover or focus.
//! - Dismiss on pointer leave, focus departure (blur), Escape key, target
//!   disablement, target removal from tree, window teardown, or target supersession.
//! - Window isolation: tooltips are owned per mounted window and never shared.
//! - Paint is authority: target must be painted in the current frame to show/stay visible.
//! - Generation tracking: new targets supersede old ones; stale timers are inert.
//! - Empty or absent tooltips stay completely inert.

use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;

use gpui::{
    deferred, div, px, AnyElement, AnyWindowHandle, App, InteractiveElement, IntoElement,
    ParentElement, SharedString, Styled, Task, Window,
};

use crate::record_probe_channel;

/// Open delay for non-empty node tooltips (contract: 300ms).
pub const TOOLTIP_DELAY: Duration = Duration::from_millis(300);

/// What the tooltip paint pass last painted: target element id, text, and bounds.
#[derive(Clone, Debug, PartialEq)]
pub struct PaintedTooltip {
    pub target_id: String,
    pub text: String,
    /// `[x, y, width, height]` in logical pixels.
    pub bounds: [f32; 4],
}

#[derive(Default)]
pub(crate) struct WindowTooltipState {
    pub target_id: Option<String>,
    pub text: Option<String>,
    pub target_bounds: Option<gpui::Bounds<gpui::Pixels>>,
    pub generation: u64,
    pub is_visible: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub painted_this_frame: bool,
    pub task: Option<Task<()>>,
}

impl WindowTooltipState {
    pub fn reset(&mut self) {
        self.target_id = None;
        self.text = None;
        self.target_bounds = None;
        self.generation = self.generation.wrapping_add(1);
        self.is_visible = false;
        self.is_hovered = false;
        self.is_focused = false;
        self.painted_this_frame = false;
        self.task = None;
    }
}

thread_local! {
    /// Window tooltip state map, keyed by window handle.
    static WINDOW_TOOLTIPS: RefCell<HashMap<AnyWindowHandle, WindowTooltipState>> =
        RefCell::new(HashMap::new());

    /// Last painted tooltip per window, rebuilt each frame.
    static PAINTED_TOOLTIPS: RefCell<HashMap<AnyWindowHandle, PaintedTooltip>> =
        RefCell::new(HashMap::new());
}

/// Reset all tooltip state across all windows. Called at test teardown or
/// when the focus/backend registries are reset.
pub fn reset_tooltip_registry() {
    WINDOW_TOOLTIPS.with(|cell| cell.borrow_mut().clear());
    PAINTED_TOOLTIPS.with(|cell| cell.borrow_mut().clear());
}

/// The painted tooltip for the first active window, or `None` if no tooltip
/// is currently painted.
pub fn painted_tooltip() -> Option<PaintedTooltip> {
    PAINTED_TOOLTIPS.with(|cell| cell.borrow().values().next().cloned())
}

/// The painted tooltip for a specific window handle, if any was painted
/// in the last frame.
pub fn painted_tooltip_for(handle: AnyWindowHandle) -> Option<PaintedTooltip> {
    PAINTED_TOOLTIPS.with(|cell| cell.borrow().get(&handle).cloned())
}

/// Whether the given target element currently has a visible tooltip.
pub fn is_tooltip_visible(target_id: &str) -> bool {
    WINDOW_TOOLTIPS.with(|cell| {
        cell.borrow()
            .values()
            .any(|state| state.is_visible && state.target_id.as_deref() == Some(target_id))
    })
}

/// Whether the given target element currently has a pending tooltip timer.
pub fn is_tooltip_pending(target_id: &str) -> bool {
    WINDOW_TOOLTIPS.with(|cell| {
        cell.borrow().values().any(|state| {
            !state.is_visible
                && state.target_id.as_deref() == Some(target_id)
                && state.task.is_some()
        })
    })
}

/// Record a painted tooltip during the paint pass.
pub(crate) fn record_painted_tooltip(
    handle: AnyWindowHandle,
    target_id: &str,
    text: &str,
    bounds: [f32; 4],
) {
    PAINTED_TOOLTIPS.with(|cell| {
        cell.borrow_mut().insert(
            handle,
            PaintedTooltip {
                target_id: target_id.to_owned(),
                text: text.to_owned(),
                bounds,
            },
        );
    });
}

/// Frame boundary hook: called from `overlay_frame_begin()` at the start of
/// every render frame.
pub(crate) fn prepare_tooltip_frame() {
    PAINTED_TOOLTIPS.with(|cell| cell.borrow_mut().clear());
    WINDOW_TOOLTIPS.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            state.painted_this_frame = false;
        }
    });
}

/// Frame boundary hook: called from `overlay_frame_end()` at the end of
/// every render frame. Cancels tooltips whose targets were not painted in
/// this frame (removal from tree).
pub(crate) fn sweep_unpainted_tooltips() {
    WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        for state in map.values_mut() {
            if state.target_id.is_some() && !state.painted_this_frame {
                record_probe_channel("tooltip.lifecycle.removed");
                state.reset();
            }
        }
    });
}

/// Record paint presence, bounds, and disabled status for an element with a tooltip.
pub(crate) fn record_tooltip_target_paint(
    window: &mut Window,
    target_id: &str,
    text: &str,
    bounds: gpui::Bounds<gpui::Pixels>,
    disabled: bool,
) {
    let handle = window.window_handle();
    WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        if let Some(state) = map.get_mut(&handle) {
            if state.target_id.as_deref() == Some(target_id) {
                if disabled || text.is_empty() {
                    record_probe_channel("tooltip.lifecycle.disabled");
                    state.reset();
                } else {
                    state.painted_this_frame = true;
                    state.text = Some(text.to_owned());
                    state.target_bounds = Some(bounds);
                }
            }
        }
    });
}

/// Pointer hover enter: start the 300ms timer for a target element.
pub(crate) fn on_pointer_enter(window: &mut Window, cx: &mut App, target_id: &str, text: &str) {
    if text.is_empty() {
        return;
    }
    let handle = window.window_handle();
    WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        let state = map.entry(handle).or_default();

        if state.target_id.as_deref() == Some(target_id) {
            state.is_hovered = true;
            state.painted_this_frame = true;
            return;
        }

        // New target supersedes any previous generation.
        state.generation = state.generation.wrapping_add(1);
        let gen = state.generation;
        state.target_id = Some(target_id.to_owned());
        state.text = Some(text.to_owned());
        state.is_visible = false;
        state.is_hovered = true;
        state.painted_this_frame = true;
        state.task = None;

        let target = target_id.to_owned();
        let task = window.spawn(cx, async move |cx| {
            cx.background_executor().timer(TOOLTIP_DELAY).await;
            cx.update(|window, cx| {
                on_timer_fired(handle, &target, gen, window, cx);
            })
            .ok();
        });
        state.task = Some(task);
        record_probe_channel("tooltip.lifecycle.pending");
    });
}

/// Pointer hover leave: cancel pending timer or hide visible tooltip immediately.
pub(crate) fn on_pointer_leave(window: &mut Window, cx: &mut App, target_id: &str) {
    let handle = window.window_handle();
    let changed = WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(state) = map.get_mut(&handle) else {
            return false;
        };
        if state.target_id.as_deref() == Some(target_id) {
            state.is_hovered = false;
            let was_active = state.is_visible || state.task.is_some();
            state.reset();
            was_active
        } else {
            false
        }
    });
    if changed {
        record_probe_channel("tooltip.lifecycle.hidden");
        window.refresh();
        cx.refresh_windows();
    }
}

/// Keyboard focus enter: start the 300ms timer for a focusable target element.
pub(crate) fn on_focus_enter(window: &mut Window, cx: &mut App, target_id: &str, text: &str) {
    if text.is_empty() {
        return;
    }
    let handle = window.window_handle();
    WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        let state = map.entry(handle).or_default();

        if state.target_id.as_deref() == Some(target_id) {
            state.is_focused = true;
            state.painted_this_frame = true;
            return;
        }

        // Focus supersedes any previous generation.
        state.generation = state.generation.wrapping_add(1);
        let gen = state.generation;
        state.target_id = Some(target_id.to_owned());
        state.text = Some(text.to_owned());
        state.is_visible = false;
        state.is_focused = true;
        state.painted_this_frame = true;
        state.task = None;

        let target = target_id.to_owned();
        let task = window.spawn(cx, async move |cx| {
            cx.background_executor().timer(TOOLTIP_DELAY).await;
            cx.update(|window, cx| {
                on_timer_fired(handle, &target, gen, window, cx);
            })
            .ok();
        });
        state.task = Some(task);
        record_probe_channel("tooltip.lifecycle.pending");
    });
}

/// Keyboard focus departure (blur): cancel pending timer or hide visible tooltip immediately.
pub(crate) fn on_focus_departure(window: &mut Window, cx: &mut App, target_id: &str) {
    let handle = window.window_handle();
    let changed = WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(state) = map.get_mut(&handle) else {
            return false;
        };
        if state.target_id.as_deref() == Some(target_id) {
            state.is_focused = false;
            let was_active = state.is_visible || state.task.is_some();
            state.reset();
            was_active
        } else {
            false
        }
    });
    if changed {
        record_probe_channel("tooltip.lifecycle.hidden");
        window.refresh();
        cx.refresh_windows();
    }
}

/// Dismiss any pending or visible tooltip in this window (Escape key, pointer press).
pub(crate) fn dismiss_tooltip(window: &mut Window, cx: &mut App) -> bool {
    let handle = window.window_handle();
    let changed = WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(state) = map.get_mut(&handle) else {
            return false;
        };
        if state.target_id.is_some() {
            let was_active = state.is_visible || state.task.is_some();
            state.reset();
            was_active
        } else {
            false
        }
    });
    if changed {
        record_probe_channel("tooltip.lifecycle.hidden");
        window.refresh();
        cx.refresh_windows();
    }
    changed
}

/// Called when the 300ms background timer fires.
pub(crate) fn on_timer_fired(
    window_handle: AnyWindowHandle,
    target_id: &str,
    generation: u64,
    window: &mut Window,
    cx: &mut App,
) {
    let should_show = WINDOW_TOOLTIPS.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(state) = map.get_mut(&window_handle) else {
            return false;
        };
        if state.generation == generation
            && state.target_id.as_deref() == Some(target_id)
            && state.painted_this_frame
            && (state.is_hovered || state.is_focused)
        {
            state.is_visible = true;
            state.task = None;
            true
        } else {
            false
        }
    });
    if should_show {
        record_probe_channel("tooltip.lifecycle.shown");
        window.refresh();
        cx.refresh_windows();
    }
}

/// Build the active tooltip element if one is currently visible and has recorded bounds.
pub(crate) fn render_active_tooltip() -> Option<AnyElement> {
    let active_info = WINDOW_TOOLTIPS.with(|cell| {
        let map = cell.borrow();
        for state in map.values() {
            if !state.is_visible {
                continue;
            }
            let (Some(target_id), Some(text)) = (&state.target_id, &state.text) else {
                continue;
            };
            let bounds = state
                .target_bounds
                .or_else(|| crate::layers::bounds_for(target_id));
            if let Some(bounds) = bounds {
                return Some((target_id.clone(), text.clone(), bounds));
            }
        }
        None
    })?;

    let (target_id, text, bounds) = active_info;

    let top = bounds.origin.y + bounds.size.height + px(4.0);
    let left = bounds.origin.x;

    let paint_target = target_id.clone();
    let paint_text = text.clone();
    let canvas_record = gpui::canvas(
        move |bounds, window, _cx| {
            let handle = window.window_handle();
            record_painted_tooltip(
                handle,
                &paint_target,
                &paint_text,
                [
                    bounds.origin.x.into(),
                    bounds.origin.y.into(),
                    bounds.size.width.into(),
                    bounds.size.height.into(),
                ],
            );
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top(px(0.0))
    .left(px(0.0))
    .size_full();

    let bubble = div()
        .px(px(8.0))
        .py(px(4.0))
        .rounded(px(6.0))
        .bg(gpui::hsla(0.0, 0.0, 0.12, 0.96))
        .text_color(gpui::hsla(0.0, 0.0, 0.96, 1.0))
        .text_sm()
        .line_height(px(14.0))
        .child(SharedString::from(text));

    let tooltip_node = div()
        .id("poodle-active-tooltip")
        .absolute()
        .top(top)
        .left(left)
        .occlude()
        .child(canvas_record)
        .child(bubble);

    Some(deferred(tooltip_node).with_priority(999).into_any_element())
}
