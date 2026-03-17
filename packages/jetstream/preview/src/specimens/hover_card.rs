//! HoverCard specimen — rich preview card shown on hover.

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
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── User hover card ──
    section_label(tree, root, "User Hover Card", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        // Trigger link
        let link = tree.create(Widget::Label { text: "@johndoe".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(col, link);

        // Card
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(240.0),
            padding: Edges::all(16.0),
            gap: 8.0,
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(col, card);

        // Avatar + name row
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 10.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(card, header);

        let avatar = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(36.0), height: Sizing::Fixed(36.0),
            corner_radii: [18.0; 4], background: Some(accent),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(header, avatar);

        let initials = tree.create(Widget::Label { text: "JD".to_string() }, UiStyle {
            text_color: Some(theme_bridge::text_inverse(theme)),
            text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(avatar, initials);

        let info = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 2.0, ..UiStyle::default()
        });
        tree.add_child(header, info);

        let name = tree.create(Widget::Label { text: "John Doe".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(info, name);

        let handle = tree.create(Widget::Label { text: "@johndoe".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(info, handle);

        let bio = tree.create(Widget::Label { text: "Software engineer. Rust enthusiast.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(card, bio);
    }

    // ── Link hover card ──
    section_label(tree, root, "Link Preview Card", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        let link = tree.create(Widget::Label { text: "example.com/article".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(col, link);

        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(260.0),
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(col, card);

        // Image placeholder
        let img = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(80.0),
            background: Some(border), corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(card, img);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(12.0), gap: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(card, body);

        let title = tree.create(Widget::Label { text: "Getting Started with Rust".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create(Widget::Label { text: "A beginner-friendly guide to systems programming.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(body, desc);

        let domain = tree.create(Widget::Label { text: "example.com".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.6)),
            text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(body, domain);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
