//! Slider specimen — single-thumb slider with value, min/max, disabled state.

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

    label(tree, root, "Standard (value: 60%)", text_secondary);
    slider_track(tree, root, 0.6, 200.0, accent, border, text_primary, 1.0);

    label(tree, root, "Min value (0%)", text_secondary);
    slider_track(tree, root, 0.0, 200.0, accent, border, text_primary, 1.0);

    label(tree, root, "Max value (100%)", text_secondary);
    slider_track(tree, root, 1.0, 200.0, accent, border, text_primary, 1.0);

    label(tree, root, "Disabled (40%)", text_secondary);
    slider_track(tree, root, 0.4, 200.0, accent, border, text_primary, 0.5);

    label(tree, root, "With Labels", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let min = tree.create(Widget::Label { text: "0".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, min);

        slider_track(tree, row, 0.75, 180.0, accent, border, text_primary, 1.0);

        let max = tree.create(Widget::Label { text: "100".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, max);

        let val = tree.create(Widget::Label { text: "75".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(11.0),
            padding: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 4.0 },
            ..UiStyle::default()
        });
        tree.add_child(row, val);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn slider_track(tree: &mut UiTree, parent: UiNodeId, fraction: f32, width: f32, accent: glam::Vec4, border: glam::Vec4, _text: glam::Vec4, opacity: f32) {
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
        corner_radii: [2.0; 4],
        background: Some(border),
        direction: Direction::Row,
        ..UiStyle::default()
    });
    tree.add_child(container, track);

    // Fill
    let fill_w = width * fraction;
    if fill_w > 0.0 {
        let fill = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(fill_w),
            height: Sizing::Fixed(4.0),
            corner_radii: [2.0; 4],
            background: Some(accent),
            ..UiStyle::default()
        });
        tree.add_child(track, fill);
    }
}
