//! SelectionSummary specimen — current selection state display with count and clear.

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

    // ── Standard ──
    section_label(tree, root, "Standard Summary", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(360.0),
            height: Sizing::Fixed(36.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            gap: 8.0,
            background: Some(theme_bridge::tint(accent, 0.08)),
            border_color: Some(theme_bridge::tint(accent, 0.3)), border_width: 1.0,
            corner_radius: 6.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        let count = tree.create(Widget::Label { text: "3 items selected".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(bar, count);

        let spacer = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(bar, spacer);

        let clear = tree.create(Widget::Label { text: "Clear selection".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(bar, clear);
    }

    // ── With actions ──
    section_label(tree, root, "With Batch Actions", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(420.0),
            height: Sizing::Fixed(40.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            gap: 8.0,
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        let count = tree.create(Widget::Label { text: "12 selected".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(bar, count);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(1.0), height: Sizing::Fixed(20.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(bar, sep);

        for &label in &["Export", "Archive", "Delete"] {
            let btn = tree.create(Widget::Button {
                label: label.to_string(), pressed: false, hovered: false,
            }, UiStyle {
                height: Sizing::Fixed(26.0),
                padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
                corner_radius: 4.0,
                text_color: Some(text_primary), text_size: Some(11.0),
                align: Align::Center, justify: Justify::Center,
                focusable: true, ..UiStyle::default()
            });
            tree.add_child(bar, btn);
        }

        let spacer = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(bar, spacer);

        let clear = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(bar, clear);
    }

    // ── Single selection ──
    section_label(tree, root, "Single Selection", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        let count = tree.create(Widget::Label { text: "1 item selected".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(bar, count);

        let clear = tree.create(Widget::Label { text: "Clear".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(bar, clear);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
