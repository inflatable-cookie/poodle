//! StateTile specimen — compact label-value tile with metric and trend.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    section_label(tree, root, "Dashboard Tiles", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(12.0)),
            NodeStyle::default());
        tree.add_child(root, row);

        tile(tree, row, "Revenue", "$12,450", "↑ 12%", success, bg_elevated, border, text_primary, text_secondary);
        tile(tree, row, "Users", "1,284", "↑ 8%", success, bg_elevated, border, text_primary, text_secondary);
        tile(tree, row, "Errors", "23", "↓ 5%", danger, bg_elevated, border, text_primary, text_secondary);
    }

    section_label(tree, root, "Compact Tiles", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(root, row);

        compact_tile(tree, row, "CPU", "42%", bg_elevated, border, text_primary, text_secondary);
        compact_tile(tree, row, "RAM", "2.1 GB", bg_elevated, border, text_primary, text_secondary);
        compact_tile(tree, row, "Disk", "67%", bg_elevated, border, text_primary, text_secondary);
        compact_tile(tree, row, "Net", "12 Mbps", bg_elevated, border, text_primary, text_secondary);
    }

    root
}

fn tile(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, trend: &str, trend_color: glam::Vec4, bg: glam::Vec4, border: glam::Vec4, text_primary: glam::Vec4, text_secondary: glam::Vec4) {
    let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(120.0))
        .with_padding(LayoutEdges::uniform(12.0))
        .with_gap(4.0)),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
    tree.add_child(parent, card);

    let l = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(card, l);

    let v = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(18.0), ..NodeStyle::default() });
    tree.add_child(card, v);

    let t = tree.create_node(Widget::Label { text: trend.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(trend_color), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(card, t);
}

fn compact_tile(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, bg: glam::Vec4, border: glam::Vec4, text_primary: glam::Vec4, text_secondary: glam::Vec4) {
    let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(80.0))
        .with_padding(LayoutEdges::uniform(8.0))
        .with_gap(2.0)
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(parent, card);

    let l = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
    tree.add_child(card, l);

    let v = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
    tree.add_child(card, v);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
