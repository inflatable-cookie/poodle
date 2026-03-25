use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_composites::{DetailShellSpec, DetailState, DetailSectionSpec};
use poodle_gpui_components::{DetailShell, DetailRow, DetailSection, Button, Eyebrow};
use poodle_primitives::{DetailRowLayout, DetailRowSpec, ButtonSpec, ButtonVariant, ControlSize, EyebrowSpec};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");
    let border = theme.resolve_color("semantic.color.border.subtle");
    let success = theme.resolve_color("semantic.color.status.success");

    div().flex().flex_col().gap(px(24.0))
        // --- Layout structure ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Layout structure"), theme))
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
        )
        // --- Multi-section layout with header ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multi-section layout with header"), theme))
                .child(
                    div().h(px(220.0)).child(
                        DetailShell::from_spec(
                            DetailShellSpec::new()
                                .with_title("Project Settings"),
                            theme,
                        )
                        .with_content(
                            div().flex().flex_col().gap(px(8.0))
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
        )
        // --- DetailRow: Basic label-value pairs ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail row: basic label-value pairs"), theme))
                .child(
                    div().flex().flex_col()
                        .child(DetailRow::from_spec(
                            DetailRowSpec::new("Name").with_value("Poodle Design System"),
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
        )

        // --- DetailRow: With description ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail row: with description"), theme))
                .child(
                    DetailRow::from_spec(
                        DetailRowSpec::new("API endpoint")
                            .with_value("https://api.example.com/v2")
                            .with_description("Base URL for all API requests.")
                            .with_truncate_value(true),
                        theme,
                    )
                )
        )

        // --- DetailRow: With action slot ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail row: with action"), theme))
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
        )

        // --- DetailRow: With custom value content ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail row: with value content"), theme))
                .child(
                    DetailRow::from_spec(
                        DetailRowSpec::new("Status"),
                        theme,
                    )
                    .with_value_content(
                        div()
                            .px(px(8.0)).py(px(2.0))
                            .rounded(px(999.0))
                            .bg(color_to_hsla(success).opacity(0.15))
                            .child(
                                div().text_xs()
                                    .text_color(color_to_hsla(success))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Active".to_string())
                            )
                    )
                )
        )

        // --- DetailRow: Stacked layout ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail row: stacked layout"), theme))
                .child(
                    DetailRow::from_spec(
                        DetailRowSpec::new("Arrangement")
                            .with_value("2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A...")
                            .with_truncate_value(true)
                            .with_layout(DetailRowLayout::Stacked),
                        theme,
                    )
                )
        )

        // --- DetailSection: With title and rows ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail section: with title and rows"), theme))
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

        // --- DetailSection: With actions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail section: with actions"), theme))
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

        // --- DetailSection: With descriptions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail section: with descriptions"), theme))
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

        // --- DetailSection: Two-column details ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detail section: two-column details"), theme))
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

        // --- Loading state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading state"), theme))
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
        )
        // --- Error state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Error state"), theme))
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
        )
}

fn region_block(
    label: &str,
    accent: poodle_tokens::typed::ColorValue,
    border: poodle_tokens::typed::ColorValue,
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
