//! Tooltip — the bubble only; anchoring and open/dismiss are host concerns.
//!
//! Contract: `docs/contracts/components/tooltip.md` §8.
//! Ported from: `packages/jetstream/components/src/tooltip.rs`.

use poodle_node::{LayoutDirection, Node, NodeRole, ShadowValue};
use poodle_specs::TooltipSpec;

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn tooltip(spec: &TooltipSpec, ctx: &RenderContext<'_>) -> Node {
    let text_color = ctx.theme().resolve_color("color.text.primary");

    // Contract §8 background = color-mix(elevated 98%, panel).
    let elevated = ctx.theme().resolve_color("color.background.elevated");
    let panel = ctx.theme().resolve_color("color.background.panel");
    let fill = mix_srgb(elevated, panel, 0.98);

    // Contract §8 border = border-default at 72% of its own alpha.
    let border_default = ctx.theme().resolve_color("color.border.default");
    let border_color = with_alpha(border_default, border_default.3 * 0.72);
    let border_width = rem_to_px(0.0625);

    // Contract §8 radius = radius.control − the spec's inset.
    let radius = ctx.theme().resolve_radius("radius.control") - rem_to_px(spec.radius_inset_rem());

    let content = spec.content.as_deref().unwrap_or("");
    let pad_x = rem_to_px(spec.padding_x_rem());
    let pad_y = rem_to_px(spec.padding_y_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let max_w = rem_to_px(spec.max_width_rem());

    let mut bubble = Node::container();
    {
        let s = &mut bubble.style;
        s.descriptor.background = Some(fill);
        // Explicit Row (see switch.rs): the old tier got taffy's Row default,
        // which is what lets the label size to its text instead of stretching.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_color;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.max_width = Some(max_w);
        s.no_wrap = true;
        s.overlay = true;
        // Contract's two-layer shadow: the backend carries one layer, so the
        // dominant first layer renders and the secondary is dropped — same
        // accepted delta as the old tier, same place.
        s.descriptor.shadow = Some(ShadowValue {
            offset_x: 0.0,
            offset_y: rem_to_px(0.5),
            blur: rem_to_px(1.25),
            color: poodle_node::ColorValue(0.0, 0.0, 0.0, 0.30),
        });
    }
    let mut label = Node::text(content);
    label.style.descriptor.text_color = Some(text_color);
    label.style.text_size = Some(font_size);
    bubble = bubble.child(label);

    if let Some(aria) = spec.aria_label.as_deref() {
        bubble.a11y.label = Some(aria.to_string());
    }
    bubble.a11y.role = Some(NodeRole::Tooltip);
    bubble
}
