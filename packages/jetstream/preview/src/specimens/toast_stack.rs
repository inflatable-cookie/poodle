//! ToastStack specimen — stacked notification toasts with auto-dismiss.

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
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_gap(16.0)),
        NodeStyle { ..NodeStyle::default() });

    section_label(tree, root, "Toast Stack", text_secondary);
    {
        let stack = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(8.0)
                .with_width(LayoutSizing::Fixed(340.0))),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, stack);

        toast(tree, stack, "✓", "Changes saved successfully", None, success, bg_elevated, border, text_primary, text_secondary);
        toast(tree, stack, "✕", "Failed to upload file", Some("Retry"), danger, bg_elevated, border, text_primary, text_inverse);
        toast(tree, stack, "⚠", "Your session will expire in 5 minutes", None, warning, bg_elevated, border, text_primary, text_secondary);
        toast(tree, stack, "ℹ", "New version available — refresh to update", Some("Refresh"), accent, bg_elevated, border, text_primary, text_inverse);
    }

    root
}

fn toast(tree: &mut UiTree, parent: UiNodeId, icon: &str, message: &str, action: Option<&str>, tone: glam::Vec4, bg: glam::Vec4, border: glam::Vec4, text_primary: glam::Vec4, action_fg: glam::Vec4) {
    let card = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_padding(LayoutEdges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
    tree.add_child(parent, card);

    // Tone bar
    let bar = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(3.0))
            .with_height(LayoutSizing::Fixed(24.0))),
        NodeStyle { background: Some(tone), corner_radii: [1.5; 4], ..NodeStyle::default() });
    tree.add_child(card, bar);

    let ic = tree.create_node(Widget::Label { text: icon.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(tone), text_size: Some(14.0), ..NodeStyle::default() });
    tree.add_child(card, ic);

    let msg = tree.create_node(Widget::Label { text: message.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)),
        NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
    tree.add_child(card, msg);

    if let Some(action_text) = action {
        let btn = tree.create_node(Widget::Button {
            label: action_text.to_string(), pressed: false, hovered: false,
        },
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(24.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { corner_radii: [4.0; 4], background: Some(tone), text_color: Some(action_fg), text_size: Some(10.0), focusable: true, ..NodeStyle::default() });
        tree.add_child(card, btn);
    }

    let close = tree.create_node(Widget::Label { text: "✕".to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(theme_bridge::tint(text_primary, 0.4)), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(card, close);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
