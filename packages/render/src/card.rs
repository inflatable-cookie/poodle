//! Card — surface container: variants, density spacing, media region.
//!
//! Contract: `docs/contracts/components/card.md`
//! Ported from: `packages/jetstream/components/src/card.rs`. First component
//! taking children — a `Vec<Node>`, slotted after the optional media region.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing, Node, NodeRole,
    ShadowLayer, StylePatch,
};
use poodle_specs::{CardLayout, CardSpec, CardVariant};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::rem_to_px;

pub fn card(spec: &CardSpec, theme: &dyn ThemeProvider, children: Vec<Node>) -> Node {
    let radius = theme.resolve_radius(spec.radius_token());
    let border_width = theme.resolve_space(spec.border_width_token());

    let gap = rem_to_px(spec.gap_rem());
    let padding_x = rem_to_px(spec.padding_x_rem());
    let padding_y = rem_to_px(spec.padding_y_rem());

    let panel = theme.resolve_color("color.background.panel");
    let elevated = theme.resolve_color("color.background.elevated");

    // Contract §8 fills.
    let fill = match spec.variant {
        CardVariant::Elevated => mix_srgb(elevated, panel, 0.98),
        _ => mix_srgb(panel, elevated, 0.10),
    };
    let hover_fill = mix_srgb(elevated, panel, 0.94);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.spacing.padding.left = padding_x;
        s.descriptor.layout.spacing.padding.right = padding_x;
        s.descriptor.layout.spacing.padding.top = padding_y;
        s.descriptor.layout.spacing.padding.bottom = padding_y;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.layout.direction = match spec.layout {
            CardLayout::Horizontal => LayoutDirection::Row,
            _ => LayoutDirection::Column,
        };
        // Border: selected wins with the accent + focus width.
        if let Some(selected_token) = spec.selected_border_token() {
            s.descriptor.border.width = theme.resolve_space("border.width.focus");
            s.descriptor.border.color = theme.resolve_color(selected_token);
        } else if let Some(border_token) = spec.border_token() {
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = theme.resolve_color(border_token);
        }
    }

    // Variant shadow stacks (contract §8/§10).
    let black = |a: f32| ColorValue(0.0, 0.0, 0.0, a);
    let mut shadows: Vec<ShadowLayer> = if matches!(spec.variant, CardVariant::Elevated) {
        let inv = theme.resolve_color("color.text.inverse");
        let bd = theme.resolve_color("color.border.default");
        vec![
            ShadowLayer {
                offset_x: 0.0,
                offset_y: rem_to_px(1.125),
                blur: rem_to_px(2.5),
                spread: 0.0,
                color: black(0.38),
                inset: false,
            },
            ShadowLayer {
                offset_x: 0.0,
                offset_y: rem_to_px(0.375),
                blur: rem_to_px(0.875),
                spread: 0.0,
                color: black(0.24),
                inset: false,
            },
            ShadowLayer {
                offset_x: 0.0,
                offset_y: rem_to_px(0.0625),
                blur: 0.0,
                spread: 0.0,
                color: with_alpha(inv, 0.10),
                inset: true,
            },
            ShadowLayer {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: rem_to_px(0.0625),
                color: with_alpha(bd, 0.12),
                inset: false,
            },
        ]
    } else {
        let bs = theme.resolve_color("color.border.subtle");
        vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.0625),
            color: with_alpha(bs, 0.18),
            inset: true,
        }]
    };
    if spec.selected_border_token().is_some() {
        let accent = theme.resolve_color("color.accent.base");
        shadows.push(ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.0625),
            color: with_alpha(accent, 0.5),
            inset: true,
        });
    }
    el.style.shadow_layers = shadows;

    if spec.is_interactive {
        el.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: None,
            text_color: None,
        });
        el.style.descriptor.cursor = CursorHint::Pointer;
    }

    // Media region: first child, overflow-clipped, inset radius; a fixed
    // 8rem leading column in horizontal layout.
    let mut children = children.into_iter();
    if spec.has_media {
        if let Some(media) = children.next() {
            let media_radius = (radius - rem_to_px(spec.media_radius_inset_rem())).max(0.0);
            let mut media_region = Node::container();
            {
                let s = &mut media_region.style;
                // Explicit Row (see switch.rs): the old tier got taffy's Row
                // default here too.
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
                s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
                s.descriptor.corner_radii.top_left = media_radius;
                s.descriptor.corner_radii.top_right = media_radius;
                s.descriptor.corner_radii.bottom_right = media_radius;
                s.descriptor.corner_radii.bottom_left = media_radius;
                if matches!(spec.layout, CardLayout::Horizontal) {
                    s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(8.0));
                }
            }
            el = el.child(media_region.child(media));
        }
    }
    for child in children {
        el = el.child(child);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    // Interactive cards are buttons; structural ones stay structure.
    if spec.is_interactive {
        el.a11y.role = Some(NodeRole::Button);
    }
    el
}
