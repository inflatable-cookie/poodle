//! TextInput specimen — placeholder, value, validation states, affordances,
//! affixes, char count, read-only, disabled, plus the size and density ladders.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::text_input::js_text_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, TextInputSpec, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Default — placeholder, empty.
        .child(group("Default (placeholder)", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Jane Doe"), theme))
        ))
        // With value.
        .child(group("With value", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("Hello world"), theme))
        ))
        // Invalid — danger border.
        .child(group("Invalid", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("bad@").with_validation_state(ValidationState::Invalid), theme))
        ))
        // Valid — success border + tick indicator.
        .child(group("Valid", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("good@email.com").with_validation_state(ValidationState::Valid), theme))
        ))
        // Pending — accent border + ring spinner.
        .child(group("Pending", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("acme-admin").with_validation_state(ValidationState::Pending), theme))
        ))
        // Leading icon — overlaid inside the field leading edge.
        .child(group("Leading icon", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Search...").with_leading_icon("search"), theme))
        ))
        // Trailing icon — overlaid inside the field trailing edge.
        .child(group("Trailing icon", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("Some text").with_trailing_icon("x-circle"), theme))
        ))
        // Prefix — static text before the input with separator.
        .child(group("With prefix", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("42.00").with_prefix("$"), theme))
        ))
        // Suffix — static text after the input with separator.
        .child(group("With suffix", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("100").with_suffix("USD"), theme))
        ))
        // Prefix and suffix together.
        .child(group("Prefix and suffix", secondary,
            div().w(300.0)
                .child(js_text_input(
                    &TextInputSpec::new()
                        .with_placeholder("0.00")
                        .with_prefix("$")
                        .with_suffix("USD")
                        .with_input_mode("decimal"),
                    theme,
                ))
        ))
        // Character count — live counter with max.
        .child(group("Char count", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("Hello").with_max_length(100).with_show_char_count(true), theme))
        ))
        // Multiline — textarea, four rows.
        .child(group("Multiline (4 rows)", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Enter description...").with_rows(4), theme))
        ))
        // Read-only — selectable but not editable.
        .child(group("Read-only", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("read-only value").with_read_only(true), theme))
        ))
        // Disabled — reduced opacity, non-interactive.
        .child(group("Disabled", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("sk-xxxx-xxxx-xxxx").with_disabled(true), theme))
        ))
        // Sizes — min-height / padding / font-size scale per the §8 size table.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0).w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("XS").with_size(ControlSize::Xs), theme))
                .child(js_text_input(&TextInputSpec::new().with_placeholder("SM").with_size(ControlSize::Sm), theme))
                .child(js_text_input(&TextInputSpec::new().with_placeholder("MD").with_size(ControlSize::Md), theme))
                .child(js_text_input(&TextInputSpec::new().with_placeholder("LG").with_size(ControlSize::Lg), theme))
                .child(js_text_input(&TextInputSpec::new().with_placeholder("XL").with_size(ControlSize::Xl), theme))
        ))
        // Densities — inline/block padding only; control height unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0).w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Compact").with_density(ControlDensity::Compact), theme))
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Default").with_density(ControlDensity::Default), theme))
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Comfortable").with_density(ControlDensity::Comfortable), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
