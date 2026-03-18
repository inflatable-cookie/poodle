//! DetailRow specimen — label-value metadata pair display.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)), NodeStyle::default());

    // ── Standard ──
    section_label(tree, root, "Standard Detail Rows", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column).with_gap(8.0)
            .with_width(LayoutSizing::Fixed(300.0))), NodeStyle::default());
        tree.add_child(root, col);

        detail(tree, col, "Name", "John Doe", text_secondary, text_primary, 90.0);
        detail(tree, col, "Email", "john@example.com", text_secondary, text_primary, 90.0);
        detail(tree, col, "Role", "Administrator", text_secondary, text_primary, 90.0);
        detail(tree, col, "Status", "Active", text_secondary, text_primary, 90.0);
    }

    // ── With separators ──
    section_label(tree, root, "With Separators", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column).with_gap(0.0)
            .with_width(LayoutSizing::Fixed(300.0))), NodeStyle::default());
        tree.add_child(root, col);

        for (i, &(lbl, val)) in [("Created", "Mar 10, 2026"), ("Updated", "Mar 16, 2026"), ("Version", "2.1.0")].iter().enumerate() {
            if i > 0 {
                let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))),
                    NodeStyle { background: Some(border), ..NodeStyle::default() });
                tree.add_child(col, sep);
            }

            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_padding(LayoutEdges { top: 8.0, right: 0.0, bottom: 8.0, left: 0.0 })
                .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(col, row);

            let l = tree.create_node(Widget::Label { text: lbl.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, l);

            let v = tree.create_node(Widget::Label { text: val.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, v);
        }
    }

    // ── Stacked layout ──
    section_label(tree, root, "Stacked Layout", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column).with_gap(12.0)), NodeStyle::default());
        tree.add_child(root, col);

        for &(lbl, val) in &[("Description", "A detailed description of the item."), ("Notes", "No additional notes.")] {
            let item = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column).with_gap(2.0)), NodeStyle::default());
            tree.add_child(col, item);

            let l = tree.create_node(Widget::Label { text: lbl.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(item, l);

            let v = tree.create_node(Widget::Label { text: val.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(item, v);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn detail(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, label_color: glam::Vec4, value_color: glam::Vec4, label_width: f32) {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row).with_gap(8.0)
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle::default());
    tree.add_child(parent, row);

    let l = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(label_width))),
        NodeStyle { text_color: Some(label_color), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(row, l);

    let v = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(value_color), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(row, v);
}
