use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, DetailItem, DetailSection, Eyebrow};
use poodle_specs::DetailSectionSpec;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, DetailItemLayout, DetailItemSpec,
    EyebrowSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
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
                    .with_body(
                        div()
                            .flex()
                            .flex_col()
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Name").with_value("Poodle Design System"),
                                theme,
                            ))
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Owner").with_value("Clay + Aura"),
                                theme,
                            ))
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Created").with_value("March 2025"),
                                theme,
                            ))
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Status").with_value("Active"),
                                theme,
                            )),
                    ),
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
                        .with_body(
                            div()
                                .flex()
                                .flex_col()
                                .child(DetailItem::from_spec(
                                    DetailItemSpec::new("Plan").with_value("Pro"),
                                    theme,
                                ))
                                .child(DetailItem::from_spec(
                                    DetailItemSpec::new("Billing cycle").with_value("Monthly"),
                                    theme,
                                ))
                                .child(DetailItem::from_spec(
                                    DetailItemSpec::new("Next invoice").with_value("April 1, 2026"),
                                    theme,
                                )),
                        ),
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
                    .with_body(
                        div()
                            .flex()
                            .flex_col()
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("API endpoint")
                                    .with_value("https://api.example.com/v2")
                                    .with_description("The base URL for all API requests.")
                                    .with_truncate_value(true),
                                theme,
                            ))
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Rate limit")
                                    .with_value("1,000 req/min")
                                    .with_description("Maximum requests per minute."),
                                theme,
                            )),
                    ),
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
                    .with_body(
                        div()
                            .flex()
                            .flex_col()
                            .child(col_item("Route", "local-brokered", theme))
                            .child(col_item("Posture", "aura-local-brokered", theme))
                            .child(col_item("Authority", "local", theme))
                            .child(col_item("Displays", "2", theme)),
                    ),
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
                        DetailSectionSpec::new()
                            .with_description("A section header carried by description text alone."),
                        theme,
                    )
                    .with_body(
                        div()
                            .flex()
                            .flex_col()
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Region").with_value("eu-west-1"),
                                theme,
                            ))
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Zone").with_value("eu-west-1a"),
                                theme,
                            )),
                    ),
                ),
        )
        // --- Density variants ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Density variants"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .child(density_demo("Compact", ControlDensity::Compact, theme))
                        .child(density_demo("Default", ControlDensity::Default, theme))
                        .child(density_demo(
                            "Comfortable",
                            ControlDensity::Comfortable,
                            theme,
                        )),
                ),
        )
}

/// A stacked detail row sized for a two-column wrapping body. The relative
/// flex-basis (half row, minus a hair) mirrors the DetailSectionGroup column
/// pattern so two items land per row inside the `columns(2)` flex-wrap body.
fn col_item(label: &str, value: &str, theme: &GpuiThemeProvider) -> Div {
    div()
        .flex_grow()
        .flex_shrink_0()
        .flex_basis(relative(0.5 - 0.01))
        .child(DetailItem::from_spec(
            DetailItemSpec::new(label)
                .with_value(value)
                .with_layout(DetailItemLayout::Stacked),
            theme,
        ))
}

fn density_demo(label: &str, density: ControlDensity, theme: &GpuiThemeProvider) -> Div {
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
            .with_body(
                div()
                    .flex()
                    .flex_col()
                    .child(col_item("Default role", "Editor", theme))
                    .child(col_item("Approvals", "Required", theme))
                    .child(col_item("Region", "eu-west-1", theme))
                    .child(col_item("Retention", "30 days", theme)),
            ),
        )
}
