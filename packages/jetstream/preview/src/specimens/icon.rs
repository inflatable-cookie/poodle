//! Icon specimen — icon display at various sizes and colors using text symbols.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Standard icons ──
    section_label(tree, root, "Icon Gallery (text symbols)", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 16.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        for &symbol in &["✎", "⚡", "⚙", "🔍", "📋", "📌", "🗑", "✓", "✕", "⬆"] {
            icon_cell(tree, row, symbol, text_primary, 16.0);
        }
    }

    // ── Size variants ──
    section_label(tree, root, "Size Variants", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 16.0, align: Align::End, ..UiStyle::default()
        });
        tree.add_child(root, row);

        for &(lbl, size) in &[("12px", 12.0_f32), ("16px", 16.0), ("20px", 20.0), ("24px", 24.0), ("32px", 32.0)] {
            let col = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, gap: 4.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(row, col);

            icon_cell(tree, col, "⚙", text_primary, size);

            let l = tree.create(Widget::Label { text: lbl.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
            });
            tree.add_child(col, l);
        }
    }

    // ── Color variants ──
    section_label(tree, root, "Color Variants", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 16.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        icon_cell(tree, row, "★", text_primary, 20.0);
        icon_cell(tree, row, "★", text_secondary, 20.0);
        icon_cell(tree, row, "★", accent, 20.0);
        icon_cell(tree, row, "★", success, 20.0);
        icon_cell(tree, row, "★", danger, 20.0);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn icon_cell(tree: &mut UiTree, parent: UiNodeId, symbol: &str, color: glam::Vec4, size: f32) {
    let icon = tree.create(Widget::Label { text: symbol.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(size), ..UiStyle::default()
    });
    tree.add_child(parent, icon);
}
