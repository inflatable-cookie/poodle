//! Scroll ownership for node viewports.
//!
//! A node whose layout overflow is `Scroll` and that is focusable or asks to
//! observe scrolling gets one retained GPUI `ScrollHandle`, keyed by element
//! id. The handle is what keyboard scrolling moves (the platform does not
//! scroll a focused viewport on its own) and what `Interaction::on_scroll`
//! reads. GPUI still owns wheel physics.

use super::*;

use gpui::{point, px, ScrollHandle};
use poodle_node::NodeScrollEvent;

/// Scroll distance of one arrow key press, matching a browser line step.
const LINE_STEP: f32 = 40.0;
/// A page key moves this share of the viewport, leaving context visible.
const PAGE_SHARE: f32 = 0.875;

type ScrollObserver = Arc<dyn Fn(&NodeScrollEvent) + Send + Sync>;

thread_local! {
    static SCROLL_HANDLES: RefCell<std::collections::HashMap<String, ScrollHandle>> =
        RefCell::new(std::collections::HashMap::new());
    static REPORTED: RefCell<std::collections::HashMap<String, (f32, f32)>> =
        RefCell::new(std::collections::HashMap::new());
}

/// The retained scroll position for a node id as distance from the content
/// start, if the node owns a scroll handle. Mounted proofs read this.
pub fn scroll_offset_for(id: &str) -> Option<(f32, f32)> {
    SCROLL_HANDLES.with(|handles| handles.borrow().get(id).map(position))
}

/// The scrollable extent beyond the viewport on each axis.
pub fn scroll_extent_for(id: &str) -> Option<(f32, f32)> {
    SCROLL_HANDLES.with(|handles| {
        handles.borrow().get(id).map(|handle| {
            let max = handle.max_offset();
            (max.width.into(), max.height.into())
        })
    })
}

fn position(handle: &ScrollHandle) -> (f32, f32) {
    let offset = handle.offset();
    (-f32::from(offset.x), -f32::from(offset.y))
}

fn handle_for(id: &str) -> ScrollHandle {
    SCROLL_HANDLES.with(|handles| {
        handles
            .borrow_mut()
            .entry(id.to_owned())
            .or_insert_with(ScrollHandle::new)
            .clone()
    })
}

fn report(id: &str, handle: &ScrollHandle, handler: &ScrollObserver) {
    let now = position(handle);
    let changed = REPORTED.with(|reported| {
        let mut reported = reported.borrow_mut();
        let previous = reported.insert(id.to_owned(), now).unwrap_or((0.0, 0.0));
        previous != now
    });
    if changed {
        handler(&NodeScrollEvent { x: now.0, y: now.1 });
    }
}

/// Where a key moves the viewport, as a new `(x, y)` position, or `None` when
/// the key does not scroll this viewport.
fn target(
    key: &str,
    current: (f32, f32),
    max: (f32, f32),
    viewport: (f32, f32),
    vertical: bool,
    horizontal: bool,
) -> Option<(f32, f32)> {
    let (x, y) = current;
    let page_y = viewport.1 * PAGE_SHARE;
    let page_x = viewport.0 * PAGE_SHARE;
    let next = match key {
        "down" if vertical => (x, y + LINE_STEP),
        "up" if vertical => (x, y - LINE_STEP),
        "right" if horizontal => (x + LINE_STEP, y),
        "left" if horizontal => (x - LINE_STEP, y),
        "pagedown" if vertical => (x, y + page_y),
        "pageup" if vertical => (x, y - page_y),
        "pagedown" if horizontal => (x + page_x, y),
        "pageup" if horizontal => (x - page_x, y),
        "home" if vertical => (x, 0.0),
        "end" if vertical => (x, max.1),
        "home" if horizontal => (0.0, y),
        "end" if horizontal => (max.0, y),
        _ => return None,
    };
    Some((
        next.0.clamp(0.0, max.0.max(0.0)),
        next.1.clamp(0.0, max.1.max(0.0)),
    ))
}

pub(super) fn apply_scroll(mut el: Stateful<Div>, node: &Node, id: &str) -> Stateful<Div> {
    let layout = &node.style.descriptor.layout;
    let vertical = layout.overflow_y == LayoutOverflow::Scroll;
    let horizontal = layout.overflow_x == LayoutOverflow::Scroll;
    let keyboard = node.interaction.focusable && !node.interaction.disabled;
    let observer = node.interaction.on_scroll.clone();
    if !(vertical || horizontal) || !(keyboard || observer.is_some()) {
        return el;
    }
    let handle = handle_for(id);
    el = el.track_scroll(&handle);

    if let Some(observer) = observer.clone() {
        let observed = handle.clone();
        let owner = id.to_owned();
        el = el.on_scroll_wheel(move |_event, window: &mut Window, cx: &mut App| {
            // GPUI's own wheel listener moves the handle later in this
            // dispatch, so read the position after it.
            let observed = observed.clone();
            let observer = observer.clone();
            let owner = owner.clone();
            window.defer(cx, move |_window, _cx| report(&owner, &observed, &observer));
        });
    }

    if keyboard {
        let keys = handle.clone();
        let owner = id.to_owned();
        el = el.on_key_down(move |event: &KeyDownEvent, window, cx| {
            let m = &event.keystroke.modifiers;
            if m.platform || m.control || m.alt || m.shift {
                return;
            }
            let max = keys.max_offset();
            let bounds = keys.bounds();
            let viewport = (bounds.size.width.into(), bounds.size.height.into());
            let Some((x, y)) = target(
                event.keystroke.key.as_str(),
                position(&keys),
                (max.width.into(), max.height.into()),
                viewport,
                vertical,
                horizontal,
            ) else {
                return;
            };
            keys.set_offset(point(px(-x), px(-y)));
            if let Some(observer) = &observer {
                report(&owner, &keys, observer);
            }
            window.prevent_default();
            cx.stop_propagation();
            cx.refresh_windows();
        });
    }
    el
}

#[cfg(test)]
mod tests {
    use super::target;

    const MAX: (f32, f32) = (300.0, 400.0);
    const VIEW: (f32, f32) = (200.0, 100.0);

    #[test]
    fn arrows_step_a_line_and_clamp_at_both_ends() {
        assert_eq!(
            target("down", (0.0, 0.0), MAX, VIEW, true, false),
            Some((0.0, 40.0))
        );
        assert_eq!(
            target("up", (0.0, 10.0), MAX, VIEW, true, false),
            Some((0.0, 0.0))
        );
        assert_eq!(
            target("down", (0.0, 395.0), MAX, VIEW, true, false),
            Some((0.0, 400.0))
        );
    }

    #[test]
    fn keys_for_an_axis_the_viewport_does_not_own_do_nothing() {
        assert_eq!(target("right", (0.0, 0.0), MAX, VIEW, true, false), None);
        assert_eq!(target("down", (0.0, 0.0), MAX, VIEW, false, true), None);
        assert_eq!(target("a", (0.0, 0.0), MAX, VIEW, true, true), None);
    }

    #[test]
    fn page_and_edge_keys_follow_the_owned_axis() {
        assert_eq!(
            target("pagedown", (0.0, 0.0), MAX, VIEW, true, false),
            Some((0.0, 87.5))
        );
        assert_eq!(
            target("end", (0.0, 0.0), MAX, VIEW, true, true),
            Some((0.0, 400.0))
        );
        assert_eq!(
            target("end", (0.0, 0.0), MAX, VIEW, false, true),
            Some((300.0, 0.0))
        );
        assert_eq!(
            target("home", (50.0, 70.0), MAX, VIEW, true, true),
            Some((50.0, 0.0))
        );
        assert_eq!(
            target("pagedown", (0.0, 0.0), MAX, VIEW, false, true),
            Some((175.0, 0.0))
        );
    }
}
