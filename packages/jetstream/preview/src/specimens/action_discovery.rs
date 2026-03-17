//! ActionDiscoveryPanel specimen — categorized action list with keyboard shortcut badges.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Action discovery panel ──
    section_label(tree, root, "Action Discovery Panel", text_secondary);
    {
        let panel = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(320.0),
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, panel);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(36.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(panel, header);

        let title = tree.create(Widget::Label { text: "Actions".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0),
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let search_hint = tree.create(Widget::Label { text: "Search...".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(header, search_hint);

        let div = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(panel, div);

        // Categories
        let content = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(8.0), gap: 12.0,
            ..UiStyle::default()
        });
        tree.add_child(panel, content);

        // File category
        category_section(tree, content, "File", &[
            ("New File", "\u{2318}N"),
            ("Open", "\u{2318}O"),
            ("Save", "\u{2318}S"),
            ("Save As", "\u{21E7}\u{2318}S"),
        ], text_primary, text_secondary, accent, bg_elevated, border);

        // Edit category
        category_section(tree, content, "Edit", &[
            ("Undo", "\u{2318}Z"),
            ("Redo", "\u{21E7}\u{2318}Z"),
            ("Cut", "\u{2318}X"),
            ("Copy", "\u{2318}C"),
        ], text_primary, text_secondary, accent, bg_elevated, border);

        // View category
        category_section(tree, content, "View", &[
            ("Toggle Sidebar", "\u{2318}B"),
            ("Zoom In", "\u{2318}+"),
            ("Zoom Out", "\u{2318}-"),
        ], text_primary, text_secondary, accent, bg_elevated, border);
    }

    root
}

fn category_section(
    tree: &mut UiTree,
    parent: UiNodeId,
    category: &str,
    actions: &[(&str, &str)],
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    _accent: glam::Vec4,
    bg_elevated: glam::Vec4,
    border: glam::Vec4,
) {
    let section = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 2.0, ..UiStyle::default()
    });
    tree.add_child(parent, section);

    // Category label
    let cat_label = tree.create(Widget::Label { text: category.to_string() }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(9.0),
        padding: Edges { top: 0.0, right: 0.0, bottom: 4.0, left: 4.0 },
        ..UiStyle::default()
    });
    tree.add_child(section, cat_label);

    for &(action, shortcut) in actions {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            corner_radii: [4.0; 4], gap: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(section, row);

        let action_label = tree.create(Widget::Label { text: action.to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(11.0),
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(row, action_label);

        // Keyboard shortcut pill
        let pill = tree.create(Widget::Panel, UiStyle {
            height: Sizing::Fixed(20.0),
            padding: Edges { top: 0.0, right: 6.0, bottom: 0.0, left: 6.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [4.0; 4], align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(row, pill);

        let shortcut_label = tree.create(Widget::Label { text: shortcut.to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(pill, shortcut_label);
    }
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
