//! PanelHeader specimen — panel title bar with collapse/close controls.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Standard panel header ──
    section_label(tree, root, "Standard Panel Header", text_secondary);
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(320.0), height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 6.0, gap: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        let title = tree.create(Widget::Label { text: "Properties".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0),
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        // Collapse button
        let collapse = tree.create(Widget::Button {
            label: "\u{25BE}".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(24.0), height: Sizing::Fixed(24.0),
            corner_radius: 4.0, text_color: Some(text_secondary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center, focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(header, collapse);

        // Close button
        let close = tree.create(Widget::Button {
            label: "\u{2715}".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(24.0), height: Sizing::Fixed(24.0),
            corner_radius: 4.0, text_color: Some(text_secondary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center, focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(header, close);
    }

    // ── Collapsed variant ──
    section_label(tree, root, "Collapsed Variant", text_secondary);
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(320.0), height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radius: 6.0, gap: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        // Expand chevron
        let expand = tree.create(Widget::Button {
            label: "\u{25B8}".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(24.0), height: Sizing::Fixed(24.0),
            corner_radius: 4.0, text_color: Some(text_secondary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center, focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(header, expand);

        let title = tree.create(Widget::Label { text: "Properties".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(header, title);
    }

    // ── Panel header with icon and actions ──
    section_label(tree, root, "With Icon and Actions", text_secondary);
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(320.0), height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 6.0, gap: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        // Icon
        let icon = tree.create(Widget::Label { text: "\u{2699}".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(header, icon);

        let title = tree.create(Widget::Label { text: "Inspector".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0),
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        // Action buttons
        for &lbl in &["\u{2795}", "\u{22EF}"] {
            let btn = tree.create(Widget::Button {
                label: lbl.to_string(), pressed: false, hovered: false,
            }, UiStyle {
                width: Sizing::Fixed(24.0), height: Sizing::Fixed(24.0),
                corner_radius: 4.0, text_color: Some(text_secondary), text_size: Some(11.0),
                align: Align::Center, justify: Justify::Center, focusable: true,
                ..UiStyle::default()
            });
            tree.add_child(header, btn);
        }

        // Collapse
        let collapse = tree.create(Widget::Button {
            label: "\u{25BE}".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(24.0), height: Sizing::Fixed(24.0),
            corner_radius: 4.0, text_color: Some(text_secondary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center, focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(header, collapse);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
