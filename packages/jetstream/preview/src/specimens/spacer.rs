//! Spacer specimen — demonstrates flexible space filling between items.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

/// Render the Spacer specimen.
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

    // ── Spacer between items ──
    let label = tree.create_node(Widget::Label {
        text: "Spacer pushes items apart".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, label);

    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(40.0))
        .with_padding(LayoutEdges::uniform(8.0))
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(root, row);

    // Left block
    let left = block(tree, "Left", accent, text_primary);
    tree.add_child(row, left);

    // Spacer (Grow fills remaining space)
    let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Grow)),
        NodeStyle::default());
    tree.add_child(row, spacer);

    // Right block
    let right = block(tree, "Right", accent, text_primary);
    tree.add_child(row, right);

    // ── Multiple spacers ──
    let label2 = tree.create_node(Widget::Label {
        text: "Two spacers — 3 items evenly distributed".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, label2);

    let row2 = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(40.0))
        .with_padding(LayoutEdges::uniform(8.0))
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(root, row2);

    let a = block(tree, "A", accent, text_primary);
    tree.add_child(row2, a);

    let sp1 = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Grow)),
        NodeStyle::default());
    tree.add_child(row2, sp1);

    let b = block(tree, "B", accent, text_primary);
    tree.add_child(row2, b);

    let sp2 = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Grow)),
        NodeStyle::default());
    tree.add_child(row2, sp2);

    let c = block(tree, "C", accent, text_primary);
    tree.add_child(row2, c);

    // ── Vertical spacer ──
    let label3 = tree.create_node(Widget::Label {
        text: "Vertical spacer".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, label3);

    let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(120.0))
        .with_height(LayoutSizing::Fixed(140.0))
        .with_padding(LayoutEdges::uniform(8.0))),
        NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(root, col);

    let top = block(tree, "Top", accent, text_primary);
    tree.add_child(col, top);

    let v_spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Grow)),
        NodeStyle::default());
    tree.add_child(col, v_spacer);

    let bottom = block(tree, "Bottom", accent, text_primary);
    tree.add_child(col, bottom);

    root
}

fn block(tree: &mut UiTree, label: &str, accent: glam::Vec4, text: glam::Vec4) -> UiNodeId {
    let node = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_padding(LayoutEdges { top: 4.0, right: 12.0, bottom: 4.0, left: 12.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(theme_bridge::tint(accent, 0.20)), corner_radii: [4.0; 4], ..NodeStyle::default() });

    let lbl = tree.create_node(Widget::Label {
        text: label.to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(node, lbl);

    node
}
