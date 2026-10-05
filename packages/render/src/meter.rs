//! Meter — value meter (bar or ring).
//!
//! Contract: `docs/contracts/components/meter.md`
//! Ported from: `packages/jetstream/components/src/meter.rs`.
//!
//! The linear shape uses a percentage-sized fill with its token-resolved
//! gradient inside the parent-owned track. Ring shape follows the documented
//! GPUI delta: without a conic-gradient primitive, it uses a circular track
//! stroked in the level-resolved fill colour with the value readout.

use poodle_node::{
    ColorValue, CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole,
};
use poodle_specs::{ControlSize, MeterShape, MeterSpec};

use crate::color::{mix_srgb, WHITE};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn meter(spec: &MeterSpec, ctx: &RenderContext<'_>) -> Node {
    // Contract §8: track bg = color-mix(in srgb, surface 96%, text-primary).
    let surface = ctx.theme().resolve_color(spec.track_fill_token());
    let text_primary = ctx.theme().resolve_color(spec.track_mix_token());
    let track_bg = mix_srgb(surface, text_primary, spec.track_mix_ratio());

    // Contract §8: pill radius from the radius.pill token (not a 999 literal).
    let radius = ctx.theme().resolve_radius("radius.pill");

    // Contract §8 Size Variants: track thickness resolves from the effective
    // size (size override → size_role against the inherited scale).
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let track_height = rem_to_px(spec.track_thickness_rem(effective_size));

    let fraction = spec.normalized_progress() as f32;
    let fill = ctx.theme().resolve_color(spec.fill_token());

    let mut root = if spec.shape == MeterShape::Ring {
        // Contract §8 ring shape: the track mixes at 88%, not the bar's 96%.
        let ring_track = mix_srgb(surface, text_primary, spec.ring_track_mix_ratio());
        ring(spec, ctx, effective_size, ring_track)
    } else {
        let mut root = Node::container();
        {
            let s = &mut root.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = 0.0;
        }
        let mut track = Node::container();
        {
            let s = &mut track.style;
            s.fill_width = true;
            s.min_height = Some(track_height);
            s.self_stretch = true;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = poodle_node::LayoutOverflow::Hidden;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.background = Some(track_bg);
        }
        track.a11y.hidden = Some(true);
        let mut indicator = Node::container();
        {
            let s = &mut indicator.style;
            s.width_pct = Some(fraction);
            s.fill_height = true;
            s.self_stretch = true;
            s.min_height = Some(track_height);
            s.descriptor.text_color = Some(fill);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.gradient = Some((90.0, vec![(mix_srgb(fill, WHITE, 0.82), 0.0), (fill, 1.0)]));
        }
        root = root.child(track.child(indicator));
        if spec.show_value {
            let mut readout = Node::text(spec.value_display_text());
            readout.style.descriptor.text_color =
                Some(ctx.theme().resolve_color(spec.value_color_token()));
            readout.style.text_size = Some(rem_to_px(0.75));
            readout.style.line_height = Some(1.0);
            readout.style.tabular_figures = true;
            root = root.child(readout);
        }
        root
    };
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    let safe_max = if spec.max <= spec.min {
        spec.min + 1.0
    } else {
        spec.max
    };
    // Contract §6: bounded-value meter semantics, distinct from progress.
    // The value, bounds and value text are the same three the ARIA range
    // keeps together; the role is what makes them a meter rather than a
    // progress bar.
    root.a11y.role = Some(NodeRole::Meter);
    root.a11y.value = Some(spec.safe_value());
    root.a11y.value_min = Some(spec.min);
    root.a11y.value_max = Some(safe_max);
    root.a11y.value_text = Some(spec.value_display_text());
    root
}

/// Ring shape: circular track stroked in the level-resolved fill colour,
/// with the value readout carrying the proportion.
fn ring(
    spec: &MeterSpec,
    ctx: &RenderContext<'_>,
    size: ControlSize,
    track_bg: ColorValue,
) -> Node {
    // The stroke colour already carries the `high` escalation from the spec.
    let diameter = rem_to_px(spec.ring_size_rem(size));
    let thickness = rem_to_px(spec.ring_thickness_rem(size));
    let fill = ctx.theme().resolve_color(spec.fill_token());

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
        readout.style.descriptor.text_color =
            Some(ctx.theme().resolve_color(spec.value_color_token()));
        readout.style.text_size = Some(diameter * 0.34);
        readout.style.tabular_figures = true;
        el = el.child(readout);
    }

    el
}
