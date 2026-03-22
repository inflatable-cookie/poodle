//! Combobox specimen — comboboxes with placeholder, value, and disabled states.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::combobox::js_combobox;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::{ComboboxOption, ComboboxSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let options = vec![
        ComboboxOption::new("react", "React"),
        ComboboxOption::new("vue", "Vue"),
        ComboboxOption::new("svelte", "Svelte"),
        ComboboxOption::new("angular", "Angular"),
    ];

    div().flex_col().gap(24.0)
        // With placeholder
        .child(group("With placeholder", secondary,
            div().w(240.0).child(
                js_combobox(
                    &ComboboxSpec::new()
                        .with_options(options.clone())
                        .with_placeholder("Select framework..."),
                    theme,
                )
            )
        ))
        // With value
        .child(group("With value", secondary,
            div().w(240.0).child(
                js_combobox(
                    &ComboboxSpec::new()
                        .with_options(options.clone())
                        .with_value("svelte"),
                    theme,
                )
            )
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(240.0).child(
                js_combobox(
                    &ComboboxSpec::new()
                        .with_options(options.clone())
                        .with_value("react")
                        .with_disabled(true),
                    theme,
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
