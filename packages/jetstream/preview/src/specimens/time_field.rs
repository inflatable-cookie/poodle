//! TimeField specimen — time-of-day input with value, placeholder, disabled states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    let states: &[(&str, &str, glam::Vec4, f32)] = &[
        ("Placeholder", "HH:MM", text_secondary, 1.0),
        ("Filled", "14:30", text_primary, 1.0),
        ("With seconds", "09:15:42", text_primary, 1.0),
        ("Disabled", "08:00", theme_bridge::tint(text_secondary, 0.5), 0.5),
    ];

    for &(name, value, text_color, opacity) in states {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 12.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let lbl = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            width: Sizing::Fixed(80.0),
            ..UiStyle::default()
        });
        tree.add_child(row, lbl);

        let input = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(140.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            gap: 4.0,
            background: Some(bg_canvas),
            border_color: Some(border_default),
            border_width: 1.0,
            corner_radius: 6.0,
            align: Align::Center,
            justify: Justify::SpaceBetween,
            opacity,
            ..UiStyle::default()
        });
        tree.add_child(row, input);

        let val = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
            text_color: Some(text_color),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(input, val);

        // Clock icon
        let icon = tree.create(Widget::Label { text: "⏱".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(input, icon);
    }

    root
}
