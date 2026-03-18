//! EmptyState specimen — placeholder with message and action button.

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

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(20.0)),
        NodeStyle::default());

    // ── With action ──
    section_label(tree, root, "With Action", text_secondary);
    {
        let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(360.0)).with_padding(LayoutEdges::uniform(32.0)).with_gap(12.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, card);

        let icon = tree.create_node(Widget::Label { text: "📁".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_size: Some(32.0), ..NodeStyle::default() });
        tree.add_child(card, icon);

        let title = tree.create_node(Widget::Label { text: "No documents yet".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(15.0), ..NodeStyle::default() });
        tree.add_child(card, title);

        let desc = tree.create_node(Widget::Label { text: "Upload your first document to get started.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(card, desc);

        let btn = tree.create_node(Widget::Button {
            label: "Upload Document".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], background: Some(accent), text_color: Some(text_inverse), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(card, btn);
    }

    // ── Minimal ──
    section_label(tree, root, "Minimal", text_secondary);
    {
        let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(360.0)).with_padding(LayoutEdges::uniform(24.0))
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, card);

        let msg = tree.create_node(Widget::Label { text: "Nothing to show".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(card, msg);
    }

    // ── Search empty ──
    section_label(tree, root, "Search Empty", text_secondary);
    {
        let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(360.0)).with_padding(LayoutEdges::uniform(32.0)).with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, card);

        let icon = tree.create_node(Widget::Label { text: "🔍".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_size: Some(24.0), ..NodeStyle::default() });
        tree.add_child(card, icon);

        let title = tree.create_node(Widget::Label { text: "No results found".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(card, title);

        let desc = tree.create_node(Widget::Label { text: "Try adjusting your search or filter criteria.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(card, desc);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
