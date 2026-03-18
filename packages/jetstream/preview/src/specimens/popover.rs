//! Popover specimen — anchored floating overlay mockup.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Simple content popover ──
    section_label(tree, root, "Content Popover", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(4.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        // Trigger
        let trigger = tree.create_node(Widget::Button {
            label: "More Info \u{25be}".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4], background: Some(bg_surface),
                border_color: Some(border), border_width: 1.0,
                text_color: Some(text_primary), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            });
        tree.add_child(col, trigger);

        // Popover card
        let card = popover_card(tree, 220.0, bg_elevated, border);
        tree.add_child(col, card);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges::uniform(12.0))
            .with_direction(LayoutDirection::Column)
            .with_gap(4.0)),
            NodeStyle::default());
        tree.add_child(card, body);

        let title = tree.create_node(Widget::Label { text: "Details".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(body, title);

        let desc = tree.create_node(Widget::Label { text: "Additional information about this item.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(body, desc);
    }

    // ── Actions popover ──
    section_label(tree, root, "Actions Popover", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(4.0)),
            NodeStyle::default());
        tree.add_child(root, col);

        let trigger = tree.create_node(Widget::Button {
            label: "\u{22ef}".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(32.0))
            .with_height(LayoutSizing::Fixed(28.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4], background: Some(bg_surface),
                border_color: Some(border), border_width: 1.0,
                text_color: Some(text_primary), text_size: Some(14.0),
                focusable: true, ..NodeStyle::default()
            });
        tree.add_child(col, trigger);

        let card = popover_card(tree, 160.0, bg_elevated, border);
        tree.add_child(col, card);

        let list = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(4.0))
            .with_gap(1.0)),
            NodeStyle::default());
        tree.add_child(card, list);

        for item in &["Edit", "Duplicate", "Archive", "Delete"] {
            let row = tree.create_node(Widget::Button {
                label: item.to_string(), pressed: false, hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(28.0))
                .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    corner_radii: [4.0; 4], text_color: Some(text_primary),
                    text_size: Some(12.0), focusable: true, ..NodeStyle::default()
                });
            tree.add_child(list, row);
        }
    }

    // ── Form popover ──
    section_label(tree, root, "Form Popover", text_secondary);
    {
        let card = popover_card(tree, 240.0, bg_elevated, border);
        tree.add_child(root, card);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(12.0))
            .with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(card, body);

        let title = tree.create_node(Widget::Label { text: "Quick Add".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(body, title);

        let bg_canvas = theme_bridge::canvas_background(theme);
        let border_default = theme_bridge::border_default(theme);

        let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_canvas), border_color: Some(border_default),
                border_width: 1.0, corner_radii: [4.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(body, input);

        let ph = tree.create_node(Widget::Label { text: "Enter value...".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(theme_bridge::tint(text_secondary, 0.6)),
                text_size: Some(11.0), ..NodeStyle::default()
            });
        tree.add_child(input, ph);

        let accent = theme_bridge::accent_base(theme);
        let text_inverse = theme_bridge::text_inverse(theme);
        let add_btn = tree.create_node(Widget::Button {
            label: "Add".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(28.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [4.0; 4], background: Some(accent),
                text_color: Some(text_inverse), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            });
        tree.add_child(body, add_btn);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn popover_card(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(width))),
        NodeStyle {
            background: Some(bg),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        })
}
