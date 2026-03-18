//! Stack specimen — demonstrates direction, gap, and alignment variants.
//!
//! Uses adapter-driven rendering: creates `StackSpec` instances and passes
//! them through the adapter to get resolved styles.

use jetstream_runtime::game_ui::*;
use pug_adapter::{RenderComponent, ThemeProvider};
use pug_jetstream::JetstreamAdapter;
use pug_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing,
    MainAxisAlignment,
};
use pug_primitives::StackSpec;
use pug_style::StyleDescriptor;

use crate::render_bridge;
use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let adapter = JetstreamAdapter::new(pug_jetstream::JetstreamThemeProvider::default());
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let _bg_surface = theme_bridge::surface_background(theme);
    let _border = theme_bridge::border_subtle(theme);

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

    // ── Vertical stack with gap variants ──
    section_label(tree, root, "Vertical Stack — Gap Variants", text_secondary);

    let row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_gap(16.0),
        ),
        NodeStyle::default(),
    );
    tree.add_child(root, row);

    for &(label, gap) in &[("gap: 0", 0.0), ("gap: 4", 4.0), ("gap: 8", 8.0), ("gap: 16", 16.0)] {
        let spec = StackSpec::new();
        let style = StyleDescriptor::new()
            .with_background(theme.resolve_color("semantic.color.background.surface"))
            .with_border(1.0, theme.resolve_color("semantic.color.border.subtle"))
            .with_corner_radii(pug_style::CornerRadii::uniform(6.0))
            .with_layout(
                LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_gap(gap)
                    .with_padding(LayoutEdges::uniform(8.0)),
            );
        let handle = adapter.render(&spec, &style, theme);
        let stack = render_bridge::materialize(tree, &handle);
        tree.add_child(row, stack);

        // Title
        let title = tree.create_node(
            Widget::Label { text: label.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_secondary),
                text_size: Some(10.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(stack, title);

        for _ in 0..3 {
            let item = swatch(tree, 48.0, 20.0, accent);
            tree.add_child(stack, item);
        }
    }

    // ── Horizontal stack ──
    section_label(tree, root, "Horizontal Stack", text_secondary);

    let h_style = StyleDescriptor::new()
        .with_background(theme.resolve_color("semantic.color.background.surface"))
        .with_border(1.0, theme.resolve_color("semantic.color.border.subtle"))
        .with_corner_radii(pug_style::CornerRadii::uniform(6.0))
        .with_layout(
            LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_padding(LayoutEdges::uniform(8.0)),
        );
    let h_handle = adapter.render(&StackSpec::new(), &h_style, theme);
    let h_stack = render_bridge::materialize(tree, &h_handle);
    tree.add_child(root, h_stack);

    for _ in 0..5 {
        let item = swatch(tree, 48.0, 28.0, accent);
        tree.add_child(h_stack, item);
    }

    // ── Alignment variants ──
    section_label(tree, root, "Alignment Variants (vertical stack, 120px tall)", text_secondary);

    let align_row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_gap(12.0),
        ),
        NodeStyle::default(),
    );
    tree.add_child(root, align_row);

    let aligns: &[(&str, CrossAxisAlignment)] = &[
        ("Start", CrossAxisAlignment::Start),
        ("Center", CrossAxisAlignment::Center),
        ("End", CrossAxisAlignment::End),
    ];

    for &(name, align) in aligns {
        let style = StyleDescriptor::new()
            .with_background(theme.resolve_color("semantic.color.background.surface"))
            .with_border(1.0, theme.resolve_color("semantic.color.border.subtle"))
            .with_corner_radii(pug_style::CornerRadii::uniform(6.0))
            .with_layout(
                LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Fixed(100.0))
                    .with_height(LayoutSizing::Fixed(120.0))
                    .with_gap(4.0)
                    .with_padding(LayoutEdges::uniform(8.0))
                    .with_alignment(MainAxisAlignment::Start, align),
            );
        let handle = adapter.render(&StackSpec::new(), &style, theme);
        let col = render_bridge::materialize(tree, &handle);
        tree.add_child(align_row, col);

        let lbl = tree.create_node(
            Widget::Label { text: name.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_primary),
                text_size: Some(10.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(col, lbl);

        let s1 = swatch(tree, 60.0, 16.0, accent);
        let s2 = swatch(tree, 40.0, 16.0, accent);
        let s3 = swatch(tree, 50.0, 16.0, accent);
        tree.add_child(col, s1);
        tree.add_child(col, s2);
        tree.add_child(col, s3);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let id = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle {
            text_color: Some(color),
            text_size: Some(11.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(parent, id);
}

fn swatch(tree: &mut UiTree, w: f32, h: f32, color: glam::Vec4) -> UiNodeId {
    tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(w))
                .with_height(LayoutSizing::Fixed(h)),
        ),
        NodeStyle {
            background: Some(theme_bridge::tint(color, 0.25)),
            corner_radii: [3.0; 4],
            ..NodeStyle::default()
        },
    )
}
