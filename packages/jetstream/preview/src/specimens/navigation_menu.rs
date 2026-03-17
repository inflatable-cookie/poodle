//! NavigationMenu specimen — navigation bar with dropdown sections.

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
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Horizontal nav menu ──
    section_label(tree, root, "Horizontal Navigation", text_secondary);
    {
        let nav = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(420.0),
            height: Sizing::Fixed(40.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            gap: 4.0,
            align: Align::Center,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, nav);

        nav_item(tree, nav, "Home", text_primary, false, accent);
        nav_item(tree, nav, "Products ▾", text_primary, true, accent);
        nav_item(tree, nav, "Resources ▾", text_primary, false, accent);
        nav_item(tree, nav, "About", text_primary, false, accent);

        // Push right
        let spacer = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(nav, spacer);

        nav_item(tree, nav, "Contact", accent, false, accent);
    }

    // ── With open dropdown ──
    section_label(tree, root, "With Open Dropdown", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 2.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        let nav = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(420.0),
            height: Sizing::Fixed(40.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            gap: 4.0,
            align: Align::Center,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(col, nav);

        nav_item(tree, nav, "Home", text_primary, false, accent);
        nav_item(tree, nav, "Products ▾", accent, true, accent);
        nav_item(tree, nav, "Resources ▾", text_primary, false, accent);

        // Dropdown panel
        let dropdown_offset = tree.create(Widget::Panel, UiStyle {
            padding: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 50.0 },
            ..UiStyle::default()
        });
        tree.add_child(col, dropdown_offset);

        let dropdown = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(340.0),
            padding: Edges::all(16.0),
            gap: 24.0,
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(dropdown_offset, dropdown);

        // Category columns
        for &(cat, items) in &[
            ("Platform", &["Overview", "Features", "Pricing"][..]),
            ("Solutions", &["Enterprise", "Startups", "Agencies"][..]),
        ] {
            let column = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, gap: 6.0, ..UiStyle::default()
            });
            tree.add_child(dropdown, column);

            let cat_label = tree.create(Widget::Label { text: cat.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(column, cat_label);

            for item in items {
                let link = tree.create(Widget::Label { text: item.to_string() }, UiStyle {
                    text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
                });
                tree.add_child(column, link);
            }
        }
    }

    // ── Vertical nav menu (sidebar) ──
    section_label(tree, root, "Vertical Navigation", text_secondary);
    {
        let nav = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(200.0),
            padding: Edges::all(8.0),
            gap: 2.0,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, nav);

        for (i, &item) in ["Dashboard", "Projects", "Team", "Settings"].iter().enumerate() {
            let is_active = i == 0;
            let bg = if is_active { Some(theme_bridge::tint(accent, 0.12)) } else { None };
            let color = if is_active { accent } else { text_primary };

            let btn = tree.create(Widget::Button {
                label: item.to_string(), pressed: false, hovered: false,
            }, UiStyle {
                width: Sizing::Grow(1.0), height: Sizing::Fixed(32.0),
                padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
                corner_radii: [6.0; 4], background: bg,
                text_color: Some(color), text_size: Some(12.0),
                align: Align::Center, focusable: true,
                ..UiStyle::default()
            });
            tree.add_child(nav, btn);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn nav_item(tree: &mut UiTree, parent: UiNodeId, label: &str, color: glam::Vec4, _active: bool, _accent: glam::Vec4) {
    let btn = tree.create(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(30.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radii: [4.0; 4], text_color: Some(color),
        text_size: Some(12.0), align: Align::Center,
        justify: Justify::Center, focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}
