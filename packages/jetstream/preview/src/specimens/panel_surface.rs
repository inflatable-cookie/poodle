//! PanelSurface specimen — panel with header and scrollable content area.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Panel with header and property rows ──
    section_label(tree, root, "Panel with Content", text_secondary);
    {
        let panel = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(300.0), height: Sizing::Fixed(220.0),
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, panel);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(panel, header);

        let title = tree.create(Widget::Label { text: "Inspector".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0),
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let collapse = tree.create(Widget::Button {
            label: "\u{25BE}".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(24.0), height: Sizing::Fixed(24.0),
            corner_radius: 4.0, text_color: Some(text_secondary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center, focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(header, collapse);

        // Divider
        let div = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(panel, div);

        // Content area with property rows
        let content = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0), height: Sizing::Grow(1.0),
            padding: Edges::all(10.0), gap: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(panel, content);

        for &(key, val) in &[("Name", "Button"), ("Width", "120px"), ("Height", "36px"), ("Variant", "primary"), ("Disabled", "false")] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                height: Sizing::Fixed(22.0),
                gap: 8.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(content, row);

            let k = tree.create(Widget::Label { text: key.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(70.0), ..UiStyle::default()
            });
            tree.add_child(row, k);

            let v = tree.create(Widget::Label { text: val.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(row, v);
        }
    }

    // ── Empty state panel ──
    section_label(tree, root, "Empty State", text_secondary);
    {
        let panel = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(300.0), height: Sizing::Fixed(160.0),
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, panel);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(panel, header);

        let title = tree.create(Widget::Label { text: "Details".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        // Divider
        let div = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(panel, div);

        // Empty content
        let empty = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Grow(1.0),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(panel, empty);

        let msg = tree.create(Widget::Label { text: "No item selected".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(empty, msg);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
