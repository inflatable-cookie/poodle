//! RangeSlider specimen — dual-thumb slider for selecting a value range.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Standard range ──
    label(tree, root, "Standard (20% – 80%)", text_secondary);
    range_track(tree, root, 0.2, 0.8, 200.0, accent, border, 1.0);

    // ── Narrow range ──
    label(tree, root, "Narrow range (40% – 60%)", text_secondary);
    range_track(tree, root, 0.4, 0.6, 200.0, accent, border, 1.0);

    // ── Full range ──
    label(tree, root, "Full range (0% – 100%)", text_secondary);
    range_track(tree, root, 0.0, 1.0, 200.0, accent, border, 1.0);

    // ── Disabled ──
    label(tree, root, "Disabled (30% – 70%)", text_secondary);
    range_track(tree, root, 0.3, 0.7, 200.0, accent, border, 0.5);

    // ── With labels ──
    label(tree, root, "With Labels", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let min = tree.create(Widget::Label { text: "$200".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, min);

        range_track(tree, row, 0.25, 0.75, 160.0, accent, border, 1.0);

        let max = tree.create(Widget::Label { text: "$800".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, max);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn range_track(
    tree: &mut UiTree,
    parent: UiNodeId,
    low: f32,
    high: f32,
    width: f32,
    accent: glam::Vec4,
    border: glam::Vec4,
    opacity: f32,
) {
    let container = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(width),
        height: Sizing::Fixed(20.0),
        direction: Direction::Row,
        align: Align::Center,
        opacity,
        ..UiStyle::default()
    });
    tree.add_child(parent, container);

    // Track background
    let track = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(width),
        height: Sizing::Fixed(4.0),
        corner_radius: 2.0,
        background: Some(border),
        direction: Direction::Row,
        ..UiStyle::default()
    });
    tree.add_child(container, track);

    // Active range fill
    let fill_start = width * low;
    let fill_w = width * (high - low);
    if fill_w > 0.0 {
        // Leading spacer
        if fill_start > 0.0 {
            let spacer = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Fixed(fill_start),
                height: Sizing::Fixed(4.0),
                ..UiStyle::default()
            });
            tree.add_child(track, spacer);
        }

        let fill = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(fill_w),
            height: Sizing::Fixed(4.0),
            corner_radius: 2.0,
            background: Some(accent),
            ..UiStyle::default()
        });
        tree.add_child(track, fill);
    }
}
