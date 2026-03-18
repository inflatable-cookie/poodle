//! Render bridge — converts adapter output into Jetstream UiTree nodes.
//!
//! This is the glue between the Pug adapter layer (which produces
//! `JetstreamNodeHandle` with resolved `taffy::Style` + `JetstreamVisuals`)
//! and the Jetstream runtime (which needs `Widget` + `taffy::Style` + `NodeStyle`
//! to create actual `UiNode` entries in the `UiTree`).
//!
//! The bridge handles:
//! - Converting `JetstreamVisuals` → `NodeStyle`
//! - sRGB → linear color space conversion (Pug tokens are sRGB, GPU wants linear)
//! - Mapping `WidgetKind` → `Widget` (with text/label from the handle)

use glam::Vec4;
use jetstream_runtime::game_ui::{BoxShadow, NodeStyle, Overflow, TextAlign, TextOverflow, UiNodeId, UiTree, Widget};
use pug_jetstream::{JetstreamColor, JetstreamMappedStyle, JetstreamNodeHandle, WidgetKind};

/// Convert a single sRGB component to linear light.
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert a `JetstreamColor` (sRGB) to a `Vec4` in linear space.
fn color_to_linear(c: &JetstreamColor) -> Vec4 {
    Vec4::new(
        srgb_to_linear(c.0),
        srgb_to_linear(c.1),
        srgb_to_linear(c.2),
        c.3, // alpha is linear
    )
}

/// Convert `JetstreamVisuals` into a `NodeStyle` with linear-space colors.
fn visuals_to_node_style(v: &pug_jetstream::JetstreamVisuals) -> NodeStyle {
    NodeStyle {
        background: v.background.as_ref().map(color_to_linear),
        border_color: v.border_color.as_ref().map(color_to_linear),
        border_width: v.border_width,
        corner_radii: v.corner_radii,
        opacity: v.opacity,
        shadow: v.shadow.map(|s| BoxShadow {
            offset_x: s.offset_x,
            offset_y: s.offset_y,
            blur: s.blur,
            spread: s.spread,
            color: color_to_linear(&s.color),
        }),
        background_gradient: None,
        z_index: v.z_index,
        overflow: Overflow::Visible,
        text_color: v.text_color.as_ref().map(color_to_linear),
        text_size: v.text_size,
        text_align: TextAlign::Left,
        text_overflow: TextOverflow::Visible,
        visible: v.visible,
        disabled: v.disabled,
        focusable: false,
        cursor_style: Default::default(),
        transform: Default::default(),
        transform_origin: Default::default(),
        transitions: vec![],
        token_key: None,
    }
}

/// Convert a `WidgetKind` + handle info into a concrete `Widget`.
fn widget_from_handle(handle: &JetstreamNodeHandle) -> Widget {
    match handle.widget_kind {
        WidgetKind::Panel => Widget::Panel,
        WidgetKind::Label => Widget::Label {
            text: handle.node_id.clone(),
        },
        WidgetKind::Button => Widget::Button {
            label: handle.node_id.clone(),
            pressed: false,
            hovered: false,
        },
        WidgetKind::Slider => Widget::Slider {
            value: 0.5,
            min: 0.0,
            max: 1.0,
        },
        WidgetKind::ProgressBar => Widget::ProgressBar { value: 0.5 },
        WidgetKind::Image => Widget::Image {
            path: String::new(),
            texture_id: None,
        },
        WidgetKind::List => Widget::List {
            scroll_offset: 0.0,
        },
        WidgetKind::TextInput => Widget::TextInput {
            text: String::new(),
            placeholder: String::new(),
            cursor: 0,
            selection_start: None,
        },
    }
}

/// Materialize a `JetstreamNodeHandle` into an actual `UiTree` node.
///
/// This is the primary bridge function. It takes the adapter's output
/// (resolved layout + visuals) and creates a real node in the tree.
pub fn materialize(tree: &mut UiTree, handle: &JetstreamNodeHandle) -> UiNodeId {
    let widget = widget_from_handle(handle);
    let layout = handle.mapped.layout.clone();
    let style = visuals_to_node_style(&handle.mapped.visuals);
    tree.create_node(widget, layout, style)
}

/// Materialize a handle and add it as a child of `parent`.
pub fn materialize_child(
    tree: &mut UiTree,
    parent: UiNodeId,
    handle: &JetstreamNodeHandle,
) -> UiNodeId {
    let id = materialize(tree, handle);
    tree.add_child(parent, id);
    id
}

/// Create a label node from adapter-resolved visuals.
///
/// For components that need text children (e.g., button labels, section headers),
/// this creates a Label widget with the given text, inheriting visual properties
/// from the mapped style.
pub fn materialize_label(
    tree: &mut UiTree,
    text: &str,
    mapped: &JetstreamMappedStyle,
) -> UiNodeId {
    let mut style = visuals_to_node_style(&mapped.visuals);
    // Labels typically don't need their own background
    style.background = None;
    style.border_color = None;
    style.border_width = 0.0;

    tree.create_node(
        Widget::Label { text: text.to_string() },
        mapped.layout.clone(),
        style,
    )
}

/// Create a button node with a specific label, using adapter-resolved styling.
pub fn materialize_button(
    tree: &mut UiTree,
    label: &str,
    mapped: &JetstreamMappedStyle,
) -> UiNodeId {
    let style = visuals_to_node_style(&mapped.visuals);
    tree.create_node(
        Widget::Button {
            label: label.to_string(),
            pressed: false,
            hovered: false,
        },
        mapped.layout.clone(),
        style,
    )
}
