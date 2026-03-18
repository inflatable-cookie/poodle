//! BlockEditor specimen — block-based content editor (simplified for game context).

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(20.0),
        ),
        NodeStyle::default(),
    );

    section_label(tree, root, "Block Editor (Simplified)", text_secondary);
    {
        let editor = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Fixed(420.0))
                    .with_gap(2.0)
                    .with_padding(LayoutEdges::uniform(12.0)),
            ),
            NodeStyle {
                background: Some(bg_surface),
                border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, editor);

        // Heading block
        block_item(tree, editor, "H", "Introduction", bg_elevated, border, accent, text_primary);
        // Paragraph block
        block_item(tree, editor, "¶", "Lorem ipsum dolor sit amet, consectetur adipiscing elit.", bg_elevated, border, text_secondary, text_primary);
        // Image block placeholder
        block_item(tree, editor, "📷", "hero-image.png (1200×600)", bg_elevated, border, text_secondary, text_primary);
        // List block
        block_item(tree, editor, "•", "First item\nSecond item\nThird item", bg_elevated, border, text_secondary, text_primary);

        // Add block button
        let add = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(6.0)
                    .with_padding(LayoutEdges { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 })
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(editor, add);

        let plus = tree.create_node(
            Widget::Label { text: "+ Add block".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(11.0), ..NodeStyle::default() },
        );
        tree.add_child(add, plus);
    }

    // Note about simplification
    let note = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_padding(LayoutEdges::uniform(10.0))
                .with_gap(6.0),
        ),
        NodeStyle {
            background: Some(theme_bridge::tint(accent, 0.08)),
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, note);

    let note_text = tree.create_node(
        Widget::Label {
            text: "ℹ Simplified for Jetstream: no drag-reorder or inline editing. Use MarkdownEditor for rich text.".to_string(),
        },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() },
    );
    tree.add_child(note, note_text);

    root
}

fn block_item(tree: &mut UiTree, parent: UiNodeId, icon: &str, content: &str, bg: glam::Vec4, border: glam::Vec4, icon_color: glam::Vec4, text_color: glam::Vec4) {
    let block = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_padding(LayoutEdges::uniform(10.0))
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Start),
        ),
        NodeStyle {
            background: Some(bg),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        },
    );
    tree.add_child(parent, block);

    let ic = tree.create_node(
        Widget::Label { text: icon.to_string() },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(20.0)),
        ),
        NodeStyle { text_color: Some(icon_color), text_size: Some(12.0), ..NodeStyle::default() },
    );
    tree.add_child(block, ic);

    let txt = tree.create_node(
        Widget::Label { text: content.to_string() },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_width(LayoutSizing::Grow),
        ),
        NodeStyle { text_color: Some(text_color), text_size: Some(12.0), ..NodeStyle::default() },
    );
    tree.add_child(block, txt);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() },
    );
    tree.add_child(parent, lbl);
}
