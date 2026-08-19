//! Pill — chip/badge label across tones, appearances, sizes and typographies.
//!
//! Contract: `docs/contracts/components/pill.md`
//! Ported from: `packages/jetstream/components/src/pill.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutSizing,
    MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{InlineTypographyMode, PillAppearance, PillFont, PillSize, PillSpec, PillTone};

use crate::color::{hex_color, mix_srgb, solid_tone_surface, with_alpha, WHITE};
use crate::presentation::rem_to_px;

/// Per-size metrics in rem: `(min_w, min_h, pad_x, pad_y, font)`.
#[expect(
    clippy::approx_constant,
    reason = "0.7071 is a design-token font metric, not an approximation of FRAC_1_SQRT_2"
)]
fn pill_metrics(size: PillSize, typography: InlineTypographyMode) -> (f32, f32, f32, f32, f32) {
    match (typography, size) {
        (InlineTypographyMode::Inherit, PillSize::Xs) => (2.4444, 1.5556, 0.7778, 0.1111, 0.5786),
        (InlineTypographyMode::Inherit, PillSize::Sm) => (2.8571, 1.6, 0.8, 0.2, 0.6429),
        (InlineTypographyMode::Inherit, PillSize::Md) => (3.2727, 1.8182, 0.9091, 0.2727, 0.7071),
        (InlineTypographyMode::Inherit, PillSize::Lg) => (3.5833, 1.8333, 1.0, 0.3333, 0.7714),
        (InlineTypographyMode::Inherit, PillSize::Xl) => (3.9231, 1.8462, 1.1538, 0.3846, 0.8357),
        (InlineTypographyMode::Default, PillSize::Xs) => (2.125, 0.875, 0.4375, 0.0625, 0.5625),
        (InlineTypographyMode::Default, PillSize::Sm) => (2.5, 1.0, 0.5, 0.125, 0.625),
        (InlineTypographyMode::Default, PillSize::Md) => (2.875, 1.25, 0.625, 0.1875, 0.6875),
        (InlineTypographyMode::Default, PillSize::Lg) => (3.25, 1.375, 0.75, 0.25, 0.75),
        (InlineTypographyMode::Default, PillSize::Xl) => (3.625, 1.5, 0.9375, 0.3125, 0.8125),
    }
}

fn pill_colors(spec: &PillSpec, theme: &dyn ThemeProvider) -> (ColorValue, ColorValue, ColorValue) {
    let surface_bg = theme.resolve_color("color.background.surface");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let accent_base = theme.resolve_color("color.accent.base");
    let tone_color = theme.resolve_color(spec.tone_color_token());
    let custom_accent = spec.accent_color.as_deref().and_then(hex_color);

    if spec.is_solid_appearance() {
        let tone_base = custom_accent.unwrap_or(tone_color);
        let is_neutral = spec.tone == PillTone::Neutral && custom_accent.is_none();
        let surface = solid_tone_surface(theme, tone_base, is_neutral);
        return (surface.background, surface.border, surface.foreground);
    }

    let fill = match spec.appearance {
        PillAppearance::Badge => match spec.tone {
            PillTone::Neutral => mix_srgb(surface_bg, text_primary, 0.96),
            _ => with_alpha(accent_base, accent_base.3 * 0.18),
        },
        _ => match spec.tone {
            PillTone::Success | PillTone::Danger | PillTone::Info | PillTone::Warning => {
                mix_srgb(tone_color, surface_bg, 0.14)
            }
            PillTone::Neutral => with_alpha(surface_bg, surface_bg.3 * 0.9),
        },
    };
    let fill = if spec.appearance == PillAppearance::Subtle {
        with_alpha(fill, fill.3 * 0.5)
    } else {
        fill
    };

    let border = match spec.appearance {
        PillAppearance::Badge => ColorValue(0.0, 0.0, 0.0, 0.0),
        _ => match spec.tone {
            PillTone::Success | PillTone::Danger | PillTone::Info | PillTone::Warning => {
                // The old GPUI tier used `border_subtle.blend(tone.opacity(0.38))`.
                // GPUI's blend mixes RGB but deliberately preserves the base alpha;
                // `border-subtle` is translucent in dark themes.
                with_alpha(mix_srgb(tone_color, border_subtle, 0.38), border_subtle.3)
            }
            PillTone::Neutral => with_alpha(border_subtle, border_subtle.3 * 0.82),
        },
    };

    let text = match spec.appearance {
        PillAppearance::Badge => match spec.tone {
            PillTone::Neutral => text_secondary,
            _ => text_primary,
        },
        _ => theme.resolve_color(spec.text_color_token()),
    };

    // Custom accent (contract §8): a parseable hex overrides fill/border/text.
    // The slate base is a Svelte literal with no semantic token. Colour-space
    // note as with checkbox hex: the custom colour lands in sRGB and converts
    // at the backend edge — the old tier mixed raw values; pinned divergence.
    if let Some(accent) = custom_accent {
        let slate_08 = ColorValue(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0, 0.08);
        let slate_12 = ColorValue(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0, 0.12);
        return (
            mix_srgb(accent, slate_08, 0.18),
            mix_srgb(accent, slate_12, 0.30),
            mix_srgb(accent, WHITE, 0.88),
        );
    }

    (fill, border, text)
}

pub fn pill(spec: &PillSpec, theme: &dyn ThemeProvider) -> Node {
    pill_with_remove(spec, theme, None)
}

pub fn pill_with_remove(
    spec: &PillSpec,
    theme: &dyn ThemeProvider,
    on_remove: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let (min_w, min_h, pad_x, pad_y, font_size) =
        pill_metrics(spec.resolved_size(), spec.typography);
    let (fill, border, text_color) = pill_colors(spec, theme);

    let label = if spec.appearance == PillAppearance::Badge {
        spec.label.to_uppercase()
    } else {
        spec.label.clone()
    };
    let weight = if spec.appearance == PillAppearance::Badge {
        700
    } else {
        600
    };
    let radius = theme.resolve_radius("radius.pill");

    let mut el = Node::text(label);
    {
        let s = &mut el.style;
        s.min_width = Some(rem_to_px(min_w));
        s.min_height = Some(rem_to_px(min_h));
        s.descriptor.background = Some(fill);
        s.descriptor.text_color = Some(text_color);
        s.text_size = Some(rem_to_px(font_size));
        s.text_weight = Some(weight);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = rem_to_px(pad_x);
        pad.right = rem_to_px(pad_x);
        pad.top = rem_to_px(pad_y);
        pad.bottom = rem_to_px(pad_y);

        // Mono tracking is the most specific rule; badge carries 0.04em.
        if spec.font == PillFont::Mono {
            s.font_family = Some(FontFamily::Mono);
            s.letter_spacing_em = Some(0.02);
        } else if spec.appearance == PillAppearance::Badge {
            s.letter_spacing_em = Some(0.04);
        }

        if spec.is_muted {
            s.descriptor.opacity = 0.72;
        }
        if spec.is_disabled {
            s.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
            s.descriptor.cursor = CursorHint::NotAllowed;
        }

        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
    }
    if spec.has_dot {
        let dot_color = if spec.is_solid_appearance() {
            text_color
        } else {
            spec.accent_color
                .as_deref()
                .and_then(hex_color)
                .unwrap_or_else(|| theme.resolve_color(spec.tone_color_token()))
        };
        let dot_size = rem_to_px(font_size * 0.5);
        let mut dot = Node::container();
        dot.style.descriptor.layout.width = LayoutSizing::Fixed(dot_size);
        dot.style.descriptor.layout.height = LayoutSizing::Fixed(dot_size);
        dot.style.descriptor.corner_radii.top_left = 999.0;
        dot.style.descriptor.corner_radii.top_right = 999.0;
        dot.style.descriptor.corner_radii.bottom_right = 999.0;
        dot.style.descriptor.corner_radii.bottom_left = 999.0;
        dot.style.descriptor.background = Some(dot_color);
        dot.style.flex_shrink_zero = true;
        el = el.child(dot);
    }

    if spec.is_removable {
        let mut remove = Node::container();
        remove.id = Some("poodle-pill-remove".to_string());
        remove.a11y.role = Some(NodeRole::Button);
        remove.a11y.label = Some(format!("Remove {}", spec.label));
        remove.style.descriptor.layout.direction = LayoutDirection::Row;
        remove.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        remove.style.descriptor.cursor = CursorHint::Pointer;

        let mut icon = Node::icon("x", theme.resolve_space("size.icon.sm"));
        icon.style.descriptor.text_color = Some(if spec.is_solid_appearance() {
            text_color
        } else {
            theme.resolve_color("color.icon.muted")
        });
        let mut remove = remove.child(icon);
        if let Some(handler) = on_remove {
            remove.interaction.on_activate = Some(handler);
        }
        el = el.child(remove);
    }

    if let Some(aria) = spec.aria_label.as_deref() {
        el.a11y.label = Some(aria.to_string());
    }
    el
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use poodle_node::NodeKind;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn removable_pill_exposes_and_runs_its_remove_action() {
        let removes = Arc::new(Mutex::new(0));
        let sink = Arc::clone(&removes);
        let spec = PillSpec::new().with_label("Filter").with_removable(true);

        let node = pill_with_remove(
            &spec,
            &theme(),
            Some(Arc::new(move || *sink.lock().unwrap() += 1)),
        );
        let remove = node
            .find(&|child| child.id.as_deref() == Some("poodle-pill-remove"))
            .expect("remove action");

        assert_eq!(remove.a11y.role, Some(NodeRole::Button));
        assert_eq!(remove.a11y.label.as_deref(), Some("Remove Filter"));
        (remove
            .interaction
            .on_activate
            .as_ref()
            .expect("activatable"))();
        assert_eq!(*removes.lock().unwrap(), 1);
    }

    #[test]
    fn default_appearance_is_tint_and_matches_explicit_tint() {
        let theme = theme();
        let default_spec = PillSpec::new().with_label("Neutral");
        assert_eq!(default_spec.appearance, PillAppearance::Tint);
        let explicit_tint = PillSpec::new()
            .with_label("Neutral")
            .with_appearance(PillAppearance::Tint);
        assert_eq!(
            pill_colors(&default_spec, &theme),
            pill_colors(&explicit_tint, &theme)
        );

        // Solid has its own exact recipe coverage below. Do not require every
        // resolved channel to differ: a theme may legitimately make one tint
        // channel meet the solid midpoint.
    }

    #[test]
    fn subtle_appearance_halves_the_tint_fill_opacity() {
        let theme = theme();
        let tint = PillSpec::new()
            .with_tone(PillTone::Success)
            .with_appearance(PillAppearance::Tint);
        let subtle = PillSpec::new()
            .with_tone(PillTone::Success)
            .with_appearance(PillAppearance::Subtle);
        let (tint_fill, tint_border, tint_text) = pill_colors(&tint, &theme);
        let (subtle_fill, subtle_border, subtle_text) = pill_colors(&subtle, &theme);
        assert_eq!(
            subtle_fill,
            with_alpha(tint_fill, tint_fill.3 * 0.5),
            "subtle halves the tint fill alpha"
        );
        assert_eq!(subtle_border, tint_border);
        assert_eq!(subtle_text, tint_text);
    }

    #[test]
    fn solid_appearance_uses_shared_opaque_recipe_and_custom_accent_base() {
        let theme = theme();
        let success = PillSpec::new()
            .with_tone(PillTone::Success)
            .with_appearance(PillAppearance::Solid);
        let expected = solid_tone_surface(
            &theme,
            theme.resolve_color(success.tone_color_token()),
            false,
        );
        assert_eq!(
            pill_colors(&success, &theme),
            (expected.background, expected.border, expected.foreground)
        );

        let custom = PillSpec::new()
            .with_appearance(PillAppearance::Solid)
            .with_accent_color("#ff9900");
        let custom_base = hex_color("#ff9900").expect("custom accent");
        let custom_expected = solid_tone_surface(&theme, custom_base, false);
        assert_eq!(
            pill_colors(&custom, &theme),
            (
                custom_expected.background,
                custom_expected.border,
                custom_expected.foreground
            )
        );

        let neutral = PillSpec::new().with_appearance(PillAppearance::Solid);
        let neutral_expected = solid_tone_surface(
            &theme,
            theme.resolve_color(neutral.tone_color_token()),
            true,
        );
        assert_eq!(
            pill_colors(&neutral, &theme),
            (
                neutral_expected.background,
                neutral_expected.border,
                neutral_expected.foreground
            )
        );
    }

    #[test]
    fn solid_dot_and_remove_affordance_use_primary_foreground() {
        let theme = theme();
        let mut spec = PillSpec::new()
            .with_label("Filter")
            .with_appearance(PillAppearance::Solid)
            .with_removable(true);
        spec.has_dot = true;
        let expected =
            solid_tone_surface(&theme, theme.resolve_color(spec.tone_color_token()), true);
        let node = pill_with_remove(&spec, &theme, None);

        let dot = node.children.first().expect("solid dot");
        assert_eq!(dot.style.descriptor.background, Some(expected.foreground));
        let remove = node
            .find(&|n| n.id.as_deref() == Some("poodle-pill-remove"))
            .expect("remove affordance");
        let icon = remove
            .find(&|n| matches!(&n.kind, NodeKind::Icon { name, .. } if name == "x"))
            .expect("remove icon");
        assert_eq!(icon.style.descriptor.text_color, Some(expected.foreground));
    }
}
