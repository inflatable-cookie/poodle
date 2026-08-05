//! CodeInput specimen — empty, partial, complete, alphanumeric, masked, invalid,
//! numbers-only, disabled, plus the size and density ladders.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_code_input;

use poodle_specs::{CodeInputSpec, ControlDensity, ControlSize, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Empty 6-digit — six empty slots, 3+3 split-after grouping.
        .child(group("6-digit (empty)", secondary,
            div().w(320.0)
                .child(js_code_input(&CodeInputSpec::new().with_aria_label("Verification code"), theme))
        ))
        // Partial — first three slots filled.
        .child(group("Partial (3 of 6)", secondary,
            div().w(320.0)
                .child(js_code_input(&CodeInputSpec::new().with_value("123"), theme))
        ))
        // Complete — all six slots filled.
        .child(group("Complete", secondary,
            div().w(320.0)
                .child(js_code_input(&CodeInputSpec::new().with_value("123456"), theme))
        ))
        // Numbers only (default) — digits-only entry.
        .child(group("Numbers only", secondary,
            div().w(320.0)
                .child(js_code_input(
                    &CodeInputSpec::new()
                        .with_value("042")
                        .with_numbers_only(true)
                        .with_aria_label("Numeric code"),
                    theme,
                ))
        ))
        // Alphanumeric — letters and digits permitted.
        .child(group("Alphanumeric", secondary,
            div().w(320.0)
                .child(js_code_input(
                    &CodeInputSpec::new()
                        .with_default_value("AB12")
                        .with_numbers_only(false)
                        .with_autocomplete("off")
                        .with_aria_label("Recovery code"),
                    theme,
                ))
        ))
        // Invalid — danger slot border with an error message below.
        .child(group("Invalid", secondary,
            div().w(320.0)
                .child(js_code_input(
                    &CodeInputSpec::new()
                        .with_value("999999")
                        .with_validation_state(ValidationState::Invalid)
                        .with_error("Invalid code. Please try again."),
                    theme,
                ))
        ))
        // Masked — 4-digit PIN, filled slots show a bullet.
        .child(group("4-digit Masked", secondary,
            div().w(320.0)
                .child(js_code_input(
                    &CodeInputSpec::new()
                        .with_length(4)
                        .with_value("1234")
                        .with_mask(true)
                        .with_aria_label("PIN"),
                    theme,
                ))
        ))
        // Disabled — first three pre-filled, reduced opacity.
        .child(group("Disabled", secondary,
            div().w(320.0)
                .child(js_code_input(
                    &CodeInputSpec::new()
                        .with_default_value("123")
                        .with_disabled(true),
                    theme,
                ))
        ))
        // Sizes — square slots scale per the §8 size table (xs–xl).
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_size(ControlSize::Xs), theme))
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_size(ControlSize::Sm), theme))
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_size(ControlSize::Md), theme))
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_size(ControlSize::Lg), theme))
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_size(ControlSize::Xl), theme))
        ))
        // Densities — inter-slot gap only; slot size unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_density(ControlDensity::Compact), theme))
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_density(ControlDensity::Default), theme))
                .child(js_code_input(&CodeInputSpec::new().with_length(4).with_value("12").with_density(ControlDensity::Comfortable), theme))
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
