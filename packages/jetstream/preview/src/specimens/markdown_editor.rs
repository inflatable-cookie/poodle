//! MarkdownEditor specimen — simplified text area with formatting toolbar.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    section_label(tree, root, "Markdown Editor", text_secondary);
    {
        let editor = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(420.0),
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, editor);

        // Toolbar
        let toolbar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 6.0, right: 8.0, bottom: 6.0, left: 8.0 },
            gap: 4.0, background: Some(bg_surface),
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(editor, toolbar);

        for &btn_label in &["B", "I", "U", "H1", "H2", "—", "•", "1.", "🔗", "📷"] {
            let btn = tree.create(Widget::Button {
                label: btn_label.to_string(), pressed: false, hovered: false,
            }, UiStyle {
                width: Sizing::Fixed(28.0), height: Sizing::Fixed(28.0),
                corner_radius: 4.0,
                text_color: Some(text_primary), text_size: Some(11.0),
                align: Align::Center, justify: Justify::Center,
                focusable: true, ..UiStyle::default()
            });
            tree.add_child(toolbar, btn);
        }

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(editor, sep);

        // Text area
        let area = tree.create(Widget::Panel, UiStyle {
            padding: Edges::all(12.0),
            height: Sizing::Fixed(160.0),
            ..UiStyle::default()
        });
        tree.add_child(editor, area);

        let content = tree.create(Widget::Label {
            text: "# Project Overview\n\nThis document describes the **main goals** and _key milestones_ for the upcoming sprint.\n\n- Finalize API design\n- Build prototype\n- Run user tests".to_string(),
        }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(area, content);

        // Footer
        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 6.0, right: 12.0, bottom: 6.0, left: 12.0 },
            justify: Justify::SpaceBetween, align: Align::Center,
            background: Some(bg_surface),
            ..UiStyle::default()
        });
        tree.add_child(editor, footer);

        let hint = tree.create(Widget::Label { text: "Markdown supported".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(footer, hint);

        let chars = tree.create(Widget::Label { text: "247 characters".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(footer, chars);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
