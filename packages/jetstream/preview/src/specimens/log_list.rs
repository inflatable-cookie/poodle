//! LogList specimen — timestamped log viewer with severity icons.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    section_label(tree, root, "Activity Log", text_secondary);
    {
        let list = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(420.0))),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, list);

        let entries = &[
            ("✓", success, "10:42:15", "Deployment completed successfully"),
            ("⚠", warning, "10:41:58", "Memory usage exceeds 80% threshold"),
            ("✓", success, "10:41:30", "Health check passed — all services responsive"),
            ("✕", danger, "10:40:12", "Failed to connect to database replica-3"),
            ("ℹ", text_secondary, "10:39:45", "Auto-scaling triggered: 2 → 4 instances"),
            ("✓", success, "10:38:00", "Build #847 finished in 2m 14s"),
        ];

        for (i, &(icon, icon_color, time, message)) in entries.iter().enumerate() {
            if i > 0 {
                let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(1.0))),
                    NodeStyle { background: Some(border), ..NodeStyle::default() });
                tree.add_child(list, sep);
            }

            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(list, row);

            let ic = tree.create_node(Widget::Label { text: icon.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(16.0))),
                NodeStyle { text_color: Some(icon_color), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, ic);

            let ts = tree.create_node(Widget::Label { text: time.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(60.0))),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(row, ts);

            let msg = tree.create_node(Widget::Label { text: message.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
            tree.add_child(row, msg);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
