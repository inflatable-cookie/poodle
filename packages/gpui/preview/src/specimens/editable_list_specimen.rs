use crate::app_state::AppState;
use crate::node_compat::{EditableList, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EyebrowSpec;

fn group(title: &str, theme: &GpuiThemeProvider, body: EditableList) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(body)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(480.0))
        // --- Editable + reorderable (handle + content + remove + add row) ---
        .child(group(
            "Editable + reorderable",
            theme,
            EditableList::new(theme)
                .aria_label("Tags")
                .add_label("Add")
                .placeholder("Add a tag…")
                .items(vec![
                    "svelte".to_string(),
                    "typescript".to_string(),
                    "design-system".to_string(),
                ])
                .editable(true)
                .reorderable(true),
        ))
        // --- Empty state (only add row) ---
        .child(group(
            "Empty (add row only)",
            theme,
            EditableList::new(theme)
                .aria_label("Tags")
                .add_label("Add")
                .placeholder("Add a tag…")
                .items(vec![])
                .editable(true),
        ))
        // --- With max items (5) → counter "3/5" ---
        .child(group(
            "With max items (5)",
            theme,
            EditableList::new(theme)
                .aria_label("Limited list")
                .add_label("Add")
                .placeholder("Add item…")
                .items(vec![
                    "Item A".to_string(),
                    "Item B".to_string(),
                    "Item C".to_string(),
                ])
                .editable(true)
                .max_items(5)
                .reorderable(true),
        ))
        // --- At max (3/3) → add row hidden, counter shows ---
        .child(group(
            "At max (3/3)",
            theme,
            EditableList::new(theme)
                .aria_label("Full list")
                .items(vec![
                    "alpha".to_string(),
                    "beta".to_string(),
                    "gamma".to_string(),
                ])
                .editable(true)
                .max_items(3),
        ))
        // --- Removable only (remove buttons, no reorder, no add) ---
        .child(group(
            "Removable only (no reorder, no add)",
            theme,
            EditableList::new(theme)
                .aria_label("Static list")
                .items(vec!["First item".to_string(), "Second item".to_string()])
                .removable(true)
                .reorderable(false),
        ))
        // --- Reorderable (handles, no add/remove) ---
        .child(group(
            "Reorderable (drag handles)",
            theme,
            EditableList::new(theme)
                .aria_label("Reorderable items")
                .items(vec![
                    "One".to_string(),
                    "Two".to_string(),
                    "Three".to_string(),
                    "Four".to_string(),
                    "Five".to_string(),
                ])
                .reorderable(true),
        ))
        // --- Workflow chrome: dirty (Save Order enabled + Cancel) ---
        .child(group(
            "Workflow chrome (dirty)",
            theme,
            EditableList::new(theme)
                .aria_label("Tags")
                .items(vec![
                    "design-system".to_string(),
                    "rust".to_string(),
                    "gpui".to_string(),
                    "new-tag".to_string(),
                ])
                .reorderable(true)
                .dirty(true),
        ))
        // --- Workflow chrome: submitting (Saving…) ---
        .child(group(
            "Workflow chrome (saving)",
            theme,
            EditableList::new(theme)
                .aria_label("Tags")
                .items(vec![
                    "design-system".to_string(),
                    "rust".to_string(),
                    "gpui".to_string(),
                ])
                .reorderable(true)
                .dirty(true)
                .submitting(true),
        ))
        // --- Error banner ---
        .child(group(
            "Error banner",
            theme,
            EditableList::new(theme)
                .aria_label("Tags")
                .items(vec!["design-system".to_string(), "rust".to_string()])
                .editable(true)
                .error_message("Failed to save: 'rust' conflicts with an existing tag."),
        ))
        // --- Info banner ---
        .child(group(
            "Info banner",
            theme,
            EditableList::new(theme)
                .aria_label("Tags")
                .items(vec![
                    "design-system".to_string(),
                    "rust".to_string(),
                    "gpui".to_string(),
                ])
                .editable(true)
                .info_message("Tags are shared across the workspace."),
        ))
        // --- Disabled ---
        .child(group(
            "Disabled",
            theme,
            EditableList::new(theme)
                .aria_label("Locked list")
                .items(vec!["locked-item".to_string(), "frozen-value".to_string()])
                .editable(true)
                .disabled(true),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "editable-list",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                sample_list(theme).with_size(size).into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                sample_list(theme).with_density(density).into_any_element()
            }),
    )
}

fn sample_list(theme: &GpuiThemeProvider) -> EditableList {
    EditableList::new(theme)
        .aria_label("Sized list")
        .add_label("Add")
        .placeholder("Add a tag…")
        .items(vec![
            "svelte".to_string(),
            "typescript".to_string(),
            "design-system".to_string(),
        ])
        .editable(true)
        .reorderable(true)
}
