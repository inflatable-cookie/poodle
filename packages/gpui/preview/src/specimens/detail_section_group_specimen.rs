use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{DetailItem, DetailSection, DetailSectionGroup, Eyebrow};
use poodle_specs::{
    ControlDensity, DetailItemLayout, DetailItemSpec, DetailSectionGroupLayout,
    DetailSectionGroupSpec, DetailSectionSpec, EyebrowSpec,
};

fn section(title: &str, a: &str, b: &str, theme: &GpuiThemeProvider) -> DetailSection {
    DetailSection::from_spec(DetailSectionSpec::new().with_title(title), theme).with_body(
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(DetailItem::from_spec(
                DetailItemSpec::new("First")
                    .with_value(a)
                    .with_layout(DetailItemLayout::Stacked),
                theme,
            ))
            .child(DetailItem::from_spec(
                DetailItemSpec::new("Second")
                    .with_value(b)
                    .with_layout(DetailItemLayout::Stacked),
                theme,
            )),
    )
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Grid layout (default auto-fit) ---
        .child(
            group_block(
                "Grid layout",
                DetailSectionGroup::from_spec(
                    DetailSectionGroupSpec::new().with_aria_label("Project metadata"),
                    theme,
                )
                .child(section("General", "Platform", "Active", theme))
                .child(section("Runtime", "eu-west-1", "Production", theme))
                .child(section("Policy", "90 days", "Required", theme)),
                theme,
            ),
        )
        // --- Stack layout ---
        .child(
            group_block(
                "Stack layout",
                DetailSectionGroup::from_spec(
                    DetailSectionGroupSpec::new()
                        .with_layout(DetailSectionGroupLayout::Stack)
                        .with_item_min_column_width("10rem"),
                    theme,
                )
                .child(section("Access", "Editor", "Workspace", theme))
                .child(section("Billing", "Team", "Monthly", theme)),
                theme,
            ),
        )
        // --- Column cap (maxColumns = 2) ---
        .child(
            group_block(
                "Column cap (maxColumns = 2)",
                DetailSectionGroup::from_spec(
                    DetailSectionGroupSpec::new()
                        .with_min_column_width("10rem")
                        .with_max_columns(2),
                    theme,
                )
                .child(section("One", "1", "—", theme))
                .child(section("Two", "2", "—", theme))
                .child(section("Three", "3", "—", theme))
                .child(section("Four", "4", "—", theme)),
                theme,
            ),
        )
        // --- Density variants ---
        .child(group_block(
            "Density: compact",
            density_group(ControlDensity::Compact, theme),
            theme,
        ))
        .child(group_block(
            "Density: default",
            density_group(ControlDensity::Default, theme),
            theme,
        ))
        .child(group_block(
            "Density: comfortable",
            density_group(ControlDensity::Comfortable, theme),
            theme,
        ))
}

fn density_group(density: ControlDensity, theme: &GpuiThemeProvider) -> DetailSectionGroup {
    DetailSectionGroup::from_spec(
        DetailSectionGroupSpec::new()
            .with_density(density)
            .with_min_column_width("12rem"),
        theme,
    )
    .child(section("Density", "Mode", "Set", theme))
    .child(section("Spacing", "Inherited", "Yes", theme))
}

fn group_block(label: &str, group: DetailSectionGroup, theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(group)
}
