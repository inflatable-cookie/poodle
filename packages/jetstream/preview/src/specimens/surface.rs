//! Surface specimen — demonstrates tone variants and border options.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

/// Render the Surface specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let border_default = theme_bridge::border_default(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Tone variants ──
    let label = tree.create(Widget::Label {
        text: "Surface Tones".to_string(),
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

    let tones: &[(&str, &str)] = &[
        ("Canvas", "semantic.color.background.canvas"),
        ("Panel", "semantic.color.background.panel"),
        ("Elevated", "semantic.color.background.elevated"),
        ("Surface", "semantic.color.background.surface"),
    ];

    for &(name, token) in tones {
        let bg = theme_bridge::resolve_vec4(theme, token);
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0),
            height: Sizing::Fixed(80.0),
            padding: Edges::all(12.0),
            gap: 4.0,
            background: Some(bg),
            border_color: Some(border_subtle),
            border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(row, card);

        let tone_name = tree.create(Widget::Label {
            text: name.to_string(),
        }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(card, tone_name);

        let tone_token = tree.create(Widget::Label {
            text: token.rsplit('.').next().unwrap_or(token).to_string(),
        }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(card, tone_token);
    }

    // ── Border variants ──
    let border_label = tree.create(Widget::Label {
        text: "Border Variants".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(root, border_label);

    let border_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        gap: 12.0,
        ..UiStyle::default()
    });
    tree.add_child(root, border_row);

    let bg_surface = theme_bridge::surface_background(theme);
    let borders: &[(&str, Option<glam::Vec4>, f32)] = &[
        ("None", None, 0.0),
        ("Subtle", Some(border_subtle), 1.0),
        ("Default", Some(border_default), 1.0),
        ("Bold", Some(border_default), 2.0),
    ];

    for &(name, color, width) in borders {
        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0),
            height: Sizing::Fixed(60.0),
            padding: Edges::all(12.0),
            background: Some(bg_surface),
            border_color: color,
            border_width: width,
            corner_radius: 8.0,
            align: Align::Center,
            justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(border_row, card);

        let lbl = tree.create(Widget::Label {
            text: name.to_string(),
        }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(11.0),
            ..UiStyle::default()
        });
        tree.add_child(card, lbl);
    }

    root
}
