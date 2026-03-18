//! EmbedPreview specimen — provider card with metadata.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(20.0)),
        NodeStyle::default());

    section_label(tree, root, "Video Embed", text_secondary);
    embed_card(tree, root, "▶ YouTube", "How to Build a Game Engine", "youtube.com · 12:34 · 1.2M views", bg_elevated, border, accent, text_primary, text_secondary);

    section_label(tree, root, "Link Embed", text_secondary);
    embed_card(tree, root, "🔗 Web", "Understanding ECS Architecture", "blog.example.com · Article · 8 min read", bg_elevated, border, accent, text_primary, text_secondary);

    section_label(tree, root, "Audio Embed", text_secondary);
    embed_card(tree, root, "♪ SoundCloud", "Ambient Soundtrack — Forest Theme", "soundcloud.com · 3:42", bg_elevated, border, accent, text_primary, text_secondary);

    root
}

fn embed_card(tree: &mut UiTree, parent: UiNodeId, provider: &str, title: &str, meta: &str, bg: glam::Vec4, border: glam::Vec4, provider_color: glam::Vec4, title_color: glam::Vec4, meta_color: glam::Vec4) {
    let card = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(360.0))),
        NodeStyle { background: Some(bg), border_color: Some(border), border_width: 1.0, corner_radii: [8.0; 4], ..NodeStyle::default() });
    tree.add_child(parent, card);

    // Placeholder thumbnail
    let thumb = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(80.0))
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle { background: Some(theme_bridge::tint(border, 0.5)), ..NodeStyle::default() });
    tree.add_child(card, thumb);

    let play = tree.create_node(Widget::Label { text: "▶".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(theme_bridge::tint(title_color, 0.4)), text_size: Some(24.0), ..NodeStyle::default() });
    tree.add_child(thumb, play);

    let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column).with_padding(LayoutEdges::uniform(12.0)).with_gap(4.0)),
        NodeStyle::default());
    tree.add_child(card, body);

    let p = tree.create_node(Widget::Label { text: provider.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(provider_color), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(body, p);

    let t = tree.create_node(Widget::Label { text: title.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(title_color), text_size: Some(13.0), ..NodeStyle::default() });
    tree.add_child(body, t);

    let m = tree.create_node(Widget::Label { text: meta.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(meta_color), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(body, m);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
