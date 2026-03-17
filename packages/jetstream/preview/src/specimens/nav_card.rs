//! NavCard specimen — navigation-oriented card with icon, title, and description.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    section_label(tree, root, "Navigation Cards", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, width: Sizing::Fixed(300.0),
            ..UiStyle::default()
        });
        tree.add_child(root, col);

        for &(icon, title, desc) in &[
            ("⚙", "Settings", "Manage your account preferences"),
            ("📊", "Analytics", "View usage statistics and reports"),
            ("🔒", "Security", "Configure authentication and permissions"),
        ] {
            nav_card(tree, col, icon, title, desc, accent, text_primary, text_secondary, bg_elevated, border);
        }
    }

    section_label(tree, root, "Compact Variant", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, width: Sizing::Fixed(260.0),
            ..UiStyle::default()
        });
        tree.add_child(root, col);

        for &(title, desc) in &[("Quick Start", "Get started in minutes"), ("API Docs", "Full reference documentation")] {
            let card = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                width: Sizing::Grow(1.0),
                padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
                gap: 8.0, background: Some(bg_elevated),
                border_color: Some(border), border_width: 1.0,
                corner_radii: [6.0; 4], align: Align::Center,
                justify: Justify::SpaceBetween,
                ..UiStyle::default()
            });
            tree.add_child(col, card);

            let info = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, gap: 2.0, ..UiStyle::default()
            });
            tree.add_child(card, info);

            let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(info, t);

            let d = tree.create(Widget::Label { text: desc.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(info, d);

            let arrow = tree.create(Widget::Label { text: "→".to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(14.0), ..UiStyle::default()
            });
            tree.add_child(card, arrow);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn nav_card(tree: &mut UiTree, parent: UiNodeId, icon: &str, title: &str, desc: &str, accent: glam::Vec4, fg: glam::Vec4, muted: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let card = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        padding: Edges::all(14.0), gap: 12.0,
        background: Some(bg), border_color: Some(border),
        border_width: 1.0, corner_radii: [8.0; 4],
        align: Align::Start, ..UiStyle::default()
    });
    tree.add_child(parent, card);

    let ic = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(32.0), height: Sizing::Fixed(32.0),
        corner_radii: [6.0; 4], background: Some(theme_bridge::tint(accent, 0.12)),
        align: Align::Center, justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(card, ic);

    let sym = tree.create(Widget::Label { text: icon.to_string() }, UiStyle {
        text_size: Some(14.0), ..UiStyle::default()
    });
    tree.add_child(ic, sym);

    let info = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 2.0,
        width: Sizing::Grow(1.0), ..UiStyle::default()
    });
    tree.add_child(card, info);

    let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(13.0), ..UiStyle::default()
    });
    tree.add_child(info, t);

    let d = tree.create(Widget::Label { text: desc.to_string() }, UiStyle {
        text_color: Some(muted), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(info, d);

    let arrow = tree.create(Widget::Label { text: "→".to_string() }, UiStyle {
        text_color: Some(muted), text_size: Some(14.0), ..UiStyle::default()
    });
    tree.add_child(card, arrow);
}
