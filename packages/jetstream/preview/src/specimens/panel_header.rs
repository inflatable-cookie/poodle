//! PanelHeader specimen — panel title bar with collapse/close controls.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    // ── Standard panel header ──
    section_label(tree, root, "Standard Panel Header", text_secondary);
    {
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(320.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 12.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, header);

        let title = tree.create_node(Widget::Label { text: "Properties".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)), NodeStyle {
            text_color: Some(text_primary), text_size: Some(12.0),
            ..NodeStyle::default()
        });
        tree.add_child(header, title);

        // Collapse button
        let collapse = tree.create_node(Widget::Button {
            label: "\u{25BE}".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(24.0))
            .with_height(LayoutSizing::Fixed(24.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [4.0; 4], text_color: Some(text_secondary), text_size: Some(12.0),
            focusable: true,
            ..NodeStyle::default()
        });
        tree.add_child(header, collapse);

        // Close button
        let close = tree.create_node(Widget::Button {
            label: "\u{2715}".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(24.0))
            .with_height(LayoutSizing::Fixed(24.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [4.0; 4], text_color: Some(text_secondary), text_size: Some(12.0),
            focusable: true,
            ..NodeStyle::default()
        });
        tree.add_child(header, close);
    }

    // ── Collapsed variant ──
    section_label(tree, root, "Collapsed Variant", text_secondary);
    {
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(320.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 12.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, header);

        // Expand chevron
        let expand = tree.create_node(Widget::Button {
            label: "\u{25B8}".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(24.0))
            .with_height(LayoutSizing::Fixed(24.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [4.0; 4], text_color: Some(text_secondary), text_size: Some(12.0),
            focusable: true,
            ..NodeStyle::default()
        });
        tree.add_child(header, expand);

        let title = tree.create_node(Widget::Label { text: "Properties".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(header, title);
    }

    // ── Panel header with icon and actions ──
    section_label(tree, root, "With Icon and Actions", text_secondary);
    {
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(320.0))
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 10.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle {
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        });
        tree.add_child(root, header);

        // Icon
        let icon = tree.create_node(Widget::Label { text: "\u{2699}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(accent), text_size: Some(13.0), ..NodeStyle::default()
        });
        tree.add_child(header, icon);

        let title = tree.create_node(Widget::Label { text: "Inspector".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)), NodeStyle {
            text_color: Some(text_primary), text_size: Some(12.0),
            ..NodeStyle::default()
        });
        tree.add_child(header, title);

        // Action buttons
        for &lbl in &["\u{2795}", "\u{22EF}"] {
            let btn = tree.create_node(Widget::Button {
                label: lbl.to_string(), pressed: false, hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(24.0))
                .with_height(LayoutSizing::Fixed(24.0))
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
                corner_radii: [4.0; 4], text_color: Some(text_secondary), text_size: Some(11.0),
                focusable: true,
                ..NodeStyle::default()
            });
            tree.add_child(header, btn);
        }

        // Collapse
        let collapse = tree.create_node(Widget::Button {
            label: "\u{25BE}".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(24.0))
            .with_height(LayoutSizing::Fixed(24.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [4.0; 4], text_color: Some(text_secondary), text_size: Some(12.0),
            focusable: true,
            ..NodeStyle::default()
        });
        tree.add_child(header, collapse);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
