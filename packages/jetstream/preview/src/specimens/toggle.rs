//! Toggle specimen — pressable toggle button.

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
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Off state ──
    section_label(tree, root, "Off State", text_secondary);
    toggle_btn(tree, root, "Bold", bg_surface, border, text_primary, false);

    // ── On state ──
    section_label(tree, root, "On State (pressed)", text_secondary);
    toggle_btn(tree, root, "Bold", accent, border, text_inverse, true);

    // ── Multiple toggles ──
    section_label(tree, root, "Formatting Toggles", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, row);

        toggle_btn(tree, row, "B", accent, border, text_inverse, true);
        toggle_btn(tree, row, "I", bg_surface, border, text_primary, false);
        toggle_btn(tree, row, "U", bg_surface, border, text_primary, false);
        toggle_btn(tree, row, "S", bg_surface, border, text_primary, false);
    }

    // ── Disabled ──
    section_label(tree, root, "Disabled", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 4.0, opacity: 0.5, ..UiStyle::default()
        });
        tree.add_child(root, row);

        toggle_btn(tree, row, "On", accent, border, text_inverse, true);
        toggle_btn(tree, row, "Off", bg_surface, border, text_primary, false);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn toggle_btn(tree: &mut UiTree, parent: UiNodeId, label: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, pressed: bool) {
    let btn = tree.create(Widget::Button {
        label: label.to_string(), pressed, hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(36.0), height: Sizing::Fixed(32.0),
        corner_radii: [6.0; 4], background: Some(bg),
        border_color: Some(border), border_width: if pressed { 0.0 } else { 1.0 },
        text_color: Some(fg), text_size: Some(12.0),
        align: Align::Center, justify: Justify::Center,
        focusable: true, ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}
