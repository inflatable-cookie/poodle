//! DurationInput specimen — input for hours/minutes/seconds durations.

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

    // ── Standard (HH:MM) ──
    label(tree, root, "Hours : Minutes", text_secondary);
    duration_row(tree, root, &["02", "30"], &["h", "m"], bg_canvas, border, text_primary, text_secondary, 1.0);

    // ── Full (HH:MM:SS) ──
    label(tree, root, "Hours : Minutes : Seconds", text_secondary);
    duration_row(tree, root, &["01", "45", "30"], &["h", "m", "s"], bg_canvas, border, text_primary, text_secondary, 1.0);

    // ── Minutes only ──
    label(tree, root, "Minutes Only", text_secondary);
    duration_row(tree, root, &["90"], &["min"], bg_canvas, border, text_primary, text_secondary, 1.0);

    // ── Empty / placeholder ──
    label(tree, root, "Empty (placeholder)", text_secondary);
    duration_row(tree, root, &["--", "--"], &["h", "m"], bg_canvas, border, text_secondary, text_secondary, 1.0);

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    duration_row(tree, root, &["01", "00"], &["h", "m"], bg_canvas, border, text_primary, text_secondary, 0.5);

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn duration_row(
    tree: &mut UiTree,
    parent: UiNodeId,
    values: &[&str],
    units: &[&str],
    bg: glam::Vec4,
    border: glam::Vec4,
    text_color: glam::Vec4,
    unit_color: glam::Vec4,
    opacity: f32,
) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 4.0,
        align: Align::Center,
        opacity,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    for (i, (&val, &unit)) in values.iter().zip(units.iter()).enumerate() {
        if i > 0 {
            let sep = tree.create(Widget::Label { text: ":".to_string() }, UiStyle {
                text_color: Some(unit_color), text_size: Some(14.0), ..UiStyle::default()
            });
            tree.add_child(row, sep);
        }

        let input = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(40.0),
            height: Sizing::Fixed(32.0),
            background: Some(bg),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            align: Align::Center,
            justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(row, input);

        let v = tree.create(Widget::Label { text: val.to_string() }, UiStyle {
            text_color: Some(text_color), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(input, v);

        let u = tree.create(Widget::Label { text: unit.to_string() }, UiStyle {
            text_color: Some(unit_color), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, u);
    }
}
