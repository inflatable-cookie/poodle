//! Icon specimen — sizes, color inheritance, accessibility, and a gallery.
//!
//! Mirrors the GPUI specimen: per-size rows of the same icons, semantic-color
//! inheritance, accessible/decorative modes (`with_aria_label`), and a
//! representative icon grid. Every glyph is a real `js_icon` sized from tokens.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::icon::js_icon;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{IconSize, IconSpec};

/// Icons rendered once per size in the sizes section.
const SIZE_ICONS: &[&str] = &["star", "heart", "settings", "search"];

/// A representative gallery covering the main icon categories.
const GALLERY_ICONS: &[&str] = &[
    // Navigation
    "arrow-left", "arrow-right", "chevron-down", "chevron-right", "menu", "x",
    // Actions
    "check", "copy", "download", "edit", "plus", "minus", "trash-2", "upload", "refresh-cw",
    // Status
    "alert-circle", "alert-triangle", "check-circle", "info", "loader", "shield",
    // Objects
    "bell", "calendar", "clock", "eye", "file", "filter", "folder", "globe", "link", "lock",
    "mail", "search", "settings", "tag", "user", "users", "zap",
    // Media
    "play", "pause", "mic",
    // More
    "bookmark", "code", "heart", "home", "key", "list", "star",
];

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let primary = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");
    let success = resolve_color(theme, "color.status.success");
    let warning = resolve_color(theme, "color.status.warning");
    let danger = resolve_color(theme, "color.status.danger");

    div().flex_col().gap(24.0)
        // --- Sizes (sm / md / lg, same icons per row) ---
        .child(group("Sizes", secondary, render_sizes(theme, primary, secondary)))
        // --- Color inheritance (semantic tones) ---
        .child(group("Color inheritance", secondary,
            div().flex_row().gap(20.0).items_center()
                .child(color_cell("check", "Primary", primary, secondary, theme))
                .child(color_cell("info", "Secondary", secondary, secondary, theme))
                .child(color_cell("star", "Accent", accent, secondary, theme))
                .child(color_cell("check-circle", "Success", success, secondary, theme))
                .child(color_cell("alert-triangle", "Warning", warning, secondary, theme))
                .child(color_cell("alert-circle", "Danger", danger, secondary, theme))
        ))
        // --- Accessibility (aria-label vs decorative) ---
        .child(group("Accessibility", secondary,
            div().flex_col().gap(8.0)
                .child(
                    div().flex_row().items_center().gap(8.0)
                        .child(div().text_color(primary)
                            .child(js_icon(&IconSpec::new("search").with_aria_label("Search"), theme)))
                        .child(label("With aria-label \"Search\" — announced by screen readers")
                            .text_color(secondary).text_size(11.0))
                )
                .child(
                    div().flex_row().items_center().gap(8.0)
                        .child(div().text_color(primary)
                            .child(js_icon(&IconSpec::new("chevron-right"), theme)))
                        .child(label("Without aria-label — decorative, hidden from assistive tech")
                            .text_color(secondary).text_size(11.0))
                )
        ))
        // --- Gallery ---
        .child(group(&format!("Gallery ({})", GALLERY_ICONS.len()), secondary,
            render_gallery(theme, primary, secondary)))
}

fn render_sizes(theme: &JetstreamThemeProvider, primary: glam::Vec4, secondary: glam::Vec4) -> JsEl {
    let sizes: &[(&str, IconSize)] = &[
        ("sm", IconSize::Sm),
        ("md", IconSize::Md),
        ("lg", IconSize::Lg),
    ];
    let mut container = div().flex_col().gap(12.0);
    for &(size_label, size) in sizes {
        let mut row = div().flex_row().gap(16.0).items_center()
            .child(label(size_label).text_color(secondary).text_size(11.0).w(24.0));
        for &name in SIZE_ICONS {
            row = row.child(div().text_color(primary)
                .child(js_icon(&IconSpec::new(name).with_size(size), theme)));
        }
        container = container.child(row);
    }
    container
}

fn color_cell(
    icon: &str,
    name: &str,
    color: glam::Vec4,
    secondary: glam::Vec4,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    div().flex_col().items_center().gap(4.0)
        .child(div().text_color(color).child(js_icon(&IconSpec::new(icon), theme)))
        .child(label(name).text_color(secondary).text_size(10.0))
}

fn render_gallery(theme: &JetstreamThemeProvider, primary: glam::Vec4, secondary: glam::Vec4) -> JsEl {
    let mut gallery = div().flex_row().flex_wrap().gap(6.0);
    for &name in GALLERY_ICONS {
        gallery = gallery.child(
            div().flex_col().items_center().gap(3.0).w(64.0).py(6.0)
                .child(div().text_color(primary).child(js_icon(&IconSpec::new(name), theme)))
                .child(label(name).text_color(secondary).text_size(9.0).text_center())
        );
    }
    gallery
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
