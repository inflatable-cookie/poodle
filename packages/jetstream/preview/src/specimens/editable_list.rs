//! EditableList specimen — add/remove list with inline text entry.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::editable_list::js_editable_list;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::EditableListSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("With items", secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(3)
                    .with_max_items(5),
                theme,
            )
        ))
        .child(group("Empty", secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_placeholder("Add a tag"),
                theme,
            )
        ))
        .child(group("Disabled", secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(2)
                    .with_disabled(true),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
