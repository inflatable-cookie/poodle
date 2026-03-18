//! WorkspaceShell specimen — full workspace with header, sidebar, content, and status bar.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    section_label(tree, root, "Full Workspace Shell", text_secondary);
    {
        let shell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(520.0))
            .with_height(LayoutSizing::Fixed(360.0))),
            NodeStyle {
                background: Some(bg_canvas),
                border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, shell);

        // ── App header ──
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_height(LayoutSizing::Fixed(36.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_gap(12.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), ..NodeStyle::default() });
        tree.add_child(shell, header);

        let logo = tree.create_node(Widget::Label { text: "\u{25C6} Workstation".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(header, logo);

        let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)),
            NodeStyle::default());
        tree.add_child(header, spacer);

        for &nav in &["Projects", "Assets", "Settings"] {
            let n = tree.create_node(Widget::Label { text: nav.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(header, n);
        }

        let header_sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(shell, header_sep);

        // ── Body (sidebar + content) ──
        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Grow)),
            NodeStyle::default());
        tree.add_child(shell, body);

        // Sidebar
        let sidebar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(140.0))
            .with_padding(LayoutEdges::uniform(8.0))
            .with_gap(2.0)),
            NodeStyle { background: Some(bg_elevated), ..NodeStyle::default() });
        tree.add_child(body, sidebar);

        for &(item, active) in &[("Dashboard", true), ("Documents", false), ("Media", false), ("Tasks", false)] {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(26.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    background: if active { Some(theme_bridge::tint(accent, 0.12)) } else { None },
                    corner_radii: [4.0; 4],
                    ..NodeStyle::default()
                });
            tree.add_child(sidebar, row);

            let lbl = tree.create_node(Widget::Label { text: item.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle {
                    text_color: Some(if active { accent } else { text_secondary }),
                    text_size: Some(11.0), ..NodeStyle::default()
                });
            tree.add_child(row, lbl);
        }

        let sidebar_sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(1.0))
            .with_height(LayoutSizing::Grow)),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(body, sidebar_sep);

        // Content
        let content = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_padding(LayoutEdges::uniform(16.0))
            .with_gap(12.0)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(body, content);

        let page_title = tree.create_node(Widget::Label { text: "Dashboard".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(16.0), ..NodeStyle::default() });
        tree.add_child(content, page_title);

        // Placeholder cards
        let cards = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(content, cards);

        for &(label, value) in &[("Projects", "12"), ("Tasks", "47"), ("Files", "238")] {
            let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_padding(LayoutEdges::uniform(10.0))
                .with_gap(2.0)
                .with_width(LayoutSizing::Fixed(90.0))),
                NodeStyle {
                    background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
                    corner_radii: [6.0; 4],
                    ..NodeStyle::default()
                });
            tree.add_child(cards, card);

            let v = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(18.0), ..NodeStyle::default() });
            tree.add_child(card, v);

            let l = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
            tree.add_child(card, l);
        }

        // ── Status bar ──
        let status_sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(shell, status_sep);

        let status = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_height(LayoutSizing::Fixed(22.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_gap(12.0)
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), ..NodeStyle::default() });
        tree.add_child(shell, status);

        let status_left = tree.create_node(Widget::Label { text: "\u{25CF} Connected".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(theme_bridge::resolve_vec4(theme, "semantic.color.status.success")),
                text_size: Some(9.0), ..NodeStyle::default()
            });
        tree.add_child(status, status_left);

        let status_right = tree.create_node(Widget::Label { text: "v0.1.0 \u{00B7} 60 fps".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(status, status_right);
    }

    let _ = text_inverse;
    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
