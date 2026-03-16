//! WorkspaceShell specimen — full workspace with header, sidebar, content, and status bar.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0,
        ..UiStyle::default()
    });

    section_label(tree, root, "Full Workspace Shell", text_secondary);
    {
        let shell = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(520.0), height: Sizing::Fixed(360.0),
            background: Some(bg_canvas),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, shell);

        // ── App header ──
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(36.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), gap: 12.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, header);

        let logo = tree.create(Widget::Label { text: "◆ Workstation".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(header, logo);

        let spacer = tree.create(Widget::Panel, UiStyle { width: Sizing::Grow(1.0), ..UiStyle::default() });
        tree.add_child(header, spacer);

        for &nav in &["Projects", "Assets", "Settings"] {
            let n = tree.create(Widget::Label { text: nav.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(header, n);
        }

        let header_sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(shell, header_sep);

        // ── Body (sidebar + content) ──
        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0), height: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(shell, body);

        // Sidebar
        let sidebar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(140.0),
            background: Some(bg_elevated),
            padding: Edges::all(8.0), gap: 2.0,
            ..UiStyle::default()
        });
        tree.add_child(body, sidebar);

        for &(item, active) in &[("Dashboard", true), ("Documents", false), ("Media", false), ("Tasks", false)] {
            let row = tree.create(Widget::Panel, UiStyle {
                height: Sizing::Fixed(26.0),
                padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
                background: if active { Some(theme_bridge::tint(accent, 0.12)) } else { None },
                corner_radius: 4.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(sidebar, row);

            let lbl = tree.create(Widget::Label { text: item.to_string() }, UiStyle {
                text_color: Some(if active { accent } else { text_secondary }),
                text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(row, lbl);
        }

        let sidebar_sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(1.0), height: Sizing::Grow(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(body, sidebar_sep);

        // Content
        let content = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0), padding: Edges::all(16.0), gap: 12.0,
            background: Some(bg_surface),
            ..UiStyle::default()
        });
        tree.add_child(body, content);

        let page_title = tree.create(Widget::Label { text: "Dashboard".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(16.0), ..UiStyle::default()
        });
        tree.add_child(content, page_title);

        // Placeholder cards
        let cards = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(content, cards);

        for &(label, value) in &[("Projects", "12"), ("Tasks", "47"), ("Files", "238")] {
            let card = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, padding: Edges::all(10.0), gap: 2.0,
                width: Sizing::Fixed(90.0),
                background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
                corner_radius: 6.0,
                ..UiStyle::default()
            });
            tree.add_child(cards, card);

            let v = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(18.0), ..UiStyle::default()
            });
            tree.add_child(card, v);

            let l = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
            });
            tree.add_child(card, l);
        }

        // ── Status bar ──
        let status_sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(shell, status_sep);

        let status = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(22.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_elevated), gap: 12.0, align: Align::Center,
            justify: Justify::SpaceBetween,
            ..UiStyle::default()
        });
        tree.add_child(shell, status);

        let status_left = tree.create(Widget::Label { text: "● Connected".to_string() }, UiStyle {
            text_color: Some(theme_bridge::resolve_vec4(theme, "semantic.color.status.success")),
            text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(status, status_left);

        let status_right = tree.create(Widget::Label { text: "v0.1.0 · 60 fps".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(status, status_right);
    }

    let _ = text_inverse;
    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
