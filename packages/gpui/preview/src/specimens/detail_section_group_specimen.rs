use crate::app_state::AppState;
use crate::node_compat::{DetailItem, DetailSection, DetailSectionGroup, Eyebrow, IntoCompatNode};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{
    ControlDensity, DetailItemLayout, DetailItemSpec, DetailSectionGroupLayout,
    DetailSectionGroupSpec, DetailSectionSpec, EyebrowSpec,
};

fn section(title: &str, a: &str, b: &str, theme: &GpuiThemeProvider) -> DetailSection {
    let mut body = Node::container();
    body.style.descriptor.layout.direction = LayoutDirection::Column;
    body.style.descriptor.layout.spacing.gap = 6.0;
    body = body
        .child(
            DetailItem::from_spec(
                DetailItemSpec::new("First")
                    .with_value(a)
                    .with_layout(DetailItemLayout::Stacked),
                theme,
            )
            .into_compat_node(),
        )
        .child(
            DetailItem::from_spec(
                DetailItemSpec::new("Second")
                    .with_value(b)
                    .with_layout(DetailItemLayout::Stacked),
                theme,
            )
            .into_compat_node(),
        );
    DetailSection::from_spec(DetailSectionSpec::new().with_title(title), theme).with_body(body)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Grid layout (default auto-fit) ---
        .child(group_block(
            "Grid layout",
            DetailSectionGroup::from_spec(
                DetailSectionGroupSpec::new().with_aria_label("Project metadata"),
                theme,
            )
            .child(section("General", "Platform", "Active", theme))
            .child(section("Runtime", "eu-west-1", "Production", theme))
            .child(section("Policy", "90 days", "Required", theme)),
            theme,
        ))
        // --- Stack layout ---
        .child(group_block(
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
        ))
        // --- Column cap (maxColumns = 2) ---
        .child(group_block(
            "Column cap",
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
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "detail-section-group",
        examples,
        SpecimenAxes::examples_only().with_densities(|density, theme: &GpuiThemeProvider| {
            density_group(density, theme).into_any_element()
        }),
    )
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
