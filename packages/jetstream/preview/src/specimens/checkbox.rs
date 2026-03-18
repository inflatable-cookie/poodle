//! Checkbox specimen — checked, unchecked, indeterminate, and disabled states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let _bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_default(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(12.0)), NodeStyle::default());

    label(tree, root, "States", text_secondary);

    let states: &[(&str, &str, bool, f32)] = &[
        ("Unchecked", "☐", false, 1.0),
        ("Checked", "☑", true, 1.0),
        ("Indeterminate", "▣", true, 1.0),
        ("Disabled unchecked", "☐", false, 0.5),
        ("Disabled checked", "☑", true, 0.5),
    ];

    for &(name, icon, checked, opacity) in states {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(8.0).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            opacity,
            ..NodeStyle::default()
        });
        tree.add_child(root, row);

        let box_bg = if checked { Some(accent) } else { None };
        let box_border = if checked { accent } else { border };
        let box_text = if checked { glam::Vec4::ONE } else { glam::Vec4::ZERO };

        let checkbox = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(18.0)).with_height(LayoutSizing::Fixed(18.0)).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [3.0; 4],
            background: box_bg,
            border_color: Some(box_border),
            border_width: if checked { 0.0 } else { 1.5 },
            ..NodeStyle::default()
        });
        tree.add_child(row, checkbox);

        let check_icon = tree.create_node(Widget::Label { text: icon.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(if checked { glam::Vec4::ONE } else { box_text }),
            text_size: Some(12.0),
            ..NodeStyle::default()
        });
        tree.add_child(checkbox, check_icon);

        let lbl = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..NodeStyle::default()
        });
        tree.add_child(row, lbl);
    }

    // ── Group ──
    label(tree, root, "Checkbox Group", text_secondary);

    let options = ["Email notifications", "Push notifications", "SMS alerts"];
    let checked_items = [true, false, true];
    for (i, &opt) in options.iter().enumerate() {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(8.0).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(root, row);

        let is_checked = checked_items[i];
        let cb = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(18.0)).with_height(LayoutSizing::Fixed(18.0)).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [3.0; 4],
            background: if is_checked { Some(accent) } else { None },
            border_color: Some(if is_checked { accent } else { border }),
            border_width: if is_checked { 0.0 } else { 1.5 },
            ..NodeStyle::default()
        });
        tree.add_child(row, cb);

        if is_checked {
            let mark = tree.create_node(Widget::Label { text: "✓".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(glam::Vec4::ONE),
                text_size: Some(11.0),
                ..NodeStyle::default()
            });
            tree.add_child(cb, mark);
        }

        let lbl = tree.create_node(Widget::Label { text: opt.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..NodeStyle::default()
        });
        tree.add_child(row, lbl);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color),
        text_size: Some(11.0),
        ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
