//! TriStateSwitch specimen — on/off/indeterminate three-position switch.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_default(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(12.0)),
        NodeStyle::default());

    label(tree, root, "Three Positions", text_secondary);

    // Off / Indeterminate / On
    let positions: &[(&str, f32, glam::Vec4)] = &[
        ("Off", 0.0, bg_surface),
        ("Indeterminate", 0.5, warning),
        ("On", 1.0, accent),
    ];

    for &(name, position, track_color) in positions {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(10.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        let track = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(44.0))
            .with_height(LayoutSizing::Fixed(20.0))
            .with_direction(LayoutDirection::Row)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)
            .with_padding(LayoutEdges::uniform(2.0))),
            NodeStyle {
                corner_radii: [10.0; 4],
                background: Some(track_color),
                border_color: Some(if position == 0.0 { border } else { track_color }),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(row, track);

        // Position the thumb: left=off, center=indeterminate, right=on
        if position > 0.0 {
            let spacer_w = if position >= 1.0 { LayoutSizing::Grow } else { LayoutSizing::Fixed(11.0) };
            let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(spacer_w)),
                NodeStyle::default());
            tree.add_child(track, spacer);
        }

        let thumb = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(16.0))
            .with_height(LayoutSizing::Fixed(16.0))),
            NodeStyle {
                corner_radii: [8.0; 4],
                background: Some(glam::Vec4::ONE),
                ..NodeStyle::default()
            });
        tree.add_child(track, thumb);

        let lbl = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(row, lbl);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
