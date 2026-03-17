//! EmbedShell specimen — container managing input and preview states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Input state ──
    section_label(tree, root, "Input State (no embed yet)", text_secondary);
    {
        let shell = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, width: Sizing::Fixed(380.0),
            padding: Edges::all(16.0), gap: 12.0,
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, shell);

        let title = tree.create(Widget::Label { text: "Add Embed".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(shell, title);

        let input = tree.create(Widget::Panel, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, input);

        let placeholder = tree.create(Widget::Label { text: "Paste URL here…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(input, placeholder);

        let hint = tree.create(Widget::Label { text: "Supports YouTube, Vimeo, SoundCloud, and web links".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.7)), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(shell, hint);
    }

    // ── Preview state ──
    section_label(tree, root, "Preview State (embed resolved)", text_secondary);
    {
        let shell = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, width: Sizing::Fixed(380.0),
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radii: [8.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, shell);

        // Preview card
        let thumb = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(80.0),
            background: Some(theme_bridge::tint(border, 0.5)),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, thumb);

        let play = tree.create(Widget::Label { text: "▶".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_primary, 0.4)), text_size: Some(24.0), ..UiStyle::default()
        });
        tree.add_child(thumb, play);

        let info = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, padding: Edges::all(12.0),
            justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, info);

        let details = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 2.0, ..UiStyle::default()
        });
        tree.add_child(info, details);

        let title = tree.create(Widget::Label { text: "Game Dev Tutorial".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(details, title);

        let meta = tree.create(Widget::Label { text: "youtube.com · 15:22".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(details, meta);

        let remove = tree.create(Widget::Label { text: "✕ Remove".to_string() }, UiStyle {
            text_color: Some(accent), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(info, remove);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
