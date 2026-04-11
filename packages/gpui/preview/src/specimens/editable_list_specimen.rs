use gpui::*;
use poodle_gpui_components::{EditableList, Eyebrow};
use poodle_components::EyebrowSpec;
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0)).max_w(px(480.0))
        // --- Editable + reorderable ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Editable + reorderable"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Tags")
                        .add_label("Add tag")
                        .placeholder("Enter a tag…")
                        .items(vec![
                            "design-system".to_string(),
                            "rust".to_string(),
                            "gpui".to_string(),
                        ])
                        .reorderable(true)
                )
        )
        // --- With max items (5) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With max items (5)"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Limited tags")
                        .add_label("Add tag")
                        .placeholder("Enter a tag…")
                        .items(vec![
                            "alpha".to_string(),
                            "beta".to_string(),
                            "gamma".to_string(),
                        ])
                        .max_items(5)
                        .reorderable(true)
                )
        )
        // --- Removable only (no reorder, no add) ---
        // Represented by setting max_items equal to the item count so the
        // spec reports can_add() == false, and leaving reorderable off.
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Removable only (no reorder, no add)"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Assigned reviewers")
                        .items(vec![
                            "alice".to_string(),
                            "bob".to_string(),
                            "carol".to_string(),
                        ])
                        .max_items(3)
                        .reorderable(false)
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Locked list")
                        .items(vec![
                            "locked-item".to_string(),
                            "frozen-value".to_string(),
                        ])
                        .disabled(true)
                )
        )
        // --- Dirty (unsaved changes) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Dirty (unsaved changes)"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Tags")
                        .items(vec![
                            "design-system".to_string(),
                            "rust".to_string(),
                            "gpui".to_string(),
                            "new-tag".to_string(),
                        ])
                        .max_items(10)
                        .dirty(true)
                )
        )
        // --- Submitting state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Submitting state"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Tags")
                        .items(vec![
                            "design-system".to_string(),
                            "rust".to_string(),
                            "gpui".to_string(),
                        ])
                        .submitting(true)
                )
        )
        // --- With error message ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With error message"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Tags")
                        .items(vec![
                            "design-system".to_string(),
                            "rust".to_string(),
                        ])
                        .error_message("Failed to save: 'rust' conflicts with an existing tag.")
                )
        )
        // --- With info message ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With info message"), theme))
                .child(
                    EditableList::new(theme)
                        .aria_label("Tags")
                        .items(vec![
                            "design-system".to_string(),
                            "rust".to_string(),
                            "gpui".to_string(),
                        ])
                        .info_message("Tags are shared across the workspace.")
                )
        )
}
