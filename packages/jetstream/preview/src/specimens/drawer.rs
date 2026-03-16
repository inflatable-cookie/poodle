//! Drawer specimen — slide-out panel mockup from edge.

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
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Right drawer ──
    section_label(tree, root, "Right Drawer (default)", text_secondary);
    {
        let frame = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(400.0),
            height: Sizing::Fixed(200.0),
            background: Some(theme_bridge::tint(bg_surface, 0.3)),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, frame);

        // Backdrop area
        let backdrop = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0),
            height: Sizing::Grow(1.0),
            align: Align::Center,
            justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(frame, backdrop);

        let hint = tree.create(Widget::Label { text: "Main content".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.4)),
            text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(backdrop, hint);

        // Drawer panel
        let drawer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(200.0),
            height: Sizing::Grow(1.0),
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            ..UiStyle::default()
        });
        tree.add_child(frame, drawer);

        drawer_header(tree, drawer, "Details", text_primary, text_secondary);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 },
            gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(drawer, body);

        for &(lbl, val) in &[("Name", "Item Alpha"), ("Status", "Active"), ("Created", "2 days ago")] {
            detail_pair(tree, body, lbl, val, text_secondary, text_primary);
        }
    }

    // ── Left drawer with navigation ──
    section_label(tree, root, "Left Drawer (navigation)", text_secondary);
    {
        let frame = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(400.0),
            height: Sizing::Fixed(200.0),
            background: Some(theme_bridge::tint(bg_surface, 0.3)),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, frame);

        let drawer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(180.0),
            height: Sizing::Grow(1.0),
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            ..UiStyle::default()
        });
        tree.add_child(frame, drawer);

        drawer_header(tree, drawer, "Navigation", text_primary, text_secondary);

        let nav = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 },
            gap: 2.0,
            ..UiStyle::default()
        });
        tree.add_child(drawer, nav);

        for (i, item) in ["Dashboard", "Projects", "Settings", "Help"].iter().enumerate() {
            let is_active = i == 1;
            let bg = if is_active { Some(theme_bridge::tint(accent, 0.15)) } else { None };
            let color = if is_active { accent } else { text_primary };

            let row = tree.create(Widget::Button {
                label: item.to_string(), pressed: false, hovered: false,
            }, UiStyle {
                width: Sizing::Grow(1.0), height: Sizing::Fixed(28.0),
                padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
                corner_radius: 4.0, background: bg, text_color: Some(color),
                text_size: Some(12.0), align: Align::Center,
                focusable: true, ..UiStyle::default()
            });
            tree.add_child(nav, row);
        }

        let _content = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Grow(1.0),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(frame, _content);

        let hint = tree.create(Widget::Label { text: "Main content".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.4)),
            text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(_content, hint);
    }

    // ── Drawer with footer actions ──
    section_label(tree, root, "Drawer with Actions", text_secondary);
    {
        let drawer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(240.0),
            height: Sizing::Fixed(180.0),
            background: Some(bg_elevated),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, drawer);

        drawer_header(tree, drawer, "Edit Item", text_primary, text_secondary);

        let body = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Grow(1.0),
            padding: Edges::all(16.0),
            ..UiStyle::default()
        });
        tree.add_child(drawer, body);

        let placeholder = tree.create(Widget::Label { text: "Form content here...".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, placeholder);

        // Footer
        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 },
            gap: 8.0,
            justify: Justify::End,
            border_color: Some(border),
            border_width: 1.0,
            ..UiStyle::default()
        });
        tree.add_child(drawer, footer);

        let cancel = tree.create(Widget::Button {
            label: "Cancel".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radius: 6.0, background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, cancel);

        let save = tree.create(Widget::Button {
            label: "Save".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radius: 6.0, background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, save);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn drawer_header(tree: &mut UiTree, parent: UiNodeId, title: &str, fg: glam::Vec4, muted: glam::Vec4) {
    let header = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        padding: Edges { top: 12.0, right: 16.0, bottom: 10.0, left: 16.0 },
        justify: Justify::SpaceBetween,
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, header);

    let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(fg), text_size: Some(14.0), ..UiStyle::default()
    });
    tree.add_child(header, t);

    let close = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
        text_color: Some(muted), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(header, close);
}

fn detail_pair(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, label_color: glam::Vec4, value_color: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 8.0, ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let l = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(label_color), text_size: Some(11.0),
        width: Sizing::Fixed(50.0), ..UiStyle::default()
    });
    tree.add_child(row, l);

    let v = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
        text_color: Some(value_color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(row, v);
}
