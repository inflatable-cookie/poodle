//! BulkActionBar specimen — batch action bar for multi-select operations.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Standard ──
    section_label(tree, root, "Standard (3 selected)", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(400.0),
            height: Sizing::Fixed(44.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            gap: 8.0,
            background: Some(accent),
            corner_radius: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        let count = tree.create(Widget::Label { text: "3 selected".to_string() }, UiStyle {
            text_color: Some(text_inverse), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(bar, count);

        let spacer = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(bar, spacer);

        bar_btn(tree, bar, "Move", text_inverse);
        bar_btn(tree, bar, "Archive", text_inverse);
        bar_btn(tree, bar, "Delete", text_inverse);

        let close = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_inverse, 0.7)),
            text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(bar, close);
    }

    // ── With danger action ──
    section_label(tree, root, "Outlined with Danger Action", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(400.0),
            height: Sizing::Fixed(44.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            gap: 8.0,
            background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        let count = tree.create(Widget::Label { text: "12 items selected".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(bar, count);

        let spacer = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(bar, spacer);

        let export = tree.create(Widget::Button {
            label: "Export".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radius: 6.0, border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(bar, export);

        let del = tree.create(Widget::Button {
            label: "Delete All".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radius: 6.0, background: Some(danger),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(bar, del);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn bar_btn(tree: &mut UiTree, parent: UiNodeId, label: &str, fg: glam::Vec4) {
    let btn = tree.create(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(28.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radius: 4.0, text_color: Some(fg),
        text_size: Some(11.0), align: Align::Center,
        justify: Justify::Center, focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}
