//! MediaPicker specimen — gallery with selection.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    section_label(tree, root, "Media Picker Dialog", text_secondary);
    {
        let dialog = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(420.0))),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [10.0; 4], ..NodeStyle::default() });
        tree.add_child(root, dialog);

        // Header
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(dialog, header);

        let title = tree.create_node(Widget::Label { text: "Select Media".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let close = tree.create_node(Widget::Label { text: "✕".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(header, close);

        // Search
        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(dialog, search);

        let search_text = tree.create_node(Widget::Label { text: "🔍 Search media…".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(search, search_text);

        // Grid
        let grid = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(4.0)
            .with_padding(LayoutEdges { top: 0.0, right: 16.0, bottom: 16.0, left: 16.0 })),
            NodeStyle::default());
        tree.add_child(dialog, grid);

        let items = [
            ("photo_01.jpg", true),
            ("banner.png", false),
            ("logo.svg", false),
            ("hero.jpg", true),
            ("icon.png", false),
            ("bg_tile.jpg", false),
        ];

        for row_items in items.chunks(3) {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(4.0)),
                NodeStyle::default());
            tree.add_child(grid, row);

            for &(name, selected) in row_items {
                let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Fixed(120.0))
                    .with_height(LayoutSizing::Fixed(80.0))
                    .with_padding(LayoutEdges::uniform(4.0))
                    .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::End)),
                    NodeStyle { background: Some(theme_bridge::tint(border, 0.3)), corner_radii: [4.0; 4], border_color: if selected { Some(accent) } else { None }, border_width: if selected { 2.0 } else { 0.0 }, ..NodeStyle::default() });
                tree.add_child(row, card);

                if selected {
                    let check = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                        .with_width(LayoutSizing::Fixed(16.0))
                        .with_height(LayoutSizing::Fixed(16.0))
                        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                        NodeStyle { corner_radii: [8.0; 4], background: Some(accent), ..NodeStyle::default() });
                    tree.add_child(card, check);

                    let mark = tree.create_node(Widget::Label { text: "✓".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                        NodeStyle { text_color: Some(text_inverse), text_size: Some(10.0), ..NodeStyle::default() });
                    tree.add_child(check, mark);
                } else {
                    let n = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                        NodeStyle { text_color: Some(theme_bridge::tint(text_primary, 0.5)), text_size: Some(8.0), ..NodeStyle::default() });
                    tree.add_child(card, n);
                }
            }
        }

        // Footer
        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep);

        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(dialog, footer);

        let selected_count = tree.create_node(Widget::Label { text: "2 selected".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(footer, selected_count);

        let insert = tree.create_node(Widget::Button {
            label: "Insert".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(30.0))
            .with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], background: Some(accent), text_color: Some(text_inverse), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(footer, insert);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
