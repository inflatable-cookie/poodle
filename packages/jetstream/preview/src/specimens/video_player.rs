//! VideoPlayer specimen — video playback with overlay controls.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    section_label(tree, root, "Standard Player", text_secondary);
    {
        let player = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(400.0),
            background: Some(glam::Vec4::new(0.05, 0.05, 0.05, 1.0)),
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, player);

        // Video area
        let video = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(225.0),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(player, video);

        let play = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(48.0), height: Sizing::Fixed(48.0),
            corner_radius: 24.0,
            background: Some(glam::Vec4::new(0.0, 0.0, 0.0, 0.5)),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(video, play);

        let icon = tree.create(Widget::Label { text: "▶".to_string() }, UiStyle {
            text_color: Some(glam::Vec4::ONE), text_size: Some(20.0), ..UiStyle::default()
        });
        tree.add_child(play, icon);

        // Controls bar
        let controls = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            gap: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(player, controls);

        let play_btn = tree.create(Widget::Label { text: "▶".to_string() }, UiStyle {
            text_color: Some(glam::Vec4::ONE), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(controls, play_btn);

        let time = tree.create(Widget::Label { text: "0:00 / 2:34".to_string() }, UiStyle {
            text_color: Some(glam::Vec4::new(0.7, 0.7, 0.7, 1.0)), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(controls, time);

        // Progress
        let track = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(3.0),
            background: Some(glam::Vec4::new(0.3, 0.3, 0.3, 1.0)),
            corner_radius: 1.5, ..UiStyle::default()
        });
        tree.add_child(controls, track);

        let vol = tree.create(Widget::Label { text: "🔊".to_string() }, UiStyle {
            text_color: Some(glam::Vec4::new(0.7, 0.7, 0.7, 1.0)), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(controls, vol);

        let fs = tree.create(Widget::Label { text: "⛶".to_string() }, UiStyle {
            text_color: Some(glam::Vec4::new(0.7, 0.7, 0.7, 1.0)), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(controls, fs);
    }

    // Note
    let note = tree.create(Widget::Panel, UiStyle {
        padding: Edges::all(10.0),
        background: Some(theme_bridge::tint(accent, 0.08)),
        corner_radius: 6.0,
        ..UiStyle::default()
    });
    tree.add_child(root, note);

    let note_text = tree.create(Widget::Label {
        text: "ℹ Jetstream video playback requires platform-specific media integration.".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(note, note_text);

    let _ = (text_primary, bg_elevated, border);
    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
