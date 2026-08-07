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
    // Pill radius (contract: 999px full-pill) and thumb border (0.0625rem).
    let pill = theme.resolve_radius("radius.pill");
    let border_w = rem_to_px(0.0625);

    let accent = theme.resolve_color(spec.range_fill_token());
    let surface = theme.resolve_color("color.background.surface");
    let border_default = theme.resolve_color("color.border.default");
    let elevated = theme.resolve_color("color.background.elevated");

    // Contract §8 track bg = color-mix(surface 88%, transparent).
    let track_bg = with_alpha(surface, surface.3 * 0.88);

    // Display the supplied values as-is. Step snapping belongs to interaction
    // updates; applying it during rendering moves valid off-grid input values.
    let lo = spec.normalized_low().clamp(0.0, 1.0) as f32;
    let hi = (spec.normalized_high().clamp(0.0, 1.0) as f32).max(lo);

    let thumb_r = thumb_size * 0.5;

    let segment = |fraction: f32, bg: Option<poodle_node::ColorValue>| -> Node {
        let mut seg = Node::container();
        let s = &mut seg.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.width_pct = Some(fraction);
        s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        s.descriptor.background = bg;
        seg
    };

    let seg_lo = segment(lo, None);
    let mut seg_fill = segment((hi - lo).max(0.0), Some(accent));
    let seg_hi = segment((1.0 - hi).max(0.0), None);
    let fill_corners = &mut seg_fill.style.descriptor.corner_radii;
    fill_corners.top_left = pill;
    fill_corners.top_right = pill;
    fill_corners.bottom_right = pill;
    fill_corners.bottom_left = pill;

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

    let make_thumb = || -> Node {
        let mut thumb = Node::container();
        thumb.position = NodePosition::Absolute {
            top: Some(thumb_top),
            left: None,
            right: Some(-thumb_r),
            bottom: None,
        };
        {
            let s = &mut thumb.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = thumb_r;
            c.top_right = thumb_r;
            c.bottom_right = thumb_r;
            c.bottom_left = thumb_r;
            s.descriptor.background = Some(elevated);
            s.descriptor.border.width = border_w;
            s.descriptor.border.color = border_default;
            s.descriptor.cursor = CursorHint::Pointer;
            s.descriptor.shadow = Some(thumb_shadow);
        }
        thumb
    };
    let mut thumb_lo = make_thumb();
    let mut thumb_hi = make_thumb();

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
            let units_per_px = (spec.max - spec.min) / track_w().max(1.0) as f64;

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

    let thumb_layer = |fraction: f32, thumb: Node| -> Node {
        let mut anchor = segment(fraction, None);
        anchor.position = NodePosition::Relative;
        let anchor = anchor.child(thumb);

        let mut layer = Node::container();
        layer.position = NodePosition::Absolute {
            top: Some(0.0),
            left: Some(0.0),
            right: Some(0.0),
            bottom: None,
        };
        layer.style.fill_width = true;
        layer.style.descriptor.layout.direction = LayoutDirection::Row;
        layer.style.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        layer.child(anchor)
    };
    let low_thumb_layer = thumb_layer(lo, thumb_lo);
    let high_thumb_layer = thumb_layer(hi, thumb_hi);

    // Full-width 6px pill; percentage segments anchor both thumbs without
    // requiring backend-specific layout bounds.
    let mut track = Node::container();
    track.position = NodePosition::Relative;
    {
        let s = &mut track.style;
        s.fill_width = true;
        s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.background = Some(track_bg);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = pill;
        c.top_right = pill;
        c.bottom_right = pill;
        c.bottom_left = pill;
    }
    let track = track
        .child(seg_lo)
        .child(seg_fill)
        .child(seg_hi)
        .child(low_thumb_layer)
        .child(high_thumb_layer);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.fill_width = true;
    }
    let mut el = el.child(track);

    if spec.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    } else {
        el.interaction.focusable = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
