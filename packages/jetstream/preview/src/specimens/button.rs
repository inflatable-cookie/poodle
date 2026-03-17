//! Button specimen — demonstrates variant, size, and disabled states.

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
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Variants ──
    section_label(tree, root, "Button Variants", text_secondary);

    let variants_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(root, variants_row);

    // Primary
    make_button(tree, variants_row, "Primary", UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radii: [6.0; 4],
        background: Some(accent),
        text_color: Some(text_inverse),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });

    // Secondary
    make_button(tree, variants_row, "Secondary", UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radii: [6.0; 4],
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        text_color: Some(text_primary),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });

    // Ghost
    make_button(tree, variants_row, "Ghost", UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radii: [6.0; 4],
        text_color: Some(accent),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });

    // Danger
    make_button(tree, variants_row, "Danger", UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radii: [6.0; 4],
        background: Some(danger),
        text_color: Some(text_inverse),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });

    // ── Sizes ──
    section_label(tree, root, "Button Sizes", text_secondary);

    let sizes_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::End,
        ..UiStyle::default()
    });
    tree.add_child(root, sizes_row);

    for &(label, height, text_size, pad_h) in &[
        ("Small", 24.0_f32, 11.0_f32, 10.0_f32),
        ("Medium", 32.0, 12.0, 16.0),
        ("Large", 40.0, 14.0, 20.0),
    ] {
        make_button(tree, sizes_row, label, UiStyle {
            height: Sizing::Fixed(height),
            padding: Edges { top: 0.0, right: pad_h, bottom: 0.0, left: pad_h },
            corner_radii: [6.0; 4],
            background: Some(accent),
            text_color: Some(text_inverse),
            text_size: Some(text_size),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
    }

    // ── Disabled state ──
    section_label(tree, root, "Disabled State", text_secondary);

    let disabled_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(root, disabled_row);

    make_button(tree, disabled_row, "Enabled", UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radii: [6.0; 4],
        background: Some(accent),
        text_color: Some(text_inverse),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });

    make_button(tree, disabled_row, "Disabled", UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radii: [6.0; 4],
        background: Some(theme_bridge::tint(accent, 0.4)),
        text_color: Some(theme_bridge::tint(text_inverse, 0.5)),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        opacity: 0.6,
        ..UiStyle::default()
    });

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

fn make_button(tree: &mut UiTree, parent: UiNodeId, label: &str, style: UiStyle) {
    let btn = tree.create(Widget::Button {
        label: label.to_string(),
        pressed: false,
        hovered: false,
    }, style);
    tree.add_child(parent, btn);
}
