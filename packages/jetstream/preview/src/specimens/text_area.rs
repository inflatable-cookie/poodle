//! TextArea specimen — multi-line text input.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border_default = theme_bridge::border_default(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Empty with placeholder ──
    label(tree, root, "Empty (placeholder)", text_secondary);
    {
        let area = textarea_box(tree, bg_canvas, border_default, 100.0);
        tree.add_child(root, area);

        let placeholder = tree.create(Widget::Label {
            text: "Enter a description...".to_string(),
        }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(area, placeholder);
    }

    // ── Filled ──
    label(tree, root, "Filled", text_secondary);
    {
        let area = textarea_box(tree, bg_canvas, border_default, 120.0);
        tree.add_child(root, area);

        let lines = [
            "This is a multi-line text area.",
            "It supports several lines of input.",
            "Each line wraps within the container.",
            "",
            "Blank lines are preserved.",
        ];
        for line in &lines {
            let l = tree.create(Widget::Label {
                text: if line.is_empty() { " ".to_string() } else { line.to_string() },
            }, UiStyle {
                text_color: Some(text_primary),
                text_size: Some(12.0),
                ..UiStyle::default()
            });
            tree.add_child(area, l);
        }
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let area = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(360.0),
            height: Sizing::Fixed(80.0),
            padding: Edges::all(10.0),
            gap: 2.0,
            background: Some(bg_canvas),
            border_color: Some(theme_bridge::border_subtle(theme)),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            opacity: 0.5,
            ..UiStyle::default()
        });
        tree.add_child(root, area);

        let txt = tree.create(Widget::Label {
            text: "This content cannot be edited.".to_string(),
        }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.6)),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(area, txt);
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

fn textarea_box(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4, height: f32) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(360.0),
        height: Sizing::Fixed(height),
        padding: Edges::all(10.0),
        gap: 2.0,
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [6.0; 4],
        ..UiStyle::default()
    })
}
