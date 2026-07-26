//! Pill — Jetstream pill/chip component backed by PillSpec.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{InlineTypographyMode, PillAppearance, PillFont, PillSize, PillSpec, PillTone};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Per-size metrics in rem: `(min_w, min_h, pad_x, pad_y, font)`. Matches contract §8 /
/// Svelte `Pill.svelte`: pad-x uses the Svelte-rendered scale (md `0.625rem`, +0.125rem
/// wider than the old contract `0.5rem`); `min_w` is the per-size floor; inherit font
/// uses the corrected `em` table.
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

fn pill_colors(spec: &PillSpec, theme: &JetstreamThemeProvider) -> (Color, Color, Color) {
    let surface_bg: Color = resolve_color(theme, "color.background.surface").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let text_secondary: Color = resolve_color(theme, "color.text.secondary").into();
    let border_subtle: Color = resolve_color(theme, "color.border.subtle").into();
    let accent_base: Color = resolve_color(theme, "color.accent.base").into();
    let tone_color: Color = resolve_color(theme, spec.tone_color_token()).into();

    let fill = match spec.appearance {
        PillAppearance::Badge => match spec.tone {
            PillTone::Neutral => surface_bg.mix_srgb(text_primary, 0.96),
            _ => accent_base.with_alpha(accent_base.a * 0.18),
        },
        _ => match spec.tone {
            PillTone::Success | PillTone::Danger | PillTone::Info | PillTone::Warning => {
                tone_color.mix_srgb(surface_bg, 0.14)
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
                tone_color.mix_srgb(border_subtle, 0.38)
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

    // Custom accent (contract §8 "Custom accent"): a parseable hex `accent_color`
    // overrides the tone fill/border/text via color-mix.
    //   fill   = color-mix(accent 18%, rgba(148,163,184,0.08))
    //   border = color-mix(accent 30%, rgba(148,163,184,0.12))
    //   text   = color-mix(accent 88%, white)
    // The slate base rgba(148,163,184,…) is a Svelte literal with no semantic token;
    // replicated literally (token gap noted in the parity doc).
    if let Some(rgb) = spec
        .accent_color
        .as_deref()
        .and_then(crate::theme_ext::hex_to_rgb255)
    {
        let accent = Color::new(
            rgb.r as f32 / 255.0,
            rgb.g as f32 / 255.0,
            rgb.b as f32 / 255.0,
            1.0,
        );
        let slate = Color::new(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0, 1.0);
        let slate_08 = slate.with_alpha(0.08);
        let slate_12 = slate.with_alpha(0.12);
        let white = Color::WHITE;
        return (
            accent.mix_srgb(slate_08, 0.18),
            accent.mix_srgb(slate_12, 0.30),
            accent.mix_srgb(white, 0.88),
        );
    }

    (fill, border, text)
}

pub fn js_pill(spec: &PillSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // `resolved_size` applies the semantic role; reading `spec.size` here would
    // carry `size_role` and never honour it.
    let (min_w, min_h, pad_x, pad_y, font_size) = pill_metrics(spec.resolved_size(), spec.typography);
    let (fill, border, text_color) = pill_colors(spec, theme);
    // `font="mono"` now resolves through the JsEl font-family channel to
    // `--poodle-pill-mono` → code-family (contract §8 `.pill[data-font="mono"]`,
    // which also sets letter-spacing 0.02em). Badge appearance sets 0.04em
    // (contract §8 badge). The pill renders as a single Label, so the
    // `--poodle-pill-gap` content gap has no inline child to separate yet; PillSpec
    // models no icon prop (Svelte composes the optional icon via a slot), so the
    // label-only render is faithful until an icon prop lands.
    let label = if spec.appearance == PillAppearance::Badge {
        spec.label.to_uppercase()
    } else {
        spec.label.clone()
    };
    let weight = if spec.appearance == PillAppearance::Badge { 700 } else { 600 };
    let radius = resolve_radius(theme, "radius.pill");

    let mut el = ui_element::label(&label)
        .min_w(rem_to_px(min_w))
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

    // Letter-spacing: mono variant (0.02em) is the most specific tracking rule
    // for the text run; otherwise badge appearance carries 0.04em. Base pills
    // have no letter-spacing rule (0 = default).
    if spec.font == PillFont::Mono {
        el = el.font_family(FontFamily::Mono).letter_spacing_em(0.02);
    } else if spec.appearance == PillAppearance::Badge {
        el = el.letter_spacing_em(0.04);
    }

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
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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

    #[test]
    fn md_pill_uses_svelte_padding_x() {
        // Contract §8 / Svelte md pad-x = 0.625rem (not the old contract 0.5rem).
        let el = js_pill(&PillSpec::new().with_size(PillSize::Md), &theme());
        let expected = taffy::LengthPercentage::length(rem_to_px(0.625));
        assert_eq!(el.layout.padding.left, expected);
        assert_eq!(el.layout.padding.right, expected);
    }

    #[test]
    fn md_pill_applies_min_width_floor() {
        // Contract §8: md min-width base = 2.875rem.
        let el = js_pill(&PillSpec::new().with_size(PillSize::Md), &theme());
        assert_eq!(
            el.layout.min_size.width,
            taffy::Dimension::length(rem_to_px(2.875))
        );
    }

    #[test]
    fn accent_overrides_tone_fill() {
        // Contract §8 "Custom accent": fill = color-mix(accent 18%, rgba(148,163,184,0.08)).
        let th = theme();
        let el = js_pill(&PillSpec::new().with_accent_color("#14b8a6"), &th);
        let rgb = crate::theme_ext::hex_to_rgb255("#14b8a6").unwrap();
        let accent = Color::new(
            rgb.r as f32 / 255.0,
            rgb.g as f32 / 255.0,
            rgb.b as f32 / 255.0,
            1.0,
        );
        let slate_08 = Color::new(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0, 0.08);
        let expected = accent.mix_srgb(slate_08, 0.18);
        assert_eq!(el.style.background, Some(expected));
    }

    #[test]
    fn accent_overrides_tone_text() {
        // Contract §8 "Custom accent": text = color-mix(accent 88%, white).
        let th = theme();
        let el = js_pill(&PillSpec::new().with_accent_color("#a855f7"), &th);
        let rgb = crate::theme_ext::hex_to_rgb255("#a855f7").unwrap();
        let accent = Color::new(
            rgb.r as f32 / 255.0,
            rgb.g as f32 / 255.0,
            rgb.b as f32 / 255.0,
            1.0,
        );
        let expected = accent.mix_srgb(Color::WHITE, 0.88);
        assert_eq!(el.style.text_color, Some(expected));
    }
}
