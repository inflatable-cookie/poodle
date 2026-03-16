//! PageLoading specimen — full-viewport loading with skeleton or spinner.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Spinner ──
    section_label(tree, root, "Spinner Loading", text_secondary);
    {
        let container = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(300.0), height: Sizing::Fixed(160.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, container);

        let inner = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 12.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(container, inner);

        let spinner = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(32.0), height: Sizing::Fixed(32.0),
            corner_radius: 16.0,
            border_color: Some(accent), border_width: 3.0,
            ..UiStyle::default()
        });
        tree.add_child(inner, spinner);

        let msg = tree.create(Widget::Label { text: "Loading…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(inner, msg);
    }

    // ── Skeleton ──
    section_label(tree, root, "Skeleton Loading", text_secondary);
    {
        let container = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 12.0,
            width: Sizing::Fixed(300.0),
            padding: Edges::all(16.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, container);

        let skel_color = theme_bridge::tint(border, 0.5);

        // Header skeleton
        let h = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(180.0), height: Sizing::Fixed(16.0),
            background: Some(skel_color), corner_radius: 4.0, ..UiStyle::default()
        });
        tree.add_child(container, h);

        // Text lines
        for &w in &[268.0, 240.0, 200.0] {
            let line = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Fixed(w), height: Sizing::Fixed(10.0),
                background: Some(skel_color), corner_radius: 3.0, ..UiStyle::default()
            });
            tree.add_child(container, line);
        }

        // Card skeleton
        let card_skel = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(60.0),
            background: Some(skel_color), corner_radius: 6.0, ..UiStyle::default()
        });
        tree.add_child(container, card_skel);
    }

    // ── Progress bar ──
    section_label(tree, root, "Progress Bar Loading", text_secondary);
    {
        let container = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0,
            width: Sizing::Fixed(300.0),
            padding: Edges::all(16.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, container);

        let track = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(268.0), height: Sizing::Fixed(4.0),
            background: Some(theme_bridge::tint(border, 0.5)),
            corner_radius: 2.0, ..UiStyle::default()
        });
        tree.add_child(container, track);

        let fill = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(160.0), height: Sizing::Fixed(4.0),
            background: Some(accent), corner_radius: 2.0, ..UiStyle::default()
        });
        tree.add_child(track, fill);

        let pct = tree.create(Widget::Label { text: "Loading 60%…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(container, pct);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
