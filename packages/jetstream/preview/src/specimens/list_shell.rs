//! ListShell specimen — list layout browse view with empty, loading, error, and populated states.

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
    let accent = theme_bridge::accent_base(theme);
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(24.0)),
        NodeStyle::default());

    // ── Populated ──
    section_label(tree, root, "Populated List", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        // Header
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(shell, header);
        let title = tree.create_node(Widget::Label { text: "Documents".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(header, title);
        let count = tree.create_node(Widget::Label { text: "3 items".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(header, count);

        for &(name, meta) in &[
            ("Project Brief.pdf", "Updated 2 hours ago · 2.4 MB"),
            ("Q4 Report.xlsx", "Updated yesterday · 1.1 MB"),
            ("Design Spec.md", "Updated 3 days ago · 48 KB"),
        ] {
            let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(1.0))),
                NodeStyle { background: Some(border), ..NodeStyle::default() });
            tree.add_child(shell, sep);

            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 })
                .with_gap(2.0)),
                NodeStyle::default());
            tree.add_child(shell, row);

            let n = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, n);

            let m = tree.create_node(Widget::Label { text: meta.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(row, m);
        }
    }

    // ── Loading ──
    section_label(tree, root, "Loading State", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        let loading = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges::uniform(32.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(shell, loading);

        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(120.0))
            .with_height(LayoutSizing::Fixed(4.0))),
            NodeStyle { background: Some(theme_bridge::tint(accent, 0.3)), corner_radii: [2.0; 4], ..NodeStyle::default() });
        tree.add_child(loading, bar);
    }

    // ── Empty ──
    section_label(tree, root, "Empty State", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        let empty = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(32.0))
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(shell, empty);

        let msg = tree.create_node(Widget::Label { text: "No documents yet".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(empty, msg);

        let hint = tree.create_node(Widget::Label { text: "Upload or create a new document to get started.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(theme_bridge::tint(text_secondary, 0.7)), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(empty, hint);
    }

    // ── Error ──
    section_label(tree, root, "Error State", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        let err = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(24.0))
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(shell, err);

        let icon = tree.create_node(Widget::Label { text: "⚠".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(warning), text_size: Some(20.0), ..NodeStyle::default() });
        tree.add_child(err, icon);

        let msg = tree.create_node(Widget::Label { text: "Failed to load documents".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(err, msg);

        let retry = tree.create_node(Widget::Button {
            label: "Retry".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0, text_color: Some(text_primary), text_size: Some(11.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(err, retry);
    }

    root
}

fn list_shell_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(360.0))),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() })
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
