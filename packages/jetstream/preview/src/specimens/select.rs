//! Select specimen — dropdown selection with placeholder, selected value, and options list.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_default(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Placeholder ──
    label(tree, root, "Placeholder", text_secondary);
    select_trigger(tree, root, "Select an option...", text_secondary, bg_canvas, border);

    // ── Selected ──
    label(tree, root, "Selected Value", text_secondary);
    select_trigger(tree, root, "Option B", text_primary, bg_canvas, border);

    // ── Open (showing options) ──
    label(tree, root, "Open State", text_secondary);
    {
        let container = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(240.0),
            gap: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(root, container);

        // Trigger (focused)
        let trigger = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_canvas),
            border_color: Some(accent),
            border_width: 2.0,
            corner_radius: 6.0,
            align: Align::Center,
            justify: Justify::SpaceBetween,
            ..UiStyle::default()
        });
        tree.add_child(container, trigger);

        let val = tree.create(Widget::Label { text: "Option B".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(trigger, val);

        let arrow = tree.create(Widget::Label { text: "▴".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(trigger, arrow);

        // Dropdown
        let dropdown = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0),
            padding: Edges::all(4.0),
            gap: 1.0,
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(container, dropdown);

        let options = ["Option A", "Option B", "Option C", "Option D"];
        for (i, &opt) in options.iter().enumerate() {
            let selected = i == 1;
            let item = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Grow(1.0),
                height: Sizing::Fixed(28.0),
                padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
                corner_radius: 4.0,
                background: if selected { Some(theme_bridge::tint(accent, 0.15)) } else { None },
                align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(dropdown, item);

            let lbl = tree.create(Widget::Label { text: opt.to_string() }, UiStyle {
                text_color: Some(if selected { accent } else { text_primary }),
                text_size: Some(12.0),
                ..UiStyle::default()
            });
            tree.add_child(item, lbl);
        }
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let trigger = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(240.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_canvas),
            border_color: Some(theme_bridge::border_subtle(theme)),
            border_width: 1.0,
            corner_radius: 6.0,
            align: Align::Center,
            justify: Justify::SpaceBetween,
            opacity: 0.5,
            ..UiStyle::default()
        });
        tree.add_child(root, trigger);

        let val = tree.create(Widget::Label { text: "Option A".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(trigger, val);

        let arrow = tree.create(Widget::Label { text: "▾".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(trigger, arrow);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn select_trigger(tree: &mut UiTree, parent: UiNodeId, text: &str, text_color: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let trigger = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Fixed(240.0),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 6.0,
        align: Align::Center,
        justify: Justify::SpaceBetween,
        ..UiStyle::default()
    });
    tree.add_child(parent, trigger);

    let val = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(text_color),
        text_size: Some(12.0),
        ..UiStyle::default()
    });
    tree.add_child(trigger, val);

    let arrow = tree.create(Widget::Label { text: "▾".to_string() }, UiStyle {
        text_color: Some(theme_bridge::tint(text_color, 0.6)),
        text_size: Some(10.0),
        ..UiStyle::default()
    });
    tree.add_child(trigger, arrow);
}
