use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{IconSize, IconSpec, EyebrowSpec};
use poodle_gpui_components::{Icon, Eyebrow};
use crate::style_bridge::color_to_hsla;

/// A curated set of icon names to display in the gallery.
const SPECIMEN_ICONS: &[&str] = &[
    // Navigation
    "arrow-left", "arrow-right", "arrow-up", "arrow-down",
    "chevron-down", "chevron-left", "chevron-right", "chevron-up",
    "menu", "x", "corner-up-left", "move",
    // Actions
    "check", "copy", "download", "edit", "external-link",
    "minus", "plus", "save", "trash-2", "upload",
    "refresh-cw", "redo", "undo", "share",
    // Status
    "alert-circle", "alert-triangle", "check-circle", "info",
    "loader", "ban", "shield", "shield-check",
    // Objects
    "bell", "calendar", "clock", "eye", "eye-off",
    "file", "file-text", "filter", "folder", "folder-open",
    "globe", "hash", "image", "layers", "link",
    "lock", "unlock", "mail", "map-pin", "message-circle",
    "monitor", "paperclip", "search", "settings", "tag",
    "terminal", "type", "user", "users", "zap",
    // Media
    "play", "pause", "volume-2", "mic",
    // More
    "bookmark", "code", "database", "git-branch",
    "heart", "home", "key", "layout-grid",
    "list", "maximize", "minimize", "palette",
    "pencil", "pie-chart", "bar-chart", "star",
];

/// Icon names used in the sizes section.
const SIZE_ICONS: &[&str] = &["star", "heart", "settings", "search"];

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Direct import \u{2014} tree-shakeable"), theme))
                .child(render_sizes_section(theme))
        )
        // --- Color Inheritance ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Color inheritance"), theme))
                .child(render_color_inheritance_section(theme))
        )
        // --- Accessibility ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Accessibility"), theme))
                .child(render_accessibility_section(theme))
        )
        // --- All Icons ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(&format!("All icons ({})", SPECIMEN_ICONS.len())), theme))
                .child(render_icon_gallery(theme))
        )
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
                    .child(Icon::from_spec(IconSpec::new(icon_name).with_size(size), theme))
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
    let success = theme.resolve_color("semantic.color.status.success");
    let warning = theme.resolve_color("semantic.color.status.warning");
    let danger = theme.resolve_color("semantic.color.status.danger");

    let items: &[(&str, &str, poodle_tokens::typed::ColorValue)] = &[
        ("check", "Primary", text_primary),
        ("info", "Secondary", text_secondary),
        ("star", "Accent", accent),
        ("check-circle", "Success", success),
        ("alert-triangle", "Warning", warning),
        ("alert-circle", "Danger", danger),
    ];

    let mut row = div().flex().gap(px(20.0)).items_center();

    for &(icon_name, label, color) in items {
        row = row.child(
            div().flex().flex_col().items_center().gap(px(4.0))
                .child(
                    div().text_color(color_to_hsla(color))
                        .child(Icon::from_spec(IconSpec::new(icon_name).with_size(IconSize::Md), theme))
                )
                .child(
                    div().text_size(px(10.0)).text_color(color_to_hsla(text_secondary))
                        .child(label.to_string())
                )
        );
    }

    row
}

fn render_accessibility_section(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    div().flex().flex_col().gap(px(8.0))
        .child(
            div().flex().items_center().gap(px(8.0))
                .child(
                    div().text_color(color_to_hsla(text_primary))
                        .child(Icon::from_spec(
                            IconSpec::new("search")
                                .with_size(IconSize::Md)
                                .with_aria_label("Search"),
                            theme,
                        ))
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("With aria-label=\"Search\" \u{2014} announced by screen readers")
                )
        )
        .child(
            div().flex().items_center().gap(px(8.0))
                .child(
                    div().text_color(color_to_hsla(text_primary))
                        .child(Icon::from_spec(
                            IconSpec::new("chevron-right").with_size(IconSize::Md),
                            theme,
                        ))
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Without aria-label \u{2014} hidden from assistive technology")
                )
        )
}

fn render_icon_gallery(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut gallery = div()
        .flex()
        .flex_wrap()
        .gap(px(6.0));

    for &name in SPECIMEN_ICONS {
        gallery = gallery.child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(3.0))
                .w(px(64.0))
                .py(px(6.0))
                .child(
                    div().text_color(color_to_hsla(text_primary))
                        .child(Icon::from_spec(IconSpec::new(name).with_size(IconSize::Md), theme))
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
