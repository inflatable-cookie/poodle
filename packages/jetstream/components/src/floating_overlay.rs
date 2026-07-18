//! Floating overlay utility — positions a surface element relative to an anchor.
//!
//! Contract: positioning utility for popover/select/tooltip-like overlays.
//! Reference: `packages/gpui/components/src/primitives/floating_overlay.rs`
//!
//! Uses Taffy absolute positioning (`.relative()` wrapper + `.absolute()` surface).
//! The surface is taken out of flow and positioned using `anchor_h` / `anchor_w`
//! estimates supplied by the caller.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_specs::OverlayPlacement;

/// Gap between anchor and floating surface (px).
const OVERLAY_GAP_PX: f32 = 4.0;

/// Wrap `anchor` in a relative container and, when `surface` is `Some`,
/// position it absolutely according to `placement`.
///
/// `anchor_h`: estimated anchor height (used for top/bottom families).
/// `anchor_w`: estimated anchor width (used for left/right families).
pub fn js_floating_overlay(
    anchor: JsEl,
    surface: Option<JsEl>,
    placement: OverlayPlacement,
    anchor_h: f32,
    anchor_w: f32,
) -> JsEl {
    let gap = OVERLAY_GAP_PX;

    let mut wrapper = ui_element::div()
        .relative()
        .flex_shrink_0()
        .child(anchor);

    if let Some(surface_el) = surface {
        let surface_container = match placement {
            // ── Bottom family ────────────────────────────────────────
            OverlayPlacement::Bottom | OverlayPlacement::BottomStart => ui_element::div()
                .absolute()
                .top(anchor_h + gap)
                .left(0.0)
                .child(surface_el),
            OverlayPlacement::BottomEnd => ui_element::div()
                .absolute()
                .top(anchor_h + gap)
                .right(0.0)
                .child(surface_el),

            // ── Top family ───────────────────────────────────────────
            OverlayPlacement::Top | OverlayPlacement::TopStart => ui_element::div()
                .absolute()
                .bottom(anchor_h + gap)
                .left(0.0)
                .child(surface_el),
            OverlayPlacement::TopEnd => ui_element::div()
                .absolute()
                .bottom(anchor_h + gap)
                .right(0.0)
                .child(surface_el),

            // ── Right family ─────────────────────────────────────────
            OverlayPlacement::Right | OverlayPlacement::RightStart => ui_element::div()
                .absolute()
                .left(anchor_w + gap)
                .top(0.0)
                .child(surface_el),
            OverlayPlacement::RightEnd => ui_element::div()
                .absolute()
                .left(anchor_w + gap)
                .bottom(0.0)
                .child(surface_el),

            // ── Left family ──────────────────────────────────────────
            OverlayPlacement::Left | OverlayPlacement::LeftStart => ui_element::div()
                .absolute()
                .right(anchor_w + gap)
                .top(0.0)
                .child(surface_el),
            OverlayPlacement::LeftEnd => ui_element::div()
                .absolute()
                .right(anchor_w + gap)
                .bottom(0.0)
                .child(surface_el),
        };
        wrapper = wrapper.child(surface_container);
    }

    wrapper
}
