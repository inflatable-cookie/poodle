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

/// Icon names used in the sizes section.
const SIZE_ICONS: &[&str] = &["star", "heart", "settings"];

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Sizes ---
        .child(section_label("SIZES", text_secondary))
        .child(render_sizes_section(theme))
        // --- Color Inheritance ---
        .child(section_label("COLOR INHERITANCE", text_secondary))
        .child(render_color_inheritance_section(theme))
        // --- All Icons ---
        .child(section_label("ALL ICONS", text_secondary))
        .child(render_icon_gallery(theme))
}

fn render_sizes_section(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let sizes: &[(&str, IconSize)] = &[
        ("sm", IconSize::Sm),
        ("md", IconSize::Md),
        ("lg", IconSize::Lg),
    ];

    let mut container = div().flex().flex_col().gap(px(12.0));

    for &(size_label, size) in sizes {
        let mut row = div().flex().gap(px(16.0)).items_center()
            .child(
                div().w(px(24.0)).text_xs().text_color(color_to_hsla(text_secondary))
                    .child(size_label.to_string())
            );

        for &icon_name in SIZE_ICONS {
            row = row.child(
                div().text_color(color_to_hsla(text_primary))
                    .child(PugIcon::new(IconSpec::new(icon_name).with_size(size), theme))
            );
        }

        container = container.child(row);
    }

    container
}

fn render_color_inheritance_section(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let accent = theme.resolve_color("semantic.color.accent.base");
    let danger = theme.resolve_color("semantic.color.status.danger");

    let items: &[(&str, pug_tokens::typed::ColorValue)] = &[
        ("check", text_primary),
        ("info", text_secondary),
        ("star", accent),
        ("triangle-alert", danger),
    ];

    let mut row = div().flex().gap(px(16.0)).items_center();

    for &(icon_name, color) in items {
        row = row.child(
            div().text_color(color_to_hsla(color))
                .child(PugIcon::new(IconSpec::new(icon_name).with_size(IconSize::Md), theme))
        );
    }

    row
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
