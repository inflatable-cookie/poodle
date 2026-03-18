//! Menubar specimen — horizontal menu bar with dropdown triggers.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    // ── Standard menubar ──
    section_label(tree, root, "Standard Menubar", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(400.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 4.0, bottom: 0.0, left: 4.0 })
            .with_gap(2.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, bar);

        for item in &["File", "Edit", "View", "Window", "Help"] {
            menubar_item(tree, bar, item, text_primary, false, accent, text_inverse);
        }
    }

    // ── With active item + dropdown ──
    section_label(tree, root, "With Active Dropdown", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(2.0)), NodeStyle::default());
        tree.add_child(root, col);

        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(400.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 4.0, bottom: 0.0, left: 4.0 })
            .with_gap(2.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(col, bar);

        menubar_item(tree, bar, "File", text_primary, false, accent, text_inverse);
        menubar_item(tree, bar, "Edit", text_primary, true, accent, text_inverse);
        menubar_item(tree, bar, "View", text_primary, false, accent, text_inverse);

        // Dropdown beneath "Edit"
        let dropdown = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(180.0))
            .with_padding(LayoutEdges { top: 0.0, right: 0.0, bottom: 0.0, left: 44.0 })), NodeStyle::default());
        tree.add_child(col, dropdown);

        let menu = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(180.0))
            .with_padding(LayoutEdges::uniform(4.0))
            .with_gap(1.0)), NodeStyle {
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(dropdown, menu);

        for &(label, shortcut) in &[("Undo", "⌘Z"), ("Redo", "⇧⌘Z")] {
            dropdown_item(tree, menu, label, shortcut, text_primary, text_secondary);
        }

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))), NodeStyle {
            background: Some(border), ..NodeStyle::default()
        });
        tree.add_child(menu, sep);

        for &(label, shortcut) in &[("Cut", "⌘X"), ("Copy", "⌘C"), ("Paste", "⌘V")] {
            dropdown_item(tree, menu, label, shortcut, text_primary, text_secondary);
        }
    }

    // ── Compact menubar ──
    section_label(tree, root, "Compact Menubar", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 2.0, bottom: 0.0, left: 2.0 })
            .with_gap(1.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [4.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, bar);

        for item in &["File", "Edit", "View"] {
            let btn = tree.create_node(Widget::Button {
                label: item.to_string(), pressed: false, hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(24.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
                corner_radii: [3.0; 4],
                text_color: Some(text_primary), text_size: Some(10.0),
                focusable: true, ..NodeStyle::default()
            });
            tree.add_child(bar, btn);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn menubar_item(tree: &mut UiTree, parent: UiNodeId, label: &str, fg: glam::Vec4, active: bool, accent: glam::Vec4, active_fg: glam::Vec4) {
    let (bg, color) = if active {
        (Some(accent), active_fg)
    } else {
        (None, fg)
    };

    let btn = tree.create_node(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(26.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
        corner_radii: [4.0; 4], background: bg,
        text_color: Some(color), text_size: Some(12.0),
        focusable: true, ..NodeStyle::default()
    });
    tree.add_child(parent, btn);
}

fn dropdown_item(tree: &mut UiTree, parent: UiNodeId, label: &str, shortcut: &str, fg: glam::Vec4, muted: glam::Vec4) {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(28.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)), NodeStyle {
        corner_radii: [4.0; 4], ..NodeStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(fg), text_size: Some(12.0), ..NodeStyle::default()
    });
    tree.add_child(row, lbl);

    let sc = tree.create_node(Widget::Label { text: shortcut.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(muted), text_size: Some(10.0), ..NodeStyle::default()
    });
    tree.add_child(row, sc);
}
