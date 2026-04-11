use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_specs::AppHeaderSpec;
use poodle_gpui_components::{AppHeader, Button, IconButton, Eyebrow};
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, IconButtonSpec, EyebrowSpec};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");

    div().flex().flex_col().gap(px(24.0))
        // --- Full app header ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Full app window header (title + menubar + utility)"), theme))
                .child(
                    AppHeader::from_spec(
                        AppHeaderSpec::new()
                            .with_title("Poodle Studio")
                            .with_drag_region(true)
                            .with_aria_label("Application header"),
                        theme,
                    )
                    .with_primary_actions(
                        div().flex().items_center().gap(px(4.0))
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Ghost)
                                        .with_label("File")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ah-file")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Ghost)
                                        .with_label("Edit")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ah-edit")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Ghost)
                                        .with_label("View")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ah-view")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Ghost)
                                        .with_label("Help")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ah-help")
                            )
                    )
                    .with_utility_items(
                        div().flex().items_center().gap(px(4.0))
                            .child(IconButton::from_spec(IconButtonSpec::new().with_icon("search").with_size(ControlSize::Sm), theme).with_id("ah-search"))
                            .child(IconButton::from_spec(IconButtonSpec::new().with_icon("bell").with_size(ControlSize::Sm), theme).with_id("ah-bell"))
                            .child(IconButton::from_spec(IconButtonSpec::new().with_icon("settings").with_size(ControlSize::Sm), theme).with_id("ah-settings"))
                    )
                )
        )

        // --- Simple with actions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With title, actions, and utility"), theme))
                .child(
                    AppHeader::from_spec(
                        AppHeaderSpec::new()
                            .with_title("Dashboard"),
                        theme,
                    )
                    .with_primary_actions(
                        div().flex().items_center().gap(px(6.0))
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Export")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ah-export")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Primary)
                                        .with_label("New Project")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ah-new")
                            )
                    )
                    .with_utility_items(
                        div().flex().items_center()
                            .child(IconButton::from_spec(IconButtonSpec::new().with_icon("settings").with_size(ControlSize::Sm), theme).with_id("ah-settings2"))
                    )
                )
        )

        // --- Title only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Title only"), theme))
                .child(
                    AppHeader::from_spec(
                        AppHeaderSpec::new()
                            .with_title("Settings"),
                        theme,
                    )
                )
        )

        // --- Custom identity (leading slot) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom identity slot"), theme))
                .child(
                    AppHeader::from_spec(
                        AppHeaderSpec::new()
                            .with_aria_label("Custom identity header"),
                        theme,
                    )
                    .with_leading(
                        div().flex().items_center().gap(px(8.0))
                            .child(
                                div().w(px(20.0)).h(px(20.0)).rounded(px(4.0))
                                    .bg(color_to_hsla(theme.resolve_color("color.accent.base")))
                            )
                            .child(
                                div().text_sm().font_weight(FontWeight::SEMIBOLD)
                                    .text_color(color_to_hsla(text_primary))
                                    .child("Acme Corp")
                            )
                    )
                    .with_utility_items(
                        div().flex().items_center().gap(px(4.0))
                            .child(IconButton::from_spec(IconButtonSpec::new().with_icon("bell").with_size(ControlSize::Sm), theme).with_id("ah-bell2"))
                            .child(IconButton::from_spec(IconButtonSpec::new().with_icon("user").with_size(ControlSize::Sm), theme).with_id("ah-user"))
                    )
                )
        )
}
