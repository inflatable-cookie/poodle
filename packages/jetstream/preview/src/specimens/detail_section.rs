//! DetailSection specimen — titled collapsible detail section.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Expanded ──
    section_label(tree, root, "Expanded Section", text_secondary);
    {
        let sec = detail_section_frame(tree, bg_elevated, border);
        tree.add_child(root, sec);

        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
            justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(sec, header);

        let title = tree.create(Widget::Label { text: "▾ General Information".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(sec, sep);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(12.0), gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(sec, body);

        detail_row(tree, body, "Name", "Acme Project", text_secondary, text_primary);
        detail_row(tree, body, "Type", "Internal", text_secondary, text_primary);
        detail_row(tree, body, "Priority", "High", text_secondary, text_primary);
    }

    // ── Collapsed ──
    section_label(tree, root, "Collapsed Section", text_secondary);
    {
        let sec = detail_section_frame(tree, bg_elevated, border);
        tree.add_child(root, sec);

        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
            justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(sec, header);

        let title = tree.create(Widget::Label { text: "▸ Advanced Settings".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let hint = tree.create(Widget::Label { text: "3 fields".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(header, hint);
    }

    // ── Multiple sections ──
    section_label(tree, root, "Stacked Sections", text_secondary);
    {
        let stack = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 2.0,
            ..UiStyle::default()
        });
        tree.add_child(root, stack);

        for &(label, expanded) in &[("▾ Overview", true), ("▸ Configuration", false), ("▸ Permissions", false)] {
            let sec = detail_section_frame(tree, bg_elevated, border);
            tree.add_child(stack, sec);

            let header = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
                align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(sec, header);

            let title = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
            });
            tree.add_child(header, title);

            if expanded {
                let sep = tree.create(Widget::Panel, UiStyle {
                    width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
                    background: Some(border), ..UiStyle::default()
                });
                tree.add_child(sec, sep);

                let body = tree.create(Widget::Panel, UiStyle {
                    padding: Edges::all(12.0),
                    ..UiStyle::default()
                });
                tree.add_child(sec, body);

                let content = tree.create(Widget::Label { text: "Section content goes here.".to_string() }, UiStyle {
                    text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
                });
                tree.add_child(body, content);
            }
        }
    }

    root
}

fn detail_section_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(360.0),
        background: Some(bg),
        border_color: Some(border), border_width: 1.0,
        corner_radius: 6.0,
        ..UiStyle::default()
    })
}

fn detail_row(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, label_color: glam::Vec4, value_color: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 12.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let l = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(label_color), text_size: Some(11.0),
        width: Sizing::Fixed(70.0), ..UiStyle::default()
    });
    tree.add_child(row, l);

    let v = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
        text_color: Some(value_color), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, v);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
