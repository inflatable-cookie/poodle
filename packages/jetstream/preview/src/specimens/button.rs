//! Button specimen — mirrors Svelte ButtonSpecimen.svelte sections.

use crate::compat::js_button;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Variants
        .child(group(
            "Variants",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Primary"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Secondary"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_label("Ghost"),
                    theme,
                )),
        ))
        // Variant × tone matrix — each tone row across primary/secondary/ghost.
        .child(group(
            "Default tone",
            secondary,
            tone_row(theme, ButtonTone::Default),
        ))
        .child(group(
            "Danger tone",
            secondary,
            tone_row(theme, ButtonTone::Danger),
        ))
        .child(group(
            "Success tone",
            secondary,
            tone_row(theme, ButtonTone::Success),
        ))
        .child(group(
            "Warning tone",
            secondary,
            tone_row(theme, ButtonTone::Warning),
        ))
        // With icons — registry icon names (matches Svelte specimen).
        .child(group(
            "With icons",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Create")
                        .with_leading_icon("plus"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Open")
                        .with_trailing_icon("external-link"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Save")
                        .with_leading_icon("save")
                        .with_trailing_icon("check"),
                    theme,
                )),
        ))
        // With chevron — disclosure indicator.
        .child(group(
            "With chevron",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Options")
                        .with_chevron(true),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Actions")
                        .with_chevron(true),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Filter")
                        .with_leading_icon("filter")
                        .with_chevron(true),
                    theme,
                )),
        ))
        // Sizes — full xs–xl ladder, bottom-aligned.
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_end()
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_size(ControlSize::Xs)
                        .with_label("XS"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_size(ControlSize::Sm)
                        .with_label("SM"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_size(ControlSize::Md)
                        .with_label("MD"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_size(ControlSize::Lg)
                        .with_label("LG"),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_size(ControlSize::Xl)
                        .with_label("XL"),
                    theme,
                )),
        ))
        // States — disabled / loading.
        .child(group(
            "States",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Disabled")
                        .with_disabled(true),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Loading")
                        .with_loading(true),
                    theme,
                ))
                .child(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Disabled secondary")
                        .with_disabled(true),
                    theme,
                )),
        ))
    // NOTE: full-width is not representable — `ButtonSpec` exposes no
    // fit/full-width/max-width field, so it is omitted rather than faked
    // with a hand-sized container (see parity/button.md).
}

/// One row of the variant × tone matrix: primary / secondary / ghost at `tone`.
fn tone_row(theme: &JetstreamThemeProvider, tone: ButtonTone) -> El {
    let label = match tone {
        ButtonTone::Default => "",
        ButtonTone::Danger => "Danger ",
        ButtonTone::Success => "Success ",
        ButtonTone::Warning => "Warning ",
    };
    div()
        .flex_row()
        .gap(8.0)
        .items_center()
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_tone(tone)
                .with_label(format!("{label}primary").trim()),
            theme,
        ))
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_tone(tone)
                .with_label(format!("{label}secondary").trim()),
            theme,
        ))
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_tone(tone)
                .with_label(format!("{label}ghost").trim()),
            theme,
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
