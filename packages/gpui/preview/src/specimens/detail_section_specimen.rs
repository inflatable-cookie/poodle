use gpui::*;
use poodle_composites::DetailSectionSpec;
use poodle_primitives::{DetailRowLayout, DetailRowSpec, ButtonSpec, ButtonVariant, ControlSize, EyebrowSpec};
use poodle_gpui_components::{DetailRow, DetailSection, Button, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- With title and rows ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With title and rows"), theme))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new()
                            .with_title("Project details")
                            .with_description("Core metadata for this project."),
                        theme,
                    )
                    .with_body(
                        div().flex().flex_col()
                            .child(DetailRow::from_spec(DetailRowSpec::new("Name").with_value("Poodle Design System"), theme))
                            .child(DetailRow::from_spec(DetailRowSpec::new("Owner").with_value("Clay + Aura"), theme))
                            .child(DetailRow::from_spec(DetailRowSpec::new("Created").with_value("March 2025"), theme))
                            .child(DetailRow::from_spec(DetailRowSpec::new("Status").with_value("Active"), theme))
                    )
                )
        )
        // --- With actions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With actions"), theme))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new()
                            .with_title("Billing"),
                        theme,
                    )
                    .with_actions(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_size(ControlSize::Sm)
                                .with_label("Edit"),
                            theme,
                        ).with_id("ds-edit")
                    )
                    .with_body(
                        div().flex().flex_col()
                            .child(DetailRow::from_spec(DetailRowSpec::new("Plan").with_value("Pro"), theme))
                            .child(DetailRow::from_spec(DetailRowSpec::new("Billing cycle").with_value("Monthly"), theme))
                            .child(DetailRow::from_spec(DetailRowSpec::new("Next invoice").with_value("April 1, 2026"), theme))
                    )
                )
        )
        // --- DetailRow with description ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("DetailRow with description"), theme))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new()
                            .with_title("Configuration"),
                        theme,
                    )
                    .with_body(
                        div().flex().flex_col()
                            .child(DetailRow::from_spec(
                                DetailRowSpec::new("API endpoint")
                                    .with_value("https://api.example.com/v2")
                                    .with_description("The base URL for all API requests.")
                                    .with_truncate_value(true),
                                theme,
                            ))
                            .child(DetailRow::from_spec(
                                DetailRowSpec::new("Rate limit")
                                    .with_value("1,000 req/min")
                                    .with_description("Maximum requests per minute."),
                                theme,
                            ))
                    )
                )
        )
        // --- Two-column details ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Two-column details"), theme))
                .child(
                    DetailSection::from_spec(
                        DetailSectionSpec::new()
                            .with_title("Runtime summary")
                            .with_description("Compact layout for denser metadata surfaces.")
                            .with_columns(2),
                        theme,
                    )
                    .with_body(
                        div().flex().flex_wrap().gap(px(4.0))
                            .child(div().w(px(192.0)).child(
                                DetailRow::from_spec(DetailRowSpec::new("Route").with_value("local-brokered").with_layout(DetailRowLayout::Stacked), theme)
                            ))
                            .child(div().w(px(192.0)).child(
                                DetailRow::from_spec(DetailRowSpec::new("Posture").with_value("aura-local-brokered").with_layout(DetailRowLayout::Stacked), theme)
                            ))
                            .child(div().w(px(192.0)).child(
                                DetailRow::from_spec(DetailRowSpec::new("Authority").with_value("local").with_layout(DetailRowLayout::Stacked), theme)
                            ))
                            .child(div().w(px(192.0)).child(
                                DetailRow::from_spec(DetailRowSpec::new("Displays").with_value("2").with_layout(DetailRowLayout::Stacked), theme)
                            ))
                    )
                )
        )
}
