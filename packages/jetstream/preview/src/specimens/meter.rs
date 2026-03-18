//! Meter specimen — gauge-like display for bounded values (storage, quota, etc.).

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutIntent, LayoutSizing};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)), NodeStyle::default());

    // ── Standard ──
    label(tree, root, "Standard (60%)", text_secondary);
    meter_bar(tree, root, 0.6, 240.0, accent, border, text_primary, text_secondary, "60 / 100 GB");

    // ── Low usage ──
    label(tree, root, "Low usage (25%)", text_secondary);
    meter_bar(tree, root, 0.25, 240.0, success, border, text_primary, text_secondary, "25 / 100 GB");

    // ── High usage (warning) ──
    label(tree, root, "High usage — warning (80%)", text_secondary);
    meter_bar(tree, root, 0.8, 240.0, warning, border, text_primary, text_secondary, "80 / 100 GB");

    // ── Critical (danger) ──
    label(tree, root, "Critical — danger (95%)", text_secondary);
    meter_bar(tree, root, 0.95, 240.0, danger, border, text_primary, text_secondary, "95 / 100 GB");

    // ── Empty ──
    label(tree, root, "Empty (0%)", text_secondary);
    meter_bar(tree, root, 0.0, 240.0, accent, border, text_primary, text_secondary, "0 / 100 GB");

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn meter_bar(
    tree: &mut UiTree,
    parent: UiNodeId,
    fraction: f32,
    width: f32,
    fill_color: glam::Vec4,
    bg_color: glam::Vec4,
    _text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    usage_text: &str,
) {
    let container = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_gap(4.0)), NodeStyle::default());
    tree.add_child(parent, container);

    let track = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(width))
        .with_height(LayoutSizing::Fixed(8.0))
        .with_direction(LayoutDirection::Row)), NodeStyle {
        corner_radii: [4.0; 4],
        background: Some(bg_color),
        ..NodeStyle::default()
    });
    tree.add_child(container, track);

    let fill_w = width * fraction.clamp(0.0, 1.0);
    if fill_w > 0.0 {
        let fill = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(fill_w))
            .with_height(LayoutSizing::Fixed(8.0))), NodeStyle {
            corner_radii: [4.0; 4],
            background: Some(fill_color),
            ..NodeStyle::default()
        });
        tree.add_child(track, fill);
    }

    let usage = tree.create_node(Widget::Label { text: usage_text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default()
    });
    tree.add_child(container, usage);
}
