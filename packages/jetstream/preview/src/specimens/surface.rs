//! Surface specimen — demonstrates tone variants and border options.
//!
//! Uses adapter-driven rendering: creates `SurfaceSpec` instances and passes
//! them through the adapter to get resolved styles.

use jetstream_runtime::game_ui::*;
use pug_adapter::{RenderComponent, ThemeProvider};
use pug_jetstream::JetstreamAdapter;
use pug_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing,
    MainAxisAlignment,
};
use pug_primitives::SurfaceSpec;
use pug_style::StyleDescriptor;
use pug_tokens::typed::ColorValue;

use crate::render_bridge;
use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let adapter = JetstreamAdapter::new(pug_jetstream::JetstreamThemeProvider::default());
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

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

    // ── Tone variants ──
    section_label(tree, root, "Surface Tones", text_secondary);

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

    let tones: &[(&str, &str)] = &[
        ("Canvas", "semantic.color.background.canvas"),
        ("Panel", "semantic.color.background.panel"),
        ("Elevated", "semantic.color.background.elevated"),
        ("Surface", "semantic.color.background.surface"),
    ];

    for &(name, token) in tones {
        // Create a SurfaceSpec and resolve through the adapter
        let spec = SurfaceSpec::new().with_label(name);
        let bg_color = theme.resolve_color(token);
        let style = StyleDescriptor::new()
            .with_background(bg_color)
            .with_border(1.0, theme.resolve_color("semantic.color.border.subtle"))
            .with_corner_radii(pug_style::CornerRadii::uniform(8.0))
            .with_layout(
                LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(80.0))
                    .with_padding(LayoutEdges::uniform(12.0))
                    .with_gap(4.0),
            );
        let handle = adapter.render(&spec, &style, theme);
        let card = render_bridge::materialize(tree, &handle);
        tree.add_child(row, card);

        // Add child labels
        let name_lbl = tree.create_node(
            Widget::Label { text: name.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_primary),
                text_size: Some(12.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(card, name_lbl);

        let token_lbl = tree.create_node(
            Widget::Label {
                text: token.rsplit('.').next().unwrap_or(token).to_string(),
            },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_secondary),
                text_size: Some(10.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(card, token_lbl);
    }

    // ── Border variants ──
    section_label(tree, root, "Border Variants", text_secondary);

    let border_row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_gap(12.0),
        ),
        NodeStyle::default(),
    );
    tree.add_child(root, border_row);

    let bg_surface = theme.resolve_color("semantic.color.background.surface");
    let border_subtle = theme.resolve_color("semantic.color.border.subtle");
    let border_default = theme.resolve_color("semantic.color.border.default");

    let borders: &[(&str, Option<ColorValue>, f32)] = &[
        ("None", None, 0.0),
        ("Subtle", Some(border_subtle), 1.0),
        ("Default", Some(border_default), 1.0),
        ("Bold", Some(border_default), 2.0),
    ];

    for &(name, ref color, width) in borders {
        let spec = SurfaceSpec::new().with_label(name);
        let mut style = StyleDescriptor::new()
            .with_background(bg_surface)
            .with_corner_radii(pug_style::CornerRadii::uniform(8.0))
            .with_layout(
                LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(60.0))
                    .with_padding(LayoutEdges::uniform(12.0))
                    .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
            );
        if let Some(c) = color {
            style = style.with_border(width, *c);
        }
        let handle = adapter.render(&spec, &style, theme);
        let card = render_bridge::materialize(tree, &handle);
        tree.add_child(border_row, card);

        let lbl = tree.create_node(
            Widget::Label { text: name.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_primary),
                text_size: Some(11.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(card, lbl);
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
