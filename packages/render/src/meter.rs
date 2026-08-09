//! Meter — value meter (bar or ring).
//!
//! Contract: `docs/contracts/components/meter.md`
//! Ported from: `packages/jetstream/components/src/meter.rs`.
//!
//! The proportional fill uses the Progress node kind so the fill is a true
//! fraction of the parent-owned track width; the backend fills with its
//! status-success colour, matching contract §8. The track shell is
//! token-resolved (`color-mix(surface 96%, text-primary)`). Ring shape is the
//! contract §12 accepted delta: no conic-gradient primitive, so a circular
//! track stroked in the level-resolved fill colour with the value readout.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
};
use poodle_specs::{ControlSize, MeterShape, MeterSpec};

use crate::color::mix_srgb;
use crate::presentation::{rem_to_px, resolve_semantic_size};

pub fn meter(spec: &MeterSpec, theme: &dyn ThemeProvider) -> Node {
    // Contract §8: track bg = color-mix(in srgb, surface 96%, text-primary).
    let surface = theme.resolve_color(spec.track_fill_token());
    let text_primary = theme.resolve_color(spec.track_mix_token());
    let track_bg = mix_srgb(surface, text_primary, spec.track_mix_ratio());

    // Contract §8: pill radius from the radius.pill token (not a 999 literal).
    let radius = theme.resolve_radius("radius.pill");

    // Contract §8 Size Variants: track thickness resolves from the effective
    // size (size override → size_role against the inherited scale).
    let effective_size =
        resolve_semantic_size(spec.size.unwrap_or(ControlSize::Md), spec.size_role);
    let track_height = rem_to_px(spec.track_thickness_rem(effective_size));

    let fraction = spec.normalized_progress() as f32;
    let fill = theme.resolve_color(spec.fill_token());

    if spec.shape == MeterShape::Ring {
        // Contract §8 ring shape: the track mixes at 88%, not the bar's 96%.
        let ring_track = mix_srgb(surface, text_primary, spec.ring_track_mix_ratio());
        return ring(spec, theme, effective_size, ring_track);
    }

    // The Progress node draws a proportional fill of `fraction` over a
    // parent-owned (stretched) track. Width is owned by the parent (contract
    // §7: width 100%), not a hardcoded absolute.
    let mut root = Node::container();
    root.kind = poodle_node::NodeKind::Progress { fraction };
    {
        let s = &mut root.style;
        s.fill_width = true;
        s.min_height = Some(track_height);
        s.self_stretch = true;
        // The node backend uses the progress text-color channel for the
        // proportional fill; keep the track background independent.
        s.descriptor.text_color = Some(fill);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(track_bg);
    }
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

/// Ring shape: circular track stroked in the level-resolved fill colour,
/// with the value readout carrying the proportion.
fn ring(
    spec: &MeterSpec,
    theme: &dyn ThemeProvider,
    size: ControlSize,
    track_bg: ColorValue,
) -> Node {
    // The stroke colour already carries the `high` escalation from the spec.
    let diameter = rem_to_px(spec.ring_size_rem(size));
    let thickness = rem_to_px(spec.ring_thickness_rem(size));
    let fill = theme.resolve_color(spec.fill_token());

    let mut el = Node::container();
    {
        let s = &mut el.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(diameter);
        s.descriptor.layout.height = LayoutSizing::Fixed(diameter);
        s.flex_none = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        let c = &mut s.descriptor.corner_radii;
        let r = diameter / 2.0;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
        s.descriptor.border.width = thickness;
        s.descriptor.border.color = fill;
        s.descriptor.background = Some(track_bg);
    }

    if spec.show_value {
        let mut readout = Node::text(spec.value_display_text());
        readout.style.descriptor.text_color = Some(theme.resolve_color(spec.value_color_token()));
        readout.style.text_size = Some(diameter * 0.34);
        el = el.child(readout);
    }

    el
}
