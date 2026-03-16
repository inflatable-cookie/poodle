//! SegmentedControl specimen — inline toggle options with selected highlight.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── 3 segments ──
    label(tree, root, "3 Segments (second selected)", text_secondary);
    segmented(tree, root, &["Day", "Week", "Month"], 1, accent, text_primary, text_secondary, bg_surface, border);

    // ── 4 segments ──
    label(tree, root, "4 Segments (first selected)", text_secondary);
    segmented(tree, root, &["All", "Active", "Draft", "Archived"], 0, accent, text_primary, text_secondary, bg_surface, border);

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let track = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges::all(2.0),
            gap: 2.0,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            opacity: 0.5,
            ..UiStyle::default()
        });
        tree.add_child(root, track);

        for (i, &seg) in ["On", "Off"].iter().enumerate() {
            let selected = i == 0;
            let btn = tree.create(Widget::Panel, UiStyle {
                height: Sizing::Fixed(28.0),
                padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
                corner_radius: 4.0,
                background: if selected { Some(theme_bridge::tint(accent, 0.20)) } else { None },
                align: Align::Center,
                justify: Justify::Center,
                ..UiStyle::default()
            });
            tree.add_child(track, btn);

            let lbl = tree.create(Widget::Label { text: seg.to_string() }, UiStyle {
                text_color: Some(if selected { text_primary } else { text_secondary }),
                text_size: Some(12.0),
                ..UiStyle::default()
            });
            tree.add_child(btn, lbl);
        }
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn segmented(
    tree: &mut UiTree,
    parent: UiNodeId,
    segments: &[&str],
    selected_idx: usize,
    accent: glam::Vec4,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    bg_surface: glam::Vec4,
    border: glam::Vec4,
) {
    let track = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        padding: Edges::all(2.0),
        gap: 2.0,
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 6.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, track);

    for (i, &seg) in segments.iter().enumerate() {
        let selected = i == selected_idx;
        let btn = tree.create(Widget::Button {
            label: seg.to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
            corner_radius: 4.0,
            background: if selected { Some(theme_bridge::tint(accent, 0.20)) } else { None },
            text_color: Some(if selected { text_primary } else { text_secondary }),
            text_size: Some(12.0),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(track, btn);
    }
}
