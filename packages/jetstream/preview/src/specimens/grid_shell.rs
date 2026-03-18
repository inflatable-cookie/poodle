//! GridShell specimen — responsive card grid browse layout.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(24.0)),
        NodeStyle::default());

    // ── 3-column grid ──
    section_label(tree, root, "3-Column Grid", text_secondary);
    {
        let shell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(12.0)
            .with_padding(LayoutEdges::uniform(16.0))),
            NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, shell);

        // Header
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(shell, header);

        let title = tree.create_node(Widget::Label { text: "Media Library".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let count = tree.create_node(Widget::Label { text: "6 items".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(header, count);

        // Grid rows
        let items = &[
            ("photo_01.jpg", "1.2 MB"),
            ("banner.png", "340 KB"),
            ("logo.svg", "12 KB"),
            ("hero_bg.jpg", "2.8 MB"),
            ("icon_set.zip", "890 KB"),
            ("avatar.png", "64 KB"),
        ];

        for row_items in items.chunks(3) {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(12.0)),
                NodeStyle::default());
            tree.add_child(shell, row);

            for &(name, size) in row_items {
                let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Fixed(120.0))),
                    NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
                tree.add_child(row, card);

                // Placeholder thumbnail
                let thumb = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(72.0))),
                    NodeStyle { background: Some(theme_bridge::tint(border, 0.5)), ..NodeStyle::default() });
                tree.add_child(card, thumb);

                let info = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_padding(LayoutEdges::uniform(8.0))
                    .with_gap(2.0)),
                    NodeStyle::default());
                tree.add_child(card, info);

                let n = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                    NodeStyle { text_color: Some(text_primary), text_size: Some(10.0), ..NodeStyle::default() });
                tree.add_child(info, n);

                let s = tree.create_node(Widget::Label { text: size.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                    NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
                tree.add_child(info, s);
            }
        }
    }

    // ── Empty grid ──
    section_label(tree, root, "Empty Grid", text_secondary);
    {
        let shell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(400.0))
            .with_padding(LayoutEdges::uniform(32.0))
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, shell);

        let msg = tree.create_node(Widget::Label { text: "No media uploaded".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(shell, msg);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
