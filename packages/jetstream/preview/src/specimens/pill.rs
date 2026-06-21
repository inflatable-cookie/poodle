//! Pill specimen — compact inline labels. Mirrors Svelte PillSpecimen.svelte.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::pill::js_pill;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    InlineTypographyMode, PillAppearance, PillFont, PillSize, PillSpec, PillTone,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Tones — all five semantic tones (incl info → status.info).
        .child(group("Tones", secondary,
            row()
                .child(js_pill(&PillSpec::new().with_label("Neutral").with_tone(PillTone::Neutral), theme))
                .child(js_pill(&PillSpec::new().with_label("Info").with_tone(PillTone::Info), theme))
                .child(js_pill(&PillSpec::new().with_label("Success").with_tone(PillTone::Success), theme))
                .child(js_pill(&PillSpec::new().with_label("Warning").with_tone(PillTone::Warning), theme))
                .child(js_pill(&PillSpec::new().with_label("Danger").with_tone(PillTone::Danger), theme))
        ))
        // Sizes — full xs–xl ladder.
        .child(group("Sizes", secondary,
            row().items_end()
                .child(js_pill(&PillSpec::new().with_label("XS").with_size(PillSize::Xs), theme))
                .child(js_pill(&PillSpec::new().with_label("SM").with_size(PillSize::Sm), theme))
                .child(js_pill(&PillSpec::new().with_label("MD").with_size(PillSize::Md), theme))
                .child(js_pill(&PillSpec::new().with_label("LG").with_size(PillSize::Lg), theme))
                .child(js_pill(&PillSpec::new().with_label("XL").with_size(PillSize::Xl), theme))
        ))
        // Solid vs subtle appearance — subtle halves the fill opacity.
        .child(group("Solid vs subtle", secondary,
            row()
                .child(js_pill(&PillSpec::new().with_label("Solid").with_tone(PillTone::Info).with_appearance(PillAppearance::Solid), theme))
                .child(js_pill(&PillSpec::new().with_label("Subtle").with_tone(PillTone::Info).with_appearance(PillAppearance::Subtle), theme))
                .child(js_pill(&PillSpec::new().with_label("Solid").with_tone(PillTone::Success).with_appearance(PillAppearance::Solid), theme))
                .child(js_pill(&PillSpec::new().with_label("Subtle").with_tone(PillTone::Success).with_appearance(PillAppearance::Subtle), theme))
        ))
        // Code font — mono family (text-family delta documented in contract §12).
        .child(group("Code font", secondary,
            row()
                .child(js_pill(&PillSpec::new().with_label("v2.4.1").with_font(PillFont::Mono), theme))
                .child(js_pill(&PillSpec::new().with_label("stable").with_font(PillFont::Mono).with_tone(PillTone::Success), theme))
                .child(js_pill(&PillSpec::new().with_label("beta").with_font(PillFont::Mono).with_tone(PillTone::Warning), theme))
        ))
        // Muted — 0.72 opacity de-emphasis.
        .child(group("Muted", secondary,
            row()
                .child(js_pill(&PillSpec::new().with_label("Muted neutral").with_muted(true), theme))
                .child(js_pill(&PillSpec::new().with_label("Muted success").with_tone(PillTone::Success).with_muted(true), theme))
                .child(js_pill(&PillSpec::new().with_label("Muted danger").with_tone(PillTone::Danger).with_muted(true), theme))
        ))
        // Badge — uppercase count/status chips.
        .child(group("Badge", secondary,
            row()
                .child(js_pill(&PillSpec::new().with_label("3").with_appearance(PillAppearance::Badge), theme))
                .child(js_pill(&PillSpec::new().with_label("12").with_appearance(PillAppearance::Badge), theme))
                .child(js_pill(&PillSpec::new().with_label("99+").with_appearance(PillAppearance::Badge), theme))
                .child(js_pill(&PillSpec::new().with_label("New").with_appearance(PillAppearance::Badge), theme))
                .child(js_pill(&PillSpec::new().with_label("Draft").with_appearance(PillAppearance::Badge).with_tone(PillTone::Neutral), theme))
        ))
        // Custom accent — accent_color overrides the semantic tone colors.
        .child(group("Custom accent", secondary,
            row()
                .child(js_pill(&PillSpec::new().with_label("Info-ish").with_accent_color("#3b82f6"), theme))
                .child(js_pill(&PillSpec::new().with_label("Positive-ish").with_accent_color("#22c55e"), theme))
                .child(js_pill(&PillSpec::new().with_label("Caution-ish").with_accent_color("#f59e0b"), theme))
                .child(js_pill(&PillSpec::new().with_label("Danger-ish").with_accent_color("#ef4444"), theme))
        ))
        // Inherit typography — pill scales with surrounding text.
        .child(group("Inherit typography", secondary,
            div().flex_row().gap(8.0).items_center().text_size(20.0)
                .child(label("Release"))
                .child(js_pill(
                    &PillSpec::new()
                        .with_label("Beta")
                        .with_typography(InlineTypographyMode::Inherit),
                    theme,
                ))
                .child(label("channel"))
        ))
        // NOTE: removable (✕), leading icon/dot, and count slot are omitted —
        // `js_pill` renders a label-only chip and ignores `is_removable` /
        // `is_selected`; `PillSpec` exposes no icon/dot/count prop. They are
        // skipped rather than faked (see parity/pill.md).
}

fn row() -> JsEl {
    div().flex_row().gap(8.0).items_center()
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
