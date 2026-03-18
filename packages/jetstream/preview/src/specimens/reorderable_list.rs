//! ReorderableList specimen — drag-and-drop reorderable list.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    section_label(tree, root, "Reorderable List", text_secondary);
    {
        let list = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(300.0))),
            NodeStyle {
                background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, list);

        for (i, &(item, dragging)) in [
            ("1. Introduction", false),
            ("2. Background", false),
            ("3. Methodology", true),
            ("4. Results", false),
            ("5. Conclusion", false),
        ].iter().enumerate() {
            if i > 0 {
                let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(1.0))),
                    NodeStyle { background: Some(border), ..NodeStyle::default() });
                tree.add_child(list, sep);
            }

            let bg = if dragging { Some(theme_bridge::tint(accent, 0.08)) } else { None };
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 })
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    background: bg,
                    border_color: if dragging { Some(accent) } else { None },
                    border_width: if dragging { 1.0 } else { 0.0 },
                    ..NodeStyle::default()
                });
            tree.add_child(list, row);

            let handle = tree.create_node(Widget::Label { text: "\u{2807}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle {
                    text_color: Some(if dragging { accent } else { text_secondary }),
                    text_size: Some(14.0), ..NodeStyle::default()
                });
            tree.add_child(row, handle);

            let lbl = tree.create_node(Widget::Label { text: item.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, lbl);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
