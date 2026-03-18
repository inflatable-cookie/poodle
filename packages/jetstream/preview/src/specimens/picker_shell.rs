//! PickerShell specimen — search-and-select picker overlay.

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

    // ── Standard picker ──
    section_label(tree, root, "Standard Picker", text_secondary);
    {
        let picker = picker_frame(tree, bg_elevated, border);
        tree.add_child(root, picker);

        // Search input
        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(picker, search);

        let search_lbl = tree.create_node(Widget::Label { text: "\u{1f50d} Search items\u{2026}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(search, search_lbl);

        // List
        for &(name, selected) in &[
            ("Apple", false),
            ("Banana", true),
            ("Cherry", false),
            ("Date", false),
            ("Elderberry", false),
        ] {
            let item = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    background: if selected { Some(theme_bridge::tint(accent, 0.08)) } else { None },
                    ..NodeStyle::default()
                });
            tree.add_child(picker, item);

            let check = tree.create_node(Widget::Label {
                text: if selected { "\u{2713}" } else { " " }.to_string(),
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(16.0))),
                NodeStyle { text_color: Some(accent), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(item, check);

            let lbl = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle {
                    text_color: Some(if selected { accent } else { text_primary }),
                    text_size: Some(12.0), ..NodeStyle::default()
                });
            tree.add_child(item, lbl);
        }
    }

    // ── With filter applied ──
    section_label(tree, root, "Filtered Results", text_secondary);
    {
        let picker = picker_frame(tree, bg_elevated, border);
        tree.add_child(root, picker);

        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(picker, search);

        let search_lbl = tree.create_node(Widget::Label { text: "\u{1f50d} ber".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(search, search_lbl);

        for &name in &["Blueberry", "Elderberry", "Strawberry"] {
            let item = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })),
                NodeStyle::default());
            tree.add_child(picker, item);

            let lbl = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(item, lbl);
        }
    }

    root
}

fn picker_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(260.0))),
        NodeStyle {
            background: Some(bg),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        })
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
