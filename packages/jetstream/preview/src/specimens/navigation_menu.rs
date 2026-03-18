//! NavigationMenu specimen — navigation bar with dropdown sections.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    // ── Horizontal nav menu ──
    section_label(tree, root, "Horizontal Navigation", text_secondary);
    {
        let nav = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(420.0))
            .with_height(LayoutSizing::Fixed(40.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
            .with_gap(4.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, nav);

        nav_item(tree, nav, "Home", text_primary, false, accent);
        nav_item(tree, nav, "Products ▾", text_primary, true, accent);
        nav_item(tree, nav, "Resources ▾", text_primary, false, accent);
        nav_item(tree, nav, "About", text_primary, false, accent);

        // Push right
        let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)), NodeStyle::default());
        tree.add_child(nav, spacer);

        nav_item(tree, nav, "Contact", accent, false, accent);
    }

    // ── With open dropdown ──
    section_label(tree, root, "With Open Dropdown", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(2.0)), NodeStyle::default());
        tree.add_child(root, col);

        let nav = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(420.0))
            .with_height(LayoutSizing::Fixed(40.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
            .with_gap(4.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(col, nav);

        nav_item(tree, nav, "Home", text_primary, false, accent);
        nav_item(tree, nav, "Products ▾", accent, true, accent);
        nav_item(tree, nav, "Resources ▾", text_primary, false, accent);

        // Dropdown panel
        let dropdown_offset = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges { top: 0.0, right: 0.0, bottom: 0.0, left: 50.0 })), NodeStyle::default());
        tree.add_child(col, dropdown_offset);

        let dropdown = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(340.0))
            .with_padding(LayoutEdges::uniform(16.0))
            .with_gap(24.0)), NodeStyle {
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(dropdown_offset, dropdown);

        // Category columns
        for &(cat, items) in &[
            ("Platform", &["Overview", "Features", "Pricing"][..]),
            ("Solutions", &["Enterprise", "Startups", "Agencies"][..]),
        ] {
            let column = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(6.0)), NodeStyle::default());
            tree.add_child(dropdown, column);

            let cat_label = tree.create_node(Widget::Label { text: cat.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default()
            });
            tree.add_child(column, cat_label);

            for item in items {
                let link = tree.create_node(Widget::Label { text: item.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                    text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default()
                });
                tree.add_child(column, link);
            }
        }
    }

    // ── Vertical nav menu (sidebar) ──
    section_label(tree, root, "Vertical Navigation", text_secondary);
    {
        let nav = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(200.0))
            .with_padding(LayoutEdges::uniform(8.0))
            .with_gap(2.0)), NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, nav);

        for (i, &item) in ["Dashboard", "Projects", "Team", "Settings"].iter().enumerate() {
            let is_active = i == 0;
            let bg = if is_active { Some(theme_bridge::tint(accent, 0.12)) } else { None };
            let color = if is_active { accent } else { text_primary };

            let btn = tree.create_node(Widget::Button {
                label: item.to_string(), pressed: false, hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
                corner_radii: [6.0; 4], background: bg,
                text_color: Some(color), text_size: Some(12.0),
                focusable: true,
                ..NodeStyle::default()
            });
            tree.add_child(nav, btn);
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

fn nav_item(tree: &mut UiTree, parent: UiNodeId, label: &str, color: glam::Vec4, _active: bool, _accent: glam::Vec4) {
    let btn = tree.create_node(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(30.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
        corner_radii: [4.0; 4], text_color: Some(color),
        text_size: Some(12.0), focusable: true,
        ..NodeStyle::default()
    });
    tree.add_child(parent, btn);
}
