//! MediaPreview specimen — media asset preview with metadata display.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    section_label(tree, root, "Image Preview", text_secondary);
    {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, width: Sizing::Fixed(360.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, card);

        let img = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(180.0),
            background: Some(theme_bridge::tint(border, 0.3)),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(card, img);

        let placeholder = tree.create(Widget::Label { text: "📷 1920×1080".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_primary, 0.3)), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(img, placeholder);

        let meta = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, padding: Edges::all(12.0), gap: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(card, meta);

        meta_row(tree, meta, "Filename", "hero-banner.jpg", text_secondary, text_primary);
        meta_row(tree, meta, "Dimensions", "1920 × 1080 px", text_secondary, text_primary);
        meta_row(tree, meta, "Size", "2.4 MB", text_secondary, text_primary);
        meta_row(tree, meta, "Type", "image/jpeg", text_secondary, text_primary);
        meta_row(tree, meta, "Uploaded", "March 14, 2026", text_secondary, text_primary);
    }

    root
}

fn meta_row(tree: &mut UiTree, parent: UiNodeId, label: &str, value: &str, label_color: glam::Vec4, value_color: glam::Vec4) {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 12.0, ..UiStyle::default()
    });
    tree.add_child(parent, row);

    let l = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(label_color), text_size: Some(10.0), width: Sizing::Fixed(70.0), ..UiStyle::default()
    });
    tree.add_child(row, l);

    let v = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
        text_color: Some(value_color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(row, v);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
