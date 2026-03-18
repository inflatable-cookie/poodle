//! AutonomousList specimen — self-managing list with add/remove/reorder.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(16.0),
        ),
        NodeStyle::default(),
    );

    section_label(tree, root, "Populated List", text_secondary);
    {
        let list = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Fixed(320.0)),
            ),
            NodeStyle {
                background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, list);

        for (i, &item) in ["Design review", "Code refactor", "Write tests"].iter().enumerate() {
            if i > 0 {
                let sep = tree.create_node(
                    Widget::Panel,
                    pug_jetstream::map_layout(
                        &LayoutIntent::new()
                            .with_width(LayoutSizing::Grow)
                            .with_height(LayoutSizing::Fixed(1.0)),
                    ),
                    NodeStyle { background: Some(border), ..NodeStyle::default() },
                );
                tree.add_child(list, sep);
            }

            let row = tree.create_node(
                Widget::Panel,
                pug_jetstream::map_layout(
                    &LayoutIntent::new()
                        .with_direction(LayoutDirection::Row)
                        .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
                        .with_gap(8.0)
                        .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center),
                ),
                NodeStyle::default(),
            );
            tree.add_child(list, row);

            let left = tree.create_node(
                Widget::Panel,
                pug_jetstream::map_layout(
                    &LayoutIntent::new()
                        .with_direction(LayoutDirection::Row)
                        .with_gap(8.0)
                        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
                ),
                NodeStyle::default(),
            );
            tree.add_child(row, left);

            let handle = tree.create_node(
                Widget::Label { text: "⠿".to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() },
            );
            tree.add_child(left, handle);

            let lbl = tree.create_node(
                Widget::Label { text: item.to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() },
            );
            tree.add_child(left, lbl);

            let remove = tree.create_node(
                Widget::Label { text: "✕".to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(theme_bridge::tint(text_secondary, 0.5)), text_size: Some(10.0), ..NodeStyle::default() },
            );
            tree.add_child(row, remove);
        }

        // Add row
        let add = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 }),
            ),
            NodeStyle::default(),
        );
        tree.add_child(list, add);

        let add_lbl = tree.create_node(
            Widget::Label { text: "+ Add item".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(11.0), ..NodeStyle::default() },
        );
        tree.add_child(add, add_lbl);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() },
    );
    tree.add_child(parent, lbl);
}
