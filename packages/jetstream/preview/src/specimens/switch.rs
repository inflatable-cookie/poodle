//! Switch specimen — on/off and disabled states.

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

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(12.0)),
        NodeStyle::default());

    label(tree, root, "Switch States", text_secondary);

    let states: &[(&str, bool, f32)] = &[
        ("Off", false, 1.0),
        ("On", true, 1.0),
        ("Disabled Off", false, 0.5),
        ("Disabled On", true, 0.5),
    ];

    for &(name, on, opacity) in states {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(10.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { opacity, ..NodeStyle::default() });
        tree.add_child(root, row);

        // Switch track
        let track_bg = if on { accent } else { bg_surface };
        let track = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(36.0))
            .with_height(LayoutSizing::Fixed(20.0))
            .with_direction(LayoutDirection::Row)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)
            .with_padding(LayoutEdges { top: 2.0, right: 2.0, bottom: 2.0, left: 2.0 })),
            NodeStyle {
                corner_radii: [10.0; 4],
                background: Some(track_bg),
                border_color: Some(if on { accent } else { border }),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(row, track);

        if on {
            // Spacer to push thumb right
            let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)),
                NodeStyle::default());
            tree.add_child(track, spacer);
        }

        // Thumb
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

    // ── With labels ──
    label(tree, root, "With Labels", text_secondary);

    for &(setting, on) in &[("Dark mode", true), ("Notifications", false), ("Auto-save", true)] {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(240.0))
            .with_gap(10.0)
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        let lbl = tree.create_node(Widget::Label { text: setting.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(row, lbl);

        let track_bg = if on { accent } else { bg_surface };
        let track = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(36.0))
            .with_height(LayoutSizing::Fixed(20.0))
            .with_direction(LayoutDirection::Row)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)
            .with_padding(LayoutEdges::uniform(2.0))),
            NodeStyle {
                corner_radii: [10.0; 4],
                background: Some(track_bg),
                border_color: Some(if on { accent } else { border }),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(row, track);

        if on {
            let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)),
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
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
