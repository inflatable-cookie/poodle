//! TimeAgo specimen — relative timestamp display (e.g., "3 minutes ago").

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    // ── Time intervals ──
    label(tree, root, "Time Intervals", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(6.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        for text in &[
            "Just now",
            "3 minutes ago",
            "1 hour ago",
            "5 hours ago",
            "Yesterday",
            "3 days ago",
            "2 weeks ago",
            "3 months ago",
            "1 year ago",
        ] {
            time_entry(tree, col, text, text_primary);
        }
    }

    // ── With tooltip hint ──
    label(tree, root, "With Full Timestamp (tooltip pattern)", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(6.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        for &(relative, full) in &[
            ("5 minutes ago", "2026-03-16 14:23:00"),
            ("2 hours ago", "2026-03-16 12:28:00"),
            ("Yesterday", "2026-03-15 09:15:00"),
        ] {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(col, row);

            let rel = tree.create_node(Widget::Label { text: relative.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(120.0))),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, rel);

            let abs = tree.create_node(Widget::Label { text: format!("({})", full) }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(row, abs);
        }
    }

    // ── In list context ──
    label(tree, root, "In List Context", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(4.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        for &(item, time) in &[
            ("Deployed v2.1.0", "3 minutes ago"),
            ("Merged PR #42", "1 hour ago"),
            ("Created branch", "Yesterday"),
        ] {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)
                .with_width(LayoutSizing::Fixed(300.0))),
                NodeStyle::default());
            tree.add_child(col, row);

            let name = tree.create_node(Widget::Label { text: item.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(row, name);

            let ts = tree.create_node(Widget::Label { text: time.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
            tree.add_child(row, ts);
        }
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn time_entry(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
