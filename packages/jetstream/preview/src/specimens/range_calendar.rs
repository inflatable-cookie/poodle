//! RangeCalendar specimen — calendar grid with range selection highlighting.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Range selected (10–20) ──
    section_label(tree, root, "Range Selected (10th \u{2013} 20th)", text_secondary);
    {
        let cal = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Fixed(232.0))
            .with_padding(LayoutEdges { top: 0.0, right: 0.0, bottom: 8.0, left: 0.0 })),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, cal);

        // Header
        let nav = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_padding(LayoutEdges { top: 12.0, right: 12.0, bottom: 8.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(cal, nav);

        let month = tree.create_node(Widget::Label { text: "March 2026".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(nav, month);

        // DOW headers
        let dow_row = grid_row(tree, cal);
        for day in &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
            day_header(tree, dow_row, day, text_secondary);
        }

        // Weeks — show range 10-20 highlighted
        let range = 10..=20_u8;
        let weeks: &[&[u8]] = &[
            &[1, 2, 3, 4, 5, 6, 7],
            &[8, 9, 10, 11, 12, 13, 14],
            &[15, 16, 17, 18, 19, 20, 21],
            &[22, 23, 24, 25, 26, 27, 28],
        ];

        let range_bg = theme_bridge::tint(accent, 0.15);

        for week in weeks {
            let row = grid_row(tree, cal);
            for &d in *week {
                let is_start = d == 10;
                let is_end = d == 20;
                let in_range = range.contains(&d);

                let bg = if is_start || is_end {
                    Some(accent)
                } else if in_range {
                    Some(range_bg)
                } else {
                    None
                };

                let fg = if is_start || is_end {
                    text_inverse
                } else {
                    text_primary
                };

                let cell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Fixed(28.0))
                    .with_height(LayoutSizing::Fixed(28.0))
                    .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                    NodeStyle {
                        corner_radii: if is_start || is_end { [14.0; 4] } else { [4.0; 4] },
                        background: bg,
                        ..NodeStyle::default()
                    });
                tree.add_child(row, cell);

                let lbl = tree.create_node(Widget::Label { text: d.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
                    NodeStyle { text_color: Some(fg), text_size: Some(11.0), ..NodeStyle::default() });
                tree.add_child(cell, lbl);
            }
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn grid_row(tree: &mut UiTree, parent: UiNodeId) -> UiNodeId {
    let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
        .with_gap(2.0)
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Stretch)),
        NodeStyle::default());
    tree.add_child(parent, row);
    row
}

fn day_header(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let cell = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(28.0))
        .with_height(LayoutSizing::Fixed(24.0))
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle::default());
    tree.add_child(parent, cell);

    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(10.0), ..NodeStyle::default() });
    tree.add_child(cell, lbl);
}
