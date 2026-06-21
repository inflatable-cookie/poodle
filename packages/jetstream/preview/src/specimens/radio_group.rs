//! RadioGroup specimen — contract §13 specimen states plus the parity shortfall
//! coverage (disabled-option, custom selected color, size matrix, densities).
//!
//! Uses the contract §13 option sets (Free/Pro/Enterprise, Small/Medium/Large/
//! Extra large). Selection / arrow-key / Space interaction is a preview-loop
//! concern, not the pure-render component.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::radio_group::js_radio_group;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ChoiceOption, ControlDensity, ControlSize, Orientation, RadioGroupSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let plan_options = || {
        vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro"),
            ChoiceOption::new("enterprise", "Enterprise"),
        ]
    };

    let size_options = vec![
        ChoiceOption::new("sm", "Small"),
        ChoiceOption::new("md", "Medium"),
        ChoiceOption::new("lg", "Large"),
        ChoiceOption::new("xl", "Extra large"),
    ];

    div().flex_col().gap(24.0)
        // Contract §13 Vertical (default) — Plan, Pro selected.
        .child(group("Vertical (default)", secondary,
            js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro"), theme)
        ))
        // Contract §13 Horizontal — Size, Medium selected.
        .child(group("Horizontal", secondary,
            js_radio_group(
                &RadioGroupSpec::new(size_options)
                    .with_value("md")
                    .with_orientation(Orientation::Horizontal),
                theme,
            )
        ))
        // Contract §13 Disabled — group-level disabled, Free selected.
        .child(group("Disabled", secondary, {
            let mut spec = RadioGroupSpec::new(plan_options()).with_value("free");
            spec.is_disabled = true;
            js_radio_group(&spec, theme)
        }))
        // Disabled option — single option muted, group otherwise interactive.
        .child(group("Disabled option", secondary,
            js_radio_group(
                &RadioGroupSpec::new(vec![
                    ChoiceOption::new("free", "Free"),
                    ChoiceOption::new("pro", "Pro"),
                    ChoiceOption::new("enterprise", "Enterprise").with_disabled(true),
                ])
                .with_value("pro"),
                theme,
            )
        ))
        // Custom selected color — per-instance selected ring + dot override.
        .child(group("Custom selected color", secondary,
            js_radio_group(
                &RadioGroupSpec::new(plan_options())
                    .with_value("pro")
                    .with_selected_color("#22c55e"),
                theme,
            )
        ))
        // Sizes — xs–xl, indicator/dot scale per the §8 size table.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_size(ControlSize::Xs), theme))
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_size(ControlSize::Sm), theme))
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_size(ControlSize::Md), theme))
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_size(ControlSize::Lg), theme))
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_size(ControlSize::Xl), theme))
        ))
        // Densities — group gap only; option height unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_density(ControlDensity::Compact), theme))
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_density(ControlDensity::Default), theme))
                .child(js_radio_group(&RadioGroupSpec::new(plan_options()).with_value("pro").with_orientation(Orientation::Horizontal).with_density(ControlDensity::Comfortable), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
