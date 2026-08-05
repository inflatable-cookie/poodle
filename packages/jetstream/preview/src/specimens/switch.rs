//! Switch specimen — contract §13 States plus the parity shortfall coverage
//! (custom colors, dual labels + tones, sizes, densities, read-only).
//!
//! Switches render statically here; toggle interaction is a preview-loop concern.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_switch;

use poodle_specs::{ControlDensity, ControlSize, SwitchSpec, SwitchTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    fn sw(checked: bool, label: &str) -> SwitchSpec {
        let mut s = SwitchSpec::new();
        if checked { s = s.with_checked(true); }
        s.label = Some(label.into());
        s
    }

    div().flex_col().gap(24.0)
        // Contract §13 States.
        .child(group("States", secondary,
            div().flex_col().gap(12.0)
                .child(js_switch(&sw(false, "Off"), theme))
                .child(js_switch(&sw(true, "On"), theme))
                .child(js_switch(&sw(false, "Disabled off").with_disabled(true), theme))
                .child(js_switch(&sw(true, "Disabled on").with_disabled(true), theme))
                .child(js_switch(&sw(true, "Read-only on").with_read_only(true), theme))
        ))
        // Custom colors (on/off hex overrides).
        .child(group("Custom colors", secondary,
            div().flex_col().gap(12.0)
                .child(js_switch(
                    &sw(true, "Custom on").with_on_color("#22c55e"),
                    theme,
                ))
                .child(js_switch(
                    &sw(false, "Custom off").with_off_color("#f97316"),
                    theme,
                ))
        ))
        // Dual labels + tones.
        .child(group("Dual labels + tones", secondary,
            div().flex_col().gap(12.0)
                .child(js_switch(
                    &SwitchSpec::new()
                        .with_left_label("Off")
                        .with_right_label("On")
                        .with_left_tone(SwitchTone::Danger)
                        .with_right_tone(SwitchTone::Success),
                    theme,
                ))
                .child(js_switch(
                    &SwitchSpec::new()
                        .with_aria_label("Setting 1")
                        .with_checked(true)
                        .with_left_label("Manual")
                        .with_right_label("Auto")
                        .with_left_tone(SwitchTone::Warning)
                        .with_right_tone(SwitchTone::Success),
                    theme,
                ))
        ))
        // Sizes.
        .child(group("Sizes", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_switch(&SwitchSpec::new().with_checked(true).with_aria_label("Size Xs").with_size(ControlSize::Xs), theme))
                .child(js_switch(&SwitchSpec::new().with_checked(true).with_aria_label("Size Sm").with_size(ControlSize::Sm), theme))
                .child(js_switch(&SwitchSpec::new().with_checked(true).with_aria_label("Size Md").with_size(ControlSize::Md), theme))
                .child(js_switch(&SwitchSpec::new().with_checked(true).with_aria_label("Size Lg").with_size(ControlSize::Lg), theme))
                .child(js_switch(&SwitchSpec::new().with_checked(true).with_aria_label("Size Xl").with_size(ControlSize::Xl), theme))
        ))
        // Densities.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(js_switch(&sw(true, "Compact").with_density(ControlDensity::Compact), theme))
                .child(js_switch(&sw(true, "Default").with_density(ControlDensity::Default), theme))
                .child(js_switch(&sw(true, "Comfortable").with_density(ControlDensity::Comfortable), theme))
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
