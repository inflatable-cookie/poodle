//! EmbedPreview specimen — provider card with metadata.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    section_label(tree, root, "Video Embed", text_secondary);
    embed_card(tree, root, "▶ YouTube", "How to Build a Game Engine", "youtube.com · 12:34 · 1.2M views", bg_elevated, border, accent, text_primary, text_secondary);

    section_label(tree, root, "Link Embed", text_secondary);
    embed_card(tree, root, "🔗 Web", "Understanding ECS Architecture", "blog.example.com · Article · 8 min read", bg_elevated, border, accent, text_primary, text_secondary);

    section_label(tree, root, "Audio Embed", text_secondary);
    embed_card(tree, root, "♪ SoundCloud", "Ambient Soundtrack — Forest Theme", "soundcloud.com · 3:42", bg_elevated, border, accent, text_primary, text_secondary);

    root
}

fn embed_card(tree: &mut UiTree, parent: UiNodeId, provider: &str, title: &str, meta: &str, bg: glam::Vec4, border: glam::Vec4, provider_color: glam::Vec4, title_color: glam::Vec4, meta_color: glam::Vec4) {
    let card = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(360.0),
        background: Some(bg), border_color: Some(border), border_width: 1.0,
        corner_radius: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, card);

    // Placeholder thumbnail
    let thumb = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Grow(1.0), height: Sizing::Fixed(80.0),
        background: Some(theme_bridge::tint(border, 0.5)),
        align: Align::Center, justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(card, thumb);

    let play = tree.create(Widget::Label { text: "▶".to_string() }, UiStyle {
        text_color: Some(theme_bridge::tint(title_color, 0.4)), text_size: Some(24.0), ..UiStyle::default()
    });
    tree.add_child(thumb, play);

    let body = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, padding: Edges::all(12.0), gap: 4.0,
        ..UiStyle::default()
    });
    tree.add_child(card, body);

    let p = tree.create(Widget::Label { text: provider.to_string() }, UiStyle {
        text_color: Some(provider_color), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(body, p);

    let t = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(title_color), text_size: Some(13.0), ..UiStyle::default()
    });
    tree.add_child(body, t);

    let m = tree.create(Widget::Label { text: meta.to_string() }, UiStyle {
        text_color: Some(meta_color), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(body, m);
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
