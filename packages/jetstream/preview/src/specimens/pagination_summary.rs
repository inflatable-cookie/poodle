//! PaginationSummary specimen — pagination state summary text.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(12.0)), NodeStyle::default());

    section_label(tree, root, "Standard Summary", text_secondary);
    summary_text(tree, root, "Showing 1–25 of 142 results", text_secondary);

    section_label(tree, root, "With selection count", text_secondary);
    summary_text(tree, root, "3 selected · Showing 26–50 of 142 results", text_secondary);

    section_label(tree, root, "Single page", text_secondary);
    summary_text(tree, root, "Showing all 12 results", text_secondary);

    section_label(tree, root, "In context (with pagination)", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(12.0)
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)
            .with_width(LayoutSizing::Fixed(380.0))), NodeStyle::default());
        tree.add_child(root, row);

        let summary = tree.create_node(Widget::Label { text: "Showing 1–25 of 142".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(row, summary);

        let nav = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(4.0)), NodeStyle::default());
        tree.add_child(row, nav);

        let prev = tree.create_node(Widget::Label { text: "← Prev".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default()
        });
        tree.add_child(nav, prev);

        let page = tree.create_node(Widget::Label { text: "1 / 6".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default()
        });
        tree.add_child(nav, page);

        let next = tree.create_node(Widget::Label { text: "Next →".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default()
        });
        tree.add_child(nav, next);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn summary_text(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
