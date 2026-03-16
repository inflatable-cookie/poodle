//! Menu specimen — dropdown menu with items, separators, and keyboard hints.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Simple menu ──
    section_label(tree, root, "Simple Menu", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        let trigger = tree.create(Widget::Button {
            label: "Options ▾".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radius: 6.0, background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
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
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn menu_card(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Fixed(width),
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radius: 8.0,
        ..UiStyle::default()
    })
}

fn menu_list(tree: &mut UiTree, parent: UiNodeId) -> UiNodeId {
    let list = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, padding: Edges::all(4.0), gap: 1.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, list);
    list
}

fn menu_item(tree: &mut UiTree, parent: UiNodeId, label: &str, shortcut: Option<&str>, fg: glam::Vec4, muted: glam::Vec4, _disabled: bool) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0), height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radius: 4.0, align: Align::Center,
        justify: Justify::SpaceBetween, ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, lbl);

    if let Some(sc) = shortcut {
        let s = tree.create(Widget::Label { text: sc.to_string() }, UiStyle {
            text_color: Some(muted), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, s);
    }
}

fn menu_item_active(tree: &mut UiTree, parent: UiNodeId, label: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0), height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radius: 4.0, background: Some(bg), align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, lbl);
}

fn menu_sep(tree: &mut UiTree, parent: UiNodeId, color: glam::Vec4) {
    let sep = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
        background: Some(color), ..UiStyle::default()
    });
    tree.add_child(parent, sep);
}
