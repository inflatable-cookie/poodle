//! CommandPaletteShell specimen — manages open/close activation state.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Closed state ──
    section_label(tree, root, "Closed (trigger shown)", text_secondary);
    {
        let trigger = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            width: Sizing::Fixed(200.0), height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, trigger);

        let icon = tree.create(Widget::Label { text: "🔍".to_string() }, UiStyle {
            text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(trigger, icon);

        let lbl = tree.create(Widget::Label { text: "Search…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0),
            width: Sizing::Grow(1.0), ..UiStyle::default()
        });
        tree.add_child(trigger, lbl);

        let kbd = tree.create(Widget::Label { text: "⌘K".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(trigger, kbd);
    }

    // ── Open state (with backdrop) ──
    section_label(tree, root, "Open (with backdrop)", text_secondary);
    {
        let backdrop = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(400.0), height: Sizing::Fixed(240.0),
            background: Some(glam::Vec4::new(bg_canvas.x * 0.5, bg_canvas.y * 0.5, bg_canvas.z * 0.5, 0.8)),
            corner_radius: 8.0,
            align: Align::Start, justify: Justify::Center,
            padding: Edges { top: 24.0, right: 40.0, bottom: 0.0, left: 40.0 },
            ..UiStyle::default()
        });
        tree.add_child(root, backdrop);

        let palette = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(320.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 10.0,
            ..UiStyle::default()
        });
        tree.add_child(backdrop, palette);

        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0,
            padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(palette, search);

        let icon = tree.create(Widget::Label { text: "🔍".to_string() }, UiStyle {
            text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(search, icon);

        let input = tree.create(Widget::Label { text: "new fi".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(search, input);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(palette, sep);

        let items = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(6.0), gap: 2.0,
            ..UiStyle::default()
        });
        tree.add_child(palette, items);

        for &(cmd, active) in &[("New File", true), ("New Folder", false)] {
            let bg = if active { Some(theme_bridge::tint(accent, 0.10)) } else { None };
            let row = tree.create(Widget::Panel, UiStyle {
                height: Sizing::Fixed(28.0),
                padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
                background: bg, corner_radius: 4.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(items, row);

            let lbl = tree.create(Widget::Label { text: cmd.to_string() }, UiStyle {
                text_color: Some(if active { accent } else { text_primary }),
                text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(row, lbl);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
