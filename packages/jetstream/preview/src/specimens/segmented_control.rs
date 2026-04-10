//! SegmentedControl specimen — segmented controls with selection and disabled state.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::segmented_control::js_segmented_control;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{ChoiceOption, SegmentedControlSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let options = vec![
        ChoiceOption::new("grid", "Grid"),
        ChoiceOption::new("list", "List"),
        ChoiceOption::new("board", "Board"),
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
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
