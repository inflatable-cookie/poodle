use gpui::*;
use poodle_composites::{RelationPickerSpec, PickerItemSpec, BrowseState, SelectionMode};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{RelationPicker, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let items = vec![
        PickerItemSpec::new("btn", "Button"),
        PickerItemSpec::new("chk", "Checkbox"),
        PickerItemSpec::new("sel", "Select"),
        PickerItemSpec::new("dlg", "Dialog"),
        PickerItemSpec::new("tbl", "Table"),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items)
                            .with_selected_ids(vec!["btn".to_string(), "dlg".to_string()])
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Ready),
                        theme,
                    )
                )
        )
}
