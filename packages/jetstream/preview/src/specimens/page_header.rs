//! PageHeader specimen — page-level header with title, actions, and breadcrumbs.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    section_label(tree, root, "Standard Page Header", text_secondary);
    {
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(440.0))
            .with_padding(LayoutEdges::uniform(16.0))
            .with_gap(8.0)), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, header);

        // Breadcrumb
        let crumbs = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(4.0)), NodeStyle::default());
        tree.add_child(header, crumbs);

        for (i, &item) in ["Home", "Projects", "Acme"].iter().enumerate() {
            if i > 0 {
                let sep = tree.create_node(Widget::Label { text: "/".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                    text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default()
                });
                tree.add_child(crumbs, sep);
            }
            let is_last = i == 2;
            let lbl = tree.create_node(Widget::Label { text: item.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(if is_last { text_primary } else { accent }),
                text_size: Some(10.0), ..NodeStyle::default()
            });
            tree.add_child(crumbs, lbl);
        }

        // Title + actions row
        let title_row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(header, title_row);

        let title = tree.create_node(Widget::Label { text: "Acme Project".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(18.0), ..NodeStyle::default()
        });
        tree.add_child(title_row, title);

        let actions = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)), NodeStyle::default());
        tree.add_child(title_row, actions);

        let edit = tree.create_node(Widget::Button {
            label: "Edit".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(actions, edit);

        let publish = tree.create_node(Widget::Button {
            label: "Publish".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(actions, publish);

        // Description
        let desc = tree.create_node(Widget::Label { text: "Main project workspace for the Acme initiative.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(header, desc);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
