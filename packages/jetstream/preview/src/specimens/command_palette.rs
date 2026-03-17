//! CommandPalette specimen — search with category grouping and keyboard navigation.

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

    section_label(tree, root, "Command Palette", text_secondary);
    {
        let palette = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(400.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [10.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, palette);

        // Search input
        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0,
            padding: Edges { top: 12.0, right: 14.0, bottom: 12.0, left: 14.0 },
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(palette, search);

        let search_icon = tree.create(Widget::Label { text: "🔍".to_string() }, UiStyle {
            text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(search, search_icon);

        let search_input = tree.create(Widget::Label { text: "Type a command…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(search, search_input);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(palette, sep);

        // Categories
        let list = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 4.0, right: 6.0, bottom: 4.0, left: 6.0 },
            gap: 2.0,
            ..UiStyle::default()
        });
        tree.add_child(palette, list);

        category_label(tree, list, "Recent", text_secondary);
        command_item(tree, list, "Open Project…", "⌘O", false, text_primary, text_secondary, accent, bg_surface);
        command_item(tree, list, "Quick Switch", "⌘P", true, text_primary, text_secondary, accent, bg_surface);

        category_label(tree, list, "Navigation", text_secondary);
        command_item(tree, list, "Go to File", "⌘⇧F", false, text_primary, text_secondary, accent, bg_surface);
        command_item(tree, list, "Go to Symbol", "⌘⇧O", false, text_primary, text_secondary, accent, bg_surface);

        category_label(tree, list, "Actions", text_secondary);
        command_item(tree, list, "New Document", "⌘N", false, text_primary, text_secondary, accent, bg_surface);
        command_item(tree, list, "Toggle Sidebar", "⌘B", false, text_primary, text_secondary, accent, bg_surface);

        // Footer
        let footer_sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(palette, footer_sep);

        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 6.0, right: 14.0, bottom: 6.0, left: 14.0 },
            gap: 12.0,
            ..UiStyle::default()
        });
        tree.add_child(palette, footer);

        for &hint in &["↑↓ Navigate", "↵ Select", "Esc Close"] {
            let h = tree.create(Widget::Label { text: hint.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
            });
            tree.add_child(footer, h);
        }
    }

    root
}

fn category_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(9.0),
        padding: Edges { top: 6.0, right: 8.0, bottom: 2.0, left: 8.0 },
        ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn command_item(tree: &mut UiTree, parent: UiNodeId, label: &str, shortcut: &str, selected: bool, text_primary: glam::Vec4, text_secondary: glam::Vec4, accent: glam::Vec4, bg_surface: glam::Vec4) {
    let bg = if selected { Some(theme_bridge::tint(accent, 0.10)) } else { None };
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
        background: bg, corner_radii: [4.0; 4],
        justify: Justify::SpaceBetween, align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(if selected { accent } else { text_primary }),
        text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, lbl);

    let kbd = tree.create(Widget::Panel, UiStyle {
        height: Sizing::Fixed(18.0),
        padding: Edges { top: 0.0, right: 6.0, bottom: 0.0, left: 6.0 },
        background: Some(bg_surface), corner_radii: [3.0; 4],
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(row, kbd);

    let k = tree.create(Widget::Label { text: shortcut.to_string() }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
    });
    tree.add_child(kbd, k);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
