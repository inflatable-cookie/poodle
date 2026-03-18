//! PageLoading specimen — full-viewport loading with skeleton or spinner.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    // ── Spinner ──
    section_label(tree, root, "Spinner Loading", text_secondary);
    {
        let container = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(300.0))
            .with_height(LayoutSizing::Fixed(160.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, container);

        let inner = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(12.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(container, inner);

        let spinner = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(32.0))
            .with_height(LayoutSizing::Fixed(32.0))), NodeStyle {
            corner_radii: [16.0; 4],
            border_color: Some(accent), border_width: 3.0,
            ..NodeStyle::default()
        });
        tree.add_child(inner, spinner);

        let msg = tree.create_node(Widget::Label { text: "Loading…".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(inner, msg);
    }

    // ── Skeleton ──
    section_label(tree, root, "Skeleton Loading", text_secondary);
    {
        let container = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(12.0)
            .with_width(LayoutSizing::Fixed(300.0))
            .with_padding(LayoutEdges::uniform(16.0))), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, container);

        let skel_color = theme_bridge::tint(border, 0.5);

        // Header skeleton
        let h = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(180.0))
            .with_height(LayoutSizing::Fixed(16.0))), NodeStyle {
            background: Some(skel_color), corner_radii: [4.0; 4], ..NodeStyle::default()
        });
        tree.add_child(container, h);

        // Text lines
        for &w in &[268.0, 240.0, 200.0] {
            let line = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(w))
                .with_height(LayoutSizing::Fixed(10.0))), NodeStyle {
                background: Some(skel_color), corner_radii: [3.0; 4], ..NodeStyle::default()
            });
            tree.add_child(container, line);
        }

        // Card skeleton
        let card_skel = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(60.0))), NodeStyle {
            background: Some(skel_color), corner_radii: [6.0; 4], ..NodeStyle::default()
        });
        tree.add_child(container, card_skel);
    }

    // ── Progress bar ──
    section_label(tree, root, "Progress Bar Loading", text_secondary);
    {
        let container = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(8.0)
            .with_width(LayoutSizing::Fixed(300.0))
            .with_padding(LayoutEdges::uniform(16.0))
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, container);

        let track = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(268.0))
            .with_height(LayoutSizing::Fixed(4.0))), NodeStyle {
            background: Some(theme_bridge::tint(border, 0.5)),
            corner_radii: [2.0; 4], ..NodeStyle::default()
        });
        tree.add_child(container, track);

        let fill = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(160.0))
            .with_height(LayoutSizing::Fixed(4.0))), NodeStyle {
            background: Some(accent), corner_radii: [2.0; 4], ..NodeStyle::default()
        });
        tree.add_child(track, fill);

        let pct = tree.create_node(Widget::Label { text: "Loading 60%…".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default()
        });
        tree.add_child(container, pct);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
