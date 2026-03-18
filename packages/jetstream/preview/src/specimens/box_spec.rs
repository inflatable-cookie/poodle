//! Box specimen — demonstrates padding variants and overflow.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

/// Render the Box specimen showing padding variants.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(16.0),
        ),
        NodeStyle::default(),
    );

    // ── Padding variants ──
    let label = tree.create_node(
        Widget::Label {
            text: "Padding Variants".to_string(),
        },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, label);

    let row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_gap(12.0),
        ),
        NodeStyle::default(),
    );
    tree.add_child(root, row);

    let paddings: &[(&str, f32)] = &[
        ("None", 0.0),
        ("Sm (4px)", 4.0),
        ("Md (8px)", 8.0),
        ("Lg (16px)", 16.0),
        ("Xl (24px)", 24.0),
    ];

    for &(name, pad) in paddings {
        let card = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_padding(LayoutEdges::uniform(pad)),
            ),
            NodeStyle {
                background: Some(bg_surface),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(row, card);

        let inner = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_width(LayoutSizing::Fixed(60.0))
                    .with_height(LayoutSizing::Fixed(32.0))
                    .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                background: Some(theme_bridge::tint(accent, 0.20)),
                corner_radii: [4.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(card, inner);

        let lbl = tree.create_node(
            Widget::Label {
                text: name.to_string(),
            },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_primary),
                text_size: Some(10.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(inner, lbl);
    }

    // ── Overflow demo ──
    let overflow_label = tree.create_node(
        Widget::Label {
            text: "Overflow: Hidden".to_string(),
        },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_padding(LayoutEdges { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 }),
        ),
        NodeStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, overflow_label);

    let overflow_box = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(200.0))
                .with_height(LayoutSizing::Fixed(60.0))
                .with_padding(LayoutEdges::uniform(8.0))
                .with_overflow(pug_layout::LayoutOverflow::Hidden, pug_layout::LayoutOverflow::Scroll),
        ),
        NodeStyle {
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, overflow_box);

    // Content wider than container
    let wide_content = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(400.0))
                .with_height(LayoutSizing::Fixed(40.0))
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
        ),
        NodeStyle {
            background: Some(theme_bridge::tint(accent, 0.15)),
            corner_radii: [4.0; 4],
            ..NodeStyle::default()
        },
    );
    tree.add_child(overflow_box, wide_content);

    let clip_label = tree.create_node(
        Widget::Label {
            text: "400px content in 200px box (clipped)".to_string(),
        },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle {
            text_color: Some(text_primary),
            text_size: Some(10.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(wide_content, clip_label);

    root
}
