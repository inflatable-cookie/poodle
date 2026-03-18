//! DataTable specimen — feature-rich table with header, sortable columns, selection, and pagination.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(24.0)), NodeStyle::default());

    let col_widths = [30.0_f32, 140.0, 100.0, 80.0, 80.0];

    // ── Full DataTable ──
    section_label(tree, root, "Full DataTable with Selection", text_secondary);
    {
        let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column)), NodeStyle {
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, wrapper);

        // Toolbar row
        let toolbar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 }).with_gap(8.0).with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(wrapper, toolbar);

        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(6.0).with_height(LayoutSizing::Fixed(28.0)).with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 }).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(toolbar, search);
        let search_lbl = tree.create_node(Widget::Label { text: "🔍 Search…".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default()
        });
        tree.add_child(search, search_lbl);

        let add_btn = tree.create_node(Widget::Button {
            label: "+ Add Row".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(28.0)).with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 }).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(toolbar, add_btn);

        // Header
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })), NodeStyle {
            background: Some(bg_surface),
            ..NodeStyle::default()
        });
        tree.add_child(wrapper, header);

        for (&label, &w) in ["☐", "Name ↑", "Email", "Role", "Status"].iter().zip(col_widths.iter()) {
            let cell = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(w))), NodeStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                ..NodeStyle::default()
            });
            tree.add_child(header, cell);
        }

        // Rows
        let data = &[
            ("☑", "Alice Chen", "alice@co.io", "Admin", "Active", true),
            ("☐", "Bob Smith", "bob@co.io", "Editor", "Active", false),
            ("☑", "Carol Lee", "carol@co.io", "Viewer", "Inactive", true),
            ("☐", "Dan Park", "dan@co.io", "Editor", "Active", false),
        ];

        for &(chk, name, email, role, status, selected) in data {
            let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))), NodeStyle {
                background: Some(border), ..NodeStyle::default()
            });
            tree.add_child(wrapper, sep);

            let row_bg = if selected { Some(theme_bridge::tint(accent, 0.06)) } else { None };
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })), NodeStyle {
                background: row_bg,
                ..NodeStyle::default()
            });
            tree.add_child(wrapper, row);

            for (&val, &w) in [chk, name, email, role, status].iter().zip(col_widths.iter()) {
                let cell = tree.create_node(Widget::Label { text: val.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(w))), NodeStyle {
                    text_color: Some(text_primary), text_size: Some(12.0),
                    ..NodeStyle::default()
                });
                tree.add_child(row, cell);
            }
        }

        // Footer / pagination
        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 }).with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(wrapper, footer);

        let info = tree.create_node(Widget::Label { text: "2 of 4 selected · Page 1 of 1".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default()
        });
        tree.add_child(footer, info);
    }

    // ── Empty state ──
    section_label(tree, root, "Empty State", text_secondary);
    {
        let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column)), NodeStyle {
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, wrapper);

        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })), NodeStyle {
            background: Some(bg_surface),
            ..NodeStyle::default()
        });
        tree.add_child(wrapper, header);
        for &label in &["☐", "Name", "Email", "Status"] {
            let cell = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(100.0))), NodeStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                ..NodeStyle::default()
            });
            tree.add_child(header, cell);
        }

        let empty = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_padding(LayoutEdges::uniform(32.0)).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(wrapper, empty);
        let msg = tree.create_node(Widget::Label { text: "No data to display".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(13.0), ..NodeStyle::default()
        });
        tree.add_child(empty, msg);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
