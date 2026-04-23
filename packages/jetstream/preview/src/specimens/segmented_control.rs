//! SegmentedControl specimen — segmented controls with selection and disabled state.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::segmented_control::js_segmented_control;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ChoiceOption, ControlSize, SegmentedControlSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let options = vec![
        ChoiceOption::new("grid", "Grid"),
        ChoiceOption::new("list", "List"),
        ChoiceOption::new("board", "Board"),
    ];

    let options_with_disabled = vec![
        ChoiceOption::new("grid", "Grid"),
        ChoiceOption::new("list", "List"),
        ChoiceOption::new("board", "Board").with_disabled(true),
    ];

    div().flex_col().gap(24.0)
        // Default with selection
        .child(group("Default (List selected)", secondary,
            div().w(300.0)
                .child(js_segmented_control(
                    &SegmentedControlSpec::new(options.clone()).with_default_value("list"),
                    theme,
                ))
        ))
        // No selection
        .child(group("No selection", secondary,
            div().w(300.0)
                .child(js_segmented_control(
                    &SegmentedControlSpec::new(options.clone()),
                    theme,
                ))
        ))
        // Disabled entire control
        .child(group("Disabled (entire control)", secondary,
            div().w(300.0)
                .child(js_segmented_control(
                    &{
                        let mut s = SegmentedControlSpec::new(options.clone()).with_default_value("list");
                        s.is_disabled = true;
                        s
                    },
                    theme,
                ))
        ))
        // One disabled option
        .child(group("With disabled option (Board)", secondary,
            div().w(300.0)
                .child(js_segmented_control(
                    &SegmentedControlSpec::new(options_with_disabled).with_default_value("list"),
                    theme,
                ))
        ))
        // Small size
        .child(group("Small size", secondary,
            div().w(300.0)
                .child(js_segmented_control(
                    &SegmentedControlSpec::new(options.clone())
                        .with_default_value("grid")
                        .with_size(ControlSize::Sm),
                    theme,
                ))
        ))
        // Large size
        .child(group("Large size", secondary,
            div().w(300.0)
                .child(js_segmented_control(
                    &SegmentedControlSpec::new(options.clone())
                        .with_default_value("grid")
                        .with_size(ControlSize::Lg),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
