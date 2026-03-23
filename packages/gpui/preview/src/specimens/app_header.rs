use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::AppHeaderSpec;
use pug_gpui_components::{AppHeader, Button};
use pug_primitives::{ButtonSpec, ButtonVariant, ControlSize, IconSpec, IconSize};
use pug_gpui_components::Icon;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    div().flex().flex_col().gap(px(24.0))
        // --- Full app header ---
        .child(section_label("FULL APP HEADER", text_secondary))
        .child(
            AppHeader::from_spec(
                AppHeaderSpec::new()
                    .with_title("Pug Studio")
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
                    .child(icon_btn("search", theme))
                    .child(icon_btn("bell", theme))
                    .child(icon_btn("settings", theme))
            )
        )

        // --- Simple with actions ---
        .child(section_label("SIMPLE WITH ACTIONS", text_secondary))
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
                    .child(icon_btn("settings", theme))
            )
        )

        // --- Title only ---
        .child(section_label("TITLE ONLY", text_secondary))
        .child(
            AppHeader::from_spec(
                AppHeaderSpec::new()
                    .with_title("Settings"),
                theme,
            )
        )

        // --- Custom identity (leading slot) ---
        .child(section_label("CUSTOM IDENTITY (LEADING SLOT)", text_secondary))
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
                            .bg(color_to_hsla(theme.resolve_color("semantic.color.accent.base")))
                    )
                    .child(
                        div().text_sm().font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_primary))
                            .child("Acme Corp")
                    )
            )
            .with_utility_items(
                div().flex().items_center().gap(px(4.0))
                    .child(icon_btn("bell", theme))
                    .child(icon_btn("user", theme))
            )
        )
}

fn icon_btn(name: &str, theme: &GpuiThemeProvider) -> Div {
    let muted = theme.resolve_color("semantic.color.text.secondary");
    div()
        .flex().items_center().justify_center()
        .w(px(28.0)).h(px(28.0)).rounded(px(4.0))
        .cursor(CursorStyle::PointingHand)
        .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.08)))
        .child(
            Icon::from_spec(
                IconSpec::new(name).with_size(IconSize::Sm),
                theme,
            ).with_color(color_to_hsla(muted))
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
