//! CardRadioGroup specimen — radio selection across cards.

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
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 16.0, ..UiStyle::default()
    });

    section_label(tree, root, "Plan Selection", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 12.0, ..UiStyle::default()
        });
        tree.add_child(root, row);

        for &(name, price, desc, selected) in &[
            ("Free", "$0/mo", "Basic features", false),
            ("Pro", "$29/mo", "Advanced tools", true),
            ("Enterprise", "Custom", "Full platform", false),
        ] {
            let card = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column,
                width: Sizing::Fixed(140.0), padding: Edges::all(12.0), gap: 4.0,
                background: Some(bg_elevated),
                border_color: Some(if selected { accent } else { border }),
                border_width: if selected { 2.0 } else { 1.0 },
                corner_radius: 8.0,
                ..UiStyle::default()
            });
            tree.add_child(row, card);

            let radio = tree.create(Widget::Label {
                text: if selected { "◉" } else { "○" }.to_string(),
            }, UiStyle {
                text_color: Some(if selected { accent } else { text_secondary }),
                text_size: Some(14.0), ..UiStyle::default()
            });
            tree.add_child(card, radio);

            let n = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
            });
            tree.add_child(card, n);

            let p = tree.create(Widget::Label { text: price.to_string() }, UiStyle {
                text_color: Some(if selected { accent } else { text_primary }),
                text_size: Some(16.0), ..UiStyle::default()
            });
            tree.add_child(card, p);

            let d = tree.create(Widget::Label { text: desc.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(card, d);
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
