//! Card — surface container: variants, density spacing, media region.
//!
//! Contract: `docs/contracts/components/card.md`
//! Ported from: `packages/jetstream/components/src/card.rs`; paint reconciled
//! to the old GPUI tier in g12.019 so composed card families keep their native
//! border and shadow recipes.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CursorHint, LayoutDirection, LayoutOverflow, Node, NodeRole, ShadowLayer,
    StylePatch,
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
    let border_subtle = theme.resolve_color("color.border.subtle");
    let border_default = theme.resolve_color("color.border.default");

    // Contract §8 fills.
    let fill = match spec.variant {
        CardVariant::Elevated => mix_srgb(elevated, panel, 0.98),
        _ => mix_srgb(panel, elevated, 0.10),
    };
    let hover_fill = mix_srgb(elevated, panel, 0.94);

    let border_color = match spec.variant {
        CardVariant::Default => with_alpha(border_subtle, border_subtle.3 * 0.18),
        CardVariant::Outlined => with_alpha(border_default, border_default.3 * 0.76),
        CardVariant::Elevated => border_default,
    };

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
        // The old GPUI tier keeps the 1px root border when selected; the
        // accent ring lives in the shadow stack instead of inflating layout.
        s.descriptor.border.width = border_width;
        if let Some(selected_token) = spec.selected_border_token() {
            s.descriptor.border.color = theme.resolve_color(selected_token);
        } else {
            s.descriptor.border.color = border_color;
        }
    }

    // Variant shadow stacks, transcribed from the old GPUI tier. GPUI 0.2.2
    // has no inset flag, so that tier painted its nominal inset/ring layers
    // as ordinary BoxShadows; mark them non-inset so the backend does the
    // same instead of dropping them.
    let black = |a: f32| ColorValue(0.0, 0.0, 0.0, a);
    let ring = rem_to_px(0.0625);
    let shadows: Vec<ShadowLayer> = if let Some(selected_token) = spec.selected_border_token() {
        let selected = theme.resolve_color(selected_token);
        vec![
            ShadowLayer {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: ring,
                color: selected,
                inset: false,
            },
            ShadowLayer {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: ring,
                color: with_alpha(selected, selected.3 * 0.12),
                inset: false,
            },
        ]
    } else if matches!(spec.variant, CardVariant::Elevated) {
        let canvas = theme.resolve_color("color.background.canvas");
        let lightness =
            (canvas.0.max(canvas.1).max(canvas.2) + canvas.0.min(canvas.1).min(canvas.2)) * 0.5;
        if lightness > 0.5 {
            let slate = |a: f32| ColorValue(49.0 / 255.0, 66.0 / 255.0, 85.0 / 255.0, a);
            vec![
                ShadowLayer {
                    offset_x: 0.0,
                    offset_y: rem_to_px(0.875),
                    blur: rem_to_px(1.75),
                    spread: 0.0,
                    color: slate(0.10),
                    inset: false,
                },
                ShadowLayer {
                    offset_x: 0.0,
                    offset_y: rem_to_px(0.25),
                    blur: rem_to_px(0.625),
                    spread: 0.0,
                    color: slate(0.06),
                    inset: false,
                },
                ShadowLayer {
                    offset_x: 0.0,
                    offset_y: ring,
                    blur: 0.0,
                    spread: 0.0,
                    color: ColorValue(1.0, 1.0, 1.0, 0.72),
                    inset: false,
                },
                ShadowLayer {
                    offset_x: 0.0,
                    offset_y: 0.0,
                    blur: 0.0,
                    spread: ring,
                    color: with_alpha(border_default, border_default.3 * 0.10),
                    inset: false,
                },
            ]
        } else {
            let inverse = theme.resolve_color("color.text.inverse");
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
                    offset_y: ring,
                    blur: 0.0,
                    spread: 0.0,
                    color: with_alpha(inverse, inverse.3 * 0.10),
                    inset: false,
                },
                ShadowLayer {
                    offset_x: 0.0,
                    offset_y: 0.0,
                    blur: 0.0,
                    spread: ring,
                    color: with_alpha(border_default, border_default.3 * 0.12),
                    inset: false,
                },
            ]
        }
    } else {
        vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: ring,
            color: with_alpha(border_subtle, border_subtle.3 * 0.18),
            inset: false,
        }]
    };
    el.style.shadow_layers = shadows;

    if spec.is_interactive {
        el.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: None,
            text_color: None,
            opacity: None,
        });
        el.style.descriptor.cursor = CursorHint::Pointer;
    }

    // Media region: first child, overflow-clipped, inset radius. The media
    // child owns its dimensions; the old GPUI tier did not impose an 8rem
    // horizontal column on the wrapper.
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
                s.flex_shrink_zero = true;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn default_card_matches_the_old_tiers_border_and_ring() {
        let theme = theme();
        let subtle = theme.resolve_color("color.border.subtle");
        let node = card(&CardSpec::new(), &theme, vec![]);
        assert_eq!(node.style.descriptor.border.width, 1.0);
        assert_eq!(
            node.style.descriptor.border.color,
            with_alpha(subtle, subtle.3 * 0.18)
        );
        assert_eq!(node.style.shadow_layers.len(), 1);
        assert_eq!(node.style.shadow_layers[0].spread, 1.0);
        assert!(!node.style.shadow_layers[0].inset);
    }

    #[test]
    fn selected_card_keeps_a_one_pixel_border_and_two_accent_rings() {
        let theme = theme();
        let accent = theme.resolve_color("color.accent.base");
        let node = card(&CardSpec::new().interactive().selected(), &theme, vec![]);
        assert_eq!(node.style.descriptor.border.width, 1.0);
        assert_eq!(node.style.descriptor.border.color, accent);
        assert_eq!(node.style.shadow_layers.len(), 2);
        assert!(node.style.shadow_layers.iter().all(|layer| !layer.inset));
        assert_eq!(node.style.shadow_layers[0].color, accent);
        assert_eq!(
            node.style.shadow_layers[1].color,
            with_alpha(accent, accent.3 * 0.12)
        );
    }
}
