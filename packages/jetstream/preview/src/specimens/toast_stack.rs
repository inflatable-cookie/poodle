//! ToastStack specimen — stacked notification toasts with auto-dismiss.

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
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 16.0, ..UiStyle::default()
    });

    section_label(tree, root, "Toast Stack", text_secondary);
    {
        let stack = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0,
            width: Sizing::Fixed(340.0),
            ..UiStyle::default()
        });
        tree.add_child(root, stack);

        toast(tree, stack, "✓", "Changes saved successfully", None, success, bg_elevated, border, text_primary, text_secondary);
        toast(tree, stack, "✕", "Failed to upload file", Some("Retry"), danger, bg_elevated, border, text_primary, text_inverse);
        toast(tree, stack, "⚠", "Your session will expire in 5 minutes", None, warning, bg_elevated, border, text_primary, text_secondary);
        toast(tree, stack, "ℹ", "New version available — refresh to update", Some("Refresh"), accent, bg_elevated, border, text_primary, text_inverse);
    }

    root
}

fn toast(tree: &mut UiTree, parent: UiNodeId, icon: &str, message: &str, action: Option<&str>, tone: glam::Vec4, bg: glam::Vec4, border: glam::Vec4, text_primary: glam::Vec4, action_fg: glam::Vec4) {
    let card = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 8.0,
        padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
        background: Some(bg), border_color: Some(border), border_width: 1.0,
        corner_radii: [8.0; 4], align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, card);

    // Tone bar
    let bar = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(3.0), height: Sizing::Fixed(24.0),
        background: Some(tone), corner_radii: [1.5; 4],
        ..UiStyle::default()
    });
    tree.add_child(card, bar);

    let ic = tree.create(Widget::Label { text: icon.to_string() }, UiStyle {
        text_color: Some(tone), text_size: Some(14.0), ..UiStyle::default()
    });
    tree.add_child(card, ic);

    let msg = tree.create(Widget::Label { text: message.to_string() }, UiStyle {
        text_color: Some(text_primary), text_size: Some(12.0),
        width: Sizing::Grow(1.0), ..UiStyle::default()
    });
    tree.add_child(card, msg);

    if let Some(action_text) = action {
        let btn = tree.create(Widget::Button {
            label: action_text.to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(24.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            corner_radii: [4.0; 4], background: Some(tone),
            text_color: Some(action_fg), text_size: Some(10.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(card, btn);
    }

    let close = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
        text_color: Some(theme_bridge::tint(text_primary, 0.4)), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(card, close);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
