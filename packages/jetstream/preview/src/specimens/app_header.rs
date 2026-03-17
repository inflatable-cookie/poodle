//! AppHeader specimen — application header with logo, navigation, and action slots.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 16.0, ..UiStyle::default()
    });

    section_label(tree, root, "Standard App Header", text_secondary);
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(520.0), height: Sizing::Fixed(40.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4], gap: 16.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        let logo = tree.create(Widget::Label { text: "◆ Pug Studio".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(header, logo);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(1.0), height: Sizing::Fixed(20.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(header, sep);

        for &(nav, active) in &[("Projects", true), ("Assets", false), ("Deploy", false)] {
            let lbl = tree.create(Widget::Label { text: nav.to_string() }, UiStyle {
                text_color: Some(if active { text_primary } else { text_secondary }),
                text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(header, lbl);
        }

        let spacer = tree.create(Widget::Panel, UiStyle { width: Sizing::Grow(1.0), ..UiStyle::default() });
        tree.add_child(header, spacer);

        // Search trigger
        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 4.0,
            height: Sizing::Fixed(26.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            border_color: Some(border), border_width: 1.0,
            corner_radii: [4.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(header, search);

        let search_lbl = tree.create(Widget::Label { text: "⌘K Search…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(search, search_lbl);

        // Avatar
        let avatar = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(24.0), height: Sizing::Fixed(24.0),
            corner_radii: [12.0; 4], background: Some(accent),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(header, avatar);

        let initials = tree.create(Widget::Label { text: "AC".to_string() }, UiStyle {
            text_color: Some(text_inverse), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(avatar, initials);
    }

    section_label(tree, root, "Compact Header", text_secondary);
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(520.0), height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4], gap: 12.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        let logo = tree.create(Widget::Label { text: "◆".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(header, logo);

        for &nav in &["Home", "Edit", "View", "Help"] {
            let lbl = tree.create(Widget::Label { text: nav.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(header, lbl);
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
