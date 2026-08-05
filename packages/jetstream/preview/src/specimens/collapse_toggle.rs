//! CollapseToggle specimen — collapse toggles in all directions and states,
//! plus labeled usage, sizes, and densities for full contract coverage.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_collapse_toggle;

use poodle_specs::{CollapseDirection, CollapseToggleSpec, ControlDensity, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Toggle paired with an adjacent descriptive text label (host-side label;
    // the component itself is icon-only per the contract anatomy).
    let labeled = |toggle: El, text: &str| -> El {
        div().flex_row().gap(6.0).items_center()
            .child(toggle)
            .child(label(text).text_color(secondary).text_size(12.0))
    };

    div().flex_col().gap(24.0)
        // Left direction — collapsed vs expanded
        .child(group("Left: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_collapsed(false), theme))
        ))
        // Right direction
        .child(group("Right: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Right).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Right).with_collapsed(false), theme))
        ))
        // Up direction
        .child(group("Up: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Up).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Up).with_collapsed(false), theme))
        ))
        // Down direction
        .child(group("Down: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Down).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Down).with_collapsed(false), theme))
        ))
        // With label — toggle beside a contextual text label (matches Svelte specimen)
        .child(group("With label", secondary,
            div().flex_row().gap(24.0).items_center()
                .child(labeled(
                    js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_collapsed(false).with_aria_label("Collapse left dock"), theme),
                    "Left (expanded)",
                ))
                .child(labeled(
                    js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_collapsed(true).with_aria_label("Expand left dock"), theme),
                    "Left (collapsed)",
                ))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_collapsed(true).with_disabled(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_collapsed(false).with_disabled(true), theme))
        ))
        // Sizes — xs–xl, padding scales per the §8 size table
        .child(group("Sizes", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_size(ControlSize::Xs), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_size(ControlSize::Sm), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_size(ControlSize::Md), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_size(ControlSize::Lg), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_size(ControlSize::Xl), theme))
        ))
        // Densities — padding-inline only; height/vertical padding unchanged
        .child(group("Densities", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_density(ControlDensity::Compact), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_density(ControlDensity::Default), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_density(ControlDensity::Comfortable), theme))
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
