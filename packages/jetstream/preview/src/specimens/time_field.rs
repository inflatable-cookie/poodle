//! TimeField specimen — time-of-day input with value, placeholder, disabled states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    let states: &[(&str, &str, glam::Vec4, f32)] = &[
        ("Placeholder", "HH:MM", text_secondary, 1.0),
        ("Filled", "14:30", text_primary, 1.0),
        ("With seconds", "09:15:42", text_primary, 1.0),
        ("Disabled", "08:00", theme_bridge::tint(text_secondary, 0.5), 0.5),
    ];

    for &(name, value, text_color, opacity) in states {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(12.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        let lbl = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(80.0))),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(row, lbl);

        let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(140.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_gap(4.0)
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_canvas),
                border_color: Some(border_default),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                opacity,
                ..NodeStyle::default()
            });
        tree.add_child(row, input);

        let val = tree.create_node(Widget::Label { text: value.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_color), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(input, val);

        // Clock icon
        let icon = tree.create_node(Widget::Label { text: "\u{23F1}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(input, icon);
    }

    root
}
