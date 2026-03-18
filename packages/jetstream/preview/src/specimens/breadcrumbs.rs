//! Breadcrumbs specimen — navigation trail.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(16.0),
        ),
        NodeStyle::default(),
    );

    section_label(tree, root, "Standard", text_secondary);
    breadcrumb_row(tree, root, &["Home", "Documents", "Project Brief"], accent, text_primary, text_secondary);

    section_label(tree, root, "Deep Nesting", text_secondary);
    breadcrumb_row(tree, root, &["Home", "Settings", "Integrations", "API Keys", "Production"], accent, text_primary, text_secondary);

    section_label(tree, root, "Single Level", text_secondary);
    breadcrumb_row(tree, root, &["Dashboard"], accent, text_primary, text_secondary);

    root
}

fn breadcrumb_row(tree: &mut UiTree, parent: UiNodeId, items: &[&str], link_color: glam::Vec4, current_color: glam::Vec4, sep_color: glam::Vec4) {
    let row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(6.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
        ),
        NodeStyle::default(),
    );
    tree.add_child(parent, row);

    for (i, &item) in items.iter().enumerate() {
        if i > 0 {
            let sep = tree.create_node(
                Widget::Label { text: "/".to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(sep_color), text_size: Some(11.0), ..NodeStyle::default() },
            );
            tree.add_child(row, sep);
        }

        let is_last = i == items.len() - 1;
        let lbl = tree.create_node(
            Widget::Label { text: item.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(if is_last { current_color } else { link_color }),
                text_size: Some(12.0), ..NodeStyle::default()
            },
        );
        tree.add_child(row, lbl);
    }
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() },
    );
    tree.add_child(parent, lbl);
}
