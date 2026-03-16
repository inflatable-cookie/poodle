//! ZonedDateTimePicker specimen — date-time picker with timezone display.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_default(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

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
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn zoned_input(tree: &mut UiTree, parent: UiNodeId, datetime: &str, tz: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, muted: glam::Vec4, opacity: f32) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 8.0, align: Align::Center, opacity,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    // DateTime field
    let dt_input = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Fixed(220.0), height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radius: 6.0,
        align: Align::Center, justify: Justify::SpaceBetween,
        ..UiStyle::default()
    });
    tree.add_child(row, dt_input);

    let val = tree.create(Widget::Label { text: datetime.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(dt_input, val);

    let icon = tree.create(Widget::Label { text: "📅".to_string() }, UiStyle {
        text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(dt_input, icon);

    // Timezone badge
    let tz_badge = tree.create(Widget::Label { text: tz.to_string() }, UiStyle {
        padding: Edges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 },
        corner_radius: 4.0, background: Some(bg),
        border_color: Some(border), border_width: 1.0,
        text_color: Some(muted), text_size: Some(10.0),
        ..UiStyle::default()
    });
    tree.add_child(row, tz_badge);
}
