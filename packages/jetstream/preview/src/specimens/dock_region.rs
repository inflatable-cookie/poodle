//! DockRegion specimen — dockable regions with resize handles and snap positions.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── 3-column docked layout ──
    section_label(tree, root, "Three-Column Dock Layout", text_secondary);
    {
        let dock = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(520.0), height: Sizing::Fixed(260.0),
            background: Some(bg_canvas), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, dock);

        // Left sidebar
        let left = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(120.0), height: Sizing::Grow(1.0),
            background: Some(bg_elevated), padding: Edges::all(10.0), gap: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(dock, left);

        let left_title = tree.create(Widget::Label { text: "Explorer".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(left, left_title);

        for &item in &["src/", "  main.rs", "  lib.rs", "assets/", "  icon.png"] {
            let lbl = tree.create(Widget::Label { text: item.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
            });
            tree.add_child(left, lbl);
        }

        // Resize handle 1
        resize_handle(tree, dock, accent);

        // Center content
        let center = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0), height: Sizing::Grow(1.0),
            background: Some(bg_surface), padding: Edges::all(12.0), gap: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(dock, center);

        let center_title = tree.create(Widget::Label { text: "Editor".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(center, center_title);

        for &line in &["fn main() {", "    println!(\"hello\");", "}"] {
            let lbl = tree.create(Widget::Label { text: line.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
            });
            tree.add_child(center, lbl);
        }

        // Resize handle 2
        resize_handle(tree, dock, accent);

        // Right inspector
        let right = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(130.0), height: Sizing::Grow(1.0),
            background: Some(bg_elevated), padding: Edges::all(10.0), gap: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(dock, right);

        let right_title = tree.create(Widget::Label { text: "Inspector".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(right, right_title);

        for &(k, v) in &[("Type", "Function"), ("Args", "0"), ("Returns", "()"), ("Visibility", "pub")] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 4.0, ..UiStyle::default()
            });
            tree.add_child(right, row);

            let kl = tree.create(Widget::Label { text: k.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(9.0),
                width: Sizing::Fixed(50.0), ..UiStyle::default()
            });
            tree.add_child(row, kl);

            let vl = tree.create(Widget::Label { text: v.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(9.0), ..UiStyle::default()
            });
            tree.add_child(row, vl);
        }
    }

    root
}

fn resize_handle(tree: &mut UiTree, parent: UiNodeId, accent: glam::Vec4) {
    let handle = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(4.0), height: Sizing::Grow(1.0),
        background: Some(theme_bridge::tint(accent, 0.25)),
        ..UiStyle::default()
    });
    tree.add_child(parent, handle);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
