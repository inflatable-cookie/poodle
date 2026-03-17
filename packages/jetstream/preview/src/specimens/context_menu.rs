//! ContextMenu specimen — right-click menu mockup.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Standard context menu ──
    section_label(tree, root, "Standard Context Menu", text_secondary);
    {
        let menu = menu_frame(tree, 180.0, bg_elevated, border);
        tree.add_child(root, menu);

        let list = menu_list(tree, menu);

        menu_item(tree, list, "Cut", Some("⌘X"), text_primary, text_secondary);
        menu_item(tree, list, "Copy", Some("⌘C"), text_primary, text_secondary);
        menu_item(tree, list, "Paste", Some("⌘V"), text_primary, text_secondary);
        menu_separator(tree, list, border);
        menu_item(tree, list, "Select All", Some("⌘A"), text_primary, text_secondary);
    }

    // ── With sections and icons ──
    section_label(tree, root, "Sectioned Context Menu", text_secondary);
    {
        let menu = menu_frame(tree, 200.0, bg_elevated, border);
        tree.add_child(root, menu);

        let list = menu_list(tree, menu);

        menu_item(tree, list, "✎  Edit", None, text_primary, text_secondary);
        menu_item(tree, list, "📋  Duplicate", None, text_primary, text_secondary);
        menu_item(tree, list, "📌  Pin to Top", None, text_primary, text_secondary);
        menu_separator(tree, list, border);
        menu_item(tree, list, "📂  Move to...", None, text_primary, text_secondary);
        menu_item(tree, list, "🏷  Add Label", None, text_primary, text_secondary);
        menu_separator(tree, list, border);
        menu_item_colored(tree, list, "🗑  Delete", danger);
    }

    // ── With disabled items ──
    section_label(tree, root, "With Disabled Items", text_secondary);
    {
        let menu = menu_frame(tree, 180.0, bg_elevated, border);
        tree.add_child(root, menu);

        let list = menu_list(tree, menu);

        menu_item(tree, list, "Undo", Some("⌘Z"), text_primary, text_secondary);
        menu_item_disabled(tree, list, "Redo", Some("⇧⌘Z"), text_secondary);
        menu_separator(tree, list, border);
        menu_item(tree, list, "Cut", Some("⌘X"), text_primary, text_secondary);
        menu_item_disabled(tree, list, "Paste", Some("⌘V"), text_secondary);
    }

    // ── With submenu indicator ──
    section_label(tree, root, "With Submenu", text_secondary);
    {
        let menu = menu_frame(tree, 180.0, bg_elevated, border);
        tree.add_child(root, menu);

        let list = menu_list(tree, menu);

        menu_item(tree, list, "View", None, text_primary, text_secondary);
        menu_item_submenu(tree, list, "Sort By", text_primary, text_secondary);
        menu_item_submenu(tree, list, "Group By", text_primary, text_secondary);
        menu_separator(tree, list, border);
        menu_item(tree, list, "Preferences", None, text_primary, text_secondary);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn menu_frame(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(width),
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [8.0; 4],
        ..UiStyle::default()
    })
}

fn menu_list(tree: &mut UiTree, parent: UiNodeId) -> UiNodeId {
    let list = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        padding: Edges::all(4.0),
        gap: 1.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, list);
    list
}

fn menu_item(tree: &mut UiTree, parent: UiNodeId, label: &str, shortcut: Option<&str>, fg: glam::Vec4, muted: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radii: [4.0; 4],
        align: Align::Center,
        justify: Justify::SpaceBetween,
        ..UiStyle::default()
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

fn menu_item_colored(tree: &mut UiTree, parent: UiNodeId, label: &str, color: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radii: [4.0; 4],
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, lbl);
}

fn menu_item_disabled(tree: &mut UiTree, parent: UiNodeId, label: &str, shortcut: Option<&str>, muted: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radii: [4.0; 4],
        align: Align::Center,
        justify: Justify::SpaceBetween,
        opacity: 0.4,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(muted), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, lbl);

    if let Some(sc) = shortcut {
        let s = tree.create(Widget::Label { text: sc.to_string() }, UiStyle {
            text_color: Some(muted), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, s);
    }
}

fn menu_item_submenu(tree: &mut UiTree, parent: UiNodeId, label: &str, fg: glam::Vec4, muted: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radii: [4.0; 4],
        align: Align::Center,
        justify: Justify::SpaceBetween,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, lbl);

    let arrow = tree.create(Widget::Label { text: "▸".to_string() }, UiStyle {
        text_color: Some(muted), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(row, arrow);
}

fn menu_separator(tree: &mut UiTree, parent: UiNodeId, color: glam::Vec4) {
    let sep = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(1.0),
        background: Some(color),
        ..UiStyle::default()
    });
    tree.add_child(parent, sep);
}
