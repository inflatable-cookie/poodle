//! CommandPaletteShell specimen — manages open/close activation state.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(20.0)), NodeStyle::default());

    // ── Closed state ──
    section_label(tree, root, "Closed (trigger shown)", text_secondary);
    {
        let trigger = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(6.0).with_width(LayoutSizing::Fixed(200.0)).with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 }).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, trigger);

        let icon = tree.create_node(Widget::Label { text: "🔍".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(trigger, icon);

        let lbl = tree.create_node(Widget::Label { text: "Search…".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Grow)), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(11.0),
            ..NodeStyle::default()
        });
        tree.add_child(trigger, lbl);

        let kbd = tree.create_node(Widget::Label { text: "⌘K".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default()
        });
        tree.add_child(trigger, kbd);
    }

    // ── Open state (with backdrop) ──
    section_label(tree, root, "Open (with backdrop)", text_secondary);
    {
        let backdrop = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Fixed(400.0)).with_height(LayoutSizing::Fixed(240.0)).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Start).with_padding(LayoutEdges { top: 24.0, right: 40.0, bottom: 0.0, left: 40.0 })), NodeStyle {
            background: Some(glam::Vec4::new(bg_canvas.x * 0.5, bg_canvas.y * 0.5, bg_canvas.z * 0.5, 0.8)),
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, backdrop);

        let palette = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Fixed(320.0))), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [10.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(backdrop, palette);

        let search = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_gap(8.0).with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 }).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(palette, search);

        let icon = tree.create_node(Widget::Label { text: "🔍".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_size: Some(13.0), ..NodeStyle::default()
        });
        tree.add_child(search, icon);

        let input = tree.create_node(Widget::Label { text: "new fi".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(search, input);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))), NodeStyle {
            background: Some(border), ..NodeStyle::default()
        });
        tree.add_child(palette, sep);

        let items = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_padding(LayoutEdges::uniform(6.0)).with_gap(2.0)), NodeStyle::default());
        tree.add_child(palette, items);

        for &(cmd, active) in &[("New File", true), ("New Folder", false)] {
            let bg = if active { Some(theme_bridge::tint(accent, 0.10)) } else { None };
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(28.0)).with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 }).with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
                background: bg, corner_radii: [4.0; 4],
                ..NodeStyle::default()
            });
            tree.add_child(items, row);

            let lbl = tree.create_node(Widget::Label { text: cmd.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(if active { accent } else { text_primary }),
                text_size: Some(12.0), ..NodeStyle::default()
            });
            tree.add_child(row, lbl);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
