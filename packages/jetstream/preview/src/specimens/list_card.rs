//! ListCard specimen — structured list item card with metadata.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    section_label(tree, root, "List Cards", text_secondary);
    {
        let list = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, width: Sizing::Fixed(340.0),
            ..UiStyle::default()
        });
        tree.add_child(root, list);

        for &(title, desc, tag, selected) in &[
            ("Project Alpha", "Last updated 2 hours ago", "Active", false),
            ("Project Beta", "Last updated yesterday", "Draft", true),
            ("Project Gamma", "Last updated 3 days ago", "Archived", false),
        ] {
            let card_border = if selected { accent } else { border };
            let card = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                width: Sizing::Grow(1.0),
                padding: Edges::all(12.0),
                gap: 10.0,
                background: Some(bg_elevated),
                border_color: Some(card_border),
                border_width: if selected { 2.0 } else { 1.0 },
                corner_radii: [8.0; 4],
                align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(list, card);

            let info = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, gap: 2.0,
                width: Sizing::Grow(1.0), ..UiStyle::default()
            });
            tree.add_child(card, info);

            let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
            });
            tree.add_child(info, t);

            let d = tree.create(Widget::Label { text: desc.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(info, d);

            let badge = tree.create(Widget::Label { text: tag.to_string() }, UiStyle {
                padding: Edges { top: 2.0, right: 8.0, bottom: 2.0, left: 8.0 },
                corner_radii: [10.0; 4], border_color: Some(border),
                border_width: 1.0, text_color: Some(text_secondary),
                text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(card, badge);
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
