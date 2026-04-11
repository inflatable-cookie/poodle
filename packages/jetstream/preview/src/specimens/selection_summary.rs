//! SelectionSummary specimen — summary of selected items.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::selection_summary::js_selection_summary;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{SelectionSummaryItem, SelectionSummarySpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let items = vec![
        SelectionSummaryItem::new("a", "Approval still"),
        SelectionSummaryItem::new("b", "Stem waveform"),
        SelectionSummaryItem::new("c", "Final mix"),
    ];

    div().flex_col().gap(24.0)
        .child(group("Multiple selections", secondary,
            js_selection_summary(&SelectionSummarySpec::new(items), theme)
        ))
        .child(group("Single selection", secondary,
            js_selection_summary(
                &SelectionSummarySpec::new(vec![
                    SelectionSummaryItem::new("x", "Selected item"),
                ]),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
