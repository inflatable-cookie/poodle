//! Card Toggle Group specimen — g12.019 node-tier migration.
//!
//! Every CardToggleGroup below renders through the node tier:
//! `poodle_render::card_toggle_group` (`Spec + Theme → Node`) interpreted by
//! `poodle_gpui_node_backend::to_gpui`. The old hand-written
//! `poodle_gpui_components::CardToggleGroup` no longer renders this specimen;
//! everything around the groups (layout, Eyebrow headings) is unchanged.
//!
//! This specimen is fully static (no handlers wired), so every instance passes
//! `None` for the `on_change` callback.

use crate::node_compat::Eyebrow;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;

use poodle_specs::{
    CardToggleGroupSpec, CardToggleOption, ControlDensity, ControlSize, EyebrowSpec,
};

/// Static view options reused across the specimen groups.
fn view_options() -> Vec<CardToggleOption> {
    vec![
        CardToggleOption::new("grid", "Grid view").with_description("Show records as cards."),
        CardToggleOption::new("list", "List view").with_description("Use dense rows."),
        CardToggleOption::new("board", "Board view")
            .with_description("Group records into columns."),
    ]
}

/// A node-tier CardToggleGroup with no handlers (this specimen is fully static).
fn node_card_toggle_group(spec: CardToggleGroupSpec, theme: &GpuiThemeProvider) -> AnyElement {
    let node = poodle_render::card_toggle_group(&spec, theme, None);
    poodle_gpui_node_backend::to_gpui(&node)
}

fn section(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(720.0))
        // --- Single selection ---
        .child(section(
            theme,
            "View mode (single selection)",
            node_card_toggle_group(
                CardToggleGroupSpec::new(view_options()).with_values(vec!["grid".into()]),
                theme,
            ),
        ))
        // --- Multiple selection (multi-select model) ---
        .child(section(
            theme,
            "Multiple selection",
            node_card_toggle_group(
                CardToggleGroupSpec::new(view_options())
                    .with_values(vec!["grid".into(), "board".into()]),
                theme,
            ),
        ))
        // --- Per-item disabled ---
        .child(section(
            theme,
            "Disabled option",
            node_card_toggle_group(
                CardToggleGroupSpec::new(vec![
                    CardToggleOption::new("draft", "Draft").with_description("In progress."),
                    CardToggleOption::new("live", "Live").with_description("Published."),
                    CardToggleOption::new("archived", "Archived")
                        .with_description("Read-only snapshot.")
                        .with_disabled(true),
                ])
                .with_values(vec!["live".into()]),
                theme,
            ),
        ))
        // --- Group disabled ---
        .child(section(
            theme,
            "Disabled group",
            node_card_toggle_group(
                CardToggleGroupSpec::new(view_options())
                    .with_values(vec!["list".into()])
                    .with_disabled(true),
                theme,
            ),
        ))
        // --- Sizes (xs → xl) ---
        .child(section(theme, "Sizes (xs / sm / md / lg / xl)", {
            let mut row = div().flex().flex_col().gap(px(12.0));
            for size in [
                ControlSize::Xs,
                ControlSize::Sm,
                ControlSize::Md,
                ControlSize::Lg,
                ControlSize::Xl,
            ] {
                row = row.child(node_card_toggle_group(
                    CardToggleGroupSpec::new(view_options())
                        .with_values(vec!["grid".into()])
                        .with_size(size),
                    theme,
                ));
            }
            row
        }))
        // --- Densities (compact / default / comfortable) ---
        .child(section(
            theme,
            "Densities (compact / default / comfortable)",
            {
                let mut row = div().flex().flex_col().gap(px(12.0));
                for density in [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ] {
                    row = row.child(node_card_toggle_group(
                        CardToggleGroupSpec::new(view_options())
                            .with_values(vec!["grid".into()])
                            .with_density(density),
                        theme,
                    ));
                }
                row
            },
        ))
}
