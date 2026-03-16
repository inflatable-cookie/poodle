//! Rating specimen — star-based rating display and input.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Full rating (5/5) ──
    label(tree, root, "Full (5/5)", text_secondary);
    star_row(tree, root, 5, 5, 16.0, warning, border);

    // ── Partial rating (3/5) ──
    label(tree, root, "Partial (3/5)", text_secondary);
    star_row(tree, root, 3, 5, 16.0, warning, border);

    // ── Empty (0/5) ──
    label(tree, root, "Empty (0/5)", text_secondary);
    star_row(tree, root, 0, 5, 16.0, warning, border);

    // ── Different scales ──
    label(tree, root, "10-point scale (7/10)", text_secondary);
    star_row(tree, root, 7, 10, 14.0, warning, border);

    // ── Accent colored ──
    label(tree, root, "Accent colored (4/5)", text_secondary);
    star_row(tree, root, 4, 5, 16.0, accent, border);

    // ── Size variants ──
    label(tree, root, "Size Variants", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        for &(lbl, size) in &[("Small", 12.0_f32), ("Medium", 16.0), ("Large", 22.0)] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(col, row);

            let l = tree.create(Widget::Label { text: lbl.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(50.0), ..UiStyle::default()
            });
            tree.add_child(row, l);

            star_row(tree, row, 3, 5, size, warning, border);
        }
    }

    // ── Disabled ──
    label(tree, root, "Disabled (4/5)", text_secondary);
    {
        let wrapper = tree.create(Widget::Panel, UiStyle {
            opacity: 0.5, ..UiStyle::default()
        });
        tree.add_child(root, wrapper);
        star_row(tree, wrapper, 4, 5, 16.0, warning, border);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn star_row(tree: &mut UiTree, parent: UiNodeId, filled: usize, total: usize, size: f32, active: glam::Vec4, inactive: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 2.0, ..UiStyle::default()
    });
    tree.add_child(parent, row);

    for i in 0..total {
        let (symbol, color) = if i < filled {
            ("★", active)
        } else {
            ("☆", inactive)
        };
        let star = tree.create(Widget::Label { text: symbol.to_string() }, UiStyle {
            text_color: Some(color), text_size: Some(size), ..UiStyle::default()
        });
        tree.add_child(row, star);
    }
}
