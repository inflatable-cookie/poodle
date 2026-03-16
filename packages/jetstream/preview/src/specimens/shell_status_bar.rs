//! ShellStatusBar specimen — status bar with left/center/right slots.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Full status bar ──
    section_label(tree, root, "Full Status Bar", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(520.0), height: Sizing::Fixed(24.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 6.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        // Left slot — connection status
        let left = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, align: Align::Center,
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(bar, left);

        let dot = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(6.0), height: Sizing::Fixed(6.0),
            corner_radius: 3.0, background: Some(success),
            ..UiStyle::default()
        });
        tree.add_child(left, dot);

        let conn = tree.create(Widget::Label { text: "Connected".to_string() }, UiStyle {
            text_color: Some(success), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(left, conn);

        // Center slot — project name
        let center = tree.create(Widget::Panel, UiStyle {
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(bar, center);

        let project = tree.create(Widget::Label { text: "my-project".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(center, project);

        // Right slot — fps + version
        let right = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 10.0, align: Align::Center,
            width: Sizing::Grow(1.0), justify: Justify::End,
            ..UiStyle::default()
        });
        tree.add_child(bar, right);

        let fps = tree.create(Widget::Label { text: "60 fps".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(right, fps);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(1.0), height: Sizing::Fixed(12.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(right, sep);

        let version = tree.create(Widget::Label { text: "v0.1.0".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(right, version);
    }

    // ── Minimal status bar ──
    section_label(tree, root, "Minimal Status Bar", text_secondary);
    {
        let bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(520.0), height: Sizing::Fixed(20.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 4.0, align: Align::Center, justify: Justify::SpaceBetween,
            ..UiStyle::default()
        });
        tree.add_child(root, bar);

        let ready = tree.create(Widget::Label { text: "Ready".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(bar, ready);

        let info = tree.create(Widget::Label { text: "Ln 42, Col 8".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(bar, info);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
