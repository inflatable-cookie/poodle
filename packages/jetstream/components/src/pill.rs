//! Pill — Jetstream pill/chip component backed by PillSpec.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{InlineTypographyMode, PillAppearance, PillSize, PillSpec, PillTone};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

fn pill_metrics(size: PillSize, typography: InlineTypographyMode) -> (f32, f32, f32, f32) {
    match (typography, size) {
        (InlineTypographyMode::Inherit, PillSize::Xs) => (1.5556, 0.5556, 0.1111, 0.6429),
        (InlineTypographyMode::Inherit, PillSize::Sm) => (1.6, 0.6, 0.2, 0.7143),
        (InlineTypographyMode::Inherit, PillSize::Md) => (1.8182, 0.7273, 0.2727, 0.7857),
        (InlineTypographyMode::Inherit, PillSize::Lg) => (1.8333, 0.8333, 0.3333, 0.8571),
        (InlineTypographyMode::Inherit, PillSize::Xl) => (1.8462, 0.9231, 0.3846, 0.9286),
        (InlineTypographyMode::Default, PillSize::Xs) => (0.875, 0.3125, 0.0625, 0.5625),
        (InlineTypographyMode::Default, PillSize::Sm) => (1.0, 0.375, 0.125, 0.625),
        (InlineTypographyMode::Default, PillSize::Md) => (1.25, 0.5, 0.1875, 0.6875),
        (InlineTypographyMode::Default, PillSize::Lg) => (1.375, 0.625, 0.25, 0.75),
        (InlineTypographyMode::Default, PillSize::Xl) => (1.5, 0.75, 0.3125, 0.8125),
    }
}

fn pill_colors(spec: &PillSpec, theme: &JetstreamThemeProvider) -> (Color, Color, Color) {
    let surface_bg: Color = resolve_color(theme, "color.background.surface").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let text_secondary: Color = resolve_color(theme, "color.text.secondary").into();
    let border_subtle: Color = resolve_color(theme, "color.border.subtle").into();
    let accent_base: Color = resolve_color(theme, "color.accent.base").into();
    let tone_color: Color = resolve_color(theme, spec.tone_color_token()).into();

    let fill = match spec.appearance {
        PillAppearance::Badge => match spec.tone {
            PillTone::Neutral => surface_bg.mix(text_primary, 0.96),
            _ => accent_base.with_alpha(accent_base.a * 0.18),
        },
        _ => match spec.tone {
            PillTone::Success | PillTone::Danger | PillTone::Info | PillTone::Warning => {
                tone_color.mix(surface_bg, 0.14)
            }
            PillTone::Neutral => surface_bg.with_alpha(surface_bg.a * 0.9),
        },
    };

    let fill = if spec.appearance == PillAppearance::Subtle {
        fill.with_alpha(fill.a * 0.5)
    } else {
        fill
    };

    let border = match spec.appearance {
        PillAppearance::Badge => Color::TRANSPARENT,
        _ => match spec.tone {
            PillTone::Success | PillTone::Danger | PillTone::Info | PillTone::Warning => {
                tone_color.mix(border_subtle, 0.38)
            }
            PillTone::Neutral => border_subtle.with_alpha(border_subtle.a * 0.82),
        },
    };

    let text = match spec.appearance {
        PillAppearance::Badge => match spec.tone {
            PillTone::Neutral => text_secondary,
            _ => text_primary,
        },
        _ => resolve_color(theme, spec.text_color_token()).into(),
    };

    (fill, border, text)
}

pub fn js_pill(spec: &PillSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let (min_h, pad_x, pad_y, font_size) = pill_metrics(spec.size, spec.typography);
    let (fill, border, text_color) = pill_colors(spec, theme);
    // Jetstream's current text API does not expose font-family or letter-spacing,
    // so `font="mono"` remains a documented runtime delta for now.
    let label = if spec.appearance == PillAppearance::Badge {
        spec.label.to_uppercase()
    } else {
        spec.label.clone()
    };
    let weight = if spec.appearance == PillAppearance::Badge { 700 } else { 600 };
    let radius = resolve_radius(theme, "radius.pill");

    let mut el = ui_element::label(&label)
        .min_h(rem_to_px(min_h))
        .bg(fill)
        .text_color(text_color)
        .text_size(rem_to_px(font_size))
        .text_weight(weight)
        .rounded(radius)
        .border(1.0)
        .border_color(border)
        .pl(rem_to_px(pad_x)).pr(rem_to_px(pad_x))
        .pt(rem_to_px(pad_y)).pb(rem_to_px(pad_y));

    if spec.is_muted {
        el = el.opacity(0.72);
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{InlineTypographyMode, PillAppearance, PillSize, PillTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn md_pill_uses_contract_default_height() {
        let el = js_pill(&PillSpec::new().with_size(PillSize::Md), &theme());
        assert_eq!(el.layout.min_size.height, taffy::Dimension::length(20.0));
    }

    #[test]
    fn inherit_md_pill_uses_proportional_height() {
        let el = js_pill(
            &PillSpec::new()
                .with_size(PillSize::Md)
                .with_typography(InlineTypographyMode::Inherit),
            &theme(),
        );
        assert_eq!(el.layout.min_size.height, taffy::Dimension::length(rem_to_px(1.8182)));
    }

    #[test]
    fn badge_uses_transparent_border() {
        let el = js_pill(
            &PillSpec::new()
                .with_appearance(PillAppearance::Badge)
                .with_tone(PillTone::Info),
            &theme(),
        );
        assert_eq!(el.style.border_color, Some(Color::TRANSPARENT));
    }

    #[test]
    fn neutral_badge_uses_secondary_text() {
        let theme = theme();
        let el = js_pill(
            &PillSpec::new()
                .with_appearance(PillAppearance::Badge)
                .with_tone(PillTone::Neutral),
            &theme,
        );
        let expected: Color = resolve_color(&theme, "color.text.secondary").into();
        assert_eq!(el.style.text_color, Some(expected));
    }
}
