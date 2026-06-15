use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{DetailItem, DetailSection, DetailSectionGroup};
use poodle_specs::{
    ControlDensity, DetailItemLayout, DetailItemSpec, DetailSectionGroupLayout,
    DetailSectionGroupSpec, DetailSectionSpec,
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
        .gap(px(20.0))
        .child(
            DetailSectionGroup::from_spec(DetailSectionGroupSpec::new(), theme)
                .child(section("General", "Platform", "Active", theme))
                .child(section("Runtime", "eu-west-1", "Production", theme))
                .child(section("Policy", "90 days", "Required", theme)),
        )
        .child(
            DetailSectionGroup::from_spec(
                DetailSectionGroupSpec::new()
                    .with_layout(DetailSectionGroupLayout::Stack)
                    .with_density(ControlDensity::Compact),
                theme,
            )
            .child(section("Access", "Editor", "Workspace", theme))
            .child(section("Billing", "Team", "Monthly", theme)),
        )
}
