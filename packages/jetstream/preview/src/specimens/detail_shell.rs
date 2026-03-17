//! DetailShell specimen — full detail page layout with header and content sections.

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
        gap: 24.0,
        ..UiStyle::default()
    });

    // ── Standard detail shell ──
    section_label(tree, root, "Standard Detail Shell", text_secondary);
    {
        let shell = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(440.0),
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, shell);

        // Page header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 16.0, right: 16.0, bottom: 16.0, left: 16.0 },
            justify: Justify::SpaceBetween, align: Align::Center,
            background: Some(bg_surface),
            ..UiStyle::default()
        });
        tree.add_child(shell, header);

        let header_left = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(header, header_left);

        let breadcrumb = tree.create(Widget::Label { text: "Documents > Project Brief".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(header_left, breadcrumb);

        let title = tree.create(Widget::Label { text: "Project Brief".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(18.0), ..UiStyle::default()
        });
        tree.add_child(header_left, title);

        let actions = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(header, actions);

        let edit = tree.create(Widget::Button {
            label: "Edit".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(actions, edit);

        let more = tree.create(Widget::Button {
            label: "⋯".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(28.0), height: Sizing::Fixed(28.0),
            corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(14.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(actions, more);

        // Separator
        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(shell, sep);

        // Content sections
        let content = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(16.0), gap: 16.0,
            ..UiStyle::default()
        });
        tree.add_child(shell, content);

        // Metadata section
        detail_field(tree, content, "Status", "Draft", text_secondary, text_primary);
        detail_field(tree, content, "Created", "March 12, 2026", text_secondary, text_primary);
        detail_field(tree, content, "Author", "Alice Chen", text_secondary, text_primary);
        detail_field(tree, content, "Tags", "design, planning", text_secondary, accent);
    }

    root
}

fn detail_field(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, label_color: glam::Vec4, value_color: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 12.0, align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let l = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(label_color), text_size: Some(11.0),
        width: Sizing::Fixed(80.0),
        ..UiStyle::default()
    });
    tree.add_child(row, l);

    let v = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
        text_color: Some(value_color), text_size: Some(12.0), ..UiStyle::default()
    });
    tree.add_child(row, v);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
