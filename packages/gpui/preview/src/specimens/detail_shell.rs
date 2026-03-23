use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{DetailShellSpec, DetailState};
use pug_gpui_components::{DetailShell, DetailRow, Button};
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
                        .child(detail_section("General", &[
                            ("Name", "My Project"),
                            ("Slug", "my-project"),
                            ("Created", "2026-01-15"),
                        ], text_secondary, border))
                        .child(detail_section("Configuration", &[
                            ("Environment", "Production"),
                            ("Region", "US West"),
                        ], text_secondary, border))
                        .child(detail_section("Integrations", &[
                            ("GitHub", "Connected"),
                            ("Slack", "Not configured"),
                        ], text_secondary, border))
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

fn detail_section(
    title: &str,
    rows: &[(&str, &str)],
    text_secondary: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
) -> Div {
    let mut section = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .pb(px(8.0))
        .border_b_1()
        .border_color(color_to_hsla(border));

    section = section.child(
        div()
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .child(title.to_string()),
    );

    for (key, value) in rows {
        section = section.child(
            div().flex().gap(px(8.0))
                .child(
                    div().text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(key.to_string()),
                )
                .child(
                    div().text_xs()
                        .child(value.to_string()),
                ),
        );
    }

    section
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
