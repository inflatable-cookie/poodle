//! Checkbox specimen — contract §13 specimen states plus the parity shortfall
//! coverage (read-only, custom selected color, size matrix, densities).
//!
//! Checkboxes render statically here; toggle / Space / read-only-revert
//! interaction is a preview-loop concern, not the pure-render component.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_checkbox;

use poodle_specs::{CheckboxSpec, ControlDensity, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    fn cb(label: &str) -> CheckboxSpec {
        CheckboxSpec::new().with_label(label)
    }

    div().flex_col().gap(24.0)
        // Contract §13 Default — three interactive checkboxes.
        .child(group("Default", secondary,
            div().flex_col().gap(12.0)
                .child(js_checkbox(&cb("Enable email notifications").with_checked(true), theme))
                .child(js_checkbox(&cb("Subscribe to marketing emails"), theme))
                .child(js_checkbox(&cb("I agree to the terms and conditions"), theme))
        ))
        // Contract §13 States — disabled (unchecked/checked), mixed, read-only.
        .child(group("States", secondary,
            div().flex_col().gap(12.0)
                .child(js_checkbox(&cb("Disabled unchecked").with_disabled(true), theme))
                .child(js_checkbox(&cb("Disabled checked").with_checked(true).with_disabled(true), theme))
                .child(js_checkbox(&cb("Mixed / indeterminate").with_mixed(true), theme))
                .child(js_checkbox(&cb("Read-only checked").with_checked(true).with_read_only(true), theme))
        ))
        // Custom selected color — per-instance checked fill/border override.
        .child(group("Custom selected color", secondary,
            div().flex_col().gap(12.0)
                .child(js_checkbox(&cb("Billable feature").with_checked(true).with_selected_color("#22c55e"), theme))
                .child(js_checkbox(&cb("Requires moderation").with_checked(true).with_selected_color("#f59e0b"), theme))
        ))
        // Sizes — xs–xl, indicator/mark scale per the §8 size table.
        .child(group("Sizes", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_checkbox(&cb("Accept terms").with_checked(true).with_size(ControlSize::Xs), theme))
                .child(js_checkbox(&cb("Accept terms").with_checked(true).with_size(ControlSize::Sm), theme))
                .child(js_checkbox(&cb("Accept terms").with_checked(true).with_size(ControlSize::Md), theme))
                .child(js_checkbox(&cb("Accept terms").with_checked(true).with_size(ControlSize::Lg), theme))
                .child(js_checkbox(&cb("Accept terms").with_checked(true).with_size(ControlSize::Xl), theme))
        ))
        // Densities — spacing only; height/vertical padding unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(js_checkbox(&cb("Compact").with_density(ControlDensity::Compact), theme))
                .child(js_checkbox(&cb("Default").with_density(ControlDensity::Default), theme))
                .child(js_checkbox(&cb("Comfortable").with_density(ControlDensity::Comfortable), theme))
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
