//! Code specimen — inline and block code display.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Inline code ──
    label(tree, root, "Inline Code", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 4.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        let prefix = tree.create(Widget::Label { text: "Use the ".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(row, prefix);

        inline_code(tree, row, "render()", bg_surface, border, text_primary);

        let suffix = tree.create(Widget::Label { text: " function".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(row, suffix);
    }

    // ── Multiple inline ──
    label(tree, root, "Multiple Inline Tokens", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        for token in &["let", "mut", "fn", "pub", "struct"] {
            inline_code(tree, row, token, bg_surface, border, accent);
        }
    }

    // ── Code block ──
    label(tree, root, "Code Block", text_secondary);
    {
        let block = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(320.0),
            padding: Edges::all(12.0),
            gap: 2.0,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, block);

        for line in &[
            "fn main() {",
            "    let x = 42;",
            "    println!(\"{}\", x);",
            "}",
        ] {
            let l = tree.create(Widget::Label { text: line.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(block, l);
        }
    }

    // ── Code block with line numbers ──
    label(tree, root, "With Line Numbers", text_secondary);
    {
        let block = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(320.0),
            padding: Edges::all(12.0),
            gap: 2.0,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, block);

        let lines = &[
            "use std::io;",
            "",
            "fn greet(name: &str) {",
            "    println!(\"Hello, {}!\", name);",
            "}",
        ];
        for (i, line) in lines.iter().enumerate() {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 12.0, ..UiStyle::default()
            });
            tree.add_child(block, row);

            let num = tree.create(Widget::Label { text: format!("{}", i + 1) }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(11.0),
                width: Sizing::Fixed(16.0), ..UiStyle::default()
            });
            tree.add_child(row, num);

            let code = tree.create(Widget::Label { text: line.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(row, code);
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

fn inline_code(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4) {
    let code = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        padding: Edges { top: 1.0, right: 6.0, bottom: 1.0, left: 6.0 },
        corner_radii: [4.0; 4],
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        text_color: Some(fg),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, code);
}
