//! Inline specimen — demonstrates horizontal flow with wrapped children.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

/// Render the Inline specimen.
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

    // ── Basic horizontal flow ──
    let label = tree.create_node(Widget::Label {
        text: "Horizontal Flow (gap: 8px)".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, label);

    let flow = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_gap(8.0)
        .with_padding(LayoutEdges::uniform(12.0))
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(root, flow);

    let items = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta"];
    for name in &items {
        let pill = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges { top: 4.0, right: 10.0, bottom: 4.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(theme_bridge::tint(accent, 0.15)), corner_radii: [12.0; 4], ..NodeStyle::default() });
        tree.add_child(flow, pill);

        let lbl = tree.create_node(Widget::Label {
            text: name.to_string(),
        }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(pill, lbl);
    }

    // ── Gap variants ──
    let gap_label = tree.create_node(Widget::Label {
        text: "Gap Variants".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, gap_label);

    for &(gap_name, gap) in &[("gap: 2", 2.0_f32), ("gap: 8", 8.0), ("gap: 16", 16.0)] {
        let row_label = tree.create_node(Widget::Label {
            text: gap_name.to_string(),
        }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(root, row_label);

        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(gap)
            .with_padding(LayoutEdges::uniform(8.0))),
            NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(root, row);

        for _ in 0..4 {
            let block = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(40.0))
                .with_height(LayoutSizing::Fixed(24.0))),
                NodeStyle { background: Some(theme_bridge::tint(accent, 0.20)), corner_radii: [4.0; 4], ..NodeStyle::default() });
            tree.add_child(row, block);
        }
    }

    root
}
