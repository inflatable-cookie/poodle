//! Progress specimen — progress bar with determinate and indeterminate states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── 0% ──
    label(tree, root, "Empty (0%)", text_secondary);
    progress_bar(tree, root, 0.0, 240.0, accent, border);

    // ── 45% ──
    label(tree, root, "In progress (45%)", text_secondary);
    progress_bar(tree, root, 0.45, 240.0, accent, border);

    // ── 100% ──
    label(tree, root, "Complete (100%)", text_secondary);
    progress_bar(tree, root, 1.0, 240.0, success, border);

    // ── Warning ──
    label(tree, root, "Warning tone (75%)", text_secondary);
    progress_bar(tree, root, 0.75, 240.0, warning, border);

    // ── Danger ──
    label(tree, root, "Danger tone (90%)", text_secondary);
    progress_bar(tree, root, 0.9, 240.0, danger, border);

    // ── Sizes ──
    label(tree, root, "Size variants", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        for &(lbl, h) in &[("Thin (2px)", 2.0), ("Default (6px)", 6.0), ("Thick (10px)", 10.0)] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(col, row);

            let l = tree.create(Widget::Label { text: lbl.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(80.0), ..UiStyle::default()
            });
            tree.add_child(row, l);

            progress_bar_sized(tree, row, 0.6, 160.0, h, accent, border);
        }
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn progress_bar(tree: &mut UiTree, parent: UiNodeId, fraction: f32, width: f32, fill_color: glam::Vec4, bg_color: glam::Vec4) {
    progress_bar_sized(tree, parent, fraction, width, 6.0, fill_color, bg_color);
}

fn progress_bar_sized(tree: &mut UiTree, parent: UiNodeId, fraction: f32, width: f32, height: f32, fill_color: glam::Vec4, bg_color: glam::Vec4) {
    let track = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(width),
        height: Sizing::Fixed(height),
        corner_radii: [height / 2.0; 4],
        background: Some(bg_color),
        direction: Direction::Row,
        ..UiStyle::default()
    });
    tree.add_child(parent, track);

    let fill_w = width * fraction.clamp(0.0, 1.0);
    if fill_w > 0.0 {
        let fill = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(fill_w),
            height: Sizing::Fixed(height),
            corner_radii: [height / 2.0; 4],
            background: Some(fill_color),
            ..UiStyle::default()
        });
        tree.add_child(track, fill);
    }
}
