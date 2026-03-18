//! DetailSection specimen — titled collapsible detail section.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)), NodeStyle::default());

    // ── Expanded ──
    section_label(tree, root, "Expanded Section", text_secondary);
    {
        let sec = detail_section_frame(tree, bg_elevated, border);
        tree.add_child(root, sec);

        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(sec, header);

        let title = tree.create_node(Widget::Label { text: "▾ General Information".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(sec, sep);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(12.0)).with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(sec, body);

        detail_row(tree, body, "Name", "Acme Project", text_secondary, text_primary);
        detail_row(tree, body, "Type", "Internal", text_secondary, text_primary);
        detail_row(tree, body, "Priority", "High", text_secondary, text_primary);
    }

    // ── Collapsed ──
    section_label(tree, root, "Collapsed Section", text_secondary);
    {
        let sec = detail_section_frame(tree, bg_elevated, border);
        tree.add_child(root, sec);

        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(sec, header);

        let title = tree.create_node(Widget::Label { text: "▸ Advanced Settings".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let hint = tree.create_node(Widget::Label { text: "3 fields".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(header, hint);
    }

    // ── Multiple sections ──
    section_label(tree, root, "Stacked Sections", text_secondary);
    {
        let stack = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column).with_gap(2.0)),
            NodeStyle::default());
        tree.add_child(root, stack);

        for &(label, expanded) in &[("▾ Overview", true), ("▸ Configuration", false), ("▸ Permissions", false)] {
            let sec = detail_section_frame(tree, bg_elevated, border);
            tree.add_child(stack, sec);

            let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(sec, header);

            let title = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
            tree.add_child(header, title);

            if expanded {
                let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))),
                    NodeStyle { background: Some(border), ..NodeStyle::default() });
                tree.add_child(sec, sep);

                let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_padding(LayoutEdges::uniform(12.0))),
                    NodeStyle::default());
                tree.add_child(sec, body);

                let content = tree.create_node(Widget::Label { text: "Section content goes here.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                    NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
                tree.add_child(body, content);
            }
        }
    }

    root
}

fn detail_section_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(360.0))),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() })
}

fn detail_row(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, label_color: glam::Vec4, value_color: glam::Vec4) {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row).with_gap(12.0)),
        NodeStyle::default());
    tree.add_child(parent, row);

    let l = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(70.0))),
        NodeStyle { text_color: Some(label_color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(row, l);

    let v = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(value_color), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(row, v);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
