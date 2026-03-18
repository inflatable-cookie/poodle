//! Menu specimen — dropdown menu with items, separators, and keyboard hints.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    // ── Simple menu ──
    section_label(tree, root, "Simple Menu", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(4.0)), NodeStyle::default());
        tree.add_child(root, col);

        let trigger = tree.create_node(Widget::Button {
            label: "Options ▾".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(col, trigger);

        let menu = menu_card(tree, 160.0, bg_elevated, border);
        tree.add_child(col, menu);

        let list = menu_list(tree, menu);
        menu_item(tree, list, "New File", None, text_primary, text_secondary, false);
        menu_item(tree, list, "Open", None, text_primary, text_secondary, false);
        menu_item(tree, list, "Save", None, text_primary, text_secondary, false);
        menu_sep(tree, list, border);
        menu_item(tree, list, "Close", None, text_primary, text_secondary, false);
    }

    // ── With shortcuts ──
    section_label(tree, root, "With Keyboard Shortcuts", text_secondary);
    {
        let menu = menu_card(tree, 200.0, bg_elevated, border);
        tree.add_child(root, menu);

        let list = menu_list(tree, menu);
        menu_item(tree, list, "Undo", Some("⌘Z"), text_primary, text_secondary, false);
        menu_item(tree, list, "Redo", Some("⇧⌘Z"), text_primary, text_secondary, false);
        menu_sep(tree, list, border);
        menu_item(tree, list, "Cut", Some("⌘X"), text_primary, text_secondary, false);
        menu_item(tree, list, "Copy", Some("⌘C"), text_primary, text_secondary, false);
        menu_item(tree, list, "Paste", Some("⌘V"), text_primary, text_secondary, false);
    }

    // ── With active item ──
    section_label(tree, root, "With Active Item", text_secondary);
    {
        let menu = menu_card(tree, 180.0, bg_elevated, border);
        tree.add_child(root, menu);

        let list = menu_list(tree, menu);
        menu_item(tree, list, "List View", None, text_primary, text_secondary, false);
        menu_item_active(tree, list, "Grid View", accent, theme_bridge::text_inverse(theme));
        menu_item(tree, list, "Table View", None, text_primary, text_secondary, false);
    }

    // ── With checkmarks ──
    section_label(tree, root, "Checkable Items", text_secondary);
    {
        let menu = menu_card(tree, 180.0, bg_elevated, border);
        tree.add_child(root, menu);

        let list = menu_list(tree, menu);
        menu_item(tree, list, "✓  Show Grid", None, text_primary, text_secondary, false);
        menu_item(tree, list, "✓  Show Rulers", None, text_primary, text_secondary, false);
        menu_item(tree, list, "    Snap to Grid", None, text_primary, text_secondary, false);
        menu_sep(tree, list, border);
        menu_item(tree, list, "    Dark Mode", None, text_primary, text_secondary, false);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn menu_card(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(width))), NodeStyle {
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radii: [8.0; 4],
        ..NodeStyle::default()
    })
}

fn menu_list(tree: &mut UiTree, parent: UiNodeId) -> UiNodeId {
    let list = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_padding(LayoutEdges::uniform(4.0))
        .with_gap(1.0)), NodeStyle::default());
    tree.add_child(parent, list);
    list
}

fn menu_item(tree: &mut UiTree, parent: UiNodeId, label: &str, shortcut: Option<&str>, fg: glam::Vec4, muted: glam::Vec4, _disabled: bool) {
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

    if let Some(sc) = shortcut {
        let s = tree.create_node(Widget::Label { text: sc.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(muted), text_size: Some(10.0), ..NodeStyle::default()
        });
        tree.add_child(row, s);
    }
}

fn menu_item_active(tree: &mut UiTree, parent: UiNodeId, label: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(28.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
        corner_radii: [4.0; 4], background: Some(bg),
        ..NodeStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(fg), text_size: Some(12.0), ..NodeStyle::default()
    });
    tree.add_child(row, lbl);
}

fn menu_sep(tree: &mut UiTree, parent: UiNodeId, color: glam::Vec4) {
    let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(1.0))), NodeStyle {
        background: Some(color), ..NodeStyle::default()
    });
    tree.add_child(parent, sep);
}
