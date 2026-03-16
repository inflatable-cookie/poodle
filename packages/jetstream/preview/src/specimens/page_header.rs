//! PageHeader specimen — page-level header with title, actions, and breadcrumbs.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    section_label(tree, root, "Standard Page Header", text_secondary);
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(440.0), padding: Edges::all(16.0), gap: 8.0,
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        // Breadcrumb
        let crumbs = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(header, crumbs);

        for (i, &item) in ["Home", "Projects", "Acme"].iter().enumerate() {
            if i > 0 {
                let sep = tree.create(Widget::Label { text: "/".to_string() }, UiStyle {
                    text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
                });
                tree.add_child(crumbs, sep);
            }
            let is_last = i == 2;
            let lbl = tree.create(Widget::Label { text: item.to_string() }, UiStyle {
                text_color: Some(if is_last { text_primary } else { accent }),
                text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(crumbs, lbl);
        }

        // Title + actions row
        let title_row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(header, title_row);

        let title = tree.create(Widget::Label { text: "Acme Project".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(18.0), ..UiStyle::default()
        });
        tree.add_child(title_row, title);

        let actions = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, ..UiStyle::default()
        });
        tree.add_child(title_row, actions);

        let edit = tree.create(Widget::Button {
            label: "Edit".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radius: 6.0, border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(actions, edit);

        let publish = tree.create(Widget::Button {
            label: "Publish".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radius: 6.0, background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(actions, publish);

        // Description
        let desc = tree.create(Widget::Label { text: "Main project workspace for the Acme initiative.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(header, desc);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
