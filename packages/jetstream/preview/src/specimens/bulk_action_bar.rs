//! BulkActionBar specimen — batch action bar for multi-select operations.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

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

    // ── Standard ──
    section_label(tree, root, "Standard (3 selected)", text_secondary);
    {
        let bar = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Fixed(400.0))
                    .with_height(LayoutSizing::Fixed(44.0))
                    .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                background: Some(accent),
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, bar);

        let count = tree.create_node(
            Widget::Label { text: "3 selected".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_inverse), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(bar, count);

        let spacer = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new().with_width(LayoutSizing::Grow),
            ),
            NodeStyle::default(),
        );
        tree.add_child(bar, spacer);

        bar_btn(tree, bar, "Move", text_inverse);
        bar_btn(tree, bar, "Archive", text_inverse);
        bar_btn(tree, bar, "Delete", text_inverse);

        let close = tree.create_node(
            Widget::Label { text: "✕".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(theme_bridge::tint(text_inverse, 0.7)),
                text_size: Some(12.0), ..NodeStyle::default()
            },
        );
        tree.add_child(bar, close);
    }

    // ── With danger action ──
    section_label(tree, root, "Outlined with Danger Action", text_secondary);
    {
        let bar = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Fixed(400.0))
                    .with_height(LayoutSizing::Fixed(44.0))
                    .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                background: Some(bg_surface),
                border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, bar);

        let count = tree.create_node(
            Widget::Label { text: "12 items selected".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(bar, count);

        let spacer = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new().with_width(LayoutSizing::Grow),
            ),
            NodeStyle::default(),
        );
        tree.add_child(bar, spacer);

        let export = tree.create_node(
            Widget::Button {
                label: "Export".to_string(), pressed: false, hovered: false,
            },
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_height(LayoutSizing::Fixed(28.0))
                    .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                    .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
                text_color: Some(text_primary), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            },
        );
        tree.add_child(bar, export);

        let del = tree.create_node(
            Widget::Button {
                label: "Delete All".to_string(), pressed: false, hovered: false,
            },
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_height(LayoutSizing::Fixed(28.0))
                    .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                    .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                corner_radii: [6.0; 4], background: Some(danger),
                text_color: Some(text_inverse), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            },
        );
        tree.add_child(bar, del);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() },
    );
    tree.add_child(parent, lbl);
}

fn bar_btn(tree: &mut UiTree, parent: UiNodeId, label: &str, fg: glam::Vec4) {
    let btn = tree.create_node(
        Widget::Button {
            label: label.to_string(), pressed: false, hovered: false,
        },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(28.0))
                .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
        ),
        NodeStyle {
            corner_radii: [4.0; 4], text_color: Some(fg),
            text_size: Some(11.0),
            focusable: true,
            ..NodeStyle::default()
        },
    );
    tree.add_child(parent, btn);
}
