//! FileUpload specimen — drop zone and file selection area.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_default(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Empty drop zone ──
    label(tree, root, "Drop Zone (empty)", text_secondary);
    {
        let zone = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(300.0),
            height: Sizing::Fixed(120.0),
            padding: Edges::all(16.0),
            gap: 8.0,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            align: Align::Center,
            justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, zone);

        let icon = tree.create(Widget::Label { text: "⬆".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(20.0), ..UiStyle::default()
        });
        tree.add_child(zone, icon);

        let msg = tree.create(Widget::Label { text: "Drop files here or click to browse".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(zone, msg);

        let hint = tree.create(Widget::Label { text: "PNG, JPG up to 10MB".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.6)), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(zone, hint);
    }

    // ── With file selected ──
    label(tree, root, "File Selected", text_secondary);
    {
        let zone = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(300.0),
            padding: Edges::all(12.0),
            gap: 8.0,
            background: Some(bg_surface),
            border_color: Some(success),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, zone);

        // File entry
        let file_row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
        });
        tree.add_child(zone, file_row);

        let file_icon = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(32.0), height: Sizing::Fixed(32.0),
            corner_radii: [4.0; 4], background: Some(border_subtle),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(file_row, file_icon);

        let fi = tree.create(Widget::Label { text: "📄".to_string() }, UiStyle {
            text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(file_icon, fi);

        let info = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 2.0, ..UiStyle::default()
        });
        tree.add_child(file_row, info);

        let fname = tree.create(Widget::Label { text: "screenshot.png".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(info, fname);

        let fsize = tree.create(Widget::Label { text: "2.4 MB".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(info, fsize);

        let remove = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(file_row, remove);
    }

    // ── Compact button variant ──
    label(tree, root, "Compact (button style)", text_secondary);
    {
        let btn = tree.create(Widget::Button {
            label: "Choose File".to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
            corner_radii: [6.0; 4],
            background: Some(accent),
            text_color: Some(text_inverse),
            text_size: Some(12.0),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(root, btn);
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let zone = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(300.0),
            height: Sizing::Fixed(80.0),
            background: Some(bg_surface),
            border_color: Some(border_subtle),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            align: Align::Center,
            justify: Justify::Center,
            opacity: 0.5,
            ..UiStyle::default()
        });
        tree.add_child(root, zone);

        let msg = tree.create(Widget::Label { text: "Upload disabled".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(zone, msg);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
