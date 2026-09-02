//! Shared block-appearance geometry for Slider and RangeSlider.
//!
//! Fit arithmetic lives in `poodle_headless::slider`. This module owns the
//! native paint metrics and forced-color role names.

use std::sync::Arc;

use poodle_node::{CursorHint, LayoutSizing, Node, NodePosition, ScrubAxis, ScrubPhase};
use poodle_specs::ControlSize;

use crate::presentation::rem_to_px;

/// Construction-time capsule span used when the host has not laid the node out.
/// Not a public fit metric.
pub const NATIVE_CAPSULE_SPAN_PX: f32 = 160.0;

pub fn capsule_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.75,
        ControlSize::Sm => 1.875,
        ControlSize::Md => 2.0,
        ControlSize::Lg => 2.25,
        ControlSize::Xl => 2.5,
    }
}

pub fn visible_thumb_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.375,
        ControlSize::Sm => 0.4375,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.5625,
        ControlSize::Xl => 0.625,
    }
}

pub fn font_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 1.0,
        ControlSize::Xl => 1.125,
    }
}

pub fn stamp_forced_color(node: &mut Node, fill: &str, text: &str) {
    node.roles
        .insert("forced-color-fill".to_owned(), fill.to_owned());
    node.roles
        .insert("forced-color-text".to_owned(), text.to_owned());
}

pub fn stamp_handle_roles(node: &mut Node) {
    node.roles
        .insert("forced-color-fill".to_owned(), "control".to_owned());
    node.roles
        .insert("forced-color-border".to_owned(), "control-border".to_owned());
    node.roles
        .insert("forced-color-focus".to_owned(), "focus-highlight".to_owned());
}

pub fn stamp_disabled_roles(node: &mut Node) {
    node.roles.insert(
        "forced-color-text".to_owned(),
        "disabled-content".to_owned(),
    );
    node.roles.insert(
        "forced-color-border".to_owned(),
        "disabled-content".to_owned(),
    );
}

pub fn block_hit(hit_px: f32, thumb: Node, thumb_name: &str) -> Node {
    let mut hit = Node::container();
    hit.roles.insert("part".to_owned(), "hit".to_owned());
    hit.roles
        .insert("thumb".to_owned(), thumb_name.to_owned());
    {
        let s = &mut hit.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(hit_px);
        s.descriptor.layout.height = LayoutSizing::Fixed(hit_px);
        s.min_width = Some(hit_px);
        s.min_height = Some(hit_px);
        s.descriptor.cursor = CursorHint::Pointer;
    }
    hit.child(thumb)
}

pub fn block_surface(hit_px: f32) -> Node {
    let mut surface = Node::container();
    surface.roles.insert("part".to_owned(), "block-surface".to_owned());
    surface.style.fill_width = true;
    surface.style.descriptor.layout.height = LayoutSizing::Fixed(hit_px);
    surface.style.min_height = Some(hit_px);
    surface.position = NodePosition::Relative;
    surface
}

pub fn block_grab(handler: Arc<dyn Fn(f32, ScrubPhase) + Send + Sync>) -> Node {
    let mut grab = Node::container();
    grab.style.fill_width = true;
    grab.style.fill_height = true;
    grab.position = NodePosition::Absolute {
        top: Some(0.0),
        left: Some(0.0),
        right: Some(0.0),
        bottom: Some(0.0),
    };
    grab.style.descriptor.cursor = CursorHint::Pointer;
    grab.interaction.on_scrub = Some(handler);
    grab.interaction.scrub_axis = ScrubAxis::Horizontal;
    grab
}

pub fn fraction_anchor(fraction: f32, height: f32, child: Node, child_half: f32) -> Node {
    let mut spacer = Node::container();
    spacer.style.width_pct = Some(fraction.clamp(0.0, 1.0));
    spacer.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    spacer.position = NodePosition::Relative;
    let mut child = child;
    child.position = NodePosition::Absolute {
        top: Some(-(child_half - height * 0.5)),
        left: None,
        right: Some(-child_half),
        bottom: None,
    };
    let mut layer = Node::container();
    layer.position = NodePosition::Absolute {
        top: Some(0.0),
        left: Some(0.0),
        right: Some(0.0),
        bottom: None,
    };
    layer.style.fill_width = true;
    layer.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    layer.child(spacer.child(child))
}

pub fn visible_thumb(size: ControlSize, fill: poodle_node::ColorValue, border: poodle_node::ColorValue) -> Node {
    let thumb_size = rem_to_px(visible_thumb_rem(size));
    let mut thumb = Node::container();
    {
        let s = &mut thumb.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = rem_to_px(0.0625);
        s.descriptor.border.color = border;
        let r = thumb_size * 0.5;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    }
    stamp_handle_roles(&mut thumb);
    thumb
}
