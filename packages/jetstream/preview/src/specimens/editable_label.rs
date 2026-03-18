//! EditableLabel specimen — display and edit modes.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    // ── Display mode ──
    label(tree, root, "Display Mode (click to edit)", text_secondary);
    {
        let display = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)
            .with_padding(LayoutEdges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 })),
            NodeStyle { corner_radii: [4.0; 4], ..NodeStyle::default() });
        tree.add_child(root, display);

        let val = tree.create_node(Widget::Label { text: "Project Alpha".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(display, val);

        let edit_icon = tree.create_node(Widget::Label { text: "✎".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(display, edit_icon);
    }

    // ── Edit mode ──
    label(tree, root, "Edit Mode (actively editing)", text_secondary);
    {
        let edit_row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, edit_row);

        let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(200.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_canvas), border_color: Some(accent), border_width: 2.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(edit_row, input);

        let val = tree.create_node(Widget::Label { text: "Project Alpha".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(input, val);

        // Confirm/cancel buttons
        let confirm = tree.create_node(Widget::Button {
            label: "✓".to_string(),
            pressed: false,
            hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(28.0))
            .with_height(LayoutSizing::Fixed(28.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [4.0; 4], background: Some(accent), text_color: Some(theme_bridge::text_inverse(theme)), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(edit_row, confirm);

        let cancel = tree.create_node(Widget::Button {
            label: "✕".to_string(),
            pressed: false,
            hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(28.0))
            .with_height(LayoutSizing::Fixed(28.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [4.0; 4], border_color: Some(border_default), border_width: 1.0, text_color: Some(text_secondary), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(edit_row, cancel);
    }

    // ── Different sizes ──
    label(tree, root, "Size Variants", text_secondary);
    for &(lbl, size) in &[("Heading", 20.0_f32), ("Body", 14.0), ("Caption", 11.0)] {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)
            .with_padding(LayoutEdges { top: 2.0, right: 4.0, bottom: 2.0, left: 4.0 })),
            NodeStyle::default());
        tree.add_child(root, row);

        let val = tree.create_node(Widget::Label { text: format!("{} text", lbl) }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(size), ..NodeStyle::default() });
        tree.add_child(row, val);

        let icon = tree.create_node(Widget::Label { text: "✎".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(theme_bridge::tint(text_secondary, 0.5)), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(row, icon);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
