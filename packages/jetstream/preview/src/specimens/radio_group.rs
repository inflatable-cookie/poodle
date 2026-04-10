//! RadioGroup specimen — radio buttons in horizontal and vertical layouts.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::radio_group::js_radio_group;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{ChoiceOption, Orientation, RadioGroupSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let options = vec![
        ChoiceOption::new("a", "Option A"),
        ChoiceOption::new("b", "Option B"),
        ChoiceOption::new("c", "Option C"),
    ];

    div().flex_col().gap(24.0)
        // Vertical (default)
        .child(group("Vertical", secondary,
            js_radio_group(&RadioGroupSpec::new(options.clone()).with_value("a"), theme)
        ))
        // Horizontal
        .child(group("Horizontal", secondary,
            js_radio_group(&RadioGroupSpec::new(options.clone()).with_value("b").with_orientation(Orientation::Horizontal), theme)
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = RadioGroupSpec::new(options.clone()).with_value("a");
            spec.is_disabled = true;
            js_radio_group(&spec, theme)
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
