//! ColorPicker specimen — color swatch selection and preview.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Swatch palette ──
    label(tree, root, "Swatch Palette", text_secondary);
    {
        let grid = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, ..UiStyle::default()
        });
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
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, ..UiStyle::default()
        });
        tree.add_child(root, row);

        swatch(tree, row, glam::Vec4::new(0.91, 0.30, 0.24, 1.0), 28.0, border_subtle, false);
        swatch(tree, row, glam::Vec4::new(0.90, 0.49, 0.13, 1.0), 28.0, border_subtle, false);
        swatch(tree, row, glam::Vec4::new(0.20, 0.60, 0.86, 1.0), 28.0, accent, true); // selected
        swatch(tree, row, glam::Vec4::new(0.56, 0.27, 0.68, 1.0), 28.0, border_subtle, false);
    }

    // ── With input field ──
    label(tree, root, "With Hex Input", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        // Preview swatch
        swatch(tree, row, glam::Vec4::new(0.20, 0.60, 0.86, 1.0), 32.0, border, false);

        // Hex input
        let input = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(100.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_canvas),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(row, input);

        let hex = tree.create(Widget::Label { text: "#3399DB".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(input, hex);
    }

    // ── Size variants ──
    label(tree, root, "Size Variants", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 12.0, align: Align::End, ..UiStyle::default()
        });
        tree.add_child(root, row);

        let blue = glam::Vec4::new(0.20, 0.60, 0.86, 1.0);
        for &(lbl, size) in &[("Sm", 20.0_f32), ("Md", 28.0), ("Lg", 36.0)] {
            let col = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, gap: 4.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(row, col);

            swatch(tree, col, blue, size, border_subtle, false);

            let l = tree.create(Widget::Label { text: lbl.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
            });
            tree.add_child(col, l);
        }
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, opacity: 0.5, ..UiStyle::default()
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
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn swatch(tree: &mut UiTree, parent: UiNodeId, color: glam::Vec4, size: f32, border: glam::Vec4, selected: bool) {
    let s = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(size),
        height: Sizing::Fixed(size),
        corner_radius: 4.0,
        background: Some(color),
        border_color: Some(border),
        border_width: if selected { 2.0 } else { 1.0 },
        ..UiStyle::default()
    });
    tree.add_child(parent, s);
}
