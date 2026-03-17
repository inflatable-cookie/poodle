//! EmbedInput specimen — URL input with resolution indicator.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 16.0, ..UiStyle::default()
    });

    section_label(tree, root, "Empty", text_secondary);
    field_with_status(tree, root, "Embed URL", "Paste a URL to embed…", "", "", text_primary, text_secondary, bg_surface, border, text_secondary);

    section_label(tree, root, "Resolved", text_secondary);
    field_with_status(tree, root, "Embed URL", "", "https://youtube.com/watch?v=abc123", "✓ YouTube video detected", text_primary, text_secondary, bg_surface, border, success);

    section_label(tree, root, "Loading", text_secondary);
    field_with_status(tree, root, "Embed URL", "", "https://vimeo.com/12345", "Resolving…", text_primary, text_secondary, bg_surface, border, accent);

    root
}

fn field_with_status(tree: &mut UiTree, parent: UiNodeId, label: &str, placeholder: &str, value: &str, status: &str, label_color: glam::Vec4, placeholder_color: glam::Vec4, bg: glam::Vec4, border: glam::Vec4, status_color: glam::Vec4) {
    let wrapper = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 4.0, width: Sizing::Fixed(360.0), ..UiStyle::default()
    });
    tree.add_child(parent, wrapper);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(label_color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(wrapper, lbl);

    let display = if value.is_empty() { placeholder } else { value };
    let color = if value.is_empty() { placeholder_color } else { label_color };

    let input = tree.create(Widget::Panel, UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg), border_color: Some(border), border_width: 1.0,
        corner_radii: [6.0; 4], align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(wrapper, input);

    let txt = tree.create(Widget::Label { text: display.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(input, txt);

    if !status.is_empty() {
        let st = tree.create(Widget::Label { text: status.to_string() }, UiStyle {
            text_color: Some(status_color), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(wrapper, st);
    }
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
