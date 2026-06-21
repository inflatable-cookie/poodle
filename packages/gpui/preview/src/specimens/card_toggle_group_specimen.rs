use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{CardToggleGroup, Eyebrow};
use poodle_specs::{CardToggleGroupSpec, CardToggleOption, ControlDensity, ControlSize, EyebrowSpec};

/// Static view options reused across the specimen groups.
fn view_options() -> Vec<CardToggleOption> {
    vec![
        CardToggleOption::new("grid", "Grid view").with_description("Show records as cards."),
        CardToggleOption::new("list", "List view").with_description("Use dense rows."),
        CardToggleOption::new("board", "Board view")
            .with_description("Group records into columns."),
    ]
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
            CardToggleGroup::from_spec(
                CardToggleGroupSpec::new(view_options()).with_values(vec!["grid".into()]),
                theme,
            ),
        ))
        // --- Multiple selection (multi-select model) ---
        .child(section(
            theme,
            "Multiple selection",
            CardToggleGroup::from_spec(
                CardToggleGroupSpec::new(view_options())
                    .with_values(vec!["grid".into(), "board".into()]),
                theme,
            ),
        ))
        // --- Per-item disabled ---
        .child(section(
            theme,
            "Disabled option",
            CardToggleGroup::from_spec(
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
            CardToggleGroup::from_spec(
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
                row = row.child(CardToggleGroup::from_spec(
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
                    row = row.child(CardToggleGroup::from_spec(
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
