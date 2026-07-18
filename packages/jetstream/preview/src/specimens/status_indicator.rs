//! StatusIndicator specimen — contract §13 specimens.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::status_indicator::js_status_indicator;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, InlineTypographyMode, StatusIndicatorSpec, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    fn indicator(tone: StatusTone, label: &str) -> StatusIndicatorSpec {
        StatusIndicatorSpec::new().with_status(tone).with_label(label)
    }

    div().flex_col().gap(24.0)
        // All statuses (6) with labels.
        .child(group("All statuses", secondary,
            div().flex_col().gap(12.0)
                .child(js_status_indicator(&indicator(StatusTone::Neutral, "Neutral"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Info, "Info"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Success, "Success"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Warning, "Warning"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Danger, "Danger"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Pending, "Pending"), theme))
        ))
        // Without labels (dot only) — aria_label provides the accessible name.
        .child(group("Without labels (dot only)", secondary,
            div().flex_row().gap(12.0)
                .child(js_status_indicator(&StatusIndicatorSpec::new().with_status(StatusTone::Success).with_aria_label("Online"), theme))
                .child(js_status_indicator(&StatusIndicatorSpec::new().with_status(StatusTone::Warning).with_aria_label("Away"), theme))
                .child(js_status_indicator(&StatusIndicatorSpec::new().with_status(StatusTone::Danger).with_aria_label("Offline"), theme))
                .child(js_status_indicator(&StatusIndicatorSpec::new().with_status(StatusTone::Neutral).with_aria_label("Unknown"), theme))
        ))
        // Slot/child content — label sourced like Svelte's default slot.
        .child(group("Slot content", secondary,
            div()
                .child(js_status_indicator(&indicator(StatusTone::Success, "Build passing"), theme))
        ))
        // Sizes ladder (xs → xl).
        .child(group("Sizes (xs → xl)", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_status_indicator(&indicator(StatusTone::Success, "xs").with_size(ControlSize::Xs), theme))
                .child(js_status_indicator(&indicator(StatusTone::Success, "sm").with_size(ControlSize::Sm), theme))
                .child(js_status_indicator(&indicator(StatusTone::Success, "md").with_size(ControlSize::Md), theme))
                .child(js_status_indicator(&indicator(StatusTone::Success, "lg").with_size(ControlSize::Lg), theme))
                .child(js_status_indicator(&indicator(StatusTone::Success, "xl").with_size(ControlSize::Xl), theme))
        ))
        // Densities — gap tightens/loosens, dot/label unchanged.
        .child(group("Densities (compact / default / comfortable)", secondary,
            div().flex_col().gap(12.0)
                .child(js_status_indicator(&indicator(StatusTone::Info, "Compact").with_density(ControlDensity::Compact), theme))
                .child(js_status_indicator(&indicator(StatusTone::Info, "Default").with_density(ControlDensity::Default), theme))
                .child(js_status_indicator(&indicator(StatusTone::Info, "Comfortable").with_density(ControlDensity::Comfortable), theme))
        ))
        // Inherit typography.
        .child(group("Inherit typography", secondary,
            div().flex_row().gap(8.0).items_center().text_size(20.0)
                .child(label("Deploy"))
                .child(js_status_indicator(
                    &StatusIndicatorSpec::new()
                        .with_status(StatusTone::Success)
                        .with_label("Healthy")
                        .with_typography(InlineTypographyMode::Inherit),
                    theme,
                ))
                .child(label("now"))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
