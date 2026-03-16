//! FormActions specimen — action row with primary/secondary button alignment.

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

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── End-aligned (default) ──
    label(tree, root, "End-aligned (default)", text_secondary);
    {
        let row = actions_row(tree, bg_surface, border);
        tree.add_child(root, row);

        let spacer = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(row, spacer);

        btn_secondary(tree, row, "Cancel", bg_surface, border, text_primary);
        btn_primary(tree, row, "Save", accent, text_inverse);
    }

    // ── Start-aligned ──
    label(tree, root, "Start-aligned", text_secondary);
    {
        let row = actions_row(tree, bg_surface, border);
        tree.add_child(root, row);

        btn_primary(tree, row, "Submit", accent, text_inverse);
        btn_secondary(tree, row, "Reset", bg_surface, border, text_primary);
    }

    // ── Space-between ──
    label(tree, root, "Space-between (delete left, save right)", text_secondary);
    {
        let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 12.0, right: 0.0, bottom: 0.0, left: 0.0 },
            gap: 8.0,
            align: Align::Center,
            justify: Justify::SpaceBetween,
            border_color: Some(border),
            border_width: 1.0,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        btn_secondary(tree, row, "Delete", bg_surface, danger, danger);
        btn_primary(tree, row, "Save Changes", accent, text_inverse);
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

fn actions_row(tree: &mut UiTree, _bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        padding: Edges { top: 12.0, right: 0.0, bottom: 0.0, left: 0.0 },
        gap: 8.0,
        align: Align::Center,
        border_color: Some(border),
        border_width: 1.0,
        ..UiStyle::default()
    })
}

fn btn_primary(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let btn = tree.create(Widget::Button {
        label: text.to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radius: 6.0,
        background: Some(bg),
        text_color: Some(fg),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}

fn btn_secondary(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4) {
    let btn = tree.create(Widget::Button {
        label: text.to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radius: 6.0,
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        text_color: Some(fg),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}
