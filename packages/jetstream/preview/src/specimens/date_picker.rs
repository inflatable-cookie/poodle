//! DatePicker specimen — date selection input with calendar dropdown mockup.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_default(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(16.0)), NodeStyle::default());

    // ── Empty / placeholder ──
    section_label(tree, root, "Empty (placeholder)", text_secondary);
    date_input(tree, root, "Select a date...", bg_canvas, border, text_secondary, 1.0);

    // ── With value ──
    section_label(tree, root, "With Value", text_secondary);
    date_input(tree, root, "March 16, 2026", bg_canvas, border, text_primary, 1.0);

    // ── Focused ──
    section_label(tree, root, "Focused", text_secondary);
    {
        let accent = theme_bridge::accent_base(theme);
        date_input(tree, root, "March 16, 2026", bg_canvas, accent, text_primary, 1.0);
    }

    // ── With label ──
    section_label(tree, root, "With Label", text_secondary);
    {
        let field = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_gap(4.0)), NodeStyle::default());
        tree.add_child(root, field);

        let lbl = tree.create_node(Widget::Label { text: "Start Date".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(field, lbl);

        date_input(tree, field, "March 16, 2026", bg_canvas, border, text_primary, 1.0);
    }

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    date_input(tree, root, "March 16, 2026", bg_canvas, border, text_primary, 0.5);

    // ── Error ──
    section_label(tree, root, "Error State", text_secondary);
    {
        let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
        let field = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_gap(4.0)), NodeStyle::default());
        tree.add_child(root, field);

        date_input(tree, field, "Invalid date", bg_canvas, danger, text_primary, 1.0);

        let err = tree.create_node(Widget::Label { text: "Please enter a valid date.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(danger), text_size: Some(10.0), ..NodeStyle::default()
        });
        tree.add_child(field, err);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn date_input(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, opacity: f32) {
    let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_width(LayoutSizing::Fixed(220.0)).with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 }).with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)), NodeStyle {
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [6.0; 4],
        opacity,
        ..NodeStyle::default()
    });
    tree.add_child(parent, input);

    let val = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(fg), text_size: Some(12.0), ..NodeStyle::default()
    });
    tree.add_child(input, val);

    let icon = tree.create_node(Widget::Label { text: "📅".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_size: Some(12.0), ..NodeStyle::default()
    });
    tree.add_child(input, icon);
}
