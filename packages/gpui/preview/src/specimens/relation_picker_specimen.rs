use gpui::*;
use poodle_composites::{BrowseState, PickerItemSpec, RelationPickerSpec, SelectionMode};
use poodle_primitives::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};
use poodle_gpui_components::{Eyebrow, RelationPicker};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let items = || vec![
        PickerItemSpec::new("btn", "Button"),
        PickerItemSpec::new("chk", "Checkbox"),
        PickerItemSpec::new("sel", "Select"),
        PickerItemSpec::new("dlg", "Dialog"),
        PickerItemSpec::new("tbl", "Table"),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- Multiple selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiple selection"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selected_ids(vec!["btn".to_string(), "dlg".to_string()])
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Ready),
                        theme,
                    )
                )
        )
        // --- Single selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single selection"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selected_ids(vec!["sel".to_string()])
                            .with_selection_mode(SelectionMode::Single)
                            .with_state(BrowseState::Ready),
                        theme,
                    )
                )
        )
        // --- Loading state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading state"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Loading),
                        theme,
                    )
                )
        )
        // --- Semantic presentation (chrome size role, comfortable density) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Semantic presentation"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selected_ids(vec!["btn".to_string()])
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Ready)
                            .with_size(ControlSize::Sm)
                            .with_size_role(SemanticControlSizeRole::Chrome)
                            .with_density(ControlDensity::Comfortable),
                        theme,
                    )
                )
        )
}
