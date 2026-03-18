//! Card specimen — contained surface for content display.

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
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(20.0),
        ),
        NodeStyle::default(),
    );

    // ── Basic card ──
    section_label(tree, root, "Basic Card", text_secondary);
    {
        let card = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Fixed(280.0))
                    .with_padding(LayoutEdges::uniform(16.0))
                    .with_gap(8.0),
            ),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, card);

        let title = tree.create_node(
            Widget::Label { text: "Card Title".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() },
        );
        tree.add_child(card, title);

        let body = tree.create_node(
            Widget::Label { text: "This is a basic card with some descriptive text content.".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(card, body);
    }

    // ── Card with image ──
    section_label(tree, root, "Card with Image", text_secondary);
    {
        let card = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Fixed(280.0)),
            ),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, card);

        let img = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(120.0)),
            ),
            NodeStyle {
                background: Some(border), corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(card, img);

        let body = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_padding(LayoutEdges::uniform(16.0))
                    .with_gap(8.0),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, body);

        let title = tree.create_node(
            Widget::Label { text: "Featured Article".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() },
        );
        tree.add_child(body, title);

        let desc = tree.create_node(
            Widget::Label { text: "An overview of the latest updates.".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(body, desc);
    }

    // ── Card with actions ──
    section_label(tree, root, "Card with Actions", text_secondary);
    {
        let card = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Fixed(280.0)),
            ),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, card);

        let body = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_padding(LayoutEdges::uniform(16.0))
                    .with_gap(8.0),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, body);

        let title = tree.create_node(
            Widget::Label { text: "Subscription".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() },
        );
        tree.add_child(body, title);

        let desc = tree.create_node(
            Widget::Label { text: "Manage your subscription plan.".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(body, desc);

        // Separator
        let sep = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(1.0)),
            ),
            NodeStyle { background: Some(border), ..NodeStyle::default() },
        );
        tree.add_child(card, sep);

        let footer = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_padding(LayoutEdges { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 })
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, footer);

        let btn = tree.create_node(
            Widget::Button {
                label: "Upgrade".to_string(), pressed: false, hovered: false,
            },
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_height(LayoutSizing::Fixed(28.0))
                    .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                    .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                corner_radii: [6.0; 4], background: Some(accent),
                text_color: Some(text_inverse), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            },
        );
        tree.add_child(footer, btn);
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
