//! EditableLabel specimen — display and edit modes.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Display mode ──
    label(tree, root, "Display Mode (click to edit)", text_secondary);
    {
        let display = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            padding: Edges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 },
            corner_radius: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(root, display);

        let val = tree.create(Widget::Label { text: "Project Alpha".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(14.0),
            ..UiStyle::default()
        });
        tree.add_child(display, val);

        let edit_icon = tree.create(Widget::Label { text: "✎".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(display, edit_icon);
    }

    // ── Edit mode ──
    label(tree, root, "Edit Mode (actively editing)", text_secondary);
    {
        let edit_row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, edit_row);

        let input = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(200.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_canvas),
            border_color: Some(accent),
            border_width: 2.0,
            corner_radius: 6.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(edit_row, input);

        let val = tree.create(Widget::Label { text: "Project Alpha".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(14.0),
            ..UiStyle::default()
        });
        tree.add_child(input, val);

        // Confirm/cancel buttons
        let confirm = tree.create(Widget::Button {
            label: "✓".to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(28.0),
            height: Sizing::Fixed(28.0),
            corner_radius: 4.0,
            background: Some(accent),
            text_color: Some(theme_bridge::text_inverse(theme)),
            text_size: Some(12.0),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(edit_row, confirm);

        let cancel = tree.create(Widget::Button {
            label: "✕".to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(28.0),
            height: Sizing::Fixed(28.0),
            corner_radius: 4.0,
            border_color: Some(border_default),
            border_width: 1.0,
            text_color: Some(text_secondary),
            text_size: Some(12.0),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(edit_row, cancel);
    }

    // ── Different sizes ──
    label(tree, root, "Size Variants", text_secondary);
    for &(lbl, size) in &[("Heading", 20.0_f32), ("Body", 14.0), ("Caption", 11.0)] {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            padding: Edges { top: 2.0, right: 4.0, bottom: 2.0, left: 4.0 },
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let val = tree.create(Widget::Label { text: format!("{} text", lbl) }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(size),
            ..UiStyle::default()
        });
        tree.add_child(row, val);

        let icon = tree.create(Widget::Label { text: "✎".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.5)),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(row, icon);
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
