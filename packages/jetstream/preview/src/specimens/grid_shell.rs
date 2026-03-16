//! GridShell specimen — responsive card grid browse layout.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 24.0,
        ..UiStyle::default()
    });

    // ── 3-column grid ──
    section_label(tree, root, "3-Column Grid", text_secondary);
    {
        let shell = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 12.0,
            padding: Edges::all(16.0),
            background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, shell);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, header);

        let title = tree.create(Widget::Label { text: "Media Library".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let count = tree.create(Widget::Label { text: "6 items".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(header, count);

        // Grid rows
        let items = &[
            ("photo_01.jpg", "1.2 MB"),
            ("banner.png", "340 KB"),
            ("logo.svg", "12 KB"),
            ("hero_bg.jpg", "2.8 MB"),
            ("icon_set.zip", "890 KB"),
            ("avatar.png", "64 KB"),
        ];

        for row_items in items.chunks(3) {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 12.0,
                ..UiStyle::default()
            });
            tree.add_child(shell, row);

            for &(name, size) in row_items {
                let card = tree.create(Widget::Panel, UiStyle {
                    direction: Direction::Column,
                    width: Sizing::Fixed(120.0),
                    background: Some(bg_elevated),
                    border_color: Some(border), border_width: 1.0,
                    corner_radius: 6.0,
                    ..UiStyle::default()
                });
                tree.add_child(row, card);

                // Placeholder thumbnail
                let thumb = tree.create(Widget::Panel, UiStyle {
                    width: Sizing::Grow(1.0), height: Sizing::Fixed(72.0),
                    background: Some(theme_bridge::tint(border, 0.5)),
                    ..UiStyle::default()
                });
                tree.add_child(card, thumb);

                let info = tree.create(Widget::Panel, UiStyle {
                    direction: Direction::Column, padding: Edges::all(8.0), gap: 2.0,
                    ..UiStyle::default()
                });
                tree.add_child(card, info);

                let n = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
                    text_color: Some(text_primary), text_size: Some(10.0), ..UiStyle::default()
                });
                tree.add_child(info, n);

                let s = tree.create(Widget::Label { text: size.to_string() }, UiStyle {
                    text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
                });
                tree.add_child(info, s);
            }
        }
    }

    // ── Empty grid ──
    section_label(tree, root, "Empty Grid", text_secondary);
    {
        let shell = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(400.0),
            padding: Edges::all(32.0),
            background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, shell);

        let msg = tree.create(Widget::Label { text: "No media uploaded".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(shell, msg);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
