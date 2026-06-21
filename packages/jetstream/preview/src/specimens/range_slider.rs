//! RangeSlider specimen — contract §13 groups (Default / With step / Disabled)
//! plus parity-shortfall coverage (several lo/hi positions, custom min/max +
//! step, the xs–xl size matrix, and densities).
//!
//! Renders statically here: drag / per-thumb keyboard / live value readout are a
//! preview-loop concern, not the pure-render component. The two thumbs and the
//! between-fill window come from the spec's low/high values via `js_range_slider`
//! (no hand-rolled fill bar).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::range_slider::js_range_slider;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, RangeSliderSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Contract §13 Default — [20, 80], 0..100; fill spans 20%..80%.
        .child(group("Default (20–80)", secondary,
            row(js_range_slider(
                &RangeSliderSpec::new(20.0, 80.0)
                    .with_bounds(0.0, 100.0)
                    .with_aria_label("Price range"),
                theme,
            ))
        ))
        // Contract §13 With step — [25, 45], 18..65, step 5; snaps to 5s.
        .child(group("With step (25–45, 18–65, step 5)", secondary,
            row(js_range_slider(
                &RangeSliderSpec::new(25.0, 45.0)
                    .with_bounds(18.0, 65.0)
                    .with_step(5.0)
                    .with_aria_label("Age range"),
                theme,
            ))
        ))
        // Positions — between-fill window at several lo/hi pairs; both thumbs +
        // the filled span come from the spec values (all 0..100).
        .child(group("Positions (narrow / full / low / high)", secondary,
            div().flex_col().gap(12.0)
                .child(row(js_range_slider(&RangeSliderSpec::new(45.0, 55.0).with_bounds(0.0, 100.0), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(0.0, 100.0).with_bounds(0.0, 100.0), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(0.0, 25.0).with_bounds(0.0, 100.0), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(75.0, 100.0).with_bounds(0.0, 100.0), theme)))
        ))
        // Custom min / max + step — bounds 0..500, [100, 350], step 50; fill from
        // (100-0)/500 = 20% to (350-0)/500 = 70%.
        .child(group("Custom min / max + step (0–500, step 50)", secondary,
            row(js_range_slider(
                &RangeSliderSpec::new(100.0, 350.0)
                    .with_bounds(0.0, 500.0)
                    .with_step(50.0)
                    .with_aria_label("Budget range"),
                theme,
            ))
        ))
        // Contract §13 Disabled — [30, 70], reduced opacity; fill visible 30%..70%.
        .child(group("Disabled (30–70)", secondary,
            row(js_range_slider(
                &RangeSliderSpec::new(30.0, 70.0)
                    .with_bounds(0.0, 100.0)
                    .with_disabled(true)
                    .with_aria_label("Disabled range"),
                theme,
            ))
        ))
        // Sizes — xs–xl, thumb diameter + min-height scale per the §8 size table.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_size(ControlSize::Xs), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_size(ControlSize::Sm), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_size(ControlSize::Md), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_size(ControlSize::Lg), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_size(ControlSize::Xl), theme)))
        ))
        // Densities — vertical hit-area only (contract §8 RangeSlider density
        // exception); track thickness / thumb / control geometry unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_density(ControlDensity::Compact), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_density(ControlDensity::Default), theme)))
                .child(row(js_range_slider(&RangeSliderSpec::new(25.0, 75.0).with_density(ControlDensity::Comfortable), theme)))
        ))
}

fn row(content: JsEl) -> JsEl {
    div().w(300.0).child(content)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
