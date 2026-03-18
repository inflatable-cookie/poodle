//! Button specimen — demonstrates variant, size, and disabled states.
//!
//! Uses the adapter-driven rendering pattern: creates `ButtonSpec` instances,
//! passes them through `RenderComponent::render()` to get resolved styles,
//! then materializes actual `UiTree` nodes via the render bridge.

use jetstream_runtime::game_ui::*;
use pug_adapter::{RenderComponent, ThemeProvider};
use pug_jetstream::JetstreamAdapter;
use pug_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing,
    MainAxisAlignment,
};
use pug_primitives::{ButtonSpec, ButtonVariant, ControlSize};
use pug_style::StyleDescriptor;

use crate::render_bridge;
use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let adapter = JetstreamAdapter::new(pug_jetstream::JetstreamThemeProvider::default());
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

    // ── Variants ──
    section_label(tree, root, "Button Variants", text_secondary);

    let variants_row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
        ),
        NodeStyle::default(),
    );
    tree.add_child(root, variants_row);

    for (label, variant) in &[
        ("Primary", ButtonVariant::Primary),
        ("Secondary", ButtonVariant::Secondary),
        ("Ghost", ButtonVariant::Ghost),
        ("Danger", ButtonVariant::Danger),
    ] {
        let spec = ButtonSpec::new()
            .with_variant(*variant)
            .with_label(*label);
        let style = button_style(ControlSize::Md);
        let handle = adapter.render(&spec, &style, theme);
        let id = render_bridge::materialize_button(tree, label, &handle.mapped);
        tree.add_child(variants_row, id);
    }

    // ── Sizes ──
    section_label(tree, root, "Button Sizes", text_secondary);

    let sizes_row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::End),
        ),
        NodeStyle::default(),
    );
    tree.add_child(root, sizes_row);

    for (label, size) in &[
        ("Small", ControlSize::Sm),
        ("Medium", ControlSize::Md),
        ("Large", ControlSize::Lg),
    ] {
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Primary)
            .with_size(*size)
            .with_label(*label);
        let style = button_style(*size);
        let handle = adapter.render(&spec, &style, theme);
        let id = render_bridge::materialize_button(tree, label, &handle.mapped);
        tree.add_child(sizes_row, id);
    }

    // ── Disabled state ──
    section_label(tree, root, "Disabled State", text_secondary);

    let disabled_row = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
        ),
        NodeStyle::default(),
    );
    tree.add_child(root, disabled_row);

    // Enabled
    let spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Primary)
        .with_label("Enabled");
    let style = button_style(ControlSize::Md);
    let handle = adapter.render(&spec, &style, theme);
    let id = render_bridge::materialize_button(tree, "Enabled", &handle.mapped);
    tree.add_child(disabled_row, id);

    // Disabled
    let spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Primary)
        .with_disabled(true)
        .with_label("Disabled");
    let style = button_style(ControlSize::Md);
    let handle = adapter.render(&spec, &style, theme);
    let id = render_bridge::materialize_button(tree, "Disabled", &handle.mapped);
    tree.add_child(disabled_row, id);

    root
}

/// Build a `StyleDescriptor` with button-appropriate layout for a given size.
fn button_style(size: ControlSize) -> StyleDescriptor {
    let (height, text_size, pad_h) = match size {
        ControlSize::Sm => (24.0, 11.0, 10.0),
        ControlSize::Md => (32.0, 12.0, 16.0),
        ControlSize::Lg => (40.0, 14.0, 20.0),
    };
    StyleDescriptor::new().with_layout(
        LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_height(LayoutSizing::Fixed(height))
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)
            .with_padding(LayoutEdges::symmetric(pad_h, 0.0)),
    ).with_typography(pug_style::TypographyDescriptor {
        size: text_size,
        ..Default::default()
    })
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
