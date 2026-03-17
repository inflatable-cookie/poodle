//! OrderBy specimen — sort-control toolbar.

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

    // ── Standard ──
    section_label(tree, root, "Standard Sort Control", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        let lbl = tree.create(Widget::Label { text: "Sort by:".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(row, lbl);

        sort_chip(tree, row, "Name ↑", true, accent, text_primary, bg_surface, border);
        sort_chip(tree, row, "Date", false, accent, text_primary, bg_surface, border);
        sort_chip(tree, row, "Size", false, accent, text_primary, bg_surface, border);
    }

    // ── Descending ──
    section_label(tree, root, "Descending Sort", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(root, row);

        let lbl = tree.create(Widget::Label { text: "Sort by:".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(row, lbl);

        sort_chip(tree, row, "Name", false, accent, text_primary, bg_surface, border);
        sort_chip(tree, row, "Date ↓", true, accent, text_primary, bg_surface, border);
        sort_chip(tree, row, "Size", false, accent, text_primary, bg_surface, border);
    }

    // ── Dropdown style ──
    section_label(tree, root, "Dropdown Style", text_secondary);
    {
        let btn = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_surface), border_color: Some(border),
            border_width: 1.0, corner_radii: [6.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, btn);

        let lbl = tree.create(Widget::Label { text: "Sort: Name (A→Z)".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(btn, lbl);

        let chevron = tree.create(Widget::Label { text: "▾".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(btn, chevron);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn sort_chip(tree: &mut UiTree, parent: UiNodeId, label: &str, active: bool, accent: glam::Vec4, fg: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let (chip_bg, chip_border, color) = if active {
        (Some(theme_bridge::tint(accent, 0.12)), accent, accent)
    } else {
        (Some(bg), border, fg)
    };

    let chip = tree.create(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(26.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radii: [13.0; 4], background: chip_bg,
        border_color: Some(chip_border), border_width: 1.0,
        text_color: Some(color), text_size: Some(11.0),
        align: Align::Center, justify: Justify::Center,
        focusable: true, ..UiStyle::default()
    });
    tree.add_child(parent, chip);
}
