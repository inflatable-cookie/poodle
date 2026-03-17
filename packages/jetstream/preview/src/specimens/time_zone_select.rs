//! TimeZoneSelect specimen — timezone dropdown selection.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Selected value ──
    section_label(tree, root, "With Selected Value", text_secondary);
    tz_select(tree, root, "America/New_York (UTC-5)", bg_canvas, border, text_primary, 1.0);

    // ── Placeholder ──
    section_label(tree, root, "Placeholder", text_secondary);
    tz_select(tree, root, "Select timezone...", bg_canvas, border, text_secondary, 1.0);

    // ── With dropdown open ──
    section_label(tree, root, "Dropdown Open", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        tz_select(tree, col, "America/New_York (UTC-5)", bg_canvas, accent, text_primary, 1.0);

        // Dropdown list
        let dropdown = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(260.0),
            padding: Edges::all(4.0),
            gap: 1.0,
            background: Some(bg_elevated),
            border_color: Some(border_subtle),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(col, dropdown);

        for &(tz, offset, is_selected) in &[
            ("America/Los_Angeles", "UTC-8", false),
            ("America/Chicago", "UTC-6", false),
            ("America/New_York", "UTC-5", true),
            ("Europe/London", "UTC+0", false),
            ("Europe/Berlin", "UTC+1", false),
            ("Asia/Tokyo", "UTC+9", false),
        ] {
            let bg = if is_selected { Some(theme_bridge::tint(accent, 0.12)) } else { None };
            let color = if is_selected { accent } else { text_primary };

            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                width: Sizing::Grow(1.0), height: Sizing::Fixed(28.0),
                padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
                corner_radii: [4.0; 4], background: bg,
                align: Align::Center, justify: Justify::SpaceBetween,
                ..UiStyle::default()
            });
            tree.add_child(dropdown, row);

            let name = tree.create(Widget::Label { text: tz.to_string() }, UiStyle {
                text_color: Some(color), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(row, name);

            let off = tree.create(Widget::Label { text: offset.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(row, off);
        }
    }

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    tz_select(tree, root, "America/New_York (UTC-5)", bg_canvas, border, text_primary, 0.5);

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn tz_select(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, opacity: f32) {
    let input = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Fixed(260.0),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radii: [6.0; 4],
        align: Align::Center, justify: Justify::SpaceBetween,
        opacity, ..UiStyle::default()
    });
    tree.add_child(parent, input);

    let val = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(input, val);

    let chevron = tree.create(Widget::Label { text: "▾".to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(input, chevron);
}
