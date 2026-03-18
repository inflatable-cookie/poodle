//! ZonedDateTimePicker specimen — date-time picker with timezone display.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_default(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    // ── With value and timezone ──
    section_label(tree, root, "With Value", text_secondary);
    zoned_input(tree, root, "Mar 16, 2026  2:30 PM", "EST (UTC-5)", bg_canvas, border, text_primary, text_secondary, 1.0);

    // ── Placeholder ──
    section_label(tree, root, "Placeholder", text_secondary);
    zoned_input(tree, root, "Select date and time...", "Timezone", bg_canvas, border, text_secondary, text_secondary, 1.0);

    // ── Different timezone ──
    section_label(tree, root, "Different Timezone", text_secondary);
    zoned_input(tree, root, "Mar 17, 2026  3:30 AM", "JST (UTC+9)", bg_canvas, border, text_primary, text_secondary, 1.0);

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    zoned_input(tree, root, "Mar 16, 2026  2:30 PM", "EST (UTC-5)", bg_canvas, border, text_primary, text_secondary, 0.5);

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn zoned_input(tree: &mut UiTree, parent: UiNodeId, datetime: &str, tz: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, muted: glam::Vec4, opacity: f32) {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_gap(8.0)
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { opacity, ..NodeStyle::default() });
    tree.add_child(parent, row);

    // DateTime field
    let dt_input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Fixed(220.0))
        .with_height(LayoutSizing::Fixed(32.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
        NodeStyle {
            background: Some(bg), border_color: Some(border),
            border_width: 1.0, corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
    tree.add_child(row, dt_input);

    let val = tree.create_node(Widget::Label { text: datetime.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(fg), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(dt_input, val);

    let icon = tree.create_node(Widget::Label { text: "\u{1F4C5}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(dt_input, icon);

    // Timezone badge
    let tz_badge = tree.create_node(Widget::Label { text: tz.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_padding(LayoutEdges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 })),
        NodeStyle {
            corner_radii: [4.0; 4], background: Some(bg),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(muted), text_size: Some(10.0),
            ..NodeStyle::default()
        });
    tree.add_child(row, tz_badge);
}
