//! Combobox specimen — text input with filterable dropdown.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(16.0)), NodeStyle::default());

    // ── Closed with value ──
    section_label(tree, root, "With Value", text_secondary);
    combo_input(tree, root, "United States", bg_canvas, border, text_primary, 1.0);

    // ── Placeholder ──
    section_label(tree, root, "Placeholder", text_secondary);
    combo_input(tree, root, "Search countries...", bg_canvas, border, text_secondary, 1.0);

    // ── Open with filtered results ──
    section_label(tree, root, "Open with Filter", text_secondary);
    {
        let col = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_gap(4.0)), NodeStyle::default());
        tree.add_child(root, col);

        combo_input(tree, col, "Uni", bg_canvas, accent, text_primary, 1.0);

        let dropdown = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Fixed(240.0)).with_padding(LayoutEdges::uniform(4.0)).with_gap(1.0)), NodeStyle {
            background: Some(bg_elevated),
            border_color: Some(border_subtle), border_width: 1.0,
            corner_radii: [8.0; 4], ..NodeStyle::default()
        });
        tree.add_child(col, dropdown);

        for &(item, highlighted) in &[
            ("United States", true),
            ("United Kingdom", false),
            ("United Arab Emirates", false),
        ] {
            let bg = if highlighted { Some(theme_bridge::tint(accent, 0.12)) } else { None };
            let color = if highlighted { accent } else { text_primary };

            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(28.0)).with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 }).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
                corner_radii: [4.0; 4], background: bg,
                ..NodeStyle::default()
            });
            tree.add_child(dropdown, row);

            let lbl = tree.create_node(Widget::Label { text: item.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default()
            });
            tree.add_child(row, lbl);
        }
    }

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    combo_input(tree, root, "United States", bg_canvas, border, text_primary, 0.5);

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn combo_input(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, opacity: f32) {
    let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_width(LayoutSizing::Fixed(240.0)).with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 }).with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)), NodeStyle {
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radii: [6.0; 4],
        opacity, ..NodeStyle::default()
    });
    tree.add_child(parent, input);

    let val = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(fg), text_size: Some(12.0), ..NodeStyle::default()
    });
    tree.add_child(input, val);

    let chevron = tree.create_node(Widget::Label { text: "▾".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(fg), text_size: Some(12.0), ..NodeStyle::default()
    });
    tree.add_child(input, chevron);
}
