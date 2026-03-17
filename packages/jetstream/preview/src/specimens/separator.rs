//! Separator specimen — demonstrates horizontal/vertical and tone variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

/// Render the Separator specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border_subtle = theme_bridge::border_subtle(theme);
    let border_default = theme_bridge::border_default(theme);
    let bg_surface = theme_bridge::surface_background(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Horizontal separators ──
    let h_label = tree.create(Widget::Label {
        text: "Horizontal Separators".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(root, h_label);

    let h_demo = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        padding: Edges::all(16.0),
        gap: 12.0,
        background: Some(bg_surface),
        border_color: Some(border_subtle),
        border_width: 1.0,
        corner_radii: [8.0; 4],
        ..UiStyle::default()
    });
    tree.add_child(root, h_demo);

    let above = tree.create(Widget::Label {
        text: "Content above".to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(12.0),
        ..UiStyle::default()
    });
    tree.add_child(h_demo, above);

    // Subtle separator
    let sep_subtle = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(1.0),
        background: Some(border_subtle),
        ..UiStyle::default()
    });
    tree.add_child(h_demo, sep_subtle);

    let between = tree.create(Widget::Label {
        text: "Subtle tone ↑ / Default tone ↓".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(10.0),
        ..UiStyle::default()
    });
    tree.add_child(h_demo, between);

    // Default separator
    let sep_default = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(1.0),
        background: Some(border_default),
        ..UiStyle::default()
    });
    tree.add_child(h_demo, sep_default);

    let below = tree.create(Widget::Label {
        text: "Content below".to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(12.0),
        ..UiStyle::default()
    });
    tree.add_child(h_demo, below);

    // ── Vertical separators ──
    let v_label = tree.create(Widget::Label {
        text: "Vertical Separators".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(root, v_label);

    let v_demo = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        height: Sizing::Fixed(60.0),
        padding: Edges::all(16.0),
        gap: 12.0,
        align: Align::Center,
        background: Some(bg_surface),
        border_color: Some(border_subtle),
        border_width: 1.0,
        corner_radii: [8.0; 4],
        ..UiStyle::default()
    });
    tree.add_child(root, v_demo);

    let left = tree.create(Widget::Label {
        text: "Left".to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(12.0),
        ..UiStyle::default()
    });
    tree.add_child(v_demo, left);

    let v_sep = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(1.0),
        height: Sizing::Grow(1.0),
        background: Some(border_subtle),
        ..UiStyle::default()
    });
    tree.add_child(v_demo, v_sep);

    let mid = tree.create(Widget::Label {
        text: "Middle".to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(12.0),
        ..UiStyle::default()
    });
    tree.add_child(v_demo, mid);

    let v_sep2 = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(1.0),
        height: Sizing::Grow(1.0),
        background: Some(border_default),
        ..UiStyle::default()
    });
    tree.add_child(v_demo, v_sep2);

    let right = tree.create(Widget::Label {
        text: "Right".to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(12.0),
        ..UiStyle::default()
    });
    tree.add_child(v_demo, right);

    root
}
