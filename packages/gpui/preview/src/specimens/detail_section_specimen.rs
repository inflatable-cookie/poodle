use crate::app_state::AppState;
use crate::node_compat::{Button, DetailItem, DetailSection, Eyebrow, IntoCompatNode};
use crate::specimens::specimen_axes::density_key;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::DetailSectionSpec;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, DetailItemLayout, DetailItemSpec,
    EyebrowSpec,
};

fn node_column(children: Vec<Node>) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.direction = LayoutDirection::Column;
    children.into_iter().fold(node, Node::child)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- With title and rows ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With title and rows"),
                    theme,
                ))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new()
                            .with_title("Project details")
                            .with_description("Core metadata for this project."),
                        theme,
                    )
                    .with_body(node_column(vec![
                        DetailItem::from_spec(
                            DetailItemSpec::new("Name").with_value("Poodle Design System"),
                            theme,
                        )
                        .into_compat_node(),
                        DetailItem::from_spec(
                            DetailItemSpec::new("Owner").with_value("Clay + Aura"),
                            theme,
                        )
                        .into_compat_node(),
                        DetailItem::from_spec(
                            DetailItemSpec::new("Created").with_value("March 2025"),
                            theme,
                        )
                        .into_compat_node(),
                        DetailItem::from_spec(
                            DetailItemSpec::new("Status").with_value("Active"),
                            theme,
                        )
                        .into_compat_node(),
                    ])),
                ),
        )
        // --- With actions ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With actions"),
                    theme,
                ))
                .child(
                    DetailSection::from_spec(DetailSectionSpec::new().with_title("Billing"), theme)
                        .with_actions(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_size(ControlSize::Sm)
                                    .with_label("Edit"),
                                theme,
                            )
                            .with_id("ds-edit"),
                        )
                        .with_body(node_column(vec![
                            DetailItem::from_spec(
                                DetailItemSpec::new("Plan").with_value("Pro"),
                                theme,
                            )
                            .into_compat_node(),
                            DetailItem::from_spec(
                                DetailItemSpec::new("Billing cycle").with_value("Monthly"),
                                theme,
                            )
                            .into_compat_node(),
                            DetailItem::from_spec(
                                DetailItemSpec::new("Next invoice").with_value("April 1, 2026"),
                                theme,
                            )
                            .into_compat_node(),
                        ])),
                ),
        )
        // --- DetailItem with description ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("DetailItem with description"),
                    theme,
                ))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new().with_title("Configuration"),
                        theme,
                    )
                    .with_body(node_column(vec![
                        DetailItem::from_spec(
                            DetailItemSpec::new("API endpoint")
                                .with_value("https://api.example.com/v2")
                                .with_description("The base URL for all API requests.")
                                .with_truncate_value(true),
                            theme,
                        )
                        .into_compat_node(),
                        DetailItem::from_spec(
                            DetailItemSpec::new("Rate limit")
                                .with_value("1,000 req/min")
                                .with_description("Maximum requests per minute."),
                            theme,
                        )
                        .into_compat_node(),
                    ])),
                ),
        )
        // --- Two-column details ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Two-column details"),
                    theme,
                ))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new()
                            .with_title("Runtime summary")
                            .with_description("Compact layout for denser metadata surfaces.")
                            .with_columns(2),
                        theme,
                    )
                    .with_body(node_column(vec![
                        col_item("Route", "local-brokered", theme),
                        col_item("Posture", "aura-local-brokered", theme),
                        col_item("Authority", "local", theme),
                        col_item("Displays", "2", theme),
                    ])),
                ),
        )
        // --- Description only (no title) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Description only (no title)"),
                    theme,
                ))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new().with_description(
                            "A section header carried by description text alone.",
                        ),
                        theme,
                    )
                    .with_body(node_column(vec![
                        DetailItem::from_spec(
                            DetailItemSpec::new("Region").with_value("eu-west-1"),
                            theme,
                        )
                        .into_compat_node(),
                        DetailItem::from_spec(
                            DetailItemSpec::new("Zone").with_value("eu-west-1a"),
                            theme,
                        )
                        .into_compat_node(),
                    ])),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "detail-section",
        examples,
        SpecimenAxes::examples_only().with_densities(|density, theme: &GpuiThemeProvider| {
            density_demo(density, theme).into_any_element()
        }),
    )
}

/// A stacked detail row sized for a two-column wrapping body. The relative
/// flex-basis (half row, minus a hair) mirrors the DetailSectionGroup column
/// pattern so two items land per row inside the `columns(2)` flex-wrap body.
fn col_item(label: &str, value: &str, theme: &GpuiThemeProvider) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.direction = LayoutDirection::Row;
    node.style.flex_grow = Some(1.0);
    node.style.flex_shrink_zero = true;
    node.child(
        DetailItem::from_spec(
            DetailItemSpec::new(label)
                .with_value(value)
                .with_layout(DetailItemLayout::Stacked),
            theme,
        )
        .into_compat_node(),
    )
}

fn density_demo(density: ControlDensity, theme: &GpuiThemeProvider) -> Div {
    let label = density_key(density);
    let muted = theme.resolve_color("color.text.muted");
    div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .text_xs()
                .text_color(crate::style_bridge::color_to_hsla(muted))
                .child(label.to_string()),
        )
        .child(
            DetailSection::from_spec(
                DetailSectionSpec::new()
                    .with_title("Workspace access")
                    .with_description("Shared settings and runtime defaults.")
                    .with_columns(2)
                    .with_density(density),
                theme,
            )
            .with_body(node_column(vec![
                col_item("Default role", "Editor", theme),
                col_item("Approvals", "Required", theme),
                col_item("Region", "eu-west-1", theme),
                col_item("Retention", "30 days", theme),
            ])),
        )
}
