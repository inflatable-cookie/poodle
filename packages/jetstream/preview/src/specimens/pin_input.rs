//! PinInput specimen — multi-digit code entry with 4 and 6 digit variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── 4-digit empty ──
    label(tree, root, "4-Digit PIN (empty)", text_secondary);
    pin_row(tree, root, 4, &[], false, bg_canvas, border_default, text_primary, text_secondary, accent);

    // ── 4-digit partial ──
    label(tree, root, "4-Digit PIN (partial)", text_secondary);
    pin_row(tree, root, 4, &["1", "2"], false, bg_canvas, border_default, text_primary, text_secondary, accent);

    // ── 4-digit complete ──
    label(tree, root, "4-Digit PIN (complete)", text_secondary);
    pin_row(tree, root, 4, &["1", "2", "3", "4"], false, bg_canvas, border_default, text_primary, text_secondary, accent);

    // ── 6-digit ──
    label(tree, root, "6-Digit Verification Code", text_secondary);
    pin_row(tree, root, 6, &["8", "4", "2"], false, bg_canvas, border_default, text_primary, text_secondary, accent);

    // ── Masked mode ──
    label(tree, root, "Masked Mode", text_secondary);
    pin_row(tree, root, 4, &["\u{25cf}", "\u{25cf}", "\u{25cf}"], true, bg_canvas, border_default, text_primary, text_secondary, accent);

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn pin_row(
    tree: &mut UiTree,
    parent: UiNodeId,
    digits: usize,
    values: &[&str],
    _masked: bool,
    bg: glam::Vec4,
    border: glam::Vec4,
    text_primary: glam::Vec4,
    _text_secondary: glam::Vec4,
    accent: glam::Vec4,
) {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_gap(8.0)),
        NodeStyle::default());
    tree.add_child(parent, row);

    for i in 0..digits {
        let is_active = i == values.len() && i < digits; // cursor position
        let has_value = i < values.len();

        let cell_border = if is_active { accent } else { border };
        let cell_border_width = if is_active { 2.0 } else { 1.0 };

        let cell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(40.0))
            .with_height(LayoutSizing::Fixed(44.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg),
                border_color: Some(cell_border),
                border_width: cell_border_width,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(row, cell);

        if has_value {
            let val = tree.create_node(Widget::Label {
                text: values[i].to_string(),
            }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(18.0), ..NodeStyle::default() });
            tree.add_child(cell, val);
        } else if is_active {
            // Cursor indicator
            let cursor = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(2.0))
                .with_height(LayoutSizing::Fixed(20.0))),
                NodeStyle { background: Some(accent), corner_radii: [1.0; 4], ..NodeStyle::default() });
            tree.add_child(cell, cursor);
        }
    }
}
