//! SearchField specimen — search input with icon and clear button.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Empty ──
    label(tree, root, "Empty", text_secondary);
    {
        let field = search_container(tree, bg_canvas, border_default);
        tree.add_child(root, field);

        let icon = tree.create(Widget::Label { text: "⌕".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(14.0),
            ..UiStyle::default()
        });
        tree.add_child(field, icon);

        let placeholder = tree.create(Widget::Label { text: "Search...".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(12.0),
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(field, placeholder);
    }

    // ── With value ──
    label(tree, root, "With Value", text_secondary);
    {
        let field = search_container(tree, bg_canvas, border_default);
        tree.add_child(root, field);

        let icon = tree.create(Widget::Label { text: "⌕".to_string() }, UiStyle {
            text_color: Some(accent),
            text_size: Some(14.0),
            ..UiStyle::default()
        });
        tree.add_child(field, icon);

        let val = tree.create(Widget::Label { text: "design tokens".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(field, val);

        // Clear button
        let clear = tree.create(Widget::Button {
            label: "✕".to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(20.0),
            height: Sizing::Fixed(20.0),
            corner_radii: [10.0; 4],
            text_color: Some(text_secondary),
            text_size: Some(10.0),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(field, clear);
    }

    // ── Focused ──
    label(tree, root, "Focused", text_secondary);
    {
        let field = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(280.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            gap: 8.0,
            background: Some(bg_canvas),
            border_color: Some(accent),
            border_width: 2.0,
            corner_radii: [6.0; 4],
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, field);

        let icon = tree.create(Widget::Label { text: "⌕".to_string() }, UiStyle {
            text_color: Some(accent),
            text_size: Some(14.0),
            ..UiStyle::default()
        });
        tree.add_child(field, icon);

        let cursor = tree.create(Widget::Label { text: "|".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(field, cursor);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn search_container(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Fixed(280.0),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        gap: 8.0,
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [6.0; 4],
        align: Align::Center,
        ..UiStyle::default()
    })
}
