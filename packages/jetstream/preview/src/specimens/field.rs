//! Field specimen — form field wrapper with label, input, description, and error.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let border_default = theme_bridge::border_default(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Standard field ──
    section_label(tree, root, "Standard Field", text_secondary);
    {
        let field = field_container(tree, root);

        let lbl = tree.create(Widget::Label { text: "Email address".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(field, lbl);

        let input = mock_input(tree, "user@example.com", bg_canvas, border_default, text_primary);
        tree.add_child(field, input);

        let desc = tree.create(Widget::Label { text: "We'll never share your email.".to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(field, desc);
    }

    // ── Required field ──
    section_label(tree, root, "Required Field", text_secondary);
    {
        let field = field_container(tree, root);

        let lbl_row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(field, lbl_row);

        let lbl = tree.create(Widget::Label { text: "Username".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(lbl_row, lbl);

        let req = tree.create(Widget::Label { text: "*".to_string() }, UiStyle {
            text_color: Some(danger),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(lbl_row, req);

        let input = mock_input(tree, "", bg_canvas, border_default, text_primary);
        tree.add_child(field, input);
    }

    // ── Field with error ──
    section_label(tree, root, "Validation Error", text_secondary);
    {
        let field = field_container(tree, root);

        let lbl = tree.create(Widget::Label { text: "Password".to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(field, lbl);

        let input = mock_input(tree, "abc", bg_canvas, danger, text_primary);
        tree.add_child(field, input);

        let err = tree.create(Widget::Label { text: "Password must be at least 8 characters.".to_string() }, UiStyle {
            text_color: Some(danger),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(field, err);
    }

    // ── Disabled field ──
    section_label(tree, root, "Disabled Field", text_secondary);
    {
        let field = field_container(tree, root);

        let lbl = tree.create(Widget::Label { text: "Organization".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_primary, 0.5)),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(field, lbl);

        let input = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(280.0),
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(theme_bridge::tint(bg_canvas, 0.5)),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            align: Align::Center,
            opacity: 0.5,
            ..UiStyle::default()
        });
        tree.add_child(field, input);

        let val = tree.create(Widget::Label { text: "Acme Corp".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.5)),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(input, val);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn field_container(tree: &mut UiTree, parent: UiNodeId) -> UiNodeId {
    let field = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        gap: 4.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, field);
    field
}

fn mock_input(
    tree: &mut UiTree,
    value: &str,
    bg: glam::Vec4,
    border: glam::Vec4,
    text_color: glam::Vec4,
) -> UiNodeId {
    let input = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(280.0),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 6.0,
        align: Align::Center,
        ..UiStyle::default()
    });

    if !value.is_empty() {
        let val = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
            text_color: Some(text_color),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(input, val);
    }

    input
}
