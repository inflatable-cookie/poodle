//! SelectionSummary specimen — current selection state display with count and clear.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    // ── Standard ──
    section_label(tree, root, "Standard Summary", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(360.0))
            .with_height(LayoutSizing::Fixed(36.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(theme_bridge::tint(accent, 0.08)), border_color: Some(theme_bridge::tint(accent, 0.3)), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(root, bar);

        let count = tree.create_node(Widget::Label { text: "3 items selected".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(bar, count);

        let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)),
            NodeStyle::default());
        tree.add_child(bar, spacer);

        let clear = tree.create_node(Widget::Label { text: "Clear selection".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(bar, clear);
    }

    // ── With actions ──
    section_label(tree, root, "With Batch Actions", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(420.0))
            .with_height(LayoutSizing::Fixed(40.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, bar);

        let count = tree.create_node(Widget::Label { text: "12 selected".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(bar, count);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(1.0))
            .with_height(LayoutSizing::Fixed(20.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(bar, sep);

        for &label in &["Export", "Archive", "Delete"] {
            let btn = tree.create_node(Widget::Button {
                label: label.to_string(), pressed: false, hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(26.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                NodeStyle { corner_radii: [4.0; 4], text_color: Some(text_primary), text_size: Some(11.0), focusable: true, ..NodeStyle::default() });
            tree.add_child(bar, btn);
        }

        let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)),
            NodeStyle::default());
        tree.add_child(bar, spacer);

        let clear = tree.create_node(Widget::Label { text: "✕".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(bar, clear);
    }

    // ── Single selection ──
    section_label(tree, root, "Single Selection", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, bar);

        let count = tree.create_node(Widget::Label { text: "1 item selected".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(bar, count);

        let clear = tree.create_node(Widget::Label { text: "Clear".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(bar, clear);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
