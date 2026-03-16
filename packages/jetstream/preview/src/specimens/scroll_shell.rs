//! ScrollShell specimen — demonstrates vertical scrolling with overflow content.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

/// Render the ScrollShell specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    let label = tree.create(Widget::Label {
        text: "Scrollable container (200px height, 20 items)".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(root, label);

    // Scrollable list
    let scroll = tree.create(Widget::List { scroll_offset: 0.0 }, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(300.0),
        height: Sizing::Fixed(200.0),
        padding: Edges::all(4.0),
        gap: 2.0,
        overflow: Overflow::Scroll,
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(root, scroll);

    for i in 0..20 {
        let item = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            align: Align::Center,
            gap: 8.0,
            background: if i % 2 == 0 {
                Some(theme_bridge::tint(accent, 0.06))
            } else {
                None
            },
            corner_radius: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(scroll, item);

        let idx = tree.create(Widget::Label {
            text: format!("{:>2}", i + 1),
        }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(item, idx);

        let item_text = tree.create(Widget::Label {
            text: format!("Scroll item {}", i + 1),
        }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(11.0),
            ..UiStyle::default()
        });
        tree.add_child(item, item_text);
    }

    root
}
