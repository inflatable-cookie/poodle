//! AudioPlayer specimen — playback controls with progress bar.

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

    section_label(tree, root, "Playing", text_secondary);
    audio_card(tree, root, "▐▐", "Forest Ambience", "Nature Sounds Vol. 2", "1:23", "3:45", 0.38, bg_elevated, border, accent, text_primary, text_secondary);

    section_label(tree, root, "Paused", text_secondary);
    audio_card(tree, root, "▶", "Battle Theme", "Game OST", "0:00", "2:12", 0.0, bg_elevated, border, accent, text_primary, text_secondary);

    root
}

fn audio_card(tree: &mut UiTree, parent: UiNodeId, state_icon: &str, title: &str, artist: &str, current: &str, total: &str, progress: f32, bg: glam::Vec4, border: glam::Vec4, accent: glam::Vec4, text_primary: glam::Vec4, text_secondary: glam::Vec4) {
    let card = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Fixed(360.0),
        padding: Edges::all(12.0), gap: 8.0,
        background: Some(bg), border_color: Some(border), border_width: 1.0,
        corner_radius: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, card);

    // Info row
    let info = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 10.0, align: Align::Center, ..UiStyle::default()
    });
    tree.add_child(card, info);

    let play = tree.create(Widget::Button {
        label: state_icon.to_string(), pressed: false, hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(36.0), height: Sizing::Fixed(36.0),
        corner_radius: 18.0, background: Some(accent),
        text_color: Some(glam::Vec4::ONE), text_size: Some(14.0),
        align: Align::Center, justify: Justify::Center,
        focusable: true, ..UiStyle::default()
    });
    tree.add_child(info, play);

    let details = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 2.0, ..UiStyle::default()
    });
    tree.add_child(info, details);

    let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
    });
    tree.add_child(details, t);

    let a = tree.create(Widget::Label { text: artist.to_string() }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(details, a);

    // Progress bar
    let track = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Grow(1.0), height: Sizing::Fixed(4.0),
        background: Some(theme_bridge::tint(border, 0.5)),
        corner_radius: 2.0, ..UiStyle::default()
    });
    tree.add_child(card, track);

    if progress > 0.0 {
        let fill_width = 336.0 * progress; // approximate
        let fill = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(fill_width), height: Sizing::Fixed(4.0),
            background: Some(accent), corner_radius: 2.0, ..UiStyle::default()
        });
        tree.add_child(track, fill);
    }

    // Time
    let times = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, justify: Justify::SpaceBetween, ..UiStyle::default()
    });
    tree.add_child(card, times);

    let c = tree.create(Widget::Label { text: current.to_string() }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(times, c);

    let tot = tree.create(Widget::Label { text: total.to_string() }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(times, tot);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
