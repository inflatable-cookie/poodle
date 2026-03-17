//! Card specimen — contained surface for content display.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Basic card ──
    section_label(tree, root, "Basic Card", text_secondary);
    {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(280.0),
            padding: Edges::all(16.0),
            gap: 8.0,
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, card);

        let title = tree.create(Widget::Label { text: "Card Title".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(card, title);

        let body = tree.create(Widget::Label { text: "This is a basic card with some descriptive text content.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(card, body);
    }

    // ── Card with image ──
    section_label(tree, root, "Card with Image", text_secondary);
    {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(280.0),
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, card);

        let img = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(120.0),
            background: Some(border), corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(card, img);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(16.0), gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(card, body);

        let title = tree.create(Widget::Label { text: "Featured Article".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create(Widget::Label { text: "An overview of the latest updates.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, desc);
    }

    // ── Card with actions ──
    section_label(tree, root, "Card with Actions", text_secondary);
    {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(280.0),
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, card);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(16.0), gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(card, body);

        let title = tree.create(Widget::Label { text: "Subscription".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create(Widget::Label { text: "Manage your subscription plan.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, desc);

        // Separator
        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(card, sep);

        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 },
            gap: 8.0, justify: Justify::End,
            ..UiStyle::default()
        });
        tree.add_child(card, footer);

        let btn = tree.create(Widget::Button {
            label: "Upgrade".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, btn);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
