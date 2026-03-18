//! ToggleGroup specimen — mutually exclusive toggle buttons.

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
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    // ── View mode toggle ──
    section_label(tree, root, "View Mode", text_secondary);
    {
        let group = toggle_group_container(tree, bg_surface, border);
        tree.add_child(root, group);

        group_item(tree, group, "List", true, accent, text_inverse, text_primary, border);
        group_item(tree, group, "Grid", false, accent, text_inverse, text_primary, border);
        group_item(tree, group, "Table", false, accent, text_inverse, text_primary, border);
    }

    // ── Alignment toggle ──
    section_label(tree, root, "Text Alignment", text_secondary);
    {
        let group = toggle_group_container(tree, bg_surface, border);
        tree.add_child(root, group);

        group_item(tree, group, "\u{25C0}", false, accent, text_inverse, text_primary, border);
        group_item(tree, group, "\u{2261}", true, accent, text_inverse, text_primary, border);
        group_item(tree, group, "\u{25B6}", false, accent, text_inverse, text_primary, border);
    }

    // ── Size toggle ──
    section_label(tree, root, "Size Selection", text_secondary);
    {
        let group = toggle_group_container(tree, bg_surface, border);
        tree.add_child(root, group);

        group_item(tree, group, "S", false, accent, text_inverse, text_primary, border);
        group_item(tree, group, "M", true, accent, text_inverse, text_primary, border);
        group_item(tree, group, "L", false, accent, text_inverse, text_primary, border);
        group_item(tree, group, "XL", false, accent, text_inverse, text_primary, border);
    }

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    {
        let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { opacity: 0.5, ..NodeStyle::default() });
        tree.add_child(root, wrapper);

        let group = toggle_group_container(tree, bg_surface, border);
        tree.add_child(wrapper, group);

        group_item(tree, group, "On", true, accent, text_inverse, text_primary, border);
        group_item(tree, group, "Off", false, accent, text_inverse, text_primary, border);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn toggle_group_container(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_gap(2.0)
        .with_padding(LayoutEdges::uniform(2.0))),
        NodeStyle {
            background: Some(bg),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        })
}

fn group_item(tree: &mut UiTree, parent: UiNodeId, label: &str, active: bool, accent: glam::Vec4, active_fg: glam::Vec4, fg: glam::Vec4, _border: glam::Vec4) {
    let (bg, color) = if active {
        (Some(accent), active_fg)
    } else {
        (None, fg)
    };

    let btn = tree.create_node(Widget::Button {
        label: label.to_string(), pressed: active, hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(28.0))
        .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle {
            corner_radii: [6.0; 4], background: bg,
            text_color: Some(color), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
    tree.add_child(parent, btn);
}
