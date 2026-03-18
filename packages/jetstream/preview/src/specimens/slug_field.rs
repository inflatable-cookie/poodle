//! SlugField specimen — URL-safe slug field with auto-generation.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    section_label(tree, root, "Auto-generated Slug", text_secondary);
    slug_pair(tree, root, "Title", "My New Blog Post", "Slug", "my-new-blog-post", true, text_primary, text_secondary, accent, bg_surface, border);

    section_label(tree, root, "Custom Slug", text_secondary);
    slug_pair(tree, root, "Title", "Product Launch 2026", "Slug", "launch-2026", false, text_primary, text_secondary, accent, bg_surface, border);

    section_label(tree, root, "Empty", text_secondary);
    slug_pair(tree, root, "Title", "", "Slug", "", true, text_primary, text_secondary, accent, bg_surface, border);

    root
}

fn slug_pair(tree: &mut UiTree, parent: UiNodeId, title_label: &str, title_value: &str, slug_label: &str, slug_value: &str, auto: bool, text_primary: glam::Vec4, text_secondary: glam::Vec4, accent: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_gap(8.0)
        .with_width(LayoutSizing::Fixed(340.0))),
        NodeStyle::default());
    tree.add_child(parent, wrapper);

    // Title field
    field(tree, wrapper, title_label, if title_value.is_empty() { "Enter title…" } else { title_value }, !title_value.is_empty(), text_primary, text_secondary, bg, border);

    // Slug field with link indicator
    let slug_row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_gap(4.0)),
        NodeStyle::default());
    tree.add_child(wrapper, slug_row);

    let label_row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_gap(6.0)
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle::default());
    tree.add_child(slug_row, label_row);

    let lbl = tree.create_node(Widget::Label { text: slug_label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(label_row, lbl);

    if auto {
        let auto_badge = tree.create_node(Widget::Label { text: "🔗 auto".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(label_row, auto_badge);
    }

    let display = if slug_value.is_empty() { "auto-generated-from-title" } else { slug_value };
    let color = if slug_value.is_empty() { text_secondary } else { text_primary };

    let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(32.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(slug_row, input);

    let txt = tree.create_node(Widget::Label { text: display.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(input, txt);
}

fn field(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, has_value: bool, text_primary: glam::Vec4, text_secondary: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_gap(4.0)),
        NodeStyle::default());
    tree.add_child(parent, wrapper);

    let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(wrapper, lbl);

    let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(32.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(wrapper, input);

    let txt = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(if has_value { text_primary } else { text_secondary }), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(input, txt);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
