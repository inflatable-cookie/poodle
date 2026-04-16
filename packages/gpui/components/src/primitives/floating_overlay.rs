//! Floating overlay utility.
//!
//! Positions a surface element absolutely relative to an anchor according to
//! an `OverlayPlacement`. The overlay is taken out of normal document flow so
//! it does not push surrounding layout.
//!
//! # Positioning model
//!
//! GPUI's build phase runs before prepaint, so the anchor's layout bounds are
//! not available during the same frame that builds the overlay. This helper uses
//! caller-supplied dimension estimates (`anchor_h`, `anchor_w`) so placement is
//! stable without cross-frame state. For typical button-triggered overlays,
//! pass `px(rem_to_px(control_height_rem(size)))` for `anchor_h`. The `anchor_w`
//! estimate is used for left/right families; bottom/top families only need
//! `anchor_h` and the estimate does not affect their correctness.

use gpui::*;
use poodle_specs::OverlayPlacement;

/// Gap between the anchor edge and the floating surface (px).
const OVERLAY_GAP_PX: f32 = 4.0;

/// Wrap `anchor` in a `relative`-positioned container and, when `surface` is
/// `Some`, position it absolutely according to `placement`.
///
/// - `anchor_h`: estimated anchor height — used for top/bottom placement families.
/// - `anchor_w`: estimated anchor width — used for left/right placement families.
///
/// The surface renders above the normal document flow and does not affect the
/// layout of surrounding elements.
pub fn floating_overlay(
    anchor: AnyElement,
    surface: Option<AnyElement>,
    placement: OverlayPlacement,
    anchor_h: Pixels,
    anchor_w: Pixels,
) -> AnyElement {
    let gap = px(OVERLAY_GAP_PX);

    let mut wrapper = div().relative().flex_shrink_0().child(anchor);

    if let Some(surface_el) = surface {
        // Each arm positions the surface relative to the anchor. The containing
        // block is the `relative` wrapper whose size equals the in-flow anchor.
        let surface_container = match placement {
            // ── Bottom family ─────────────────────────────────────────────────
            // Surface top = anchor bottom + gap
            OverlayPlacement::Bottom | OverlayPlacement::BottomStart => div()
                .absolute()
                .top(anchor_h + gap)
                .left(px(0.0))
                .child(surface_el),
            OverlayPlacement::BottomEnd => div()
                .absolute()
                .top(anchor_h + gap)
                .right(px(0.0))
                .child(surface_el),

            // ── Top family ────────────────────────────────────────────────────
            // Surface bottom = anchor top − gap (bottom measured from wrapper bottom,
            // which equals anchor_h, so bottom(anchor_h + gap) → surface bottom at −gap
            // relative to wrapper top → gap above the anchor).
            OverlayPlacement::Top | OverlayPlacement::TopStart => div()
                .absolute()
                .bottom(anchor_h + gap)
                .left(px(0.0))
                .child(surface_el),
            OverlayPlacement::TopEnd => div()
                .absolute()
                .bottom(anchor_h + gap)
                .right(px(0.0))
                .child(surface_el),

            // ── Right family ──────────────────────────────────────────────────
            // Surface left = anchor right + gap.  anchor_w is an estimate; accurate
            // when the anchor is square (icon button) or close to the token width.
            OverlayPlacement::Right | OverlayPlacement::RightStart => div()
                .absolute()
                .left(anchor_w + gap)
                .top(px(0.0))
                .child(surface_el),
            OverlayPlacement::RightEnd => div()
                .absolute()
                .left(anchor_w + gap)
                .bottom(px(0.0))
                .child(surface_el),

            // ── Left family ───────────────────────────────────────────────────
            // right(anchor_w + gap): surface right edge at (wrapper.width − anchor_w − gap).
            // Accurate when wrapper.width ≈ anchor_w (square anchors). For wider
            // anchors the gap is proportionally larger; visual offset is still ≥ gap.
            OverlayPlacement::Left | OverlayPlacement::LeftStart => div()
                .absolute()
                .right(anchor_w + gap)
                .top(px(0.0))
                .child(surface_el),
            OverlayPlacement::LeftEnd => div()
                .absolute()
                .right(anchor_w + gap)
                .bottom(px(0.0))
                .child(surface_el),
        };
        wrapper = wrapper.child(surface_container);
    }

    wrapper.into_any_element()
}
