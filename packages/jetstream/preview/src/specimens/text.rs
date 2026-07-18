//! Text specimen — typography primitive across tone / size / weight axes.
//!
//! Mirrors the GPUI specimen: all six tones, the xs/sm/md sizes, the
//! medium/semibold/bold weights, relaxed leading, inline span, compact stacked
//! spacing, and a clamped paragraph. Every node is a real `js_text` resolving
//! color/size/weight/line-height from `TextSpec`.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::text::js_text;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{TextElement, TextLeading, TextSize, TextSpacing, TextSpec, TextTone, TextWeight};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // --- Tones (all six contract values) ---
        .child(group("Tones", secondary,
            div().flex_col().gap(6.0)
                .child(js_text(&TextSpec::new("Default body text."), theme))
                .child(js_text(&TextSpec::new("Secondary supporting text.").with_tone(TextTone::Secondary), theme))
                .child(js_text(&TextSpec::new("Muted hint text.").with_tone(TextTone::Muted), theme))
                .child(js_text(&TextSpec::new("Success confirmation text.").with_tone(TextTone::Success), theme))
                .child(js_text(&TextSpec::new("Danger validation text.").with_tone(TextTone::Danger), theme))
                .child(js_text(&TextSpec::new("Warning advisory text.").with_tone(TextTone::Warning), theme))
        ))
        // --- Sizes (xs / sm / md) ---
        .child(group("Sizes", secondary,
            div().flex_col().gap(6.0)
                .child(js_text(&TextSpec::new("Extra-small label text.").with_size(TextSize::Xs), theme))
                .child(js_text(&TextSpec::new("Small supporting text.").with_size(TextSize::Sm), theme))
                .child(js_text(&TextSpec::new("Medium body text.").with_size(TextSize::Md), theme))
        ))
        // --- Weights (medium / semibold / bold) + relaxed leading ---
        .child(group("Weights", secondary,
            div().flex_col().gap(6.0)
                .child(js_text(&TextSpec::new("Medium weight text.").with_weight(TextWeight::Medium), theme))
                .child(js_text(
                    &TextSpec::new("Relaxed semibold body copy.")
                        .with_leading(TextLeading::Relaxed)
                        .with_weight(TextWeight::Semibold),
                    theme,
                ))
                .child(js_text(&TextSpec::new("Bold emphasis text.").with_weight(TextWeight::Bold), theme))
        ))
        // --- Inline span + compact spacing + clamp ---
        .child(group("Variants", secondary,
            div().flex_col().gap(6.0)
                .child(js_text(&TextSpec::new("Inline span phrase.").with_element(TextElement::Span), theme))
                .child(js_text(&TextSpec::new("Compact-spaced paragraph.").with_spacing(TextSpacing::Compact), theme))
                .child(js_text(
                    &TextSpec::new("Clamped text that would otherwise overflow several lines.")
                        .with_clamp(2),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
