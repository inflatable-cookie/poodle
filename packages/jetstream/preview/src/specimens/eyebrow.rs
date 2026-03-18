//! Eyebrow specimen — small uppercase label for section headings and categories.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutIntent, LayoutSizing};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)), NodeStyle::default());

    // ── Default ──
    label(tree, root, "Default", text_secondary);
    eyebrow(tree, root, "CATEGORY", text_secondary, 10.0);

    // ── Primary color ──
    label(tree, root, "Primary Color", text_secondary);
    eyebrow(tree, root, "SECTION TITLE", text_primary, 10.0);

    // ── Accent color ──
    label(tree, root, "Accent Color", text_secondary);
    eyebrow(tree, root, "FEATURED", accent, 10.0);

    // ── Size variants ──
    label(tree, root, "Size Variants", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column).with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        eyebrow(tree, col, "SMALL (9PX)", text_secondary, 9.0);
        eyebrow(tree, col, "MEDIUM (10PX)", text_secondary, 10.0);
        eyebrow(tree, col, "LARGE (12PX)", text_secondary, 12.0);
    }

    // ── In context ──
    label(tree, root, "In Context (above heading)", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column).with_gap(4.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        eyebrow(tree, col, "GETTING STARTED", accent, 10.0);

        let heading = tree.create_node(Widget::Label { text: "Welcome to the platform".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(18.0), ..NodeStyle::default() });
        tree.add_child(col, heading);

        let body = tree.create_node(Widget::Label { text: "Learn the basics and get up to speed quickly.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(col, body);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn eyebrow(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4, size: f32) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(size), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
