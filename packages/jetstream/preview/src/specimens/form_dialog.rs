//! FormDialog specimen — modal dialog with form fields and validation.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(24.0)),
        NodeStyle::default());

    // ── Standard form dialog ──
    section_label(tree, root, "Create Item Dialog", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 360.0);
        tree.add_child(root, dialog);

        // Header
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 16.0, right: 16.0, bottom: 12.0, left: 16.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(dialog, header);

        let title = tree.create_node(Widget::Label { text: "Create New Item".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(15.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let close = tree.create_node(Widget::Label { text: "✕".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(header, close);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep);

        // Form body
        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(16.0))
            .with_gap(12.0)),
            NodeStyle::default());
        tree.add_child(dialog, body);

        form_field(tree, body, "Name", "Enter item name", "", text_primary, text_secondary, bg_surface, border);
        form_field(tree, body, "Description", "Optional description", "", text_primary, text_secondary, bg_surface, border);
        form_field(tree, body, "Category", "Select category", "", text_primary, text_secondary, bg_surface, border);

        // Footer
        let sep2 = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep2);

        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 12.0, right: 16.0, bottom: 16.0, left: 16.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch)),
            NodeStyle::default());
        tree.add_child(dialog, footer);

        let cancel = tree.create_node(Widget::Button {
            label: "Cancel".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0, text_color: Some(text_primary), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(footer, cancel);

        let submit = tree.create_node(Widget::Button {
            label: "Create".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], background: Some(accent), text_color: Some(text_inverse), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(footer, submit);
    }

    // ── With validation errors ──
    section_label(tree, root, "With Validation Errors", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 360.0);
        tree.add_child(root, dialog);

        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges { top: 16.0, right: 16.0, bottom: 12.0, left: 16.0 })),
            NodeStyle::default());
        tree.add_child(dialog, header);

        let title = tree.create_node(Widget::Label { text: "Edit Profile".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(15.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(16.0))
            .with_gap(12.0)),
            NodeStyle::default());
        tree.add_child(dialog, body);

        form_field(tree, body, "Email", "", "alice@co.io", text_primary, text_secondary, bg_surface, border);
        form_field_error(tree, body, "Username", "", "", "Username is required", text_primary, text_secondary, bg_surface, border, danger);

        let sep2 = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep2);

        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 12.0, right: 16.0, bottom: 16.0, left: 16.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch)),
            NodeStyle::default());
        tree.add_child(dialog, footer);

        let save = tree.create_node(Widget::Button {
            label: "Save".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [6.0; 4], background: Some(accent), text_color: Some(text_inverse), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(footer, save);
    }

    root
}

fn dialog_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4, width: f32) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(width))),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [10.0; 4], ..NodeStyle::default() })
}

fn form_field(tree: &mut UiTree, parent: UiNodeId, label: &str, placeholder: &str, value: &str, label_color: glam::Vec4, placeholder_color: glam::Vec4, bg: glam::Vec4, border: glam::Vec4) {
    let field = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_gap(4.0)),
        NodeStyle::default());
    tree.add_child(parent, field);

    let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(label_color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(field, lbl);

    let display = if value.is_empty() { placeholder } else { value };
    let color = if value.is_empty() { placeholder_color } else { label_color };

    let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(32.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(field, input);

    let txt = tree.create_node(Widget::Label { text: display.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(input, txt);
}

fn form_field_error(tree: &mut UiTree, parent: UiNodeId, label: &str, placeholder: &str, value: &str, error: &str, label_color: glam::Vec4, placeholder_color: glam::Vec4, bg: glam::Vec4, _border: glam::Vec4, danger: glam::Vec4) {
    let field = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_gap(4.0)),
        NodeStyle::default());
    tree.add_child(parent, field);

    let lbl = tree.create_node(Widget::Label { text: label.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(label_color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(field, lbl);

    let display = if value.is_empty() { placeholder } else { value };
    let color = if value.is_empty() { placeholder_color } else { label_color };

    let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(32.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg), border_color: Some(danger), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
    tree.add_child(field, input);

    let txt = tree.create_node(Widget::Label { text: display.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(input, txt);

    let err = tree.create_node(Widget::Label { text: error.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(danger), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(field, err);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
