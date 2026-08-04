//! Badge — small status label.
//!
//! Contract: `docs/contracts/components/badge.md`
//! Ported from: `packages/jetstream/components/src/badge.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{BadgeSpec, BadgeVariant};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::rem_to_px;

pub fn badge(spec: &BadgeSpec, theme: &dyn ThemeProvider) -> Node {
    let content = spec.content.clone().unwrap_or_default();

    // Contract colour rules per variant:
    // - accent: bg = color-mix(accent-base 18%, transparent), text = text-primary
    // - muted: bg = color-mix(surface 78%, elevated), text = text-secondary
    let (bg, text_color) = match spec.variant {
        BadgeVariant::Accent => {
            let accent = theme.resolve_color("color.accent.base");
            let text = theme.resolve_color("color.text.primary");
            (with_alpha(accent, accent.3 * 0.18), text)
        }
        BadgeVariant::Muted => {
            let surface = theme.resolve_color("color.background.surface");
            let elevated = theme.resolve_color("color.background.elevated");
            let text = theme.resolve_color("color.text.secondary");
            (mix_srgb(surface, elevated, 0.78), text)
        }
    };

    let mut el = Node::text(content);
    {
        let s = &mut el.style;
        s.min_height = Some(rem_to_px(1.25));
        s.descriptor.layout.spacing.padding.left = rem_to_px(0.4375);
        s.descriptor.layout.spacing.padding.right = rem_to_px(0.4375);
        s.descriptor.layout.spacing.padding.top = rem_to_px(0.125);
        s.descriptor.layout.spacing.padding.bottom = rem_to_px(0.125);
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
        s.descriptor.background = Some(bg);
        s.descriptor.text_color = Some(text_color);
        s.text_size = Some(rem_to_px(0.6875));
        s.letter_spacing_em = Some(0.04);
        // Explicit Row (see switch.rs): the old tier got taffy's Row default.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
    // text-transform: uppercase stays a caller concern, as in the old tier:
    // content should be pre-uppercased.
}
