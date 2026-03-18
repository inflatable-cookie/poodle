//! ScrollShell specimen — demonstrates vertical scrolling with overflow content.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

/// Render the ScrollShell specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    let label = tree.create_node(Widget::Label {
        text: "Scrollable container (200px height, 20 items)".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, label);

    // Scrollable list
    let scroll = tree.create_node(Widget::List { scroll_offset: 0.0 }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Fixed(300.0))
        .with_height(LayoutSizing::Fixed(200.0))
        .with_padding(LayoutEdges::uniform(4.0))
        .with_gap(2.0)
        .with_overflow(pug_layout::LayoutOverflow::Hidden, pug_layout::LayoutOverflow::Scroll)),
        NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [8.0; 4],
            ..NodeStyle::default()
        });
    tree.add_child(root, scroll);

    for i in 0..20 {
        let item = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(28.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: if i % 2 == 0 {
                    Some(theme_bridge::tint(accent, 0.06))
                } else {
                    None
                },
                corner_radii: [4.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(scroll, item);

        let idx = tree.create_node(Widget::Label {
            text: format!("{:>2}", i + 1),
        }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(item, idx);

        let item_text = tree.create_node(Widget::Label {
            text: format!("Scroll item {}", i + 1),
        }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(item, item_text);
    }

    root
}
