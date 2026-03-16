//! DateTimePicker specimen — combined date and time input.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_default(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Empty ──
    section_label(tree, root, "Empty (placeholder)", text_secondary);
    datetime_input(tree, root, "Select date and time...", bg_canvas, border, text_secondary, 1.0);

    // ── With value ──
    section_label(tree, root, "With Value", text_secondary);
    datetime_input(tree, root, "Mar 16, 2026  2:30 PM", bg_canvas, border, text_primary, 1.0);

    // ── Split fields ──
    section_label(tree, root, "Split Fields (date + time)", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        segment_field(tree, row, "Mar 16, 2026", 140.0, bg_canvas, border, text_primary);
        segment_field(tree, row, "2:30 PM", 100.0, bg_canvas, border, text_primary);
    }

    // ── With label ──
    section_label(tree, root, "With Label", text_secondary);
    {
        let field = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, field);

        let lbl = tree.create(Widget::Label { text: "Event Start".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(field, lbl);

        datetime_input(tree, field, "Mar 16, 2026  2:30 PM", bg_canvas, border, text_primary, 1.0);
    }

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    datetime_input(tree, root, "Mar 16, 2026  2:30 PM", bg_canvas, border, text_primary, 0.5);

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn datetime_input(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, opacity: f32) {
    let input = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Fixed(260.0),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 6.0,
        align: Align::Center,
        justify: Justify::SpaceBetween,
        opacity,
        ..UiStyle::default()
    });
    tree.add_child(parent, input);

    let val = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(input, val);

    let icon = tree.create(Widget::Label { text: "📅".to_string() }, UiStyle {
        text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(input, icon);
}

fn segment_field(tree: &mut UiTree, parent: UiNodeId, text: &str, width: f32, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4) {
    let input = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(width),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radius: 6.0, align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, input);

    let val = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(input, val);
}
