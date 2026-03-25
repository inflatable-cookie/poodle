use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{DetailRowLayout, DetailRowSpec, ButtonSpec, ButtonVariant, ControlSize, EyebrowSpec};
use poodle_gpui_components::{DetailRow, Button, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let success = theme.resolve_color("semantic.color.status.success");

    div().flex().flex_col().gap(px(24.0))
        // --- Basic label-value pairs ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic label-value pairs"), theme))
                .child(
                    div().flex().flex_col()
                        .child(DetailRow::from_spec(DetailRowSpec::new("Name").with_value("Poodle Design System"), theme))
                        .child(DetailRow::from_spec(DetailRowSpec::new("Version").with_value("2.1.0"), theme))
                        .child(DetailRow::from_spec(DetailRowSpec::new("License").with_value("MIT"), theme))
                )
        )
        // --- With description ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With description"), theme))
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
        // --- With action slot ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With action slot"), theme))
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
        // --- With value slot ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With value slot"), theme))
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
        // --- Stacked layout ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Stacked layout"), theme))
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
}
