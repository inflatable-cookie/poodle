use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{DetailShellSpec, DetailState, DetailSectionSpec};
use pug_gpui_components::{DetailShell, DetailRow, DetailSection, Button};
use pug_primitives::{DetailRowSpec, ButtonSpec, ButtonVariant, ControlSize};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let accent = theme.resolve_color("semantic.color.accent.base");
    let border = theme.resolve_color("semantic.color.border.subtle");

    div().flex().flex_col().gap(px(16.0))
        // --- Layout structure ---
        .child(section_label("LAYOUT STRUCTURE", text_secondary))
        .child(
            div().h(px(180.0)).child(
                DetailShell::from_spec(
                    DetailShellSpec::new(),
                    theme,
                )
                .with_header(
                    region_block("Header Region", accent, border)
                )
                .with_content(
                    div().flex().flex_col().gap(px(6.0))
                        .child(region_block("Section 1", accent, border))
                        .child(region_block("Section 2", accent, border))
                        .child(region_block("Section 3", accent, border))
                )
            )
        )
        // --- Multi-section layout with header ---
        .child(section_label("MULTI-SECTION LAYOUT WITH HEADER", text_secondary))
        .child(
            div().h(px(220.0)).child(
                DetailShell::from_spec(
                    DetailShellSpec::new()
                        .with_title("Project Settings"),
                    theme,
                )
                .with_content(
                    div().flex().flex_col().gap(px(10.0))
                        .child(
                            DetailSection::from_spec(
                                DetailSectionSpec::new().with_title("General"),
                                theme,
                            ).with_body(
                                div().flex().flex_col()
                                    .child(DetailRow::from_spec(DetailRowSpec::new("Name").with_value("My Project"), theme))
                                    .child(DetailRow::from_spec(DetailRowSpec::new("Slug").with_value("my-project"), theme))
                                    .child(DetailRow::from_spec(DetailRowSpec::new("Created").with_value("2026-01-15"), theme))
                            )
                        )
                        .child(
                            DetailSection::from_spec(
                                DetailSectionSpec::new().with_title("Configuration"),
                                theme,
                            ).with_body(
                                div().flex().flex_col()
                                    .child(DetailRow::from_spec(DetailRowSpec::new("Environment").with_value("Production"), theme))
                                    .child(DetailRow::from_spec(DetailRowSpec::new("Region").with_value("US West"), theme))
                            )
                        )
                        .child(
                            DetailSection::from_spec(
                                DetailSectionSpec::new().with_title("Integrations"),
                                theme,
                            ).with_body(
                                div().flex().flex_col()
                                    .child(DetailRow::from_spec(DetailRowSpec::new("GitHub").with_value("Connected"), theme))
                                    .child(DetailRow::from_spec(DetailRowSpec::new("Slack").with_value("Not configured"), theme))
                            )
                        )
                )
            )
        )
        // --- DetailRow: Basic label-value pairs ---
        .child(section_label("DETAIL ROW: BASIC LABEL-VALUE PAIRS", text_secondary))
        .child(
            div().flex().flex_col()
                .child(DetailRow::from_spec(
                    DetailRowSpec::new("Name").with_value("Pug Design System"),
                    theme,
                ))
                .child(DetailRow::from_spec(
                    DetailRowSpec::new("Version").with_value("2.1.0"),
                    theme,
                ))
                .child(DetailRow::from_spec(
                    DetailRowSpec::new("License").with_value("MIT"),
                    theme,
                ))
        )

        // --- DetailRow: With description ---
        .child(section_label("DETAIL ROW: WITH DESCRIPTION", text_secondary))
        .child(
            DetailRow::from_spec(
                DetailRowSpec::new("API endpoint")
                    .with_value("https://api.example.com/v2")
                    .with_description("Base URL for all API requests.")
                    .with_truncate_value(true),
                theme,
            )
        )

        // --- DetailRow: With action slot ---
        .child(section_label("DETAIL ROW: WITH ACTION", text_secondary))
        .child(
            DetailRow::from_spec(
                DetailRowSpec::new("Email").with_value("clay@example.com"),
                theme,
            )
            .with_action(
                Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(ControlSize::Sm)
                        .with_label("Change"),
                    theme,
                ).with_id("dr-change")
            )
        )

        // --- DetailRow: With custom value content ---
        .child(section_label("DETAIL ROW: WITH VALUE CONTENT", text_secondary))
        .child(
            DetailRow::from_spec(
                DetailRowSpec::new("Status"),
                theme,
            )
            .with_value_content(
                div()
                    .px(px(8.0)).py(px(2.0))
                    .rounded(px(999.0))
                    .bg(hsla(0.35, 0.5, 0.2, 0.3))
                    .child(
                        div().text_xs()
                            .text_color(hsla(0.35, 0.6, 0.6, 1.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child("Active".to_string())
                    )
            )
        )

        // --- DetailSection: With title and rows ---
        .child(section_label("DETAIL SECTION: WITH TITLE AND ROWS", text_secondary))
        .child(
            DetailSection::from_spec(
                DetailSectionSpec::new()
                    .with_title("Project details")
                    .with_description("Core metadata for this project."),
                theme,
            )
            .with_body(
                div().flex().flex_col()
                    .child(DetailRow::from_spec(DetailRowSpec::new("Name").with_value("Pug Design System"), theme))
                    .child(DetailRow::from_spec(DetailRowSpec::new("Owner").with_value("Clay + Aura"), theme))
                    .child(DetailRow::from_spec(DetailRowSpec::new("Created").with_value("March 2025"), theme))
                    .child(DetailRow::from_spec(DetailRowSpec::new("Status").with_value("Active"), theme))
            )
        )

        // --- DetailSection: With actions ---
        .child(section_label("DETAIL SECTION: WITH ACTIONS", text_secondary))
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

        // --- DetailSection: With descriptions ---
        .child(section_label("DETAIL SECTION: WITH DESCRIPTIONS", text_secondary))
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

        // --- Loading state ---
        .child(section_label("LOADING STATE", text_secondary))
        .child(
            div().h(px(100.0)).child(
                DetailShell::from_spec(
                    DetailShellSpec::new()
                        .with_title("Loading")
                        .with_state(DetailState::Loading),
                    theme,
                )
            )
        )
        // --- Error state ---
        .child(section_label("ERROR STATE", text_secondary))
        .child(
            div().h(px(100.0)).child(
                DetailShell::from_spec(
                    DetailShellSpec::new()
                        .with_title("Error")
                        .with_state(DetailState::Error),
                    theme,
                )
            )
        )
}

fn region_block(
    label: &str,
    accent: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
) -> Div {
    div()
        .h(px(32.0))
        .rounded(px(4.0))
        .border_1()
        .border_color(color_to_hsla(border))
        .bg(color_to_hsla(accent).opacity(0.08))
        .flex()
        .items_center()
        .px(px(8.0))
        .child(
            div().text_xs()
                .text_color(color_to_hsla(accent))
                .child(label.to_string()),
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
