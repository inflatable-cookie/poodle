//! Accordion specimen — expandable disclosure panels.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Standard accordion ──
    section_label(tree, root, "Standard (one open)", text_secondary);
    {
        let acc = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(320.0),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0, background: Some(bg_elevated),
            ..UiStyle::default()
        });
        tree.add_child(root, acc);

        accordion_item(tree, acc, "Getting Started", Some("Follow these steps to set up your project and begin development."), text_primary, text_secondary, border, true);
        accordion_item(tree, acc, "Configuration", None, text_primary, text_secondary, border, false);
        accordion_item(tree, acc, "Advanced Usage", None, text_primary, text_secondary, border, false);
    }

    // ── Multiple open ──
    section_label(tree, root, "Multiple Open", text_secondary);
    {
        let acc = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(320.0),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0, background: Some(bg_elevated),
            ..UiStyle::default()
        });
        tree.add_child(root, acc);

        accordion_item(tree, acc, "Section A", Some("Content for section A is visible here."), text_primary, text_secondary, border, true);
        accordion_item(tree, acc, "Section B", Some("Content for section B is also expanded."), text_primary, text_secondary, border, true);
        accordion_item(tree, acc, "Section C", None, text_primary, text_secondary, border, false);
    }

    // ── All collapsed ──
    section_label(tree, root, "All Collapsed", text_secondary);
    {
        let acc = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(320.0),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0, background: Some(bg_elevated),
            ..UiStyle::default()
        });
        tree.add_child(root, acc);

        accordion_item(tree, acc, "FAQ Item 1", None, text_primary, text_secondary, border, false);
        accordion_item(tree, acc, "FAQ Item 2", None, text_primary, text_secondary, border, false);
        accordion_item(tree, acc, "FAQ Item 3", None, text_primary, text_secondary, border, false);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn accordion_item(tree: &mut UiTree, parent: UiNodeId, title: &str, content: Option<&str>, fg: glam::Vec4, muted: glam::Vec4, border: glam::Vec4, open: bool) {
    let item = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        border_color: Some(border), border_width: 1.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, item);

    // Header
    let header = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(40.0),
        padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
        align: Align::Center, justify: Justify::SpaceBetween,
        ..UiStyle::default()
    });
    tree.add_child(item, header);

    let lbl = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(13.0), ..UiStyle::default()
    });
    tree.add_child(header, lbl);

    let chevron = tree.create(Widget::Label {
        text: if open { "▾" } else { "▸" }.to_string(),
    }, UiStyle {
        text_color: Some(muted), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(header, chevron);

    // Content (if open)
    if let Some(text) = content {
        let body = tree.create(Widget::Panel, UiStyle {
            padding: Edges { top: 0.0, right: 12.0, bottom: 12.0, left: 12.0 },
            ..UiStyle::default()
        });
        tree.add_child(item, body);

        let txt = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
            text_color: Some(muted), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, txt);
    }
}
