//! VideoPlayer specimen — video playback with overlay controls.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    section_label(tree, root, "Standard Player", text_secondary);
    {
        let player = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(400.0))),
            NodeStyle {
                background: Some(glam::Vec4::new(0.05, 0.05, 0.05, 1.0)),
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, player);

        // Video area
        let video = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(225.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(player, video);

        let play = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(48.0))
            .with_height(LayoutSizing::Fixed(48.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [24.0; 4],
                background: Some(glam::Vec4::new(0.0, 0.0, 0.0, 0.5)),
                ..NodeStyle::default()
            });
        tree.add_child(video, play);

        let icon = tree.create_node(Widget::Label { text: "\u{25B6}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(glam::Vec4::ONE), text_size: Some(20.0), ..NodeStyle::default() });
        tree.add_child(play, icon);

        // Controls bar
        let controls = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(player, controls);

        let play_btn = tree.create_node(Widget::Label { text: "\u{25B6}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(glam::Vec4::ONE), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(controls, play_btn);

        let time = tree.create_node(Widget::Label { text: "0:00 / 2:34".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(glam::Vec4::new(0.7, 0.7, 0.7, 1.0)), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(controls, time);

        // Progress
        let track = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(3.0))),
            NodeStyle {
                background: Some(glam::Vec4::new(0.3, 0.3, 0.3, 1.0)),
                corner_radii: [1.5; 4], ..NodeStyle::default()
            });
        tree.add_child(controls, track);

        let vol = tree.create_node(Widget::Label { text: "\u{1F50A}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(glam::Vec4::new(0.7, 0.7, 0.7, 1.0)), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(controls, vol);

        let fs = tree.create_node(Widget::Label { text: "\u{26F6}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(glam::Vec4::new(0.7, 0.7, 0.7, 1.0)), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(controls, fs);
    }

    // Note
    let note = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_padding(LayoutEdges::uniform(10.0))),
        NodeStyle {
            background: Some(theme_bridge::tint(accent, 0.08)),
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
    tree.add_child(root, note);

    let note_text = tree.create_node(Widget::Label {
        text: "\u{2139} Jetstream video playback requires platform-specific media integration.".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(note, note_text);

    let _ = (text_primary, bg_elevated, border);
    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
