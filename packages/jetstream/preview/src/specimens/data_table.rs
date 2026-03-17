//! DataTable specimen — feature-rich table with header, sortable columns, selection, and pagination.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 24.0,
        ..UiStyle::default()
    });

    let col_widths = [30.0_f32, 140.0, 100.0, 80.0, 80.0];

    // ── Full DataTable ──
    section_label(tree, root, "Full DataTable with Selection", text_secondary);
    {
        let wrapper = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, wrapper);

        // Toolbar row
        let toolbar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            gap: 8.0, align: Align::Center, justify: Justify::SpaceBetween,
            ..UiStyle::default()
        });
        tree.add_child(wrapper, toolbar);

        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(toolbar, search);
        let search_lbl = tree.create(Widget::Label { text: "🔍 Search…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(search, search_lbl);

        let add_btn = tree.create(Widget::Button {
            label: "+ Add Row".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(toolbar, add_btn);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            background: Some(bg_surface),
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            ..UiStyle::default()
        });
        tree.add_child(wrapper, header);

        for (&label, &w) in ["☐", "Name ↑", "Email", "Role", "Status"].iter().zip(col_widths.iter()) {
            let cell = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(w), ..UiStyle::default()
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
            let sep = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
                background: Some(border), ..UiStyle::default()
            });
            tree.add_child(wrapper, sep);

            let row_bg = if selected { Some(theme_bridge::tint(accent, 0.06)) } else { None };
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
                background: row_bg,
                ..UiStyle::default()
            });
            tree.add_child(wrapper, row);

            for (&val, &w) in [chk, name, email, role, status].iter().zip(col_widths.iter()) {
                let cell = tree.create(Widget::Label { text: val.to_string() }, UiStyle {
                    text_color: Some(text_primary), text_size: Some(12.0),
                    width: Sizing::Fixed(w), ..UiStyle::default()
                });
                tree.add_child(row, cell);
            }
        }

        // Footer / pagination
        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(wrapper, footer);

        let info = tree.create(Widget::Label { text: "2 of 4 selected · Page 1 of 1".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(footer, info);
    }

    // ── Empty state ──
    section_label(tree, root, "Empty State", text_secondary);
    {
        let wrapper = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, wrapper);

        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            background: Some(bg_surface),
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            ..UiStyle::default()
        });
        tree.add_child(wrapper, header);
        for &label in &["☐", "Name", "Email", "Status"] {
            let cell = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(100.0), ..UiStyle::default()
            });
            tree.add_child(header, cell);
        }

        let empty = tree.create(Widget::Panel, UiStyle {
            padding: Edges::all(32.0),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(wrapper, empty);
        let msg = tree.create(Widget::Label { text: "No data to display".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(empty, msg);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
