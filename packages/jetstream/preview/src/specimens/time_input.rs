//! TimeInput specimen — with value, placeholder, min/max, sizes, densities, disabled.

use crate::compat::js_time_input;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, TimeInputSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // With value
        .child(group(
            "With value",
            secondary,
            div().w(200.0).child(js_time_input(
                &TimeInputSpec::new().with_default_value("14:30"),
                theme,
            )),
        ))
        // Placeholder (no value)
        .child(group(
            "Placeholder",
            secondary,
            div()
                .w(200.0)
                .child(js_time_input(&TimeInputSpec::new(), theme)),
        ))
        // With min/max constraints
        .child(group(
            "With min/max constraints",
            secondary,
            div().w(200.0).child(js_time_input(
                &{
                    let mut s = TimeInputSpec::new().with_default_value("09:00");
                    s.min = Some("08:00".to_string());
                    s.max = Some("18:00".to_string());
                    s
                },
                theme,
            )),
        ))
        // Sizes (full ladder, xs..xl)
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_size(ControlSize::Xs),
                        theme,
                    )),
                )
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_size(ControlSize::Sm),
                        theme,
                    )),
                )
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_size(ControlSize::Md),
                        theme,
                    )),
                )
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_size(ControlSize::Lg),
                        theme,
                    )),
                )
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_size(ControlSize::Xl),
                        theme,
                    )),
                ),
        ))
        // Densities
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_density(ControlDensity::Compact),
                        theme,
                    )),
                )
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_density(ControlDensity::Default),
                        theme,
                    )),
                )
                .child(
                    div().w(200.0).child(js_time_input(
                        &TimeInputSpec::new()
                            .with_default_value("09:00")
                            .with_density(ControlDensity::Comfortable),
                        theme,
                    )),
                ),
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = TimeInputSpec::new().with_default_value("16:45");
            spec.is_disabled = true;
            div().w(200.0).child(js_time_input(&spec, theme))
        }))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
