//! MediaThumbnail specimen — compact thumbnails with aspect ratio and fallback.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)), NodeStyle::default());

    section_label(tree, root, "Square Thumbnails", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)), NodeStyle::default());
        tree.add_child(root, row);

        for &(name, size) in &[("photo.jpg", 64.0), ("banner.png", 64.0), ("icon.svg", 64.0)] {
            thumb(tree, row, name, size, size, border, text_primary, text_secondary);
        }
    }

    section_label(tree, root, "Landscape (16:9)", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)), NodeStyle::default());
        tree.add_child(root, row);

        for &name in &["hero.jpg", "cover.png"] {
            thumb(tree, row, name, 128.0, 72.0, border, text_primary, text_secondary);
        }
    }

    section_label(tree, root, "Fallback (no preview)", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)), NodeStyle::default());
        tree.add_child(root, row);

        for &(name, ext) in &[("data.csv", "CSV"), ("report.pdf", "PDF"), ("archive.zip", "ZIP")] {
            let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(64.0))
                .with_height(LayoutSizing::Fixed(64.0))
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
                background: Some(theme_bridge::tint(border, 0.3)),
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
            tree.add_child(row, card);

            let wrapper = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(2.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
            tree.add_child(card, wrapper);

            let ext_lbl = tree.create_node(Widget::Label { text: ext.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default()
            });
            tree.add_child(wrapper, ext_lbl);

            let n = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
                text_color: Some(theme_bridge::tint(text_secondary, 0.7)), text_size: Some(7.0), ..NodeStyle::default()
            });
            tree.add_child(wrapper, n);
        }
    }

    root
}

fn thumb(tree: &mut UiTree, parent: UiNodeId, name: &str, w: f32, h: f32, border: glam::Vec4, _text_primary: glam::Vec4, text_secondary: glam::Vec4) {
    let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_gap(4.0)), NodeStyle::default());
    tree.add_child(parent, card);

    let img = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(w))
        .with_height(LayoutSizing::Fixed(h))), NodeStyle {
        background: Some(theme_bridge::tint(border, 0.3)),
        corner_radii: [6.0; 4],
        ..NodeStyle::default()
    });
    tree.add_child(card, img);

    let lbl = tree.create_node(Widget::Label { text: name.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(text_secondary), text_size: Some(9.0), ..NodeStyle::default()
    });
    tree.add_child(card, lbl);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
