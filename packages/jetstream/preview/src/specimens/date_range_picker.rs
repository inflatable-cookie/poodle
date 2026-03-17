//! DateRangePicker specimen — dual-input date range selection.

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
    range_input(tree, root, "Start date", "End date", bg_canvas, border, text_secondary, 1.0);

    // ── With values ──
    section_label(tree, root, "With Values", text_secondary);
    range_input(tree, root, "Mar 10, 2026", "Mar 20, 2026", bg_canvas, border, text_primary, 1.0);

    // ── With label ──
    section_label(tree, root, "With Label", text_secondary);
    {
        let field = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, field);

        let lbl = tree.create(Widget::Label { text: "Date Range".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(field, lbl);

        range_input(tree, field, "Mar 1, 2026", "Mar 31, 2026", bg_canvas, border, text_primary, 1.0);
    }

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    range_input(tree, root, "Mar 10, 2026", "Mar 20, 2026", bg_canvas, border, text_primary, 0.5);

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn range_input(tree: &mut UiTree, parent: UiNodeId, start: &str, end: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, opacity: f32) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::Center,
        opacity,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    date_field(tree, row, start, bg, border, fg);

    let arrow = tree.create(Widget::Label { text: "→".to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, arrow);

    date_field(tree, row, end, bg, border, fg);
}

fn date_field(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4) {
    let input = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(140.0),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [6.0; 4],
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, input);

    let val = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(input, val);
}
