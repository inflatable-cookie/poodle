//! Inset (inner) shadow layers, projected without a `BoxShadow.inset` flag.
//!
//! `gpui::BoxShadow` on crates.io 0.2.2 carries no `inset` field, so an inset
//! layer cannot ride the ordinary shadow refinement. g15.045 got one from a
//! Zed fork; g16.005 restored the published crate identity, and dropping the
//! layers with it would silently degrade Accordion, ActionDiscoveryPanel,
//! ListCard, Popover, and Tabs. So the backend paints them itself.
//!
//! Every inset layer Poodle declares has `blur == 0`, which makes the CSS
//! definition exactly a solid band inside the padding box:
//!
//! - the shadow shape is the padding box, offset by `(dx, dy)` and shrunk by
//!   `spread` on every side;
//! - the painted region is the padding box MINUS that shape, clipped to it.
//!
//! For zero blur that region is a per-side band, and the widths fall straight
//! out of the geometry (`band_widths`). One `PaintQuad` with per-side border
//! widths and the element's INNER corner radii paints it exactly — the same
//! mechanism, and the same `canvas` seam, the g15.052 focus ring already uses.
//!
//! Two shapes cover all current usage, and both are exact here:
//!
//! - an inner ring (`offset 0,0`, `spread S`) — Tabs drop target, ListCard
//!   highlighted, ActionDiscoveryPanel active;
//! - an edge band (`offset (0, D)` or `(W, 0)`, `spread 0`) — the Popover and
//!   Accordion top highlights, and the ListCard active leading bar.
//!
//! A blurred inset layer would not be exact. Nothing declares one; if one ever
//! appears, its band is painted at full opacity and
//! `surface.extended.shadow-inset-blur-approximated` records that the blur was
//! not honoured, so the approximation is visible in probe evidence rather than
//! being folklore.

use gpui::{ParentElement, Pixels, Styled, px};
use poodle_node::{Node, ShadowLayer};

use super::{PaintedInsetShadow, color, record_painted_inset_shadows, record_probe_channel};

/// Per-side band widths, in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Bands {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Bands {
    fn is_empty(self) -> bool {
        self.left <= 0.0 && self.right <= 0.0 && self.top <= 0.0 && self.bottom <= 0.0
    }
}

/// The band widths a zero-blur CSS inset shadow paints inside the padding box.
///
/// Derived from the shadow rect (`box + offset - spread`) rather than special
/// cased per shape, so a combined offset-and-spread layer is right too.
pub(crate) fn band_widths(layer: &ShadowLayer) -> Bands {
    Bands {
        left: (layer.spread + layer.offset_x).max(0.0),
        right: (layer.spread - layer.offset_x).max(0.0),
        top: (layer.spread + layer.offset_y).max(0.0),
        bottom: (layer.spread - layer.offset_y).max(0.0),
    }
}

/// CSS shrinks each corner radius by the border it sits inside. Poodle's
/// borders are uniform in practice, but per-side widths exist, so take the
/// larger of the two edges meeting at the corner.
fn inner_radius(radius: f32, a: f32, b: f32) -> Pixels {
    px((radius - a.max(b)).max(0.0))
}

/// Prepend the inset-shadow painter, if this node declares any inset layer.
///
/// Prepended rather than appended so the bands paint UNDER the node's own
/// children, which is where CSS puts an inset shadow: after the background,
/// before the content.
pub(super) fn apply<E: ParentElement + 'static>(mut el: E, node: &Node, id: &str) -> E {
    let layers: Vec<ShadowLayer> = node
        .style
        .shadow_layers
        .iter()
        .filter(|layer| layer.inset)
        .copied()
        .collect();
    if layers.is_empty() {
        return el;
    }
    record_probe_channel("surface.extended.shadow-inset");
    if layers.iter().any(|layer| layer.blur > 0.0) {
        record_probe_channel("surface.extended.shadow-inset-blur-approximated");
    }

    let border = &node.style.descriptor.border;
    let border_left = node.style.border_left_width.unwrap_or(border.width);
    let border_right = node.style.border_right_width.unwrap_or(border.width);
    let border_top = node.style.border_top_width.unwrap_or(border.width);
    let border_bottom = node.style.border_bottom_width.unwrap_or(border.width);
    let radii = node.style.descriptor.corner_radii;
    let recorded_id = id.to_owned();

    el = el.child(gpui::canvas(
        move |_, _, _| {},
        move |bounds, (), window, _cx| {
            let mut painted: Vec<PaintedInsetShadow> = Vec::new();
            // The canvas is anchored at the element's top-left inset, so its
            // bounds ARE the padding box — exactly the box a CSS inset shadow
            // is clipped to.
            for layer in &layers {
                let bands = band_widths(layer);
                if bands.is_empty() {
                    continue;
                }
                window.paint_quad(
                    gpui::PaintQuad {
                        bounds,
                        corner_radii: gpui::Corners {
                            top_left: inner_radius(radii.top_left, border_left, border_top),
                            top_right: inner_radius(radii.top_right, border_right, border_top),
                            bottom_right: inner_radius(
                                radii.bottom_right,
                                border_right,
                                border_bottom,
                            ),
                            bottom_left: inner_radius(radii.bottom_left, border_left, border_bottom),
                        },
                        background: gpui::transparent_black().into(),
                        border_widths: gpui::Edges {
                            top: px(bands.top),
                            right: px(bands.right),
                            bottom: px(bands.bottom),
                            left: px(bands.left),
                        },
                        border_color: color(layer.color),
                        border_style: gpui::BorderStyle::default(),
                    },
                );
                painted.push(PaintedInsetShadow {
                    left: bands.left,
                    right: bands.right,
                    top: bands.top,
                    bottom: bands.bottom,
                    color: layer.color,
                    bounds: [
                        f32::from(bounds.origin.x),
                        f32::from(bounds.origin.y),
                        f32::from(bounds.size.width),
                        f32::from(bounds.size.height),
                    ],
                });
            }
            // Written only from the real paint pass, so an assertion against
            // this is evidence that bands were emitted — not that a style was
            // declared.
            record_painted_inset_shadows(&recorded_id, painted);
        },
    )
    // Anchored exactly like the g15.052 ring canvas: an absolute child that
    // is NOT anchored sits at its justify-static position, so the explicit
    // top/left plus `size_full` is what makes these bounds the padding box.
    .absolute()
    .top(px(0.0))
    .left(px(0.0))
    .size_full());
    el
}
