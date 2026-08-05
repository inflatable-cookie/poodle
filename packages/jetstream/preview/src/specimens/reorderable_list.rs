//! ReorderableList specimen — drag-and-drop list with grip handles.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_reorderable_list;
use crate::compat::{rem_to_px, size_font_rem};

use poodle_specs::{ControlSize, ReorderableListSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    let items: Vec<El> = vec![
        label("Introduction").text_color(text_primary).text_size(body_font),
        label("Background").text_color(text_primary).text_size(body_font),
        label("Methods").text_color(text_primary).text_size(body_font),
        label("Results").text_color(text_primary).text_size(body_font),
    ];

    let drag_items: Vec<El> = vec![
        label("Track A").text_color(text_primary).text_size(body_font),
        label("Track B (dragging)").text_color(text_primary).text_size(body_font),
        label("Track C").text_color(text_primary).text_size(body_font),
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

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
