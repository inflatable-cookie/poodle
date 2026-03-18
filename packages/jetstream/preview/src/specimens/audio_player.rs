//! AudioPlayer specimen — playback controls with progress bar.

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

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(20.0),
        ),
        NodeStyle::default(),
    );

    section_label(tree, root, "Playing", text_secondary);
    audio_card(tree, root, "▐▐", "Forest Ambience", "Nature Sounds Vol. 2", "1:23", "3:45", 0.38, bg_elevated, border, accent, text_primary, text_secondary);

    section_label(tree, root, "Paused", text_secondary);
    audio_card(tree, root, "▶", "Battle Theme", "Game OST", "0:00", "2:12", 0.0, bg_elevated, border, accent, text_primary, text_secondary);

    root
}

fn audio_card(tree: &mut UiTree, parent: UiNodeId, state_icon: &str, title: &str, artist: &str, current: &str, total: &str, progress: f32, bg: glam::Vec4, border: glam::Vec4, accent: glam::Vec4, text_primary: glam::Vec4, text_secondary: glam::Vec4) {
    let card = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Fixed(360.0))
                .with_padding(LayoutEdges::uniform(12.0))
                .with_gap(8.0),
        ),
        NodeStyle {
            background: Some(bg), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        },
    );
    tree.add_child(parent, card);

    // Info row
    let info = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(10.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
        ),
        NodeStyle::default(),
    );
    tree.add_child(card, info);

    let play = tree.create_node(
        Widget::Button {
            label: state_icon.to_string(), pressed: false, hovered: false,
        },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(36.0))
                .with_height(LayoutSizing::Fixed(36.0))
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
        ),
        NodeStyle {
            corner_radii: [18.0; 4], background: Some(accent),
            text_color: Some(glam::Vec4::ONE), text_size: Some(14.0),
            focusable: true, ..NodeStyle::default()
        },
    );
    tree.add_child(info, play);

    let details = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(2.0),
        ),
        NodeStyle::default(),
    );
    tree.add_child(info, details);

    let t = tree.create_node(
        Widget::Label { text: title.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() },
    );
    tree.add_child(details, t);

    let a = tree.create_node(
        Widget::Label { text: artist.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() },
    );
    tree.add_child(details, a);

    // Progress bar
    let track = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(4.0)),
        ),
        NodeStyle {
            background: Some(theme_bridge::tint(border, 0.5)),
            corner_radii: [2.0; 4], ..NodeStyle::default()
        },
    );
    tree.add_child(card, track);

    if progress > 0.0 {
        let fill_width = 336.0 * progress; // approximate
        let fill = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_width(LayoutSizing::Fixed(fill_width))
                    .with_height(LayoutSizing::Fixed(4.0)),
            ),
            NodeStyle {
                background: Some(accent), corner_radii: [2.0; 4], ..NodeStyle::default()
            },
        );
        tree.add_child(track, fill);
    }

    // Time
    let times = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Stretch),
        ),
        NodeStyle::default(),
    );
    tree.add_child(card, times);

    let c = tree.create_node(
        Widget::Label { text: current.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() },
    );
    tree.add_child(times, c);

    let tot = tree.create_node(
        Widget::Label { text: total.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() },
    );
    tree.add_child(times, tot);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() },
    );
    tree.add_child(parent, lbl);
}
