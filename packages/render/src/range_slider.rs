//! RangeSlider — a low and a high value along one track.
//!
//! Contract: `docs/contracts/components/range-slider.md`
//! Ported from: `packages/jetstream/components/src/range_slider.rs`.
//!
//! Reports both values together: the pair is the value. Each thumb drags
//! separately; only the thumbs are draggable, not the window between them.
//! Snapping, clamping and the no-crossing rule come from
//! `poodle_headless::slider::range_slider_transition` — the same machine the
//! web target drives.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeDragEvent,
    NodeDragPhase, NodePosition, ShadowValue,
};
use poodle_specs::{ControlSize, RangeSliderSpec};

use crate::color::with_alpha;
use crate::presentation::{rem_to_px, resolve_semantic_size};

/// Host callbacks: continuous change + end-of-drag commit, both `(low, high)`.
#[derive(Default)]
pub struct RangeSliderHandlers {
    pub on_change: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
}

/// Fixed reference track width (matches the reference tier's layout budget).
fn track_w() -> f32 {
    rem_to_px(10.0)
}

/// Thumb diameter in rem per the contract §8 size table (same as Slider).
fn thumb_diameter_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Root min-height in rem per the contract §8 size table.
fn min_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.375,
        ControlSize::Md | ControlSize::Lg | ControlSize::Xl => 1.5,
    }
}

/// Snap a normalized fraction's underlying value to the step grid anchored at
/// `min`, then re-normalize to 0..1. A zero/absent step returns the fraction
/// unchanged.
fn snap_fraction(frac: f32, min: f64, max: f64, step: f64) -> f32 {
    if step <= 0.0 || max <= min {
        return frac.clamp(0.0, 1.0);
    }
    let raw = min + (frac as f64) * (max - min);
    let snapped = (min + ((raw - min) / step).round() * step).clamp(min, max);
    (((snapped - min) / (max - min)) as f32).clamp(0.0, 1.0)
}

pub fn range_slider(
    spec: &RangeSliderSpec,
    theme: &dyn ThemeProvider,
    handlers: RangeSliderHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7/§8: thumb diameter + min-height from the size table,
    // track thickness fixed at 0.375rem.
    let thumb_size = rem_to_px(thumb_diameter_rem(effective_size));
    let track_h = rem_to_px(0.375);
    let container_h = rem_to_px(min_height_rem(effective_size));

    // Pill radius (contract: 999px full-pill) and thumb border (0.0625rem).
    let pill = theme.resolve_radius("radius.pill");
    let border_w = rem_to_px(0.0625);

    let accent = theme.resolve_color(spec.range_fill_token());
    let surface = theme.resolve_color("color.background.surface");
    let border_default = theme.resolve_color("color.border.default");
    let elevated = theme.resolve_color("color.background.elevated");

    // Contract §8 track bg = color-mix(surface 88%, transparent).
    let track_bg = with_alpha(surface, surface.3 * 0.88);

    // Normalized positions, step-snapped to the contract grid.
    let lo = snap_fraction(spec.normalized_low() as f32, spec.min, spec.max, spec.step);
    let hi = snap_fraction(spec.normalized_high() as f32, spec.min, spec.max, spec.step).max(lo);

    let tw = track_w();
    let thumb_r = thumb_size * 0.5;

    // Three proportional segments (fixed px widths, never negative).
    let lo_w = (lo * tw).max(0.0);
    let fill_w = ((hi - lo) * tw).max(0.0);
    let hi_w = ((1.0 - hi) * tw).max(0.0);

    let segment = |w: f32, bg: poodle_node::ColorValue| -> Node {
        let mut seg = Node::container();
        let s = &mut seg.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(w);
        s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        s.descriptor.background = Some(bg);
        seg
    };

    // Leading unfilled segment: pill-rounded left.
    let mut seg_lo = segment(lo_w, track_bg);
    seg_lo.style.descriptor.corner_radii.top_left = pill;
    seg_lo.style.descriptor.corner_radii.bottom_left = pill;

    // Filled window between the thumbs: accent.
    let seg_fill = segment(fill_w, accent);

    // Trailing unfilled segment: pill-rounded right.
    let mut seg_hi = segment(hi_w, track_bg);
    seg_hi.style.descriptor.corner_radii.top_right = pill;
    seg_hi.style.descriptor.corner_radii.bottom_right = pill;

    // Contract §8 thumb drop shadow (offset/blur contract-exact rem;
    // black@0.18 is the one noted literal).
    let thumb_shadow = ShadowValue {
        offset_x: 0.0,
        offset_y: rem_to_px(0.125),
        blur: rem_to_px(0.5),
        color: poodle_node::ColorValue(0.0, 0.0, 0.0, 0.18),
    };

    // Thumbs absolutely positioned at the segment junctions, vertically
    // centred on the track.
    let thumb_top = -(thumb_r - track_h * 0.5);

    let make_thumb = |left: f32| -> Node {
        let mut thumb = Node::container();
        thumb.position = NodePosition::Absolute {
            top: Some(thumb_top),
            left: Some(left),
            right: None,
            bottom: None,
        };
        {
            let s = &mut thumb.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = pill;
            c.top_right = pill;
            c.bottom_right = pill;
            c.bottom_left = pill;
            s.descriptor.background = Some(elevated);
            s.descriptor.border.width = border_w;
            s.descriptor.border.color = border_default;
            s.descriptor.cursor = CursorHint::Pointer;
            s.descriptor.shadow = Some(thumb_shadow);
        }
        thumb
    };
    let mut thumb_lo = make_thumb(lo_w - thumb_r);
    let mut thumb_hi = make_thumb((lo_w + fill_w) - thumb_r);

    // One shared pair of values, so dragging either thumb reports both.
    {
        use poodle_headless::slider::{
            range_slider_transition, RangeSliderContext, RangeSliderEffect, RangeSliderEvent,
            RangeThumb,
        };

        if !(spec.is_disabled
            || (handlers.on_change.is_none() && handlers.on_value_commit.is_none()))
        {
            let low = Arc::new(AtomicU64::new(spec.low.to_bits()));
            let high = Arc::new(AtomicU64::new(spec.high.to_bits()));
            let context = RangeSliderContext {
                value: (spec.low, spec.high),
                min: spec.min,
                max: spec.max,
                step: spec.step,
                disabled: false,
            };
            let units_per_px = (spec.max - spec.min) / tw.max(1.0) as f64;

            let arm = |thumb: RangeThumb,
                       low: Arc<AtomicU64>,
                       high: Arc<AtomicU64>,
                       on_change: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
                       on_value_commit: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>|
             -> Arc<dyn Fn(&NodeDragEvent) + Send + Sync> {
                Arc::new(move |event: &NodeDragEvent| {
                    let live = (
                        f64::from_bits(low.load(Ordering::SeqCst)),
                        f64::from_bits(high.load(Ordering::SeqCst)),
                    );
                    let context = RangeSliderContext {
                        value: live,
                        ..context
                    };

                    let (raw, machine_event): (f64, fn(RangeThumb, f64) -> RangeSliderEvent) =
                        match event.phase {
                            NodeDragPhase::Start => return,
                            NodeDragPhase::Move => (
                                match thumb {
                                    RangeThumb::Lower => live.0,
                                    RangeThumb::Upper => live.1,
                                } + event.delta_x as f64 * units_per_px,
                                |thumb, raw| RangeSliderEvent::Input { thumb, raw },
                            ),
                            NodeDragPhase::End => (
                                match thumb {
                                    RangeThumb::Lower => live.0,
                                    RangeThumb::Upper => live.1,
                                },
                                |thumb, raw| RangeSliderEvent::Commit { thumb, raw },
                            ),
                        };

                    let (next, effects) =
                        range_slider_transition(context, machine_event(thumb, raw));
                    low.store(next.value.0.to_bits(), Ordering::SeqCst);
                    high.store(next.value.1.to_bits(), Ordering::SeqCst);

                    for effect in effects {
                        match effect {
                            RangeSliderEffect::EmitValueChange { value } => {
                                if let Some(handler) = &on_change {
                                    handler(value.0, value.1);
                                }
                            }
                            RangeSliderEffect::EmitValueCommit { value } => {
                                if let Some(handler) = &on_value_commit {
                                    handler(value.0, value.1);
                                }
                            }
                        }
                    }
                })
            };

            thumb_lo.interaction.on_drag = Some(arm(
                RangeThumb::Lower,
                Arc::clone(&low),
                Arc::clone(&high),
                handlers.on_change.clone(),
                handlers.on_value_commit.clone(),
            ));
            thumb_hi.interaction.on_drag = Some(arm(
                RangeThumb::Upper,
                low,
                high,
                handlers.on_change,
                handlers.on_value_commit,
            ));
        }
    }

    // Track row: relative container holding the three segments + both thumbs.
    let mut track = Node::container();
    track.position = NodePosition::Relative;
    {
        let s = &mut track.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(tw);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    let track = track
        .child(seg_lo)
        .child(seg_fill)
        .child(seg_hi)
        .child(thumb_lo)
        .child(thumb_hi);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(container_h);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    let mut el = el.child(track);

    if spec.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
