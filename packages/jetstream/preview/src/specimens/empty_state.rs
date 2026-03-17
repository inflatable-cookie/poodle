//! EmptyState specimen — placeholder with message and action button.

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
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── With action ──
    section_label(tree, root, "With Action", text_secondary);
    {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(360.0), padding: Edges::all(32.0), gap: 12.0,
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, card);

        let icon = tree.create(Widget::Label { text: "📁".to_string() }, UiStyle {
            text_size: Some(32.0), ..UiStyle::default()
        });
        tree.add_child(card, icon);

        let title = tree.create(Widget::Label { text: "No documents yet".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(15.0), ..UiStyle::default()
        });
        tree.add_child(card, title);

        let desc = tree.create(Widget::Label { text: "Upload your first document to get started.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(card, desc);

        let btn = tree.create(Widget::Button {
            label: "Upload Document".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(card, btn);
    }

    // ── Minimal ──
    section_label(tree, root, "Minimal", text_secondary);
    {
        let card = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(360.0), padding: Edges::all(24.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, card);

        let msg = tree.create(Widget::Label { text: "Nothing to show".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(card, msg);
    }

    // ── Search empty ──
    section_label(tree, root, "Search Empty", text_secondary);
    {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(360.0), padding: Edges::all(32.0), gap: 8.0,
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, card);

        let icon = tree.create(Widget::Label { text: "🔍".to_string() }, UiStyle {
            text_size: Some(24.0), ..UiStyle::default()
        });
        tree.add_child(card, icon);

        let title = tree.create(Widget::Label { text: "No results found".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(card, title);

        let desc = tree.create(Widget::Label { text: "Try adjusting your search or filter criteria.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(card, desc);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
