//! NumberInput specimen — number entries at default, min, max, and disabled states.

use crate::compat::js_number_input;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, NumberInputSpec, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Default
        .child(group(
            "Default (50)",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(50.0)
                    .with_aria_label("Quantity 1")
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_step(1.0),
                theme,
            )),
        ))
        // At min
        .child(group(
            "At min (0)",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(0.0)
                    .with_aria_label("Quantity 2")
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_step(1.0),
                theme,
            )),
        ))
        // At max
        .child(group(
            "At max (100)",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(100.0)
                    .with_aria_label("Quantity 3")
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_step(1.0),
                theme,
            )),
        ))
        // Disabled
        .child(group(
            "Disabled",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(42.0)
                    .with_aria_label("Quantity 4")
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_disabled(true),
                theme,
            )),
        ))
        // Invalid state
        .child(group(
            "Invalid state",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(50.0)
                    .with_aria_label("Quantity 5")
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_validation_state(ValidationState::Invalid),
                theme,
            )),
        ))
        // With prefix ($)
        .child(group(
            "With prefix ($)",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(42.50)
                    .with_aria_label("Quantity 6")
                    .with_min(0.0)
                    .with_max(1000.0)
                    .with_prefix("$")
                    .with_precision(2),
                theme,
            )),
        ))
        // With suffix (px)
        .child(group(
            "With suffix (px)",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(16.0)
                    .with_aria_label("Quantity 7")
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_suffix("px"),
                theme,
            )),
        ))
        // With steppers (gated on show_steppers)
        .child(group(
            "With steppers",
            secondary,
            div().w(300.0).child(js_number_input(
                &NumberInputSpec::new(3.0)
                    .with_aria_label("Quantity 8")
                    .with_min(0.0)
                    .with_max(10.0)
                    .with_step(1.0)
                    .with_steppers(true),
                theme,
            )),
        ))
        // Sizes (xs–xl)
        .child(group(
            "Sizes (xs–xl)",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 9")
                            .with_size(ControlSize::Xs),
                        theme,
                    )),
                )
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 10")
                            .with_size(ControlSize::Sm),
                        theme,
                    )),
                )
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 11")
                            .with_size(ControlSize::Md),
                        theme,
                    )),
                )
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 12")
                            .with_size(ControlSize::Lg),
                        theme,
                    )),
                )
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 13")
                            .with_size(ControlSize::Xl),
                        theme,
                    )),
                ),
        ))
        // Densities (compact / default / comfortable)
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 14")
                            .with_density(ControlDensity::Compact),
                        theme,
                    )),
                )
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 15")
                            .with_density(ControlDensity::Default),
                        theme,
                    )),
                )
                .child(
                    div().w(300.0).child(js_number_input(
                        &NumberInputSpec::new(50.0)
                            .with_aria_label("Quantity 16")
                            .with_density(ControlDensity::Comfortable),
                        theme,
                    )),
                ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
