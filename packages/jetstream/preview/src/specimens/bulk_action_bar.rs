//! BulkActionBar specimen — contextual action bar for bulk selections.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::bulk_action_bar::js_bulk_action_bar;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{BulkAction, BulkActionBarSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let actions = vec![
        BulkAction::new("delete", "Delete"),
        BulkAction::new("archive", "Archive"),
    ];

    div().flex_col().gap(24.0)
        .child(group("With actions", secondary,
            js_bulk_action_bar(
                &BulkActionBarSpec::new()
                    .with_selection_count(3)
                    .with_actions(actions),
                theme,
            )
        ))
        .child(group("Empty selection", secondary,
            js_bulk_action_bar(&BulkActionBarSpec::new(), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
