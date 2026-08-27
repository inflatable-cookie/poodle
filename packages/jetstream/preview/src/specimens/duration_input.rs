//! DurationInput specimen — value display, h/m/s segments, empty/zero,
//! invalid, disabled, sizes, and densities for full contract coverage.
//!
//! Note: the contract has no stepper buttons (increment/decrement is a
//! keyboard concern: ArrowUp/ArrowDown), so there is no "with steppers"
//! group — steppers are not part of the DurationInput anatomy.

use crate::compat::js_duration_input;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, DurationInputSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Value display — hours + minutes
        .child(group(
            "Value display (H:M)",
            secondary,
            div().w(240.0).child(js_duration_input(
                &DurationInputSpec::new()
                    .with_segments(2, 30, 0)
                    .with_show_seconds(false),
                theme,
            )),
        ))
        // Segments — hours, minutes, seconds (showSeconds)
        .child(group(
            "Segments (H:M:S)",
            secondary,
            div().w(240.0).child(js_duration_input(
                &DurationInputSpec::new().with_segments(1, 15, 45),
                theme,
            )),
        ))
        // Empty / zero — default zero display, no value bound
        .child(group(
            "Empty / zero",
            secondary,
            div()
                .w(240.0)
                .child(js_duration_input(&DurationInputSpec::new(), theme)),
        ))
        // Invalid — total below min bound shows danger border
        .child(group(
            "Invalid (out of bounds)",
            secondary,
            div().w(240.0).child(js_duration_input(
                &DurationInputSpec::new()
                    .with_segments(0, 0, 5)
                    .with_min_total_seconds(60),
                theme,
            )),
        ))
        // Disabled
        .child(group(
            "Disabled",
            secondary,
            div().w(240.0).child(js_duration_input(
                &DurationInputSpec::new()
                    .with_segments(1, 0, 0)
                    .with_disabled(true),
                theme,
            )),
        ))
        // Sizes — xs–xl, field width + font scale per §8
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
                            .with_size(ControlSize::Xs),
                        theme,
                    )),
                )
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
                            .with_size(ControlSize::Sm),
                        theme,
                    )),
                )
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
                            .with_size(ControlSize::Md),
                        theme,
                    )),
                )
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
                            .with_size(ControlSize::Lg),
                        theme,
                    )),
                )
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
                            .with_size(ControlSize::Xl),
                        theme,
                    )),
                ),
        ))
        // Densities — gap/inline padding only; height unchanged
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
                            .with_density(ControlDensity::Compact),
                        theme,
                    )),
                )
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
                            .with_density(ControlDensity::Default),
                        theme,
                    )),
                )
                .child(
                    div().w(240.0).child(js_duration_input(
                        &DurationInputSpec::new()
                            .with_segments(0, 30, 0)
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
