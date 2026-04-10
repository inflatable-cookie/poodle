//! ReorderableList specimen — drag-and-drop list with grip handles.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::reorderable_list::js_reorderable_list;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::ReorderableListSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    let items: Vec<JsEl> = vec![
        label("Introduction").text_color(text_primary).text_size(13.0),
        label("Background").text_color(text_primary).text_size(13.0),
        label("Methods").text_color(text_primary).text_size(13.0),
        label("Results").text_color(text_primary).text_size(13.0),
    ];

    let drag_items: Vec<JsEl> = vec![
        label("Track A").text_color(text_primary).text_size(13.0),
        label("Track B (dragging)").text_color(text_primary).text_size(13.0),
        label("Track C").text_color(text_primary).text_size(13.0),
    ];

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            js_reorderable_list(
                &ReorderableListSpec::new().with_item_count(4),
                theme,
                items,
            )
        ))
        .child(group("Active drag on item 2", secondary,
            js_reorderable_list(
                &ReorderableListSpec::new()
                    .with_item_count(3)
                    .with_active_drag_index(1),
                theme,
                drag_items,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
