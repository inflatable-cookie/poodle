//! RadioGroup specimen — single-selection group with selected and disabled states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_default(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    label(tree, root, "Radio Group (second selected)", text_secondary);

    let options = ["Option A", "Option B (selected)", "Option C", "Option D"];
    for (i, &opt) in options.iter().enumerate() {
        let selected = i == 1;
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        let radio = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(18.0))
            .with_height(LayoutSizing::Fixed(18.0))),
            NodeStyle {
                corner_radii: [9.0; 4],
                border_color: Some(if selected { accent } else { border }),
                border_width: if selected { 5.0 } else { 1.5 },
                ..NodeStyle::default()
            });
        tree.add_child(row, radio);

        let lbl = tree.create_node(Widget::Label { text: opt.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(row, lbl);
    }

    // ── Disabled ──
    label(tree, root, "Disabled Group", text_secondary);

    for (i, &opt) in ["Enabled", "Disabled selected"].iter().enumerate() {
        let selected = i == 1;
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { opacity: 0.5, ..NodeStyle::default() });
        tree.add_child(root, row);

        let radio = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(18.0))
            .with_height(LayoutSizing::Fixed(18.0))),
            NodeStyle {
                corner_radii: [9.0; 4],
                border_color: Some(if selected { accent } else { border }),
                border_width: if selected { 5.0 } else { 1.5 },
                ..NodeStyle::default()
            });
        tree.add_child(row, radio);

        let lbl = tree.create_node(Widget::Label { text: opt.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(row, lbl);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
