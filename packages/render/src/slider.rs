//! Slider — a value along a track: fill + remainder segments, absolute thumb.
//!
//! Contract: `docs/contracts/components/slider.md`
//! Ported from: `packages/jetstream/components/src/slider.rs`.
//!
//! The drag handler accumulates from per-frame deltas (the vocabulary carries
//! no absolute positions — those depend on layout the component never sees),
//! snapped and clamped by `poodle_headless::slider::slider_transition`, the
//! same machine the web target drives.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node,
    NodeDragEvent, NodeDragPhase, NodePosition, ShadowValue,
};
use poodle_specs::{ControlSize, SliderSpec};

use crate::color::with_alpha;
use crate::presentation::{rem_to_px, resolve_semantic_size};

/// Fixed track length — 10 rem, matching the GPUI reference basis.
fn track_w() -> f32 {
    rem_to_px(10.0)
}

/// Thumb diameter in rem — contract §8 size table.
fn thumb_diameter_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Root min-height in rem — contract §8 size table (lg/xl inherit md).
fn min_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.375,
        ControlSize::Md | ControlSize::Lg | ControlSize::Xl => 1.5,
    }
}

/// Handlers: `change` fires per-frame during a drag (clamped, snapped);
/// `commit` fires once at drag end with the settled value.
#[derive(Default, Clone)]
pub struct SliderHandlers {
    pub change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
}

pub fn slider(spec: &SliderSpec, theme: &dyn ThemeProvider, handlers: &SliderHandlers) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let thumb_size = rem_to_px(thumb_diameter_rem(effective_size));
    let track_h = rem_to_px(0.375);
    let container_h = rem_to_px(min_height_rem(effective_size));

    let pill = theme.resolve_radius("radius.pill");
    let border_w = rem_to_px(0.0625);

    let accent = theme.resolve_color(spec.range_fill_token());
    let surface = theme.resolve_color("color.background.surface");
    let border_default = theme.resolve_color("color.border.default");
    let elevated = theme.resolve_color("color.background.elevated");

    // Contract §8 track bg = color-mix(surface 88%, transparent): surface at
    // 88% of its own alpha.
    let track_bg = with_alpha(surface, surface.3 * 0.88);

    let range = (spec.max - spec.min).max(0.001);
    let fraction = ((spec.value - spec.min) / range).clamp(0.0, 1.0) as f32;

    let tw = track_w();
    let fill_w = fraction * tw;
    let rem_w = (tw - fill_w).max(0.0);
    let thumb_r = thumb_size * 0.5;

    // Drags do not bubble: every segment under the pointer carries the same
    // handler.
    let drag_handler: Option<Arc<dyn Fn(&NodeDragEvent) + Send + Sync>> = if spec.is_disabled
        || (handlers.change.is_none() && handlers.commit.is_none())
    {
        None
    } else {
        use std::sync::atomic::{AtomicU64, Ordering};

        // The running value for this drag, travelling as its bit pattern —
        // the handler must be Fn + Send + Sync, ruling out a captured local.
        let live = Arc::new(AtomicU64::new(spec.value.to_bits()));
        let context = poodle_headless::slider::SliderContext {
            value: spec.value,
            min: spec.min,
            max: spec.max,
            step: spec.step,
            disabled: false,
        };
        let units_per_px = (spec.max - spec.min) / track_w().max(1.0) as f64;
        let on_change = handlers.change.clone();
        let on_commit = handlers.commit.clone();

        Some(Arc::new(move |event: &NodeDragEvent| match event.phase {
            NodeDragPhase::Start => {}
            NodeDragPhase::Move => {
                let current = f64::from_bits(live.load(Ordering::SeqCst));
                let (next, effects) = poodle_headless::slider::slider_transition(
                    poodle_headless::slider::SliderContext {
                        value: current,
                        ..context
                    },
                    poodle_headless::slider::SliderEvent::Input {
                        raw: current + event.delta_x as f64 * units_per_px,
                    },
                );
                live.store(next.value.to_bits(), Ordering::SeqCst);
                for effect in effects {
                    if let poodle_headless::slider::SliderEffect::EmitValueChange { value } = effect
                    {
                        if let Some(handler) = &on_change {
                            handler(value);
                        }
                    }
                }
            }
            NodeDragPhase::End => {
                let current = f64::from_bits(live.load(Ordering::SeqCst));
                let (_, effects) = poodle_headless::slider::slider_transition(
                    poodle_headless::slider::SliderContext {
                        value: current,
                        ..context
                    },
                    poodle_headless::slider::SliderEvent::Commit { raw: current },
                );
                for effect in effects {
                    if let poodle_headless::slider::SliderEffect::EmitValueCommit { value } = effect
                    {
                        if let Some(handler) = &on_commit {
                            handler(value);
                        }
                    }
                }
            }
        }))
    };

    let draggable = |node: &mut Node| {
        if let Some(handler) = &drag_handler {
            node.style.descriptor.cursor = CursorHint::Pointer;
            node.interaction.on_drag = Some(Arc::clone(handler));
        }
    };

    // Fill segment: left portion in accent, left corners rounded.
    let mut fill = Node::container();
    {
        let s = &mut fill.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(fill_w);
        s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        s.descriptor.background = Some(accent);
        s.descriptor.corner_radii.top_left = pill;
        s.descriptor.corner_radii.bottom_left = pill;
    }
    draggable(&mut fill);

    // Remainder segment: the rest, right corners rounded.
    let mut remainder = Node::container();
    {
        let s = &mut remainder.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_w);
        s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        s.descriptor.background = Some(track_bg);
        s.descriptor.corner_radii.top_right = pill;
        s.descriptor.corner_radii.bottom_right = pill;
    }
    draggable(&mut remainder);

    // Thumb: absolute at the fill/remainder junction, vertically centred on
    // the track, with the contract's drop shadow.
    let thumb_top = -(thumb_r - track_h * 0.5);
    let thumb_left = fill_w - thumb_r;
    let mut thumb = Node::container();
    {
        let s = &mut thumb.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
        s.descriptor.corner_radii.top_left = pill;
        s.descriptor.corner_radii.top_right = pill;
        s.descriptor.corner_radii.bottom_right = pill;
        s.descriptor.corner_radii.bottom_left = pill;
        s.descriptor.background = Some(elevated);
        s.descriptor.border.width = border_w;
        s.descriptor.border.color = border_default;
        s.descriptor.cursor = CursorHint::Pointer;
        s.descriptor.shadow = Some(ShadowValue {
            offset_x: 0.0,
            offset_y: rem_to_px(0.125),
            blur: rem_to_px(0.5),
            color: ColorValue(0.0, 0.0, 0.0, 0.18),
        });
    }
    thumb.position = NodePosition::Absolute {
        top: Some(thumb_top),
        left: Some(thumb_left),
        right: None,
        bottom: None,
    };
    draggable(&mut thumb);

    // Track row: relative container holding fill, remainder, thumb.
    let mut track = Node::container();
    {
        let s = &mut track.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(tw);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    track.position = NodePosition::Relative;
    let track = track.child(fill).child(remainder).child(thumb);

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
    } else {
        el.interaction.focusable = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
