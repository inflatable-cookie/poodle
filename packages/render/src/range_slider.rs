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

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodePosition, ScrubPhase,
    ShadowValue,
};
use poodle_specs::{ControlSize, RangeSliderSpec, SliderVariant};

use crate::color::with_alpha;
use crate::generated::range_slider::RANGE_SLIDER_DEFINITION;
use crate::presentation::{rem_to_px, resolve_semantic_size};

/// Host callbacks: continuous change + end-of-drag commit, both `(low, high)`.
#[derive(Default)]
pub struct RangeSliderHandlers {
    pub on_change: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
}

// Card 046 R3 — the render takes its vocabulary from the generated
// definition instead of its own literals. The node tree carries no
// attribute channel (the `Node` vocabulary is frozen), so the names gate
// the treatments rather than flowing into a DOM: a rename in the definition
// drops the treatment until restored, which is the drift direction the
// proof demonstrates (the same shape as Button's `state_declared`, card
// 042).

/// The definition's state-attribute names gate the treatments. `data-variant`
/// gates the variant anatomy; `data-fill-split` gates the corner-squaring
/// treatment.
fn state_declared(id: &str, name: &str) -> bool {
    RANGE_SLIDER_DEFINITION
        .attributes
        .iter()
        .any(|attribute| attribute.id == id && attribute.name == name)
}

/// The definition's anatomy parts gate the nodes the render builds: the
/// two thumbs render only while the definition declares both control parts
/// (card 046 R2.1 — the render still hard-codes the count two, but it is
/// anchored to the declared pair rather than an assumption).
fn part_declared(id: &str) -> bool {
    RANGE_SLIDER_DEFINITION
        .parts
        .iter()
        .any(|part| part.id == id)
}

/// The definition's RNG-17 geometry hooks gate the fill treatments: the
/// negative segment's danger role renders while the definition declares
/// `--poodle-range-negative-span`, the positive segment's accent role
/// while it declares `--poodle-range-positive-span`.
fn style_prop_declared(id: &str, name: &str) -> bool {
    RANGE_SLIDER_DEFINITION
        .style_props
        .iter()
        .any(|prop| prop.id == id && prop.name == name)
}

/// The frozen `poodle-specs` variant enum projected onto the definition's
/// words (CROSS-14 — the native counterpart of the web components' value
/// derivation).
fn variant_name(variant: SliderVariant) -> &'static str {
    match variant {
        SliderVariant::Standard => "standard",
        SliderVariant::Embedded => "embedded",
    }
}

/// Thumb diameter in rem per the contract §8 size table (same as Slider).
/// Each rung applies while the definition's size ladder declares it (card
/// 046 R3); a rung the definition does not declare falls back to the
/// default rung — the same fall-through shape as Button's density arms
/// (card 042).
fn thumb_diameter_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs if RANGE_SLIDER_DEFINITION.sizes.contains(&"xs") => 0.75,
        ControlSize::Sm if RANGE_SLIDER_DEFINITION.sizes.contains(&"sm") => 0.875,
        ControlSize::Md if RANGE_SLIDER_DEFINITION.sizes.contains(&"md") => 1.0,
        ControlSize::Lg if RANGE_SLIDER_DEFINITION.sizes.contains(&"lg") => 1.125,
        ControlSize::Xl if RANGE_SLIDER_DEFINITION.sizes.contains(&"xl") => 1.25,
        _ => 1.0,
    }
}

/// Visible track thickness in rem per the contract §8 size table.
fn track_thickness_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs if RANGE_SLIDER_DEFINITION.sizes.contains(&"xs") => 0.1875,
        ControlSize::Sm if RANGE_SLIDER_DEFINITION.sizes.contains(&"sm") => 0.25,
        ControlSize::Md if RANGE_SLIDER_DEFINITION.sizes.contains(&"md") => 0.375,
        ControlSize::Lg if RANGE_SLIDER_DEFINITION.sizes.contains(&"lg") => 0.5,
        ControlSize::Xl if RANGE_SLIDER_DEFINITION.sizes.contains(&"xl") => 0.625,
        _ => 0.375,
    }
}

pub fn range_slider(
    spec: &RangeSliderSpec,
    theme: &dyn ThemeProvider,
    handlers: RangeSliderHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7/§8: thumb diameter, track thickness, and min-height share
    // the size axis.
    let thumb_size = rem_to_px(thumb_diameter_rem(effective_size));
    let track_h = rem_to_px(track_thickness_rem(effective_size));
    // Pill radius (contract: 999px full-pill) and thumb border (0.0625rem).
    let pill = theme.resolve_radius("radius.pill");
    let border_w = rem_to_px(0.0625);

    let accent = theme.resolve_color(spec.range_fill_token());
    let negative = theme.resolve_color("color.status.danger");
    let surface = theme.resolve_color("color.background.surface");
    let border_default = theme.resolve_color("color.border.default");
    let elevated = theme.resolve_color("color.background.elevated");

    // Contract §8 track bg = color-mix(surface 88%, transparent).
    let track_bg = with_alpha(surface, surface.3 * 0.88);

    // Display the supplied values as-is. Step snapping belongs to interaction
    // updates; applying it during rendering moves valid off-grid input values.
    let visual = poodle_headless::slider::range_slider_visual_state(
        poodle_headless::slider::RangeSliderControlContext {
            value: (spec.low, spec.high),
            min: spec.min,
            max: spec.max,
            step: spec.step,
            disabled: spec.is_disabled,
            law: spec.law,
            polarity: spec.polarity,
            center_value: spec.center_value,
            pointer_active: false,
            active_thumb: None,
        },
    );
    let lo = visual.lower_norm as f32;
    let hi = visual.upper_norm.max(visual.lower_norm) as f32;

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

    // The fill segments' roles are gated on the declared RNG-17 geometry
    // hooks (card 046 R3): the negative segment carries the danger role
    // while the definition declares `--poodle-range-negative-span`, the
    // positive segment the accent role while it declares
    // `--poodle-range-positive-span`. The spans themselves stay
    // machine-computed (R1); the hooks declare which VisualState fields
    // feed them (CROSS-14).
    let negative_role = style_prop_declared("range-negative-span", "--poodle-range-negative-span")
        .then_some(negative);
    let positive_role = style_prop_declared("range-positive-span", "--poodle-range-positive-span")
        .then_some(accent);

    let seg_lo = segment(lo, None);
    let mut seg_negative = segment(visual.negative_fill_span_norm as f32, negative_role);
    let mut seg_positive = segment(visual.positive_fill_span_norm as f32, positive_role);
    let seg_hi = segment((1.0 - hi).max(0.0), None);
    for fill in [&mut seg_negative, &mut seg_positive] {
        let fill_corners = &mut fill.style.descriptor.corner_radii;
        fill_corners.top_left = pill;
        fill_corners.top_right = pill;
        fill_corners.bottom_right = pill;
        fill_corners.bottom_left = pill;
    }
    if visual.fill_split_at_center && state_declared("fill-split", "data-fill-split") {
        seg_negative.style.descriptor.corner_radii.top_right = 0.0;
        seg_negative.style.descriptor.corner_radii.bottom_right = 0.0;
        seg_positive.style.descriptor.corner_radii.top_left = 0.0;
        seg_positive.style.descriptor.corner_radii.bottom_left = 0.0;
    }

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
    let thumb_lo = make_thumb();
    let thumb_hi = make_thumb();

    // One shared pair of values, so dragging either thumb reports both.
    let mut scrub_handler: Option<Arc<dyn Fn(f32, ScrubPhase) + Send + Sync>> = None;
    {
        use poodle_headless::slider::{
            range_slider_control_transition, RangeSliderControlContext, RangeSliderControlEvent,
            RangeSliderEffect, RangeThumb,
        };

        if !(spec.is_disabled
            || (handlers.on_change.is_none() && handlers.on_value_commit.is_none()))
        {
            let low = Arc::new(AtomicU64::new(spec.low.to_bits()));
            let high = Arc::new(AtomicU64::new(spec.high.to_bits()));
            let context = RangeSliderControlContext {
                value: (spec.low, spec.high),
                min: spec.min,
                max: spec.max,
                step: spec.step,
                disabled: false,
                law: spec.law,
                polarity: spec.polarity,
                center_value: spec.center_value,
                pointer_active: false,
                active_thumb: None,
            };

            // Which thumb this gesture owns: 0 = undecided, 1 = lower,
            // 2 = upper. Decided on the press and held for the rest, so a thumb
            // dragged past its partner keeps the gesture instead of handing it
            // over halfway.
            let active = Arc::new(AtomicU8::new(0));
            // gpui delivers a click at the END of a drag as well as for a bare
            // press, so a press arriving after moves is a release, not a new
            // gesture — it must not re-pick the thumb from where the pointer
            // happens to have stopped.
            let dragged = Arc::new(AtomicBool::new(false));

            let on_change = handlers.on_change.clone();
            let on_value_commit = handlers.on_value_commit.clone();

            let scrub: Arc<dyn Fn(f32, ScrubPhase) + Send + Sync> =
                Arc::new(move |fraction: f32, phase: ScrubPhase| {
                    let live = (
                        f64::from_bits(low.load(Ordering::SeqCst)),
                        f64::from_bits(high.load(Ordering::SeqCst)),
                    );
                    let active_thumb = match active.load(Ordering::SeqCst) {
                        1 => Some(RangeThumb::Lower),
                        2 => Some(RangeThumb::Upper),
                        _ => None,
                    };
                    let event = match phase {
                        ScrubPhase::Press => {
                            if dragged.swap(false, Ordering::SeqCst) {
                                // The click that ends a drag. The value is
                                // already where the drag left it.
                                return;
                            }
                            RangeSliderControlEvent::PointerBegin {
                                value_norm: fraction as f64,
                            }
                        }
                        ScrubPhase::Drag => {
                            dragged.store(true, Ordering::SeqCst);
                            if active_thumb.is_some() {
                                RangeSliderControlEvent::PointerMove {
                                    value_norm: fraction as f64,
                                }
                            } else {
                                RangeSliderControlEvent::PointerBegin {
                                    value_norm: fraction as f64,
                                }
                            }
                        }
                    };

                    let context = RangeSliderControlContext {
                        value: live,
                        pointer_active: active_thumb.is_some(),
                        active_thumb,
                        ..context
                    };
                    let (next, effects) = range_slider_control_transition(context, event);
                    low.store(next.value.0.to_bits(), Ordering::SeqCst);
                    high.store(next.value.1.to_bits(), Ordering::SeqCst);
                    active.store(
                        match next.active_thumb {
                            Some(RangeThumb::Lower) => 1,
                            Some(RangeThumb::Upper) => 2,
                            None => 0,
                        },
                        Ordering::SeqCst,
                    );

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
                });
            scrub_handler = Some(scrub);
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
    let mut track = track
        .child(seg_lo)
        .child(seg_negative)
        .child(seg_positive)
        .child(seg_hi);
    // The variant anatomy is the definition's vocabulary (card 046 R3):
    // the standard two-thumb rendering applies while the definition
    // declares the `standard` variant, the `data-variant` attribute, and
    // both control parts — the two thumbs the definition records as
    // distinct parts (R2.1, the native answer: the count two is still
    // hard-coded here, but it renders only while the declared pair exists).
    let standard_declared = matches!(spec.variant, SliderVariant::Standard)
        && RANGE_SLIDER_DEFINITION
            .variants
            .contains(&variant_name(spec.variant))
        && state_declared("variant", "data-variant")
        && part_declared("control-lower")
        && part_declared("control-upper");
    // The embedded variant's center reference marker renders while the
    // definition declares the `embedded` variant, the `data-variant`
    // attribute, and the `center` part.
    let embedded_declared = matches!(spec.variant, SliderVariant::Embedded)
        && RANGE_SLIDER_DEFINITION
            .variants
            .contains(&variant_name(spec.variant))
        && state_declared("variant", "data-variant")
        && part_declared("center");
    if standard_declared {
        track = track.child(low_thumb_layer).child(high_thumb_layer);
    } else if embedded_declared {
        let mut marker = Node::container();
        marker.style.descriptor.layout.width = LayoutSizing::Fixed(border_w);
        marker.style.descriptor.layout.height = LayoutSizing::Fixed(track_h * 3.0);
        marker.style.descriptor.background = Some(border_default);
        marker.position = NodePosition::Absolute {
            top: Some(-track_h),
            left: None,
            right: Some(0.0),
            bottom: None,
        };
        let mut anchor = segment(visual.center_norm as f32, None);
        anchor.position = NodePosition::Relative;
        track = track.child(anchor.child(marker));
    }

    // The scrub belongs to a full-width grab overlay, not to either thumb: the
    // fraction is measured across whichever node carries it, and a fraction
    // measured across a thumb's own few pixels is meaningless. It also makes
    // the track clickable, which a per-thumb delta could never be. Same shape
    // as Slider's, and as ResizeHandle's contract putting the grab area on an
    // overlay rather than the visible line.
    if let Some(handler) = &scrub_handler {
        let mut grab = Node::container();
        grab.style.fill_width = true;
        grab.position = NodePosition::Absolute {
            top: Some(-thumb_r),
            left: Some(0.0),
            right: Some(0.0),
            bottom: Some(-thumb_r),
        };
        grab.style.descriptor.cursor = CursorHint::Pointer;
        grab.interaction.on_scrub = Some(Arc::clone(handler));
        track = track.child(grab);
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{RangeSliderSpec, SliderPolarity};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> RangeSliderSpec {
        RangeSliderSpec::new(20.0, 80.0).with_bounds(0.0, 100.0)
    }

    /// Records every `(low, high)` the component reports.
    fn armed() -> (Node, std::sync::Arc<std::sync::Mutex<Vec<(f64, f64)>>>) {
        let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = std::sync::Arc::clone(&seen);
        let node = range_slider(
            &spec(),
            &theme(),
            RangeSliderHandlers {
                on_change: Some(Arc::new(move |lo, hi| sink.lock().unwrap().push((lo, hi)))),
                ..RangeSliderHandlers::default()
            },
        );
        (node, seen)
    }

    fn scrub(node: &Node) -> Arc<dyn Fn(f32, ScrubPhase) + Send + Sync> {
        Arc::clone(
            node.find(&|n| n.interaction.on_scrub.is_some())
                .expect("something carries the scrub")
                .interaction
                .on_scrub
                .as_ref()
                .unwrap(),
        )
    }

    /// The fraction is measured across whichever node carries the handler, so
    /// it has to be the full-width grab area. On a thumb it would be measured
    /// across that thumb's own few pixels and jump wildly.
    #[test]
    fn the_grab_overlay_carries_the_scrub_and_the_thumbs_do_not() {
        let (node, _) = armed();
        let carrier = node
            .find(&|n| n.interaction.on_scrub.is_some())
            .expect("grab area");
        assert!(carrier.style.fill_width, "the scrub must span the track");
        assert!(
            matches!(carrier.position, NodePosition::Absolute { .. }),
            "the grab area is an overlay, not a layout participant"
        );
        // Exactly one node scrubs: two would fight over the same gesture.
        fn count_scrubs(node: &Node) -> usize {
            usize::from(node.interaction.on_scrub.is_some())
                + node.children.iter().map(count_scrubs).sum::<usize>()
        }
        assert_eq!(count_scrubs(&node), 1);
    }

    /// Pressing the track moves the *nearer* thumb there — the behaviour a
    /// delta could not express at all, because a delta has no idea where the
    /// press landed.
    #[test]
    fn pressing_the_track_moves_the_nearest_thumb_to_the_press() {
        let (node, seen) = armed();
        scrub(&node)(0.1, ScrubPhase::Press);
        assert_eq!(seen.lock().unwrap().last().copied(), Some((10.0, 80.0)));

        let (node, seen) = armed();
        scrub(&node)(0.9, ScrubPhase::Press);
        assert_eq!(seen.lock().unwrap().last().copied(), Some((20.0, 90.0)));
    }

    /// Once the press has chosen a thumb, the gesture keeps it. Dragging the
    /// lower thumb up past the upper must not silently hand the drag over.
    #[test]
    fn a_drag_keeps_the_thumb_the_press_chose() {
        let (node, seen) = armed();
        let scrub = scrub(&node);
        scrub(0.1, ScrubPhase::Press); // grabs the lower thumb
        scrub(0.5, ScrubPhase::Drag);
        scrub(0.95, ScrubPhase::Drag); // past the upper thumb
        let last = seen.lock().unwrap().last().copied().unwrap();
        assert_eq!(
            last.0, 80.0,
            "the lower thumb keeps the gesture, clamped at its partner"
        );
    }

    /// gpui delivers a click at the end of a drag as well as for a bare press.
    /// Treating that as a new press would re-pick a thumb from wherever the
    /// pointer stopped and move it again.
    #[test]
    fn the_click_that_ends_a_drag_changes_nothing() {
        let (node, seen) = armed();
        let scrub = scrub(&node);
        scrub(0.1, ScrubPhase::Press);
        scrub(0.4, ScrubPhase::Drag);
        let after_drag = seen.lock().unwrap().last().copied().unwrap();
        scrub(0.4, ScrubPhase::Press); // the release click
        assert_eq!(seen.lock().unwrap().last().copied(), Some(after_drag));
    }

    #[test]
    fn track_thickness_scales_with_size() {
        assert_eq!(track_thickness_rem(ControlSize::Xs), 0.1875);
        assert_eq!(track_thickness_rem(ControlSize::Sm), 0.25);
        assert_eq!(track_thickness_rem(ControlSize::Md), 0.375);
        assert_eq!(track_thickness_rem(ControlSize::Lg), 0.5);
        assert_eq!(track_thickness_rem(ControlSize::Xl), 0.625);
    }

    /// The native answer to b045 R2.1, encoded: the standard render builds
    /// exactly two thumbs. The node path has no repetition mechanism either
    /// — the count is hard-coded, anchored to the two control parts the
    /// definition declares (card 046 R3 gates the block on
    /// `control-lower`/`control-upper`). The finding is structural, not
    /// web-specific.
    #[test]
    fn the_standard_render_builds_exactly_two_thumbs() {
        let (node, _) = armed();
        assert_eq!(
            count_shadowed(&node),
            2,
            "standard renders exactly two thumbs — the hard-coded pair"
        );
    }

    /// The embedded variant renders no thumbs: the center reference marker
    /// carries no drop shadow (the `center` part, hidden in standard).
    #[test]
    fn the_embedded_render_builds_no_thumbs() {
        let embedded = spec().with_embedded_control(SliderPolarity::Unipolar);
        let node = range_slider(&embedded, &theme(), RangeSliderHandlers::default());
        assert_eq!(
            count_shadowed(&node),
            0,
            "embedded renders the center marker, not thumbs"
        );
    }

    /// R5's clamped state at the render level: a crossed spec (low > high)
    /// is ordered by the machine's normalize path before display, so the
    /// render shows the ordered pair — the thumbs can never visually cross
    /// (RNG-12, `normalize_range_value`).
    #[test]
    fn a_crossed_spec_renders_the_ordered_clamped_pair() {
        let mut crossed = spec();
        crossed.low = 80.0;
        crossed.high = 20.0;
        let node = range_slider(&crossed, &theme(), RangeSliderHandlers::default());
        assert_eq!(
            thumb_anchor_fractions(&node),
            vec![0.2, 0.8],
            "crossed (80, 20) displays as the ordered pair (20, 80)"
        );
    }

    /// Thumb anchors are the relative width-percentage segments whose child
    /// is the shadowed thumb; their fractions are the thumb positions.
    fn thumb_anchor_fractions(node: &Node) -> Vec<f32> {
        fn carries_shadow(node: &Node) -> bool {
            node.children
                .iter()
                .any(|child| child.style.descriptor.shadow.is_some())
        }
        fn walk(node: &Node, out: &mut Vec<f32>) {
            if let Some(fraction) = node.style.width_pct {
                if carries_shadow(node) {
                    out.push(fraction);
                }
            }
            for child in &node.children {
                walk(child, out);
            }
        }
        let mut out = Vec::new();
        walk(node, &mut out);
        out.sort_by(|a, b| a.partial_cmp(b).expect("fractions are finite"));
        out
    }

    /// The thumbs are the only shadowed nodes (the grab overlay carries no
    /// shadow).
    fn count_shadowed(node: &Node) -> usize {
        usize::from(node.style.descriptor.shadow.is_some())
            + node.children.iter().map(count_shadowed).sum::<usize>()
    }
}
