//! Table specimen — static data table with header, rows, and alignment.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    let col_widths = [120.0_f32, 80.0, 60.0, 80.0];

    // ── Standard table ──
    section_label(tree, root, "Standard Table", text_secondary);
    {
        let table = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, table);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            background: Some(bg_surface),
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            ..UiStyle::default()
        });
        tree.add_child(table, header);

        for (&label, &w) in ["Name", "Role", "Status", "Joined"].iter().zip(col_widths.iter()) {
            let cell = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(w), ..UiStyle::default()
            });
            tree.add_child(header, cell);
        }

        // Rows
        let data = &[
            ["Alice Chen", "Admin", "Active", "Jan 2025"],
            ["Bob Smith", "Editor", "Active", "Mar 2025"],
            ["Carol Lee", "Viewer", "Inactive", "Jun 2025"],
        ];

        for row_data in data {
            let sep = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
                background: Some(border), ..UiStyle::default()
            });
            tree.add_child(table, sep);

            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
                ..UiStyle::default()
            });
            tree.add_child(table, row);

            for (val, &w) in row_data.iter().zip(col_widths.iter()) {
                let cell = tree.create(Widget::Label { text: val.to_string() }, UiStyle {
                    text_color: Some(text_primary), text_size: Some(12.0),
                    width: Sizing::Fixed(w), ..UiStyle::default()
                });
                tree.add_child(row, cell);
            }
        }
    }

    // ── Compact ──
    section_label(tree, root, "Compact Variant", text_secondary);
    {
        let table = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            border_color: Some(border), border_width: 1.0,
            corner_radius: 6.0, ..UiStyle::default()
        });
        tree.add_child(root, table);

        for (i, &(key, val)) in [("CPU", "42%"), ("Memory", "2.1 GB"), ("Disk", "67%")].iter().enumerate() {
            if i > 0 {
                let sep = tree.create(Widget::Panel, UiStyle {
                    width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
                    background: Some(border), ..UiStyle::default()
                });
                tree.add_child(table, sep);
            }

            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                padding: Edges { top: 6.0, right: 10.0, bottom: 6.0, left: 10.0 },
                justify: Justify::SpaceBetween, align: Align::Center,
                width: Sizing::Fixed(200.0),
                ..UiStyle::default()
            });
            tree.add_child(table, row);

            let k = tree.create(Widget::Label { text: key.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(row, k);

            let v = tree.create(Widget::Label { text: val.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(row, v);
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
