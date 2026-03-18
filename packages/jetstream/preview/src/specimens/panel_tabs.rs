//! PanelTabs specimen — tabbed panel views for switching between panel content.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Panel with tab bar ──
    section_label(tree, root, "Panel Tabs", text_secondary);
    {
        let panel = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(340.0))
            .with_height(LayoutSizing::Fixed(240.0))),
            NodeStyle {
                background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, panel);

        // Tab bar
        let tab_bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_height(LayoutSizing::Fixed(34.0))
            .with_padding(LayoutEdges { top: 0.0, right: 4.0, bottom: 0.0, left: 4.0 })
            .with_gap(0.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::End)),
            NodeStyle { background: Some(bg_elevated), ..NodeStyle::default() });
        tree.add_child(panel, tab_bar);

        for (i, &label) in ["Properties", "Styles", "Events"].iter().enumerate() {
            let is_active = i == 0;
            let color = if is_active { accent } else { text_secondary };

            let tab = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_padding(LayoutEdges { top: 8.0, right: 14.0, bottom: 6.0, left: 14.0 })
                .with_gap(4.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(tab_bar, tab);

            let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
            tree.add_child(tab, lbl);

            if is_active {
                let underline = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(2.0))),
                    NodeStyle { background: Some(accent), corner_radii: [1.0; 4], ..NodeStyle::default() });
                tree.add_child(tab, underline);
            }
        }

        // Divider
        let div = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(panel, div);

        // Content area (Properties tab active)
        let content = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Grow)
            .with_padding(LayoutEdges::uniform(12.0))
            .with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(panel, content);

        for &(key, val) in &[("id", "btn-submit"), ("label", "Save Changes"), ("variant", "primary"), ("size", "md")] {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_height(LayoutSizing::Fixed(22.0))
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle::default());
            tree.add_child(content, row);

            let k = tree.create_node(Widget::Label { text: key.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(60.0))),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(row, k);

            let v_bg = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(22.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
                    corner_radii: [4.0; 4],
                    ..NodeStyle::default()
                });
            tree.add_child(row, v_bg);

            let v = tree.create_node(Widget::Label { text: val.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(v_bg, v);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
