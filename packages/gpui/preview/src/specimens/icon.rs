use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{IconSize, IconSpec};
use pug_gpui_components::PugIcon;
use crate::style_bridge::color_to_hsla;

/// A curated set of icon names to display in the specimen.
const SPECIMEN_ICONS: &[&str] = &[
    // Navigation
    "arrow-left", "arrow-right", "chevron-down", "chevron-left",
    "chevron-right", "chevron-up", "menu", "x",
    // Actions
    "check", "copy", "download", "edit", "external-link",
    "minus", "plus", "save", "trash-2", "upload",
    // Status
    "alert-circle", "check-circle", "info", "loader",
    // Objects
    "bell", "calendar", "clock", "eye", "file", "filter",
    "folder", "lock", "mail", "search", "settings", "user",
    // More
    "heart", "star",
];

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    div().flex().flex_col().gap(px(16.0))
        // Size variants
        .child(section_label("SIZES", text_secondary))
        .child(
            div().flex().gap(px(16.0)).items_end()
                .child(
                    div().flex().flex_col().items_center().gap(px(4.0))
                        .child(
                            div().text_color(color_to_hsla(text_primary))
                                .child(PugIcon::new(IconSpec::new("star").with_size(IconSize::Sm), theme))
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("sm".to_string())
                        )
                )
                .child(
                    div().flex().flex_col().items_center().gap(px(4.0))
                        .child(
                            div().text_color(color_to_hsla(text_primary))
                                .child(PugIcon::new(IconSpec::new("star").with_size(IconSize::Md), theme))
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("md".to_string())
                        )
                )
                .child(
                    div().flex().flex_col().items_center().gap(px(4.0))
                        .child(
                            div().text_color(color_to_hsla(text_primary))
                                .child(PugIcon::new(IconSpec::new("star").with_size(IconSize::Lg), theme))
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("lg".to_string())
                        )
                )
        )
        // Icon gallery
        .child(section_label("ICON GALLERY", text_secondary))
        .child(render_icon_gallery(theme))
}

fn render_icon_gallery(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut gallery = div()
        .flex()
        .flex_wrap()
        .gap(px(8.0));

    for &name in SPECIMEN_ICONS {
        gallery = gallery.child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .w(px(64.0))
                .py(px(6.0))
                .child(
                    div().text_color(color_to_hsla(text_primary))
                        .child(PugIcon::new(IconSpec::new(name).with_size(IconSize::Md), theme))
                )
                .child(
                    div().text_color(color_to_hsla(text_secondary))
                        .overflow_hidden()
                        .w_full()
                        .text_center()
                        .child(
                            div().text_size(px(9.0)).child(name.to_string())
                        )
                )
        );
    }

    gallery
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
