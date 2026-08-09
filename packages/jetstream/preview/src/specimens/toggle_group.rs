//! ToggleGroup specimen — single/multiple selection, disabled states, plus
//! full size and density matrices. Mirrors the contract §13 specimens and the
//! Svelte ToggleGroupSpecimen (Grid/List/Board + alignment sets).
//!
//! NOTE: `js_toggle_group` is render-only — selection is drawn from the spec
//! value; click/key/`on_change` wiring lives in the preview event loop per
//! Jetstream architecture (accepted limit, see parity/toggle-group.md).

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use crate::compat::js_toggle_group;
use poodle_specs::{
    ControlDensity, ControlSize, ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Single selection — Grid / List / Board, Grid selected.
        .child(group(
            "Single selection",
            secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(view_opts())
                    .with_selection_mode(ToggleGroupSelectionMode::Single)
                    .with_aria_label("View mode")
                    .with_value(vec!["grid".into()]),
                theme,
            ),
        ))
        // Four options — Left / Center / Right / Justify, Left selected.
        .child(group(
            "Four options",
            secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(vec![
                    ToggleGroupOption::new("left", "Left"),
                    ToggleGroupOption::new("center", "Center"),
                    ToggleGroupOption::new("right", "Right"),
                    ToggleGroupOption::new("justify", "Justify"),
                ])
                .with_aria_label("Text alignment")
                .with_value(vec!["left".into()]),
                theme,
            ),
        ))
        // Multiple selection — Design + Docs selected, Engineering off.
        .child(group(
            "Multiple selection",
            secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(vec![
                    ToggleGroupOption::new("design", "Design"),
                    ToggleGroupOption::new("engineering", "Engineering"),
                    ToggleGroupOption::new("docs", "Docs"),
                ])
                .with_selection_mode(ToggleGroupSelectionMode::Multiple)
                .with_aria_label("Filter tags")
                .with_value(vec!["design".into(), "docs".into()]),
                theme,
            ),
        ))
        // Allow deactivation — single mode that can clear on re-select.
        .child(group(
            "Allow deactivation",
            secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(view_opts())
                    .with_aria_label("Optional view mode")
                    .with_allow_deactivation(true)
                    .with_value(vec!["grid".into()]),
                theme,
            ),
        ))
        // With disabled item — List muted, Grid selected.
        .child(group(
            "With disabled item",
            secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(vec![
                    ToggleGroupOption::new("grid", "Grid"),
                    ToggleGroupOption::new("list", "List").with_disabled(true),
                    ToggleGroupOption::new("board", "Board"),
                ])
                .with_aria_label("Toggle group with disabled item")
                .with_value(vec!["grid".into()]),
                theme,
            ),
        ))
        // Fully disabled — whole group muted, List shows selected styling.
        .child(group(
            "Fully disabled",
            secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(view_opts())
                    .with_aria_label("Disabled toggle group")
                    .with_value(vec!["list".into()])
                    .with_disabled(true),
                theme,
            ),
        ))
        // Sizes — full xs–xl ladder.
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_end()
                .child(sized(theme, ControlSize::Xs))
                .child(sized(theme, ControlSize::Sm))
                .child(sized(theme, ControlSize::Md))
                .child(sized(theme, ControlSize::Lg))
                .child(sized(theme, ControlSize::Xl)),
        ))
        // Densities — compact / default / comfortable (height unchanged).
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_row()
                .gap(16.0)
                .items_center()
                .child(dense(theme, ControlDensity::Compact))
                .child(dense(theme, ControlDensity::Default))
                .child(dense(theme, ControlDensity::Comfortable)),
        ))
}

fn view_opts() -> Vec<ToggleGroupOption> {
    vec![
        ToggleGroupOption::new("grid", "Grid"),
        ToggleGroupOption::new("list", "List"),
        ToggleGroupOption::new("board", "Board"),
    ]
}

fn sized(theme: &JetstreamThemeProvider, size: ControlSize) -> El {
    js_toggle_group(
        &ToggleGroupSpec::new(view_opts())
            .with_value(vec!["grid".into()])
            .with_size(size),
        theme,
    )
}

fn dense(theme: &JetstreamThemeProvider, density: ControlDensity) -> El {
    js_toggle_group(
        &ToggleGroupSpec::new(view_opts())
            .with_value(vec!["grid".into()])
            .with_density(density),
        theme,
    )
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
