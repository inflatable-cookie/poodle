//! ShellStatusBar specimen — status bar with left/center/right slots.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Full status bar ──
    section_label(tree, root, "Full Status Bar", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(520.0))
            .with_height(LayoutSizing::Fixed(24.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
        tree.add_child(root, bar);

        // Left slot — connection status
        let left = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_width(LayoutSizing::Grow)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(bar, left);

        let dot = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(6.0))
            .with_height(LayoutSizing::Fixed(6.0))),
            NodeStyle { corner_radii: [3.0; 4], background: Some(success), ..NodeStyle::default() });
        tree.add_child(left, dot);

        let conn = tree.create_node(Widget::Label { text: "Connected".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(success), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(left, conn);

        // Center slot — project name
        let center = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(bar, center);

        let project = tree.create_node(Widget::Label { text: "my-project".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(center, project);

        // Right slot — fps + version
        let right = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(10.0)
            .with_width(LayoutSizing::Grow)
            .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(bar, right);

        let fps = tree.create_node(Widget::Label { text: "60 fps".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(right, fps);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(1.0))
            .with_height(LayoutSizing::Fixed(12.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(right, sep);

        let version = tree.create_node(Widget::Label { text: "v0.1.0".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(right, version);
    }

    // ── Minimal status bar ──
    section_label(tree, root, "Minimal Status Bar", text_secondary);
    {
        let bar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(520.0))
            .with_height(LayoutSizing::Fixed(20.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [4.0; 4], ..NodeStyle::default() });
        tree.add_child(root, bar);

        let ready = tree.create_node(Widget::Label { text: "Ready".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(bar, ready);

        let info = tree.create_node(Widget::Label { text: "Ln 42, Col 8".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default() });
        tree.add_child(bar, info);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
