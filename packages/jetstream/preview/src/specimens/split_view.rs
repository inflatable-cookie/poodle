//! SplitView specimen — resizable split panes with drag handles.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Horizontal split (left | right) ──
    section_label(tree, root, "Horizontal Split", text_secondary);
    {
        let split = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(440.0))
            .with_height(LayoutSizing::Fixed(180.0))),
            NodeStyle { border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, split);

        // Left pane
        let left = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(200.0))
            .with_height(LayoutSizing::Grow)
            .with_padding(LayoutEdges::uniform(12.0))
            .with_gap(6.0)),
            NodeStyle { background: Some(bg_elevated), ..NodeStyle::default() });
        tree.add_child(split, left);

        let left_title = tree.create_node(Widget::Label { text: "Left Pane".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(left, left_title);

        let left_desc = tree.create_node(Widget::Label { text: "Resizable content area".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(left, left_desc);

        // Drag handle (vertical)
        let handle = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(6.0))
            .with_height(LayoutSizing::Grow)
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(split, handle);

        // Grip dots
        let grip = tree.create_node(Widget::Label { text: "\u{22EE}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(handle, grip);

        // Right pane
        let right = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Grow)
            .with_padding(LayoutEdges::uniform(12.0))
            .with_gap(6.0)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(split, right);

        let right_title = tree.create_node(Widget::Label { text: "Right Pane".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(right, right_title);

        let right_desc = tree.create_node(Widget::Label { text: "Adjusts with drag handle".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(right, right_desc);
    }

    // ── Vertical split (top / bottom) ──
    section_label(tree, root, "Vertical Split", text_secondary);
    {
        let split = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(440.0))
            .with_height(LayoutSizing::Fixed(200.0))),
            NodeStyle { border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, split);

        // Top pane
        let top = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(90.0))
            .with_padding(LayoutEdges::uniform(12.0))
            .with_gap(6.0)),
            NodeStyle { background: Some(bg_elevated), ..NodeStyle::default() });
        tree.add_child(split, top);

        let top_title = tree.create_node(Widget::Label { text: "Top Pane".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(top, top_title);

        let top_desc = tree.create_node(Widget::Label { text: "Code editor area".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(top, top_desc);

        // Drag handle (horizontal)
        let handle = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(6.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(split, handle);

        let grip = tree.create_node(Widget::Label { text: "\u{22EF}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(handle, grip);

        // Bottom pane
        let bottom = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Grow)
            .with_padding(LayoutEdges::uniform(12.0))
            .with_gap(6.0)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(split, bottom);

        let bottom_title = tree.create_node(Widget::Label { text: "Bottom Pane".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(bottom, bottom_title);

        let bottom_desc = tree.create_node(Widget::Label { text: "Console / output area".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(bottom, bottom_desc);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
