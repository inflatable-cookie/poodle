//! Slider specimen — contract §13 states (Default / With step / Disabled) plus
//! parity-shortfall coverage (value low/mid/high, custom min/max + step, the
//! xs–xl size matrix, and densities).
//!
//! Sliders render statically here: drag / keyboard / live value readout are a
//! preview-loop concern, not the pure-render component. Track fill + thumb
//! position come from the spec's value via `js_slider` (no hand-rolled fill).

use crate::compat::js_slider;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, SliderPolarity, SliderSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Single-value slider, 0..100 unless overridden.
    fn s(value: f64) -> SliderSpec {
        SliderSpec::new(value).with_bounds(0.0, 100.0)
    }

    div()
        .flex_col()
        .gap(24.0)
        // Contract §13 Default — Volume 65, 0..100, step 1.
        .child(group(
            "Default",
            secondary,
            row(js_slider(
                &{
                    let mut spec = s(65.0);
                    spec.step = 1.0;
                    spec.aria_label = Some("Volume".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Contract §13 With step — Opacity 100, 0..100, step 10.
        .child(group(
            "With step",
            secondary,
            row(js_slider(
                &{
                    let mut spec = s(100.0);
                    spec.step = 10.0;
                    spec.aria_label = Some("Opacity".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Value low / mid / high — proportional track fill + thumb from the
        // spec value (all 0..100). Demonstrates fill driven by normalized value.
        .child(group(
            "Value (low / mid / high)",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(row(js_slider(&s(10.0), theme)))
                .child(row(js_slider(&s(50.0), theme)))
                .child(row(js_slider(&s(90.0), theme))),
        ))
        // Custom min / max + step — bounds 50..200, value 125, step 25. Fill at
        // (125-50)/(200-50) = 50%, snapping anchored at min.
        .child(group(
            "Custom min / max + step (50–200, step 25)",
            secondary,
            row(js_slider(
                &{
                    let mut spec = SliderSpec::new(125.0).with_bounds(50.0, 200.0);
                    spec.step = 25.0;
                    spec.aria_label = Some("Temperature".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Contract §13 Disabled — value 40, reduced opacity.
        .child(group("Disabled", secondary, {
            let mut spec = s(40.0);
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled slider".to_string());
            row(js_slider(&spec, theme))
        }))
        .child(group(
            "Embedded controls",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(row(js_slider(
                    &SliderSpec::new(0.35)
                        .with_bounds(0.0, 1.0)
                        .with_embedded_control(SliderPolarity::Unipolar),
                    theme,
                )))
                .child(row(js_slider(
                    &SliderSpec::new(-0.45)
                        .with_bounds(-1.0, 1.0)
                        .with_embedded_control(SliderPolarity::Bipolar),
                    theme,
                ))),
        ))
        // Sizes — xs–xl, thumb diameter + min-height scale per the §8 size table.
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(row(js_slider(&s(60.0).with_size(ControlSize::Xs), theme)))
                .child(row(js_slider(&s(60.0).with_size(ControlSize::Sm), theme)))
                .child(row(js_slider(&s(60.0).with_size(ControlSize::Md), theme)))
                .child(row(js_slider(&s(60.0).with_size(ControlSize::Lg), theme)))
                .child(row(js_slider(&s(60.0).with_size(ControlSize::Xl), theme))),
        ))
        // Densities — spacing axis only; track thickness / thumb / min-height
        // unchanged (contract §8 density note for the single-thumb slider).
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(row(js_slider(
                    &s(60.0).with_density(ControlDensity::Compact),
                    theme,
                )))
                .child(row(js_slider(
                    &s(60.0).with_density(ControlDensity::Default),
                    theme,
                )))
                .child(row(js_slider(
                    &s(60.0).with_density(ControlDensity::Comfortable),
                    theme,
                ))),
        ))
}

fn row(content: El) -> El {
    div().w(300.0).child(content)
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
