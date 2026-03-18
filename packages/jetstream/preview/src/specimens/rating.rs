//! Rating specimen — star-based rating display and input.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

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
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        for &(lbl, size) in &[("Small", 12.0_f32), ("Medium", 16.0), ("Large", 22.0)] {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(col, row);

            let l = tree.create_node(Widget::Label { text: lbl.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(50.0))),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(row, l);

            star_row(tree, row, 3, 5, size, warning, border);
        }
    }

    // ── Disabled ──
    label(tree, root, "Disabled (4/5)", text_secondary);
    {
        let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { opacity: 0.5, ..NodeStyle::default() });
        tree.add_child(root, wrapper);
        star_row(tree, wrapper, 4, 5, 16.0, warning, border);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn star_row(tree: &mut UiTree, parent: UiNodeId, filled: usize, total: usize, size: f32, active: glam::Vec4, inactive: glam::Vec4) {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_gap(2.0)),
        NodeStyle::default());
    tree.add_child(parent, row);

    for i in 0..total {
        let (symbol, color) = if i < filled {
            ("\u{2605}", active)
        } else {
            ("\u{2606}", inactive)
        };
        let star = tree.create_node(Widget::Label { text: symbol.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(color), text_size: Some(size), ..NodeStyle::default() });
        tree.add_child(row, star);
    }
}
