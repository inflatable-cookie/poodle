//! Collapsible specimen — simple show/hide content toggle.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Open state ──
    section_label(tree, root, "Open State", text_secondary);
    collapsible_block(tree, root, "Show Details", "Here are the expanded details. This section can contain any content including text, forms, or other components.", text_primary, text_secondary, border, true);

    // ── Closed state ──
    section_label(tree, root, "Closed State", text_secondary);
    collapsible_block(tree, root, "Show Details", "", text_primary, text_secondary, border, false);

    // ── Nested ──
    section_label(tree, root, "Nested Collapsibles", text_secondary);
    {
        let outer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(300.0),
            ..UiStyle::default()
        });
        tree.add_child(root, outer);

        // Outer trigger
        let trigger = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, align: Align::Center,
            padding: Edges { top: 6.0, right: 0.0, bottom: 6.0, left: 0.0 },
            ..UiStyle::default()
        });
        tree.add_child(outer, trigger);

        let chevron = tree.create(Widget::Label { text: "▾".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(trigger, chevron);

        let lbl = tree.create(Widget::Label { text: "Parent Section".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(trigger, lbl);

        // Nested content
        let nested = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 16.0 },
            gap: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(outer, nested);

        collapsible_block(tree, nested, "Child A", "Content of child A.", text_primary, text_secondary, border, true);
        collapsible_block(tree, nested, "Child B", "", text_primary, text_secondary, border, false);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn collapsible_block(tree: &mut UiTree, parent: UiNodeId, label: &str, content: &str, fg: glam::Vec4, muted: glam::Vec4, _border: glam::Vec4, open: bool) {
    let block = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, ..UiStyle::default()
    });
    tree.add_child(parent, block);

    let trigger = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 6.0, align: Align::Center,
        padding: Edges { top: 6.0, right: 0.0, bottom: 6.0, left: 0.0 },
        ..UiStyle::default()
    });
    tree.add_child(block, trigger);

    let chevron = tree.create(Widget::Label {
        text: if open { "▾" } else { "▸" }.to_string(),
    }, UiStyle {
        text_color: Some(muted), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(trigger, chevron);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(13.0), ..UiStyle::default()
    });
    tree.add_child(trigger, lbl);

    if open && !content.is_empty() {
        let body = tree.create(Widget::Panel, UiStyle {
            padding: Edges { top: 0.0, right: 0.0, bottom: 8.0, left: 16.0 },
            ..UiStyle::default()
        });
        tree.add_child(block, body);

        let txt = tree.create(Widget::Label { text: content.to_string() }, UiStyle {
            text_color: Some(muted), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, txt);
    }
}
