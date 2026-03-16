//! LogList specimen — timestamped log viewer with severity icons.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 16.0, ..UiStyle::default()
    });

    section_label(tree, root, "Activity Log", text_secondary);
    {
        let list = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, width: Sizing::Fixed(420.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
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
                let sep = tree.create(Widget::Panel, UiStyle {
                    width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
                    background: Some(border), ..UiStyle::default()
                });
                tree.add_child(list, sep);
            }

            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
                gap: 8.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(list, row);

            let ic = tree.create(Widget::Label { text: icon.to_string() }, UiStyle {
                text_color: Some(icon_color), text_size: Some(12.0),
                width: Sizing::Fixed(16.0), ..UiStyle::default()
            });
            tree.add_child(row, ic);

            let ts = tree.create(Widget::Label { text: time.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(60.0), ..UiStyle::default()
            });
            tree.add_child(row, ts);

            let msg = tree.create(Widget::Label { text: message.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(row, msg);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
