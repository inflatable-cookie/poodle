//! MarkdownEditor specimen — simplified text area with formatting toolbar.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    section_label(tree, root, "Markdown Editor", text_secondary);
    {
        let editor = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(420.0))),
            NodeStyle { background: Some(bg_elevated), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
        tree.add_child(root, editor);

        // Toolbar
        let toolbar = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 6.0, right: 8.0, bottom: 6.0, left: 8.0 })
            .with_gap(4.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(editor, toolbar);

        for &btn_label in &["B", "I", "U", "H1", "H2", "—", "•", "1.", "🔗", "📷"] {
            let btn = tree.create_node(Widget::Button {
                label: btn_label.to_string(), pressed: false, hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(28.0))
                .with_height(LayoutSizing::Fixed(28.0))
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                NodeStyle { corner_radii: [4.0; 4], text_color: Some(text_primary), text_size: Some(11.0), focusable: true, ..NodeStyle::default() });
            tree.add_child(toolbar, btn);
        }

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(editor, sep);

        // Text area
        let area = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_padding(LayoutEdges::uniform(12.0))
            .with_height(LayoutSizing::Fixed(160.0))),
            NodeStyle::default());
        tree.add_child(editor, area);

        let content = tree.create_node(Widget::Label {
            text: "# Project Overview\n\nThis document describes the **main goals** and _key milestones_ for the upcoming sprint.\n\n- Finalize API design\n- Build prototype\n- Run user tests".to_string(),
        }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(area, content);

        // Footer
        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_padding(LayoutEdges { top: 6.0, right: 12.0, bottom: 6.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(bg_surface), ..NodeStyle::default() });
        tree.add_child(editor, footer);

        let hint = tree.create_node(Widget::Label { text: "Markdown supported".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(footer, hint);

        let chars = tree.create_node(Widget::Label { text: "247 characters".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(footer, chars);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
