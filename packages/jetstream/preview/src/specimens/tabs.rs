//! Tabs specimen — tabbed interface with underline and pill variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Underline tabs ──
    section_label(tree, root, "Underline Tabs", text_secondary);
    {
        let tabs = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(0.0)),
            NodeStyle { border_color: Some(border), border_width: 1.0, ..NodeStyle::default() });
        tree.add_child(root, tabs);

        for (i, &label) in ["General", "Security", "Notifications", "Billing"].iter().enumerate() {
            let is_active = i == 0;
            let color = if is_active { accent } else { text_secondary };

            let tab = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_padding(LayoutEdges { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(tabs, tab);

            let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(tab, lbl);

            if is_active {
                let underline = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(2.0))),
                    NodeStyle { background: Some(accent), corner_radii: [1.0; 4], ..NodeStyle::default() });
                tree.add_child(tab, underline);
            }
        }
    }

    // ── Pill tabs ──
    section_label(tree, root, "Pill Tabs", text_secondary);
    {
        let tabs = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(4.0)
            .with_padding(LayoutEdges::uniform(4.0))),
            NodeStyle {
                background: Some(bg_surface),
                border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, tabs);

        for (i, &label) in ["All", "Active", "Archived"].iter().enumerate() {
            let is_active = i == 0;
            let bg = if is_active { Some(theme_bridge::elevated_background(theme)) } else { None };
            let color = if is_active { text_primary } else { text_secondary };

            let tab = tree.create_node(Widget::Button {
                label: label.to_string(), pressed: false, hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(28.0))
                .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                NodeStyle {
                    corner_radii: [6.0; 4], background: bg,
                    text_color: Some(color), text_size: Some(12.0),
                    focusable: true, ..NodeStyle::default()
                });
            tree.add_child(tabs, tab);
        }
    }

    // ── With count badges ──
    section_label(tree, root, "With Count Badges", text_secondary);
    {
        let tabs = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(0.0)),
            NodeStyle { border_color: Some(border), border_width: 1.0, ..NodeStyle::default() });
        tree.add_child(root, tabs);

        for &(label, count, active) in &[("Open", "12", true), ("Closed", "48", false), ("All", "60", false)] {
            let color = if active { accent } else { text_secondary };

            let tab = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(6.0)
                .with_padding(LayoutEdges { top: 8.0, right: 14.0, bottom: 8.0, left: 14.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(tabs, tab);

            let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(tab, lbl);

            let badge = tree.create_node(Widget::Label { text: count.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_padding(LayoutEdges { top: 1.0, right: 6.0, bottom: 1.0, left: 6.0 })),
                NodeStyle {
                    corner_radii: [8.0; 4], background: Some(border),
                    text_color: Some(text_secondary), text_size: Some(9.0),
                    ..NodeStyle::default()
                });
            tree.add_child(tab, badge);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
