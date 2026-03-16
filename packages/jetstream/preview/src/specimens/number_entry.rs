//! NumberEntry specimen — numeric input with increment/decrement buttons.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Standard ──
    label(tree, root, "Standard (value: 5)", text_secondary);
    number_input(tree, root, "5", bg_canvas, border, bg_surface, border_subtle, text_primary, text_secondary, 1.0);

    // ── With bounds ──
    label(tree, root, "With bounds (min: 0, max: 100, value: 42)", text_secondary);
    number_input(tree, root, "42", bg_canvas, border, bg_surface, border_subtle, text_primary, text_secondary, 1.0);

    // ── At minimum ──
    label(tree, root, "At minimum (decrement disabled)", text_secondary);
    {
        let group = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(32.0),
            ..UiStyle::default()
        });
        tree.add_child(root, group);

        // Disabled decrement
        let dec = tree.create(Widget::Button {
            label: "−".to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(32.0),
            height: Sizing::Fixed(32.0),
            background: Some(bg_surface),
            border_color: Some(border_subtle),
            border_width: 1.0,
            text_color: Some(theme_bridge::tint(text_secondary, 0.3)),
            text_size: Some(14.0),
            align: Align::Center,
            justify: Justify::Center,
            opacity: 0.5,
            ..UiStyle::default()
        });
        tree.add_child(group, dec);

        let input = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(60.0),
            height: Sizing::Fixed(32.0),
            background: Some(bg_canvas),
            border_color: Some(border),
            border_width: 1.0,
            align: Align::Center,
            justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(group, input);

        let val = tree.create(Widget::Label { text: "0".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(input, val);

        let inc = tree.create(Widget::Button {
            label: "+".to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(32.0),
            height: Sizing::Fixed(32.0),
            background: Some(bg_surface),
            border_color: Some(border_subtle),
            border_width: 1.0,
            text_color: Some(text_primary),
            text_size: Some(14.0),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(group, inc);
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    number_input(tree, root, "10", bg_canvas, border, bg_surface, border_subtle, text_primary, text_secondary, 0.5);

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

fn number_input(
    tree: &mut UiTree,
    parent: UiNodeId,
    value: &str,
    bg_canvas: glam::Vec4,
    border: glam::Vec4,
    bg_surface: glam::Vec4,
    border_subtle: glam::Vec4,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    opacity: f32,
) {
    let group = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        height: Sizing::Fixed(32.0),
        opacity,
        ..UiStyle::default()
    });
    tree.add_child(parent, group);

    // Decrement
    let dec = tree.create(Widget::Button {
        label: "−".to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(32.0),
        height: Sizing::Fixed(32.0),
        background: Some(bg_surface),
        border_color: Some(border_subtle),
        border_width: 1.0,
        text_color: Some(text_secondary),
        text_size: Some(14.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: opacity > 0.9,
        ..UiStyle::default()
    });
    tree.add_child(group, dec);

    // Value input
    let input = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(60.0),
        height: Sizing::Fixed(32.0),
        background: Some(bg_canvas),
        border_color: Some(border),
        border_width: 1.0,
        align: Align::Center,
        justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(group, input);

    let val = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(12.0),
        ..UiStyle::default()
    });
    tree.add_child(input, val);

    // Increment
    let inc = tree.create(Widget::Button {
        label: "+".to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(32.0),
        height: Sizing::Fixed(32.0),
        background: Some(bg_surface),
        border_color: Some(border_subtle),
        border_width: 1.0,
        text_color: Some(text_secondary),
        text_size: Some(14.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: opacity > 0.9,
        ..UiStyle::default()
    });
    tree.add_child(group, inc);
}
