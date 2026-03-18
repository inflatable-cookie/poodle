//! FileUpload specimen — drop zone and file selection area.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    // ── Empty drop zone ──
    label(tree, root, "Drop Zone (empty)", text_secondary);
    {
        let zone = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(300.0))
            .with_height(LayoutSizing::Fixed(120.0))
            .with_padding(LayoutEdges::uniform(16.0))
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, zone);

        let icon = tree.create_node(Widget::Label { text: "⬆".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(20.0), ..NodeStyle::default() });
        tree.add_child(zone, icon);

        let msg = tree.create_node(Widget::Label { text: "Drop files here or click to browse".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(zone, msg);

        let hint = tree.create_node(Widget::Label { text: "PNG, JPG up to 10MB".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(theme_bridge::tint(text_secondary, 0.6)), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(zone, hint);
    }

    // ── With file selected ──
    label(tree, root, "File Selected", text_secondary);
    {
        let zone = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(300.0))
            .with_padding(LayoutEdges::uniform(12.0))
            .with_gap(8.0)),
            NodeStyle { background: Some(bg_surface), border_color: Some(success), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, zone);

        // File entry
        let file_row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(zone, file_row);

        let file_icon = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(32.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [4.0; 4], background: Some(border_subtle), ..NodeStyle::default() });
        tree.add_child(file_row, file_icon);

        let fi = tree.create_node(Widget::Label { text: "📄".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(file_icon, fi);

        let info = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_gap(2.0)),
            NodeStyle::default());
        tree.add_child(file_row, info);

        let fname = tree.create_node(Widget::Label { text: "screenshot.png".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(info, fname);

        let fsize = tree.create_node(Widget::Label { text: "2.4 MB".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(info, fsize);

        let remove = tree.create_node(Widget::Label { text: "✕".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(file_row, remove);
    }

    // ── Compact button variant ──
    label(tree, root, "Compact (button style)", text_secondary);
    {
        let btn = tree.create_node(Widget::Button {
            label: "Choose File".to_string(),
            pressed: false,
            hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], background: Some(accent), text_color: Some(text_inverse), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(root, btn);
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let zone = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(300.0))
            .with_height(LayoutSizing::Fixed(80.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), border_color: Some(border_subtle), border_width: 1.0, corner_radii: [8.0; 4], opacity: 0.5, ..NodeStyle::default() });
        tree.add_child(root, zone);

        let msg = tree.create_node(Widget::Label { text: "Upload disabled".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(zone, msg);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
