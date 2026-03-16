//! Popover specimen — anchored floating overlay mockup.

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
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Simple content popover ──
    section_label(tree, root, "Content Popover", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        // Trigger
        let trigger = tree.create(Widget::Button {
            label: "More Info ▾".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radius: 6.0, background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(col, trigger);

        // Popover card
        let card = popover_card(tree, 220.0, bg_elevated, border);
        tree.add_child(col, card);

        let body = tree.create(Widget::Panel, UiStyle {
            padding: Edges::all(12.0),
            direction: Direction::Column, gap: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(card, body);

        let title = tree.create(Widget::Label { text: "Details".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create(Widget::Label { text: "Additional information about this item.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(body, desc);
    }

    // ── Actions popover ──
    section_label(tree, root, "Actions Popover", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        let trigger = tree.create(Widget::Button {
            label: "⋯".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(32.0), height: Sizing::Fixed(28.0),
            corner_radius: 6.0, background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(14.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(col, trigger);

        let card = popover_card(tree, 160.0, bg_elevated, border);
        tree.add_child(col, card);

        let list = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, padding: Edges::all(4.0), gap: 1.0,
            ..UiStyle::default()
        });
        tree.add_child(card, list);

        for item in &["Edit", "Duplicate", "Archive", "Delete"] {
            let row = tree.create(Widget::Button {
                label: item.to_string(), pressed: false, hovered: false,
            }, UiStyle {
                width: Sizing::Grow(1.0), height: Sizing::Fixed(28.0),
                padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
                corner_radius: 4.0, text_color: Some(text_primary),
                text_size: Some(12.0), align: Align::Center,
                focusable: true, ..UiStyle::default()
            });
            tree.add_child(list, row);
        }
    }

    // ── Form popover ──
    section_label(tree, root, "Form Popover", text_secondary);
    {
        let card = popover_card(tree, 240.0, bg_elevated, border);
        tree.add_child(root, card);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, padding: Edges::all(12.0), gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(card, body);

        let title = tree.create(Widget::Label { text: "Quick Add".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let bg_canvas = theme_bridge::canvas_background(theme);
        let border_default = theme_bridge::border_default(theme);

        let input = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            background: Some(bg_canvas), border_color: Some(border_default),
            border_width: 1.0, corner_radius: 4.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(body, input);

        let ph = tree.create(Widget::Label { text: "Enter value...".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.6)),
            text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(input, ph);

        let accent = theme_bridge::accent_base(theme);
        let text_inverse = theme_bridge::text_inverse(theme);
        let add_btn = tree.create(Widget::Button {
            label: "Add".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(28.0),
            corner_radius: 4.0, background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(body, add_btn);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn popover_card(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(width),
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 8.0,
        ..UiStyle::default()
    })
}
