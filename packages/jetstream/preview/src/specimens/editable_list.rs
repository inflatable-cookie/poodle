//! EditableList specimen — rows, add row, counter, workflow chrome, banners.
//!
//! Covers the renderable contract anatomy: handle + content + remove rows, the
//! add affordance, empty state, max/counter, workflow header (cancel/submit +
//! saving), error/info banners, reorderable handles, and size/density sweeps.
//! Windowed mode / long-list warning are feature-scope unbuilt in
//! `js_editable_list` and are intentionally omitted.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_editable_list;

use poodle_specs::{ControlDensity, ControlSize, EditableListSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Editable + reorderable: handle + content + remove + add row
        .child(group(
            "Editable + reorderable",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(3)
                    .with_add_label("Add")
                    .with_placeholder("Add a tag…")
                    .with_aria_label("Tags")
                    .with_editable(true)
                    .with_reorderable(true),
                theme,
            ),
        ))
        // Empty: only the add row
        .child(group(
            "Empty (add row only)",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(0)
                    .with_add_label("Add")
                    .with_placeholder("Add a tag…")
                    .with_editable(true),
                theme,
            ),
        ))
        // Max items (5): counter "3/5", add row visible
        .child(group(
            "With max items (5)",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(3)
                    .with_add_label("Add")
                    .with_placeholder("Add item…")
                    .with_aria_label("Limited list")
                    .with_editable(true)
                    .with_max_items(5),
                theme,
            ),
        ))
        // At max (3/3): add row hidden, counter shows
        .child(group(
            "At max (3/3)",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(3)
                    .with_aria_label("Full list")
                    .with_editable(true)
                    .with_max_items(3),
                theme,
            ),
        ))
        // Removable only: remove buttons, no reorder handle, no add row
        .child(group(
            "Removable only (no reorder, no add)",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(2)
                    .with_aria_label("Static list")
                    .with_removable(true)
                    .with_reorderable(false),
                theme,
            ),
        ))
        // Reorderable: drag handles, no add/remove
        .child(group(
            "Reorderable (drag handles)",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(5)
                    .with_aria_label("Reorderable items")
                    .with_reorderable(true),
                theme,
            ),
        ))
        // Workflow chrome — dirty: Save Order enabled + Cancel
        .child(group(
            "Workflow chrome (dirty)",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(4)
                    .with_aria_label("Tags")
                    .with_reorderable(true)
                    .with_dirty(true),
                theme,
            ),
        ))
        // Workflow chrome — submitting: "Saving…"
        .child(group(
            "Workflow chrome (saving)",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(3)
                    .with_aria_label("Tags")
                    .with_reorderable(true)
                    .with_dirty(true)
                    .with_submitting(true),
                theme,
            ),
        ))
        // Error banner
        .child(group(
            "Error banner",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(2)
                    .with_aria_label("Tags")
                    .with_editable(true)
                    .with_error_message("Failed to save: 'rust' conflicts with an existing tag."),
                theme,
            ),
        ))
        // Info banner
        .child(group(
            "Info banner",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(3)
                    .with_aria_label("Tags")
                    .with_editable(true)
                    .with_info_message("Tags are shared across the workspace."),
                theme,
            ),
        ))
        // Disabled: reduced opacity, no interaction
        .child(group(
            "Disabled",
            secondary,
            js_editable_list(
                &EditableListSpec::new()
                    .with_item_count(2)
                    .with_aria_label("Locked list")
                    .with_editable(true)
                    .with_disabled(true),
                theme,
            ),
        ))
        // Sizes sweep (xs → xl)
        .child(group("Sizes", secondary, sizes_row(theme)))
        // Densities sweep (compact / default / comfortable)
        .child(group("Densities", secondary, densities_row(theme)))
}

const SIZES: &[(ControlSize, &str)] = &[
    (ControlSize::Xs, "xs"),
    (ControlSize::Sm, "sm"),
    (ControlSize::Md, "md"),
    (ControlSize::Lg, "lg"),
    (ControlSize::Xl, "xl"),
];

const DENSITIES: &[(ControlDensity, &str)] = &[
    (ControlDensity::Compact, "compact"),
    (ControlDensity::Default, "default"),
    (ControlDensity::Comfortable, "comfortable"),
];

fn sized_list(theme: &JetstreamThemeProvider, spec: EditableListSpec) -> El {
    js_editable_list(
        &spec
            .with_item_count(3)
            .with_add_label("Add")
            .with_placeholder("Add a tag…")
            .with_aria_label("Sized list")
            .with_editable(true)
            .with_reorderable(true),
        theme,
    )
}

fn sizes_row(theme: &JetstreamThemeProvider) -> El {
    let mut col = div().flex_col().gap(12.0);
    for (size, _label) in SIZES {
        col = col.child(sized_list(theme, EditableListSpec::new().with_size(*size)));
    }
    col
}

fn densities_row(theme: &JetstreamThemeProvider) -> El {
    let mut col = div().flex_col().gap(12.0);
    for (density, _label) in DENSITIES {
        col = col.child(sized_list(theme, EditableListSpec::new().with_density(*density)));
    }
    col
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
