//! Meter specimen — contract §13 specimens (sizes, thresholds, ranges).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::meter::js_meter;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, MeterSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Sizes ladder — value=50, xs..xl track thickness steps.
        .child(group("Sizes (xs → xl)", secondary,
            div().flex_col().gap(12.0).w(300.0)
                .child(js_meter(&MeterSpec::new().with_value(50.0).with_size(ControlSize::Xs), theme))
                .child(js_meter(&MeterSpec::new().with_value(50.0).with_size(ControlSize::Sm), theme))
                .child(js_meter(&MeterSpec::new().with_value(50.0).with_size(ControlSize::Md), theme))
                .child(js_meter(&MeterSpec::new().with_value(50.0).with_size(ControlSize::Lg), theme))
                .child(js_meter(&MeterSpec::new().with_value(50.0).with_size(ControlSize::Xl), theme))
        ))
        // Default (50%)
        .child(group("Default (50%)", secondary,
            div().w(300.0)
                .child(js_meter(&MeterSpec::new().with_value(50.0).with_aria_label("Storage usage"), theme))
        ))
        // With thresholds — 82%, above high threshold.
        .child(group("With thresholds — 82% (above high)", secondary,
            div().w(300.0)
                .child(js_meter(
                    &MeterSpec::new()
                        .with_value(82.0)
                        .with_low(25.0)
                        .with_high(75.0)
                        .with_optimum(50.0)
                        .with_aria_label("CPU usage"),
                    theme,
                ))
        ))
        // Low value (within normal range) — 30%.
        .child(group("Low value — 30% (within normal range)", secondary,
            div().w(300.0)
                .child(js_meter(
                    &MeterSpec::new()
                        .with_value(30.0)
                        .with_low(25.0)
                        .with_high(75.0)
                        .with_optimum(50.0)
                        .with_aria_label("Memory usage"),
                    theme,
                ))
        ))
        // Custom range (0–500) — 350/500 = 70%.
        .child(group("Custom range — 350 / 500 API calls used", secondary,
            div().w(300.0)
                .child(js_meter(
                    &MeterSpec::new()
                        .with_value(350.0)
                        .with_min(0.0)
                        .with_max(500.0)
                        .with_aria_label("API calls"),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
