//! RelationPicker specimen — searchable related entity picker.

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

    // ── With selected entities ──
    section_label(tree, root, "With Selected Relations", text_secondary);
    {
        let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(8.0)
            .with_width(LayoutSizing::Fixed(320.0))),
            NodeStyle::default());
        tree.add_child(root, wrapper);

        // Label
        let label = tree.create_node(Widget::Label { text: "Related Projects".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(wrapper, label);

        // Selected tags
        let tags = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)),
            NodeStyle::default());
        tree.add_child(wrapper, tags);

        for &name in &["Alpha Project", "Beta Launch"] {
            let tag = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(4.0)
                .with_height(LayoutSizing::Fixed(24.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    background: Some(theme_bridge::tint(accent, 0.12)),
                    corner_radii: [4.0; 4],
                    ..NodeStyle::default()
                });
            tree.add_child(tags, tag);

            let n = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(accent), text_size: Some(11.0), ..NodeStyle::default() });
            tree.add_child(tag, n);

            let x = tree.create_node(Widget::Label { text: "\u{2715}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(theme_bridge::tint(accent, 0.6)), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(tag, x);
        }

        // Search input
        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(wrapper, search);

        let search_lbl = tree.create_node(Widget::Label { text: "\u{1f50d} Search projects\u{2026}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(search, search_lbl);

        // Dropdown results
        let dropdown = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border), border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(wrapper, dropdown);

        for &(name, desc) in &[
            ("Gamma Initiative", "Marketing \u{00b7} 3 members"),
            ("Delta Core", "Engineering \u{00b7} 8 members"),
            ("Epsilon Research", "Research \u{00b7} 2 members"),
        ] {
            let item = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
                .with_gap(2.0)),
                NodeStyle::default());
            tree.add_child(dropdown, item);

            let n = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(item, n);

            let d = tree.create_node(Widget::Label { text: desc.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(item, d);
        }
    }

    // ── Empty ──
    section_label(tree, root, "Empty Picker", text_secondary);
    {
        let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(8.0)
            .with_width(LayoutSizing::Fixed(320.0))),
            NodeStyle::default());
        tree.add_child(root, wrapper);

        let label = tree.create_node(Widget::Label { text: "Assigned To".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(wrapper, label);

        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(wrapper, search);

        let search_lbl = tree.create_node(Widget::Label { text: "\u{1f50d} Search people\u{2026}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(search, search_lbl);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
