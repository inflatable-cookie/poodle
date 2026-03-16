//! TextInput specimen — single-line input with placeholder, filled, disabled, error states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── States ──
    let states: &[(&str, &str, glam::Vec4, glam::Vec4, f32)] = &[
        ("Placeholder", "Enter your name...", text_secondary, border_default, 1.0),
        ("Filled", "John Doe", text_primary, border_default, 1.0),
        ("Focused", "Typing...", text_primary, accent, 2.0),
        ("Error", "invalid-email", text_primary, danger, 2.0),
        ("Disabled", "Read only", theme_bridge::tint(text_secondary, 0.5), border_subtle, 1.0),
    ];

    for &(label, value, text_color, border_color, border_width) in states {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 12.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            width: Sizing::Fixed(80.0),
            ..UiStyle::default()
        });
        tree.add_child(row, lbl);

        let opacity = if label == "Disabled" { 0.5 } else { 1.0 };

        let input = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(240.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_canvas),
            border_color: Some(border_color),
            border_width,
            corner_radius: 6.0,
            align: Align::Center,
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
    }

    // ── Error message ──
    let err = tree.create(Widget::Label {
        text: "  ↳ Please enter a valid email address".to_string(),
    }, UiStyle {
        text_color: Some(danger),
        text_size: Some(10.0),
        padding: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 92.0 },
        ..UiStyle::default()
    });
    tree.add_child(root, err);

    root
}
