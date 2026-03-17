//! SplitButton specimen — button + separator + dropdown trigger.

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

    // ── Primary SplitButton ──
    label(tree, root, "Primary Split Button", text_secondary);
    split_button_group(tree, root, "Save", accent, text_inverse, theme_bridge::tint(text_inverse, 0.3));

    // ── Secondary SplitButton ──
    label(tree, root, "Secondary Split Button", text_secondary);
    split_button_group(tree, root, "Export", bg_surface, text_primary, border);

    // ── Sizes ──
    label(tree, root, "Sizes", text_secondary);

    let sizes_col = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        gap: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(root, sizes_col);

    for &(lbl, h, ts) in &[("Small", 24.0_f32, 11.0_f32), ("Medium", 32.0, 12.0), ("Large", 40.0, 14.0)] {
        let group = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(h),
            ..UiStyle::default()
        });
        tree.add_child(sizes_col, group);

        let main = tree.create(Widget::Button {
            label: lbl.to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(h),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radii: [0.0; 4],
            background: Some(accent),
            text_color: Some(text_inverse),
            text_size: Some(ts),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(group, main);

        // Separator
        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(1.0),
            height: Sizing::Grow(1.0),
            background: Some(theme_bridge::tint(text_inverse, 0.3)),
            ..UiStyle::default()
        });
        tree.add_child(group, sep);

        // Dropdown trigger
        let trigger = tree.create(Widget::Button {
            label: "▾".to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(h),
            height: Sizing::Fixed(h),
            corner_radii: [0.0; 4],
            background: Some(accent),
            text_color: Some(text_inverse),
            text_size: Some(ts),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(group, trigger);
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

fn split_button_group(
    tree: &mut UiTree,
    parent: UiNodeId,
    main_label: &str,
    bg: glam::Vec4,
    text: glam::Vec4,
    sep_color: glam::Vec4,
) {
    let group = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        height: Sizing::Fixed(32.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, group);

    // Main action
    let main = tree.create(Widget::Button {
        label: main_label.to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
        corner_radii: [0.0; 4],
        background: Some(bg),
        text_color: Some(text),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(group, main);

    // Separator
    let sep = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(1.0),
        height: Sizing::Grow(1.0),
        background: Some(sep_color),
        ..UiStyle::default()
    });
    tree.add_child(group, sep);

    // Dropdown trigger
    let trigger = tree.create(Widget::Button {
        label: "▾".to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        width: Sizing::Fixed(32.0),
        height: Sizing::Fixed(32.0),
        corner_radii: [0.0; 4],
        background: Some(bg),
        text_color: Some(text),
        text_size: Some(12.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(group, trigger);
}
