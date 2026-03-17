//! FilterToolbar specimen — search and filter chip controls.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Standard filter toolbar ──
    section_label(tree, root, "Standard Toolbar", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(480.0),
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            gap: 8.0, align: Align::Center,
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        // Search
        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4], align: Align::Center,
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(bar, search);

        let search_lbl = tree.create(Widget::Label { text: "🔍 Filter…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(search, search_lbl);

        // Filter chips
        filter_chip(tree, bar, "Type ▾", false, accent, text_inverse, text_primary, bg_surface, border);
        filter_chip(tree, bar, "Status ▾", false, accent, text_inverse, text_primary, bg_surface, border);
    }

    // ── With active filters ──
    section_label(tree, root, "With Active Filters", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0,
            width: Sizing::Fixed(480.0),
            padding: Edges::all(12.0),
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        // Search row
        let search_row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(bar, search_row);

        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4], align: Align::Center,
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(search_row, search);

        let search_lbl = tree.create(Widget::Label { text: "🔍 project brief".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(search, search_lbl);

        filter_chip(tree, search_row, "Type ▾", false, accent, text_inverse, text_primary, bg_surface, border);
        filter_chip(tree, search_row, "Active ✕", true, accent, text_inverse, text_primary, bg_surface, border);

        // Active filter tags
        let tags = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(bar, tags);

        let active_label = tree.create(Widget::Label { text: "Active filters:".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(tags, active_label);

        for &tag in &["Status: Active ✕", "Type: Document ✕"] {
            let chip = tree.create(Widget::Panel, UiStyle {
                height: Sizing::Fixed(22.0),
                padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
                background: Some(theme_bridge::tint(accent, 0.12)),
                corner_radii: [4.0; 4], align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(tags, chip);

            let t = tree.create(Widget::Label { text: tag.to_string() }, UiStyle {
                text_color: Some(accent), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(chip, t);
        }

        let clear = tree.create(Widget::Label { text: "Clear all".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(tags, clear);
    }

    root
}

fn filter_chip(tree: &mut UiTree, parent: UiNodeId, label: &str, active: bool, accent: glam::Vec4, active_fg: glam::Vec4, fg: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let (chip_bg, chip_border, color) = if active {
        (Some(accent), accent, active_fg)
    } else {
        (Some(bg), border, fg)
    };

    let btn = tree.create(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radii: [6.0; 4], background: chip_bg,
        border_color: Some(chip_border), border_width: 1.0,
        text_color: Some(color), text_size: Some(11.0),
        align: Align::Center, justify: Justify::Center,
        focusable: true, ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
