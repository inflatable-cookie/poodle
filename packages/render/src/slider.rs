//! Slider — a value along a track: fill + remainder segments, absolute thumb.
//!
//! Contract: `docs/contracts/components/slider.md`
//!
//! Pointer value is an axis-normalized position. The GPUI backend derives the
//! fraction from the node that carries `on_scrub`; this renderer maps that
//! fraction through `slider_control_transition`. Keyboard arrows, Home, and
//! End go through `slider_transition` INPUT then COMMIT. Hosts do not
//! reproduce snap/clamp math.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use poodle_headless::slider::{safe_slider_max, SliderEffect};
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutSizing, Node,
    NodeKey, NodeModifiers, NodePosition, NodeRole, ScrubAxis, ScrubPhase, ShadowValue,
};
use poodle_specs::{ControlSize, Orientation, SliderSpec, SliderVariant};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

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

/// Visible track thickness in rem — contract §8 size table.
fn track_thickness_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.1875,
        ControlSize::Sm => 0.25,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.625,
    }
}

/// Cross-axis min-height in rem — contract §8 size table.
fn control_min_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.375,
        ControlSize::Md => 1.5,
        ControlSize::Lg => 1.625,
        ControlSize::Xl => 1.75,
    }
}

fn orientation_name(orientation: Orientation) -> &'static str {
    match orientation {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
    }
}

fn scrub_axis(orientation: Orientation) -> ScrubAxis {
    match orientation {
        Orientation::Horizontal => ScrubAxis::Horizontal,
        Orientation::Vertical => ScrubAxis::Vertical,
    }
}

fn pill(node: &mut Node, radius: f32) {
    let corners = &mut node.style.descriptor.corner_radii;
    corners.top_left = radius;
    corners.top_right = radius;
    corners.bottom_right = radius;
    corners.bottom_left = radius;
}

fn emit_effects(
    effects: impl IntoIterator<Item = SliderEffect>,
    on_change: &Option<Arc<dyn Fn(f64) + Send + Sync>>,
    on_value_commit: &Option<Arc<dyn Fn(f64) + Send + Sync>>,
) {
    for effect in effects {
        match effect {
            SliderEffect::EmitValueChange { value } => {
                if let Some(handler) = on_change {
                    handler(value);
                }
            }
            SliderEffect::EmitValueCommit { value } => {
                if let Some(handler) = on_value_commit {
                    handler(value);
                }
            }
        }
    }
}

/// Handlers: `on_change` fires during interaction (clamped, snapped);
/// `on_value_commit` fires once at pointer release or after each accepted key.
#[derive(Default, Clone)]
pub struct SliderHandlers {
    pub on_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
}

pub fn slider(spec: &SliderSpec, ctx: &RenderContext<'_>, handlers: &SliderHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);

    let thumb_size = rem_to_px(thumb_diameter_rem(effective_size));
    let track_thickness = rem_to_px(track_thickness_rem(effective_size));
    let cross_size = rem_to_px(control_min_height_rem(effective_size));
    let pill_radius = ctx.theme().resolve_radius("radius.pill");
    let border_w = rem_to_px(0.0625);
    let vertical = spec.orientation == Orientation::Vertical;
    let axis = scrub_axis(spec.orientation);
    let safe_max = safe_slider_max(spec.min, spec.max);

    let accent = ctx.theme().resolve_color(spec.range_fill_token());
    let negative = ctx.theme().resolve_color("color.status.danger");
    let surface = ctx.theme().resolve_color("color.background.surface");
    let border_default = ctx.theme().resolve_color("color.border.default");
    let elevated = ctx.theme().resolve_color("color.background.elevated");
    let track_bg = with_alpha(surface, surface.3 * 0.88);

    let visual = poodle_headless::slider::slider_visual_state(
        poodle_headless::slider::SliderControlContext {
            value: spec.value,
            min: spec.min,
            max: spec.max,
            step: spec.step,
            disabled: spec.is_disabled,
            law: spec.law,
            polarity: spec.polarity,
            center_value: spec.center_value,
            pointer_active: false,
        },
    );
    let fraction = visual.value_norm as f32;
    let fill_start = visual.fill_start_norm as f32;
    let fill_span = if spec.variant == SliderVariant::Embedded {
        visual.fill_span_norm as f32
    } else {
        fraction
    };
    let fill_origin = if spec.variant == SliderVariant::Embedded {
        fill_start
    } else {
        0.0
    };

    let thumb_r = thumb_size * 0.5;
    let interactive =
        !spec.is_disabled && (handlers.on_change.is_some() || handlers.on_value_commit.is_some());

    let live = Arc::new(AtomicU64::new(spec.value.to_bits()));
    let pointer_active = Arc::new(AtomicBool::new(false));

    let scrub_handler: Option<Arc<dyn Fn(f32, ScrubPhase) + Send + Sync>> = if interactive {
        let context = poodle_headless::slider::SliderControlContext {
            value: spec.value,
            min: spec.min,
            max: spec.max,
            step: spec.step,
            disabled: false,
            law: spec.law,
            polarity: spec.polarity,
            center_value: spec.center_value,
            pointer_active: false,
        };
        let live = Arc::clone(&live);
        let active = Arc::clone(&pointer_active);
        let on_change = handlers.on_change.clone();
        let on_value_commit = handlers.on_value_commit.clone();
        Some(Arc::new(move |fraction: f32, phase| {
            let current = f64::from_bits(live.load(Ordering::SeqCst));
            let pointer_active = active.load(Ordering::SeqCst);
            let event = match phase {
                ScrubPhase::Press => poodle_headless::slider::SliderControlEvent::PointerBegin {
                    value_norm: fraction as f64,
                },
                ScrubPhase::Drag if pointer_active => {
                    poodle_headless::slider::SliderControlEvent::PointerMove {
                        value_norm: fraction as f64,
                    }
                }
                ScrubPhase::Drag => poodle_headless::slider::SliderControlEvent::PointerBegin {
                    value_norm: fraction as f64,
                },
                ScrubPhase::Release => poodle_headless::slider::SliderControlEvent::PointerEnd,
            };
            let (next, effects) = poodle_headless::slider::slider_control_transition(
                poodle_headless::slider::SliderControlContext {
                    value: current,
                    pointer_active,
                    ..context
                },
                event,
            );
            live.store(next.value.to_bits(), Ordering::SeqCst);
            active.store(next.pointer_active, Ordering::SeqCst);
            emit_effects(effects, &on_change, &on_value_commit);
        }))
    } else {
        None
    };

    let key_handler: Option<Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync>> =
        if interactive {
            let live = Arc::clone(&live);
            let on_change = handlers.on_change.clone();
            let on_value_commit = handlers.on_value_commit.clone();
            let min = spec.min;
            let max = spec.max;
            let step = spec.step;
            Some(Arc::new(move |key: NodeKey, _mods: NodeModifiers| {
                let current = f64::from_bits(live.load(Ordering::SeqCst));
                let direction = match key {
                    NodeKey::ArrowLeft | NodeKey::ArrowDown => -1.0,
                    NodeKey::ArrowRight | NodeKey::ArrowUp => 1.0,
                    _ => 0.0,
                };
                let raw = match key {
                    NodeKey::Home => min,
                    NodeKey::End => safe_max,
                    NodeKey::ArrowLeft
                    | NodeKey::ArrowDown
                    | NodeKey::ArrowRight
                    | NodeKey::ArrowUp => current + direction * step,
                    _ => return None,
                };
                let context = poodle_headless::slider::SliderContext {
                    value: current,
                    min,
                    max,
                    step,
                    disabled: false,
                };
                let (changed, change_effects) = poodle_headless::slider::slider_transition(
                    context,
                    poodle_headless::slider::SliderEvent::Input { raw },
                );
                let (committed, commit_effects) = poodle_headless::slider::slider_transition(
                    changed,
                    poodle_headless::slider::SliderEvent::Commit { raw: changed.value },
                );
                live.store(committed.value.to_bits(), Ordering::SeqCst);
                emit_effects(
                    change_effects.into_iter().chain(commit_effects),
                    &on_change,
                    &on_value_commit,
                );
                None
            }))
        } else {
            None
        };

    let mut thumb = Node::container();
    {
        let s = &mut thumb.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
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
    pill(&mut thumb, thumb_r);
    thumb.position = if vertical {
        NodePosition::Absolute {
            top: Some(-thumb_r),
            left: Some(-(thumb_r - track_thickness * 0.5)),
            right: None,
            bottom: None,
        }
    } else {
        NodePosition::Absolute {
            top: Some(-(thumb_r - track_thickness * 0.5)),
            left: None,
            right: Some(-thumb_r),
            bottom: None,
        }
    };

    let fill_color = if visual.fill_tone == poodle_headless::slider::SliderFillTone::Negative {
        negative
    } else {
        accent
    };
    let mut fill = Node::container();
    fill.position = NodePosition::Relative;
    {
        let s = &mut fill.style;
        if vertical {
            s.height_pct = Some(fill_span);
            s.descriptor.layout.width = LayoutSizing::Fixed(track_thickness);
        } else {
            s.width_pct = Some(fill_span);
            s.descriptor.layout.height = LayoutSizing::Fixed(track_thickness);
        }
        s.descriptor.background = Some(fill_color);
    }
    pill(&mut fill, pill_radius);
    let fill = if spec.variant == SliderVariant::Embedded {
        fill
    } else {
        fill.child(thumb)
    };

    let mut track = Node::container();
    {
        let s = &mut track.style;
        if vertical {
            s.fill_height = true;
            s.descriptor.layout.width = LayoutSizing::Fixed(track_thickness);
            s.descriptor.layout.direction = LayoutDirection::Column;
        } else {
            s.fill_width = true;
            s.descriptor.layout.height = LayoutSizing::Fixed(track_thickness);
            s.descriptor.layout.direction = LayoutDirection::Row;
        }
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.background = Some(track_bg);
    }
    pill(&mut track, pill_radius);
    track.position = NodePosition::Relative;

    let leading_pct = if vertical {
        (1.0 - fill_origin - fill_span).max(0.0)
    } else {
        fill_origin
    };
    let trailing_pct = if vertical { fill_origin } else { 0.0 };
    if spec.variant == SliderVariant::Embedded || vertical {
        if leading_pct > 0.0 {
            let mut leading = Node::container();
            if vertical {
                leading.style.height_pct = Some(leading_pct);
                leading.style.descriptor.layout.width = LayoutSizing::Fixed(track_thickness);
            } else {
                leading.style.width_pct = Some(leading_pct);
                leading.style.descriptor.layout.height = LayoutSizing::Fixed(track_thickness);
            }
            track = track.child(leading);
        }
        track = track.child(fill);
        if trailing_pct > 0.0 {
            let mut trailing = Node::container();
            trailing.style.height_pct = Some(trailing_pct);
            trailing.style.descriptor.layout.width = LayoutSizing::Fixed(track_thickness);
            track = track.child(trailing);
        }
    } else {
        track = track.child(fill);
    }

    let mut grab = Node::container();
    {
        let s = &mut grab.style;
        if vertical {
            s.fill_height = true;
        } else {
            s.fill_width = true;
        }
    }
    grab.position = if vertical {
        NodePosition::Absolute {
            top: Some(0.0),
            left: Some(-thumb_r),
            right: Some(-thumb_r),
            bottom: Some(0.0),
        }
    } else {
        NodePosition::Absolute {
            top: Some(-thumb_r),
            left: Some(0.0),
            right: Some(0.0),
            bottom: Some(-thumb_r),
        }
    };
    if let Some(handler) = &scrub_handler {
        grab.style.descriptor.cursor = CursorHint::Pointer;
        grab.interaction.on_scrub = Some(Arc::clone(handler));
        grab.interaction.scrub_axis = axis;
    }
    let track = track.child(grab);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        if vertical {
            s.fill_height = true;
            s.descriptor.layout.width = LayoutSizing::Fixed(cross_size);
            s.min_width = Some(cross_size);
            s.min_height = Some(rem_to_px(10.0));
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        } else {
            s.fill_width = true;
        }
    }
    let mut el = el.child(track);

    el.a11y.role = Some(NodeRole::Slider);
    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.value = Some(visual.value);
    el.a11y.value_min = Some(spec.min);
    el.a11y.value_max = Some(safe_max);
    el.a11y.value_text = spec.value_text.clone();
    el.a11y.orientation = Some(orientation_name(spec.orientation).to_owned());

    if spec.is_disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    } else {
        el.interaction.focusable = true;
        el.interaction.on_key = key_handler;
        el.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
            width: ctx.theme().resolve_border_width("border.width.focus"),
            offset: rem_to_px(0.125),
        });
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn find_scrub(node: &Node) -> Option<&Node> {
        node.find(&|n| n.interaction.on_scrub.is_some())
    }

    fn armed(spec: SliderSpec) -> (Node, Arc<std::sync::Mutex<Vec<(String, f64)>>>) {
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let change = Arc::clone(&seen);
        let commit = Arc::clone(&seen);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = slider(
            &spec,
            &ctx,
            &SliderHandlers {
                on_change: Some(Arc::new(move |value| {
                    change.lock().unwrap().push(("change".into(), value));
                })),
                on_value_commit: Some(Arc::new(move |value| {
                    commit.lock().unwrap().push(("commit".into(), value));
                })),
            },
        );
        (node, seen)
    }

    #[test]
    fn the_track_carries_the_scrub_and_the_thumb_does_not() {
        let spec = SliderSpec::new(0.5).with_bounds(0.0, 1.0);
        let (node, _) = armed(spec);
        let scrub = find_scrub(&node).expect("a node carries the scrub handler");
        assert!(
            scrub.style.fill_width,
            "the scrub belongs to the full-width track"
        );
        assert_eq!(scrub.interaction.scrub_axis, ScrubAxis::Horizontal);
    }

    #[test]
    fn commit_only_hosts_still_install_the_scrub() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SliderSpec::new(0.5).with_bounds(0.0, 1.0);
        let node = slider(
            &spec,
            &ctx,
            &SliderHandlers {
                on_change: None,
                on_value_commit: Some(Arc::new(|_| {})),
            },
        );
        assert!(find_scrub(&node).is_some());
    }

    #[test]
    fn no_change_handler_means_no_scrub() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SliderSpec::new(0.5).with_bounds(0.0, 1.0);
        let node = slider(&spec, &ctx, &SliderHandlers::default());
        assert!(find_scrub(&node).is_none());
        assert!(node.interaction.on_key.is_none());
    }

    #[test]
    fn a_vertical_slider_declares_its_axis_and_uses_height_pct() {
        let spec = SliderSpec::new(40.0)
            .with_bounds(0.0, 100.0)
            .with_orientation(Orientation::Vertical);
        let (node, _) = armed(spec);
        let scrub = find_scrub(&node).expect("scrub");
        assert_eq!(scrub.interaction.scrub_axis, ScrubAxis::Vertical);
        assert!(scrub.style.fill_height);
        let fill = node
            .find(&|n| n.style.height_pct == Some(0.4))
            .expect("vertical fill");
        assert_eq!(fill.style.height_pct, Some(0.4));
        assert!(fill.style.width_pct.is_none());
    }

    #[test]
    fn pointer_release_commits_through_the_scrub() {
        let spec = SliderSpec::new(0.0).with_bounds(0.0, 100.0);
        let (node, seen) = armed(spec);
        let scrub = Arc::clone(
            find_scrub(&node)
                .unwrap()
                .interaction
                .on_scrub
                .as_ref()
                .unwrap(),
        );
        scrub(0.44, ScrubPhase::Press);
        scrub(0.76, ScrubPhase::Drag);
        scrub(0.76, ScrubPhase::Release);
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [
                ("change".into(), 44.0),
                ("change".into(), 76.0),
                ("commit".into(), 76.0)
            ]
        );
    }

    #[test]
    fn arrows_home_and_end_emit_change_then_commit() {
        let spec = SliderSpec::new(50.0).with_bounds(0.0, 100.0);
        let mut spec = spec;
        spec.step = 10.0;
        spec.aria_label = Some("Volume".into());
        spec.value_text = Some("half".into());
        let (node, seen) = armed(spec);
        let key = node.interaction.on_key.as_ref().unwrap();
        let mods = NodeModifiers::default();
        key(NodeKey::ArrowRight, mods);
        key(NodeKey::Home, mods);
        key(NodeKey::End, mods);
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [
                ("change".into(), 60.0),
                ("commit".into(), 60.0),
                ("change".into(), 0.0),
                ("commit".into(), 0.0),
                ("change".into(), 100.0),
                ("commit".into(), 100.0),
            ]
        );
    }

    #[test]
    fn the_focusable_node_carries_slider_intent() {
        let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
        spec.aria_label = Some("Volume".into());
        spec.value_text = Some("quiet".into());
        let (node, _) = armed(spec);
        assert_eq!(node.a11y.role, Some(NodeRole::Slider));
        assert_eq!(node.a11y.label.as_deref(), Some("Volume"));
        assert_eq!(node.a11y.value, Some(40.0));
        assert_eq!(node.a11y.value_min, Some(0.0));
        assert_eq!(node.a11y.value_max, Some(100.0));
        assert_eq!(node.a11y.value_text.as_deref(), Some("quiet"));
        assert_eq!(node.a11y.orientation.as_deref(), Some("horizontal"));
        assert!(node.interaction.focusable);
        assert!(node.style.focus_ring.is_some());
        assert!(!node.interaction.disabled);
    }

    #[test]
    fn disabled_slider_is_inert() {
        let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
        spec.is_disabled = true;
        spec.aria_label = Some("Volume".into());
        let (node, seen) = armed(spec);
        assert!(find_scrub(&node).is_none());
        assert!(node.interaction.on_key.is_none());
        assert!(node.interaction.disabled);
        assert!(!node.interaction.focusable);
        assert!(node.style.focus_ring.is_none());
        assert_eq!(node.a11y.role, Some(NodeRole::Slider));
        assert!(seen.lock().unwrap().is_empty());
    }

    #[test]
    fn track_thickness_scales_with_size() {
        assert_eq!(track_thickness_rem(ControlSize::Xs), 0.1875);
        assert_eq!(track_thickness_rem(ControlSize::Sm), 0.25);
        assert_eq!(track_thickness_rem(ControlSize::Md), 0.375);
        assert_eq!(track_thickness_rem(ControlSize::Lg), 0.5);
        assert_eq!(track_thickness_rem(ControlSize::Xl), 0.625);
    }
}
