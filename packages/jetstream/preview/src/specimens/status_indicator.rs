//! StatusIndicator specimen — colored dot with label for status display.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Status tones ──
    label(tree, root, "Status Tones", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        status_row(tree, col, "Online", success, text_primary);
        status_row(tree, col, "Away", warning, text_primary);
        status_row(tree, col, "Busy", danger, text_primary);
        status_row(tree, col, "Info", accent, text_primary);
    }

    // ── Sizes ──
    label(tree, root, "Size Variants", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        for &(lbl, size) in &[("Small (6px)", 6.0), ("Medium (8px)", 8.0), ("Large (10px)", 10.0)] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(col, row);

            let dot = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Fixed(size),
                height: Sizing::Fixed(size),
                corner_radius: size / 2.0,
                background: Some(success),
                ..UiStyle::default()
            });
            tree.add_child(row, dot);

            let l = tree.create(Widget::Label { text: lbl.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(row, l);
        }
    }

    // ── Inline with text ──
    label(tree, root, "Inline Usage", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 12.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        for &(text, color) in &[("Server A: Online", success), ("Server B: Degraded", warning), ("Server C: Down", danger)] {
            let item = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 4.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(row, item);

            let dot = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Fixed(8.0),
                height: Sizing::Fixed(8.0),
                corner_radius: 4.0,
                background: Some(color),
                ..UiStyle::default()
            });
            tree.add_child(item, dot);

            let l = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(item, l);
        }
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn status_row(tree: &mut UiTree, parent: UiNodeId, text: &str, dot_color: glam::Vec4, text_color: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let dot = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(8.0),
        height: Sizing::Fixed(8.0),
        corner_radius: 4.0,
        background: Some(dot_color),
        ..UiStyle::default()
    });
    tree.add_child(row, dot);

    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(text_color), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, lbl);
}
