//! Dialog specimen — modal overlay with header, body, and actions.
//!
//! Since Jetstream preview renders a static UiTree, we show the dialog
//! as a card-based mockup rather than a true overlay.

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

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)), NodeStyle::default());

    // ── Standard dialog ──
    section_label(tree, root, "Standard Dialog", text_secondary);
    {
        let dialog = dialog_frame(tree, 320.0, bg_elevated, border);
        tree.add_child(root, dialog);

        // Header
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_padding(LayoutEdges { top: 16.0, right: 20.0, bottom: 12.0, left: 20.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(dialog, header);

        let title = tree.create_node(Widget::Label { text: "Confirm Action".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(16.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let close = tree.create_node(Widget::Label { text: "✕".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(header, close);

        // Separator
        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep);

        // Body
        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges { top: 12.0, right: 20.0, bottom: 12.0, left: 20.0 })),
            NodeStyle::default());
        tree.add_child(dialog, body);

        let msg = tree.create_node(Widget::Label { text: "Are you sure you want to proceed? This action cannot be undone.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(body, msg);

        // Footer
        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_padding(LayoutEdges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch)),
            NodeStyle::default());
        tree.add_child(dialog, footer);

        btn_secondary(tree, footer, "Cancel", bg_surface, border, text_primary);
        btn_primary(tree, footer, "Confirm", accent, text_inverse);
    }

    // ── Dialog with form ──
    section_label(tree, root, "Dialog with Form Content", text_secondary);
    {
        let dialog = dialog_frame(tree, 360.0, bg_elevated, border);
        tree.add_child(root, dialog);

        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges { top: 16.0, right: 20.0, bottom: 12.0, left: 20.0 })),
            NodeStyle::default());
        tree.add_child(dialog, header);

        let title = tree.create_node(Widget::Label { text: "Create New Item".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(16.0), ..NodeStyle::default() });
        tree.add_child(header, title);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges { top: 12.0, right: 20.0, bottom: 12.0, left: 20.0 })
            .with_gap(12.0)),
            NodeStyle::default());
        tree.add_child(dialog, body);

        let bg_canvas = theme_bridge::canvas_background(theme);
        let border_default = theme_bridge::border_default(theme);

        for &(lbl, placeholder) in &[("Name", "Enter name"), ("Description", "Optional description")] {
            let field = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column).with_gap(4.0)),
                NodeStyle::default());
            tree.add_child(body, field);

            let l = tree.create_node(Widget::Label { text: lbl.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(field, l);

            let input = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle { background: Some(bg_canvas), border_color: Some(border_default), border_width: 1.0, corner_radii: [6.0; 4], ..NodeStyle::default() });
            tree.add_child(field, input);

            let ph = tree.create_node(Widget::Label { text: placeholder.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(theme_bridge::tint(text_secondary, 0.6)), text_size: Some(12.0), ..NodeStyle::default() });
            tree.add_child(input, ph);
        }

        let sep2 = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(dialog, sep2);

        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_padding(LayoutEdges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch)),
            NodeStyle::default());
        tree.add_child(dialog, footer);

        btn_secondary(tree, footer, "Cancel", bg_surface, border, text_primary);
        btn_primary(tree, footer, "Create", accent, text_inverse);
    }

    // ── Compact / small ──
    section_label(tree, root, "Small Dialog", text_secondary);
    {
        let dialog = dialog_frame(tree, 260.0, bg_elevated, border);
        tree.add_child(root, dialog);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_padding(LayoutEdges::uniform(20.0))
            .with_gap(12.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(dialog, body);

        let msg = tree.create_node(Widget::Label { text: "Delete this item?".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(body, msg);

        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row).with_gap(8.0)),
            NodeStyle::default());
        tree.add_child(body, row);

        let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
        btn_secondary(tree, row, "Cancel", bg_surface, border, text_primary);
        btn_primary(tree, row, "Delete", danger, text_inverse);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn dialog_frame(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(width))),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [12.0; 4], ..NodeStyle::default() })
}

fn btn_primary(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let btn = tree.create_node(Widget::Button {
        label: text.to_string(), pressed: false, hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(32.0))
        .with_padding(LayoutEdges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle { corner_radii: [6.0; 4], background: Some(bg), text_color: Some(fg), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
    tree.add_child(parent, btn);
}

fn btn_secondary(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4) {
    let btn = tree.create_node(Widget::Button {
        label: text.to_string(), pressed: false, hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(32.0))
        .with_padding(LayoutEdges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle { corner_radii: [6.0; 4], background: Some(bg), border_color: Some(border), border_width: 1.0, text_color: Some(fg), text_size: Some(12.0), focusable: true, ..NodeStyle::default() });
    tree.add_child(parent, btn);
}
