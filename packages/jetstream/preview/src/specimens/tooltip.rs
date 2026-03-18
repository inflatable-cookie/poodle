//! Tooltip specimen — small informational overlay on hover.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    // Dark tooltip background (inverted)
    let tooltip_bg = glam::Vec4::new(0.15, 0.15, 0.18, 0.95);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Basic tooltip ──
    section_label(tree, root, "Basic Tooltip", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(4.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        let trigger = tree.create_node(Widget::Button {
            label: "Hover me".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4], background: Some(bg_surface),
                border_color: Some(border), border_width: 1.0,
                text_color: Some(text_primary), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            });
        tree.add_child(col, trigger);

        tooltip_bubble(tree, col, "This is a tooltip", tooltip_bg, text_inverse);
    }

    // ── Placement variants ──
    section_label(tree, root, "Placement Variants (mockup)", text_secondary);
    {
        let grid = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(12.0)),
            NodeStyle::default());
        tree.add_child(root, grid);

        for &(label, placement) in &[
            ("Top", "\u{2191} Tooltip above"),
            ("Bottom", "\u{2193} Tooltip below"),
            ("Left", "\u{2190} Tooltip left"),
            ("Right", "\u{2192} Tooltip right"),
        ] {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(grid, row);

            let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(50.0))),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(row, lbl);

            tooltip_bubble(tree, row, placement, tooltip_bg, text_inverse);
        }
    }

    // ── Multi-line tooltip ──
    section_label(tree, root, "Multi-line Tooltip", text_secondary);
    {
        let tip = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges { top: 6.0, right: 10.0, bottom: 6.0, left: 10.0 })
            .with_gap(2.0)),
            NodeStyle { corner_radii: [6.0; 4], background: Some(tooltip_bg), ..NodeStyle::default() });
        tree.add_child(root, tip);

        let line1 = tree.create_node(Widget::Label { text: "Keyboard shortcut".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_inverse), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(tip, line1);

        let line2 = tree.create_node(Widget::Label { text: "\u{2318} + S to save".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(theme_bridge::tint(text_inverse, 0.7)),
                text_size: Some(10.0), ..NodeStyle::default()
            });
        tree.add_child(tip, line2);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn tooltip_bubble(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let tip = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_padding(LayoutEdges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 })),
        NodeStyle {
            corner_radii: [4.0; 4],
            background: Some(bg),
            text_color: Some(fg),
            text_size: Some(11.0),
            ..NodeStyle::default()
        });
    tree.add_child(parent, tip);
}
