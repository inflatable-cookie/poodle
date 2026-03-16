//! MediaPicker specimen — gallery with selection.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    section_label(tree, root, "Media Picker Dialog", text_secondary);
    {
        let dialog = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, width: Sizing::Fixed(420.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 10.0,
            ..UiStyle::default()
        });
        tree.add_child(root, dialog);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 },
            justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(dialog, header);

        let title = tree.create(Widget::Label { text: "Select Media".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let close = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(header, close);

        // Search
        let search = tree.create(Widget::Panel, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            margin: Edges { top: 0.0, right: 16.0, bottom: 8.0, left: 16.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radius: 6.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(dialog, search);

        let search_text = tree.create(Widget::Label { text: "🔍 Search media…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(search, search_text);

        // Grid
        let grid = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0,
            padding: Edges { top: 0.0, right: 16.0, bottom: 16.0, left: 16.0 },
            ..UiStyle::default()
        });
        tree.add_child(dialog, grid);

        let items = [
            ("photo_01.jpg", true),
            ("banner.png", false),
            ("logo.svg", false),
            ("hero.jpg", true),
            ("icon.png", false),
            ("bg_tile.jpg", false),
        ];

        for row_items in items.chunks(3) {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 4.0, ..UiStyle::default()
            });
            tree.add_child(grid, row);

            for &(name, selected) in row_items {
                let card = tree.create(Widget::Panel, UiStyle {
                    width: Sizing::Fixed(120.0), height: Sizing::Fixed(80.0),
                    background: Some(theme_bridge::tint(border, 0.3)),
                    corner_radius: 4.0,
                    border_color: if selected { Some(accent) } else { None },
                    border_width: if selected { 2.0 } else { 0.0 },
                    align: Align::End, justify: Justify::End,
                    padding: Edges::all(4.0),
                    ..UiStyle::default()
                });
                tree.add_child(row, card);

                if selected {
                    let check = tree.create(Widget::Panel, UiStyle {
                        width: Sizing::Fixed(16.0), height: Sizing::Fixed(16.0),
                        corner_radius: 8.0, background: Some(accent),
                        align: Align::Center, justify: Justify::Center,
                        ..UiStyle::default()
                    });
                    tree.add_child(card, check);

                    let mark = tree.create(Widget::Label { text: "✓".to_string() }, UiStyle {
                        text_color: Some(text_inverse), text_size: Some(10.0), ..UiStyle::default()
                    });
                    tree.add_child(check, mark);
                } else {
                    let n = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
                        text_color: Some(theme_bridge::tint(text_primary, 0.5)), text_size: Some(8.0), ..UiStyle::default()
                    });
                    tree.add_child(card, n);
                }
            }
        }

        // Footer
        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(dialog, sep);

        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 },
            gap: 8.0, justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(dialog, footer);

        let selected_count = tree.create(Widget::Label { text: "2 selected".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(footer, selected_count);

        let insert = tree.create(Widget::Button {
            label: "Insert".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(30.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radius: 6.0, background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, insert);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
