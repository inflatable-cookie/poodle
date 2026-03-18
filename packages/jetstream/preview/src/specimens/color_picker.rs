//! ColorPicker specimen — color swatch selection and preview.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(16.0)), NodeStyle::default());

    // ── Swatch palette ──
    label(tree, root, "Swatch Palette", text_secondary);
    {
        let grid = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(6.0)), NodeStyle::default());
        tree.add_child(root, grid);

        let swatches = [
            glam::Vec4::new(0.91, 0.30, 0.24, 1.0),  // red
            glam::Vec4::new(0.90, 0.49, 0.13, 1.0),  // orange
            glam::Vec4::new(0.95, 0.77, 0.06, 1.0),  // yellow
            glam::Vec4::new(0.18, 0.80, 0.44, 1.0),  // green
            glam::Vec4::new(0.20, 0.60, 0.86, 1.0),  // blue
            glam::Vec4::new(0.56, 0.27, 0.68, 1.0),  // purple
            glam::Vec4::new(0.58, 0.65, 0.65, 1.0),  // gray
            glam::Vec4::new(0.20, 0.29, 0.37, 1.0),  // dark
        ];
        for color in &swatches {
            swatch(tree, grid, *color, 28.0, border_subtle, false);
        }
    }

    // ── Selected state ──
    label(tree, root, "Selected Swatch", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(6.0)), NodeStyle::default());
        tree.add_child(root, row);

        swatch(tree, row, glam::Vec4::new(0.91, 0.30, 0.24, 1.0), 28.0, border_subtle, false);
        swatch(tree, row, glam::Vec4::new(0.90, 0.49, 0.13, 1.0), 28.0, border_subtle, false);
        swatch(tree, row, glam::Vec4::new(0.20, 0.60, 0.86, 1.0), 28.0, accent, true); // selected
        swatch(tree, row, glam::Vec4::new(0.56, 0.27, 0.68, 1.0), 28.0, border_subtle, false);
    }

    // ── With input field ──
    label(tree, root, "With Hex Input", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(8.0).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(root, row);

        // Preview swatch
        swatch(tree, row, glam::Vec4::new(0.20, 0.60, 0.86, 1.0), 32.0, border, false);

        // Hex input
        let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(100.0)).with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 }).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_canvas),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(row, input);

        let hex = tree.create_node(Widget::Label { text: "#3399DB".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(input, hex);
    }

    // ── Size variants ──
    label(tree, root, "Size Variants", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(12.0).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::End)), NodeStyle::default());
        tree.add_child(root, row);

        let blue = glam::Vec4::new(0.20, 0.60, 0.86, 1.0);
        for &(lbl, size) in &[("Sm", 20.0_f32), ("Md", 28.0), ("Lg", 36.0)] {
            let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_gap(4.0).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
            tree.add_child(row, col);

            swatch(tree, col, blue, size, border_subtle, false);

            let l = tree.create_node(Widget::Label { text: lbl.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default()
            });
            tree.add_child(col, l);
        }
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(6.0)), NodeStyle {
            opacity: 0.5, ..NodeStyle::default()
        });
        tree.add_child(root, row);

        for color in &[
            glam::Vec4::new(0.91, 0.30, 0.24, 1.0),
            glam::Vec4::new(0.18, 0.80, 0.44, 1.0),
            glam::Vec4::new(0.20, 0.60, 0.86, 1.0),
        ] {
            swatch(tree, row, *color, 28.0, border_subtle, false);
        }
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn swatch(tree: &mut UiTree, parent: UiNodeId, color: glam::Vec4, size: f32, border: glam::Vec4, selected: bool) {
    let s = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(size)).with_height(LayoutSizing::Fixed(size))), NodeStyle {
        corner_radii: [4.0; 4],
        background: Some(color),
        border_color: Some(border),
        border_width: if selected { 2.0 } else { 1.0 },
        ..NodeStyle::default()
    });
    tree.add_child(parent, s);
}
