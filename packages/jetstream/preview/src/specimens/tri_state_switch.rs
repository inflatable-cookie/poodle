//! TriStateSwitch specimen — on/off/indeterminate three-position switch.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_default(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 12.0,
        ..UiStyle::default()
    });

    label(tree, root, "Three Positions", text_secondary);

    // Off / Indeterminate / On
    let positions: &[(&str, f32, glam::Vec4)] = &[
        ("Off", 0.0, bg_surface),
        ("Indeterminate", 0.5, warning),
        ("On", 1.0, accent),
    ];

    for &(name, position, track_color) in positions {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 10.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let track = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(44.0),
            height: Sizing::Fixed(20.0),
            corner_radius: 10.0,
            background: Some(track_color),
            border_color: Some(if position == 0.0 { border } else { track_color }),
            border_width: 1.0,
            direction: Direction::Row,
            align: Align::Center,
            padding: Edges::all(2.0),
            ..UiStyle::default()
        });
        tree.add_child(row, track);

        // Position the thumb: left=off, center=indeterminate, right=on
        if position > 0.0 {
            let spacer_w = if position >= 1.0 { Sizing::Grow(1.0) } else { Sizing::Fixed(11.0) };
            let spacer = tree.create(Widget::Panel, UiStyle {
                width: spacer_w,
                ..UiStyle::default()
            });
            tree.add_child(track, spacer);
        }

        let thumb = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(16.0),
            height: Sizing::Fixed(16.0),
            corner_radius: 8.0,
            background: Some(glam::Vec4::ONE),
            ..UiStyle::default()
        });
        tree.add_child(track, thumb);

        let lbl = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(row, lbl);
    }

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
