//! RelationPicker specimen — entity relationship picker.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::relation_picker::js_relation_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    BrowseState, DrillDownConfig, DrillDownItem, DrillDownLeafGroup, DrillDownLevel,
    PickerItemSpec, RelationPickerSpec, SelectionMode,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let items = vec![
        PickerItemSpec::new("a", "Approval still"),
        PickerItemSpec::new("b", "Stem waveform"),
        PickerItemSpec::new("c", "Final mix"),
    ];
    let drill_config = DrillDownConfig::new(
        vec![
            DrillDownLevel::new(
                "category",
                "Category",
                vec![
                    DrillDownItem::new("primitives", "Primitives").with_count(2),
                    DrillDownItem::new("composites", "Composites").with_count(2),
                ],
            ),
            DrillDownLevel::new(
                "subcategory",
                "Subcategory",
                vec![
                    DrillDownItem::new("controls", "Controls").with_count(2),
                    DrillDownItem::new("display", "Display").with_count(2),
                ],
            ),
        ],
        vec![
            DrillDownLeafGroup::new(
                "controls",
                vec![
                    PickerItemSpec::new("btn", "Button"),
                    PickerItemSpec::new("checkbox", "Checkbox"),
                ],
            ),
            DrillDownLeafGroup::new(
                "display",
                vec![
                    PickerItemSpec::new("badge", "Badge"),
                    PickerItemSpec::new("pill", "Pill"),
                ],
            ),
        ],
    );

    div().flex_col().gap(24.0)
        .child(group("With selection", secondary,
            js_relation_picker(
                &RelationPickerSpec::new(items.clone())
                    .with_selected_ids(vec![String::from("b"), String::from("c")])
                    .with_selection_mode(SelectionMode::Multiple),
                theme,
            )
        ))
        .child(group("No selection", secondary,
            js_relation_picker(&RelationPickerSpec::new(items), theme)
        ))
        .child(group("Single selection", secondary,
            js_relation_picker(
                &RelationPickerSpec::new(vec![
                    PickerItemSpec::new("btn", "Button"),
                    PickerItemSpec::new("dialog", "Dialog"),
                    PickerItemSpec::new("table", "Table"),
                ])
                .with_selection_mode(SelectionMode::Single)
                .with_selected_ids(vec![String::from("dialog")]),
                theme,
            )
        ))
        .child(group("Loading", secondary,
            js_relation_picker(
                &RelationPickerSpec::new(vec![
                    PickerItemSpec::new("btn", "Button"),
                    PickerItemSpec::new("dialog", "Dialog"),
                ])
                .with_state(BrowseState::Loading),
                theme,
            )
        ))
        .child(group("Drill-down", secondary,
            js_relation_picker(
                &RelationPickerSpec::new(vec![])
                    .with_drill_down(drill_config)
                    .with_drill_down_path(vec![String::from("primitives")]),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
