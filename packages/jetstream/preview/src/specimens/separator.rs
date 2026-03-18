//! Separator specimen — demonstrates horizontal/vertical and tone variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

/// Render the Separator specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let border_default = theme_bridge::border_default(theme);
    let bg_surface = theme_bridge::surface_background(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Horizontal separators ──
    let h_label = tree.create_node(Widget::Label {
        text: "Horizontal Separators".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, h_label);

    let h_demo = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_padding(LayoutEdges::uniform(16.0))
        .with_gap(12.0)),
        NodeStyle { background: Some(bg_surface), border_color: Some(border_subtle), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
    tree.add_child(root, h_demo);

    let above = tree.create_node(Widget::Label {
        text: "Content above".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(h_demo, above);

    // Subtle separator
    let sep_subtle = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(1.0))),
        NodeStyle { background: Some(border_subtle), ..NodeStyle::default() });
    tree.add_child(h_demo, sep_subtle);

    let between = tree.create_node(Widget::Label {
        text: "Subtle tone ↑ / Default tone ↓".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(h_demo, between);

    // Default separator
    let sep_default = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(1.0))),
        NodeStyle { background: Some(border_default), ..NodeStyle::default() });
    tree.add_child(h_demo, sep_default);

    let below = tree.create_node(Widget::Label {
        text: "Content below".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(h_demo, below);

    // ── Vertical separators ──
    let v_label = tree.create_node(Widget::Label {
        text: "Vertical Separators".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, v_label);

    let v_demo = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_height(LayoutSizing::Fixed(60.0))
        .with_padding(LayoutEdges::uniform(16.0))
        .with_gap(12.0)
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg_surface), border_color: Some(border_subtle), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
    tree.add_child(root, v_demo);

    let left = tree.create_node(Widget::Label {
        text: "Left".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(v_demo, left);

    let v_sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(1.0))
        .with_height(LayoutSizing::Grow)),
        NodeStyle { background: Some(border_subtle), ..NodeStyle::default() });
    tree.add_child(v_demo, v_sep);

    let mid = tree.create_node(Widget::Label {
        text: "Middle".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(v_demo, mid);

    let v_sep2 = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(1.0))
        .with_height(LayoutSizing::Grow)),
        NodeStyle { background: Some(border_default), ..NodeStyle::default() });
    tree.add_child(v_demo, v_sep2);

    let right = tree.create_node(Widget::Label {
        text: "Right".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(v_demo, right);

    root
}
