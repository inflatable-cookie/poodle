//! RangeSlider specimen — dual-thumb slider for selecting a value range.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    // ── Standard range ──
    label(tree, root, "Standard (20% \u{2013} 80%)", text_secondary);
    range_track(tree, root, 0.2, 0.8, 200.0, accent, border, 1.0);

    // ── Narrow range ──
    label(tree, root, "Narrow range (40% \u{2013} 60%)", text_secondary);
    range_track(tree, root, 0.4, 0.6, 200.0, accent, border, 1.0);

    // ── Full range ──
    label(tree, root, "Full range (0% \u{2013} 100%)", text_secondary);
    range_track(tree, root, 0.0, 1.0, 200.0, accent, border, 1.0);

    // ── Disabled ──
    label(tree, root, "Disabled (30% \u{2013} 70%)", text_secondary);
    range_track(tree, root, 0.3, 0.7, 200.0, accent, border, 0.5);

    // ── With labels ──
    label(tree, root, "With Labels", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        let min = tree.create_node(Widget::Label { text: "$200".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(row, min);

        range_track(tree, row, 0.25, 0.75, 160.0, accent, border, 1.0);

        let max = tree.create_node(Widget::Label { text: "$800".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(row, max);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn range_track(
    tree: &mut UiTree,
    parent: UiNodeId,
    low: f32,
    high: f32,
    width: f32,
    accent: glam::Vec4,
    border: glam::Vec4,
    opacity: f32,
) {
    let container = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(width))
        .with_height(LayoutSizing::Fixed(20.0))
        .with_direction(LayoutDirection::Row)
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { opacity, ..NodeStyle::default() });
    tree.add_child(parent, container);

    // Track background
    let track = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(width))
        .with_height(LayoutSizing::Fixed(4.0))
        .with_direction(LayoutDirection::Row)),
        NodeStyle { corner_radii: [2.0; 4], background: Some(border), ..NodeStyle::default() });
    tree.add_child(container, track);

    // Active range fill
    let fill_start = width * low;
    let fill_w = width * (high - low);
    if fill_w > 0.0 {
        // Leading spacer
        if fill_start > 0.0 {
            let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(fill_start))
                .with_height(LayoutSizing::Fixed(4.0))),
                NodeStyle::default());
            tree.add_child(track, spacer);
        }

        let fill = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(fill_w))
            .with_height(LayoutSizing::Fixed(4.0))),
            NodeStyle { corner_radii: [2.0; 4], background: Some(accent), ..NodeStyle::default() });
        tree.add_child(track, fill);
    }
}
