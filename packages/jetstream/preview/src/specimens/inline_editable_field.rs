//! InlineEditableField specimen — click-to-edit text field with display/edit toggle.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

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

    // ── Display mode ──
    section_label(tree, root, "Display Mode (click to edit)", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_width(LayoutSizing::Fixed(300.0))
            .with_padding(LayoutEdges { top: 6.0, right: 8.0, bottom: 6.0, left: 8.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [4.0; 4], ..NodeStyle::default() });
        tree.add_child(root, row);

        let value = tree.create_node(Widget::Label { text: "Acme Project".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(row, value);

        let edit_icon = tree.create_node(Widget::Label { text: "✎".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(row, edit_icon);
    }

    // ── Edit mode ──
    section_label(tree, root, "Edit Mode (active)", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_width(LayoutSizing::Fixed(300.0))
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_width(LayoutSizing::Grow)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), border_color: Some(accent), border_width: 2.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(row, input);

        let txt = tree.create_node(Widget::Label { text: "Acme Project".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(input, txt);

        let save = tree.create_node(Widget::Button {
            label: "✓".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(28.0))
            .with_height(LayoutSizing::Fixed(28.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], background: Some(accent), text_color: Some(glam::Vec4::ONE), text_size: Some(14.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(row, save);

        let cancel = tree.create_node(Widget::Button {
            label: "✕".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(28.0))
            .with_height(LayoutSizing::Fixed(28.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0, text_color: Some(text_secondary), text_size: Some(14.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(row, cancel);
    }

    // ── In context ──
    section_label(tree, root, "In Context (label + value)", text_secondary);
    {
        let ctx = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(root, ctx);

        for &(label, value) in &[("Project Name", "Acme Project"), ("Description", "A brief project overview"), ("Status", "Active")] {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(12.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(ctx, row);

            let l = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(100.0))),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
            tree.add_child(row, l);

            let v = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, v);

            let edit = tree.create_node(Widget::Label { text: "✎".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(theme_bridge::tint(text_secondary, 0.5)), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(row, edit);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
