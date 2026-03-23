//! Select specimen — dropdowns with placeholder, selected value, and disabled.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::select::js_select;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{ChoiceOption, SelectSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let options = vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
    ];

    div().flex_col().gap(24.0)
        // Default (with placeholder)
        .child(group("Default", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_placeholder("Choose fruit..."), theme))
        ))
        // With value
        .child(group("With value", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_value("banana"), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = SelectSpec::new(options.clone()).with_value("apple");
            spec.is_disabled = true;
            div().w(240.0).child(js_select(&spec, theme))
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
