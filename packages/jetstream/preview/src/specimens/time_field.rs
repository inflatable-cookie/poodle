//! TimeField specimen — with value, placeholder, min/max, sizes, densities, disabled.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::time_field::js_time_field;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, TimeFieldSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // With value
        .child(group("With value", secondary,
            div().w(200.0)
                .child(js_time_field(&TimeFieldSpec::new().with_default_value("14:30"), theme))
        ))
        // Placeholder (no value)
        .child(group("Placeholder", secondary,
            div().w(200.0)
                .child(js_time_field(&TimeFieldSpec::new(), theme))
        ))
        // With min/max constraints
        .child(group("With min/max constraints", secondary,
            div().w(200.0).child(js_time_field(&{
                let mut s = TimeFieldSpec::new().with_default_value("09:00");
                s.min = Some("08:00".to_string());
                s.max = Some("18:00".to_string());
                s
            }, theme))
        ))
        // Sizes (full ladder, xs..xl)
        .child(group("Sizes", secondary,
            div().flex_col().gap(8.0)
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_size(ControlSize::Xs), theme)))
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_size(ControlSize::Sm), theme)))
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_size(ControlSize::Md), theme)))
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_size(ControlSize::Lg), theme)))
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_size(ControlSize::Xl), theme)))
        ))
        // Densities
        .child(group("Densities", secondary,
            div().flex_col().gap(8.0)
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_density(ControlDensity::Compact), theme)))
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_density(ControlDensity::Default), theme)))
                .child(div().w(200.0).child(js_time_field(&TimeFieldSpec::new().with_default_value("09:00").with_density(ControlDensity::Comfortable), theme)))
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = TimeFieldSpec::new().with_default_value("16:45");
            spec.is_disabled = true;
            div().w(200.0).child(js_time_field(&spec, theme))
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
