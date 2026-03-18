//! Grid specimen — demonstrates column layouts emulated via nested rows.
//!
//! Jetstream has no native CSS Grid. Grids are emulated with rows of
//! equal-width panels, matching the adapter's approach.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

/// Render the Grid specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_primary = theme_bridge::text_primary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Column counts ──
    for cols in [2, 3, 4] {
        let label = tree.create_node(Widget::Label {
            text: format!("{} Columns, gap 8px", cols),
        }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(root, label);

        // Two rows of items
        for _ in 0..2 {
            let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_gap(8.0)),
                NodeStyle::default());
            tree.add_child(root, row);

            for i in 0..cols {
                let cell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(40.0))
                    .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                    NodeStyle { background: Some(bg_surface), border_color: Some(border), border_width: 1.0, corner_radii: [4.0; 4], ..NodeStyle::default() });
                tree.add_child(row, cell);

                let cell_label = tree.create_node(Widget::Label {
                    text: format!("Cell {}", i + 1),
                }, pug_jetstream::map_layout(&LayoutIntent::new()),
                    NodeStyle { text_color: Some(text_primary), text_size: Some(10.0), ..NodeStyle::default() });
                tree.add_child(cell, cell_label);
            }
        }
    }

    // ── Mixed-width columns ──
    let mixed_label = tree.create_node(Widget::Label {
        text: "Mixed widths (1fr + 2fr + 1fr)".to_string(),
    }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(root, mixed_label);

    let mixed_row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_gap(8.0)),
        NodeStyle::default());
    tree.add_child(root, mixed_row);

    for &weight in &[1.0_f32, 2.0, 1.0] {
        let cell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(48.0))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { background: Some(theme_bridge::tint(accent, 0.15)), border_color: Some(border), border_width: 1.0, corner_radii: [4.0; 4], ..NodeStyle::default() });
        tree.add_child(mixed_row, cell);

        let w_label = tree.create_node(Widget::Label {
            text: format!("{}fr", weight as i32),
        }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(cell, w_label);
    }

    root
}
