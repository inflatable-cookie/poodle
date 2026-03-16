//! NavCardGrid specimen — responsive grid of navigation cards.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    section_label(tree, root, "2-Column Grid", text_secondary);
    {
        let items = &[
            ("⚙", "Settings", "Preferences"),
            ("📊", "Analytics", "Reports"),
            ("🔒", "Security", "Auth config"),
            ("📋", "Templates", "Manage"),
        ];

        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        for chunk in items.chunks(2) {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 8.0, ..UiStyle::default()
            });
            tree.add_child(col, row);

            for &(icon, title, desc) in chunk {
                grid_card(tree, row, icon, title, desc, accent, text_primary, text_secondary, bg_elevated, border, 150.0);
            }
        }
    }

    section_label(tree, root, "3-Column Grid", text_secondary);
    {
        let items = &[
            ("📄", "Docs"), ("🔧", "Tools"), ("📦", "Packages"),
            ("🎨", "Design"), ("🧪", "Testing"), ("🚀", "Deploy"),
        ];

        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        for chunk in items.chunks(3) {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 8.0, ..UiStyle::default()
            });
            tree.add_child(col, row);

            for &(icon, title) in chunk {
                let card = tree.create(Widget::Panel, UiStyle {
                    direction: Direction::Column,
                    width: Sizing::Fixed(100.0), height: Sizing::Fixed(80.0),
                    padding: Edges::all(12.0), gap: 6.0,
                    background: Some(bg_elevated), border_color: Some(border),
                    border_width: 1.0, corner_radius: 8.0,
                    align: Align::Center, justify: Justify::Center,
                    ..UiStyle::default()
                });
                tree.add_child(row, card);

                let sym = tree.create(Widget::Label { text: icon.to_string() }, UiStyle {
                    text_size: Some(20.0), ..UiStyle::default()
                });
                tree.add_child(card, sym);

                let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
                    text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
                });
                tree.add_child(card, t);
            }
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn grid_card(tree: &mut UiTree, parent: UiNodeId, icon: &str, title: &str, desc: &str, accent: glam::Vec4, fg: glam::Vec4, muted: glam::Vec4, bg: glam::Vec4, border: glam::Vec4, width: f32) {
    let card = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(width),
        padding: Edges::all(14.0), gap: 8.0,
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radius: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, card);

    let ic = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(28.0), height: Sizing::Fixed(28.0),
        corner_radius: 6.0, background: Some(theme_bridge::tint(accent, 0.12)),
        align: Align::Center, justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(card, ic);

    let sym = tree.create(Widget::Label { text: icon.to_string() }, UiStyle {
        text_size: Some(13.0), ..UiStyle::default()
    });
    tree.add_child(ic, sym);

    let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(card, t);

    let d = tree.create(Widget::Label { text: desc.to_string() }, UiStyle {
        text_color: Some(muted), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(card, d);
}
