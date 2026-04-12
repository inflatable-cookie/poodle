use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, DetailItem, Eyebrow};
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlSize, DetailItemLayout, DetailItemPresentation,
    DetailItemSpec, EyebrowSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let success = theme.resolve_color("color.status.success");

    div().flex().flex_col().gap(px(24.0))
        // --- Basic label-value pairs ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic label-value pairs"), theme))
                .child(
                    div().flex().flex_col()
                        .child(DetailItem::from_spec(DetailItemSpec::new("Name").with_value("Poodle Design System"), theme))
                        .child(DetailItem::from_spec(DetailItemSpec::new("Version").with_value("2.1.0"), theme))
                        .child(DetailItem::from_spec(DetailItemSpec::new("License").with_value("MIT"), theme))
                )
        )
        // --- With description ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With description"), theme))
                .child(
                    DetailItem::from_spec(
                        DetailItemSpec::new("API endpoint")
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
                    DetailItem::from_spec(
                        DetailItemSpec::new("Email").with_value("clay@example.com"),
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
                    DetailItem::from_spec(
                        DetailItemSpec::new("Status"),
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
        // --- Surface presentation ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Surface presentation"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Name")
                                .with_value("Alice Chen")
                                .with_presentation(DetailItemPresentation::Surface),
                            theme,
                        ))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Role")
                                .with_value("Engineer")
                                .with_presentation(DetailItemPresentation::Surface),
                            theme,
                        ))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Email")
                                .with_value("alice@example.com")
                                .with_presentation(DetailItemPresentation::Surface),
                            theme,
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_size(ControlSize::Sm)
                                    .with_label("Edit"),
                                theme,
                            ).with_id("dr-surface-edit")
                        ))
                )
        )
        // --- Stacked layout ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Stacked layout"), theme))
                .child(
                    DetailItem::from_spec(
                        DetailItemSpec::new("Arrangement")
                            .with_value("2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A...")
                            .with_truncate_value(true)
                            .with_layout(DetailItemLayout::Stacked),
                        theme,
                    )
                )
        )
}
