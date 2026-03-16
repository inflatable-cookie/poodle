//! IconButton specimen — demonstrates icon-only buttons with variant and size combos.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Variants ──
    label(tree, root, "IconButton Variants", text_secondary);

    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(root, row);

    // Primary icon button (filled)
    icon_btn(tree, row, "✕", 32.0, Some(accent), Some(glam::Vec4::ONE));
    // Secondary (outlined)
    icon_btn_outlined(tree, row, "⟳", 32.0, bg_surface, border, text_primary);
    // Ghost
    icon_btn(tree, row, "⋯", 32.0, None, Some(text_primary));
    // Ghost with accent
    icon_btn(tree, row, "★", 32.0, None, Some(accent));

    // ── Sizes ──
    label(tree, root, "Sizes", text_secondary);

    let size_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::End,
        ..UiStyle::default()
    });
    tree.add_child(root, size_row);

    for &size in &[24.0_f32, 32.0, 40.0] {
        icon_btn_outlined(tree, size_row, "⊕", size, bg_surface, border, text_primary);
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);

    let dis_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(root, dis_row);

    icon_btn(tree, dis_row, "✕", 32.0, Some(accent), Some(glam::Vec4::ONE));
    // Disabled version
    let dis = tree.create(Widget::Button {
        label: "✕".to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(32.0),
        height: Sizing::Fixed(32.0),
        corner_radius: 6.0,
        background: Some(theme_bridge::tint(accent, 0.4)),
        text_color: Some(theme_bridge::tint(glam::Vec4::ONE, 0.5)),
        text_size: Some(14.0),
        align: Align::Center,
        justify: Justify::Center,
        opacity: 0.6,
        ..UiStyle::default()
    });
    tree.add_child(dis_row, dis);

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

fn icon_btn(
    tree: &mut UiTree,
    parent: UiNodeId,
    icon: &str,
    size: f32,
    bg: Option<glam::Vec4>,
    text_color: Option<glam::Vec4>,
) {
    let btn = tree.create(Widget::Button {
        label: icon.to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(size),
        height: Sizing::Fixed(size),
        corner_radius: 6.0,
        background: bg,
        text_color,
        text_size: Some(14.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}

fn icon_btn_outlined(
    tree: &mut UiTree,
    parent: UiNodeId,
    icon: &str,
    size: f32,
    bg: glam::Vec4,
    border: glam::Vec4,
    text_color: glam::Vec4,
) {
    let btn = tree.create(Widget::Button {
        label: icon.to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(size),
        height: Sizing::Fixed(size),
        corner_radius: 6.0,
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        text_color: Some(text_color),
        text_size: Some(14.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}
