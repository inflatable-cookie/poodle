//! FilterToolbar specimen — search and filter chip controls.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Standard filter toolbar ──
    section_label(tree, root, "Standard Toolbar", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(480.0))
            .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, bar);

        // Search
        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_width(LayoutSizing::Grow)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(bar, search);

        let search_lbl = tree.create_node(Widget::Label { text: "🔍 Filter…".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(search, search_lbl);

        // Filter chips
        filter_chip(tree, bar, "Type ▾", false, accent, text_inverse, text_primary, bg_surface, border);
        filter_chip(tree, bar, "Status ▾", false, accent, text_inverse, text_primary, bg_surface, border);
    }

    // ── With active filters ──
    section_label(tree, root, "With Active Filters", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(8.0)
            .with_width(LayoutSizing::Fixed(480.0))
            .with_padding(LayoutEdges::uniform(12.0))),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, bar);

        // Search row
        let search_row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(bar, search_row);

        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_width(LayoutSizing::Grow)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(search_row, search);

        let search_lbl = tree.create_node(Widget::Label { text: "🔍 project brief".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(search, search_lbl);

        filter_chip(tree, search_row, "Type ▾", false, accent, text_inverse, text_primary, bg_surface, border);
        filter_chip(tree, search_row, "Active ✕", true, accent, text_inverse, text_primary, bg_surface, border);

        // Active filter tags
        let tags = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(bar, tags);

        let active_label = tree.create_node(Widget::Label { text: "Active filters:".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(tags, active_label);

        for &tag in &["Status: Active ✕", "Type: Document ✕"] {
            let chip = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(22.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle { background: Some(theme_bridge::tint(accent, 0.12)), corner_radii: [4.0; 4], ..NodeStyle::default() });
            tree.add_child(tags, chip);

            let t = tree.create_node(Widget::Label { text: tag.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(accent), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(chip, t);
        }

        let clear = tree.create_node(Widget::Label { text: "Clear all".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(tags, clear);
    }

    root
}

fn filter_chip(tree: &mut UiTree, parent: UiNodeId, label: &str, active: bool, accent: glam::Vec4, active_fg: glam::Vec4, fg: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let (chip_bg, chip_border, color) = if active {
        (Some(accent), accent, active_fg)
    } else {
        (Some(bg), border, fg)
    };

    let btn = tree.create_node(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(28.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle { corner_radii: [6.0; 4], background: chip_bg, border_color: Some(chip_border), border_width: 1.0, text_color: Some(color), text_size: Some(11.0), focusable: true, ..NodeStyle::default() });
    tree.add_child(parent, btn);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
