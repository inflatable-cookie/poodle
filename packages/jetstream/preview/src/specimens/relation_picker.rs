//! RelationPicker specimen — entity relationship picker.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::relation_picker::js_relation_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::{PickerItemSpec, RelationPickerSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let items = vec![
        PickerItemSpec::new("a", "Approval still"),
        PickerItemSpec::new("b", "Stem waveform"),
        PickerItemSpec::new("c", "Final mix"),
    ];

    div().flex_col().gap(24.0)
        .child(group("With selection", secondary,
            js_relation_picker(
                &RelationPickerSpec::new(items.clone())
                    .with_selected_ids(vec![String::from("b")]),
                theme,
            )
        ))
        .child(group("No selection", secondary,
            js_relation_picker(&RelationPickerSpec::new(items), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
