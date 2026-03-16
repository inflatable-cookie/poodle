//! Box specimen — demonstrates padding variants and overflow.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

/// Render the Box specimen showing padding variants.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Padding variants ──
    let label = tree.create(Widget::Label {
        text: "Padding Variants".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(root, label);

    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        gap: 12.0,
        ..UiStyle::default()
    });
    tree.add_child(root, row);

    let paddings: &[(&str, f32)] = &[
        ("None", 0.0),
        ("Sm (4px)", 4.0),
        ("Md (8px)", 8.0),
        ("Lg (16px)", 16.0),
        ("Xl (24px)", 24.0),
    ];

    for &(name, pad) in paddings {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(pad),
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(row, card);

        let inner = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(60.0),
            height: Sizing::Fixed(32.0),
            background: Some(theme_bridge::tint(accent, 0.20)),
            corner_radius: 4.0,
            align: Align::Center,
            justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(card, inner);

        let lbl = tree.create(Widget::Label {
            text: name.to_string(),
        }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(inner, lbl);
    }

    // ── Overflow demo ──
    let overflow_label = tree.create(Widget::Label {
        text: "Overflow: Hidden".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        padding: Edges { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 },
        ..UiStyle::default()
    });
    tree.add_child(root, overflow_label);

    let overflow_box = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(200.0),
        height: Sizing::Fixed(60.0),
        overflow: Overflow::Hidden,
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 6.0,
        padding: Edges::all(8.0),
        ..UiStyle::default()
    });
    tree.add_child(root, overflow_box);

    // Content wider than container
    let wide_content = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(400.0),
        height: Sizing::Fixed(40.0),
        background: Some(theme_bridge::tint(accent, 0.15)),
        corner_radius: 4.0,
        align: Align::Center,
        justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(overflow_box, wide_content);

    let clip_label = tree.create(Widget::Label {
        text: "400px content in 200px box (clipped)".to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(10.0),
        ..UiStyle::default()
    });
    tree.add_child(wide_content, clip_label);

    root
}
