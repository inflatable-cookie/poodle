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

use poodle_headless::slider::{
    layout_slider_block, physical_to_value_norm, resolved_visible_text, safe_slider_max,
    SliderEffect, SLIDER_BLOCK_HIT_PX,
};
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutOverflow,
    LayoutSizing, MainAxisAlignment, Node, NodeKey, NodeModifiers, NodePosition, NodeRole,
    ScrubAxis, ScrubPhase, ShadowValue,
};
use poodle_specs::{ControlSize, Orientation, SliderSpec, SliderVariant};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::slider_block::{
    block_hit, block_surface, capsule_height_rem, font_size_rem, fraction_anchor,
    stamp_disabled_roles, stamp_forced_color, visible_thumb,
};

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

fn standard_focus_ring(ctx: &RenderContext<'_>, spec: &SliderSpec) -> FocusRing {
    FocusRing {
        color: with_alpha(
            ctx.theme().resolve_color(spec.focus_ring_color_token()),
            0.32,
        ),
        width: rem_to_px(0.1875),
        offset: 0.0,
    }
}

fn embedded_focus_ring(ctx: &RenderContext<'_>, spec: &SliderSpec) -> FocusRing {
    FocusRing {
        color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
        width: rem_to_px(0.125),
        offset: rem_to_px(0.0625),
    }
}

fn bind_slider_control(
    node: &mut Node,
    spec: &SliderSpec,
    value: f64,
    safe_max: f64,
    key_handler: Option<Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync>>,
    ring: FocusRing,
) {
    node.a11y.role = Some(NodeRole::Slider);
    if let Some(label) = spec.aria_label.as_deref() {
        node.a11y.label = Some(label.to_string());
    }
    node.a11y.value = Some(value);
    node.a11y.value_min = Some(spec.min);
    node.a11y.value_max = Some(safe_max);
    node.a11y.value_text = spec.value_text.clone();
    node.a11y.orientation = Some(orientation_name(spec.orientation).to_owned());
    if spec.is_disabled {
        node.interaction.disabled = true;
    } else {
        node.interaction.focusable = true;
        node.a11y.tab_index = Some(0);
        node.interaction.on_key = key_handler;
        node.style.focus_ring = Some(ring);
    }
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
        let rtl = spec.direction.is_rtl();
        Some(Arc::new(move |fraction: f32, phase| {
            let current = f64::from_bits(live.load(Ordering::SeqCst));
            let pointer_active = active.load(Ordering::SeqCst);
            let value_norm = physical_to_value_norm(fraction as f64, rtl);
            let event = match phase {
                ScrubPhase::Press => {
                    poodle_headless::slider::SliderControlEvent::PointerBegin { value_norm }
                }
                ScrubPhase::Drag if pointer_active => {
                    poodle_headless::slider::SliderControlEvent::PointerMove { value_norm }
                }
                ScrubPhase::Drag => {
                    poodle_headless::slider::SliderControlEvent::PointerBegin { value_norm }
                }
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

    if spec.variant == SliderVariant::Block {
        return paint_slider_block(
            spec,
            ctx,
            effective_size,
            &visual,
            fraction,
            accent,
            negative,
            surface,
            border_default,
            elevated,
            scrub_handler,
            key_handler,
            interactive,
        );
    }

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
    if vertical {
        let mut trailing = Node::container();
        trailing.style.height_pct = Some(fill_origin);
        trailing.style.descriptor.layout.width = LayoutSizing::Fixed(track_thickness);
        track = track.child(trailing);
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

    if spec.variant == SliderVariant::Embedded {
        bind_slider_control(
            &mut el,
            spec,
            visual.value,
            safe_max,
            key_handler,
            embedded_focus_ring(ctx, spec),
        );
    }
    if spec.is_disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
    }

    el
}

#[allow(clippy::too_many_arguments)]
fn paint_slider_block(
    spec: &SliderSpec,
    ctx: &RenderContext<'_>,
    effective_size: ControlSize,
    visual: &poodle_headless::slider::SliderVisualState,
    fraction: f32,
    accent: ColorValue,
    negative: ColorValue,
    surface_color: ColorValue,
    border_default: ColorValue,
    elevated: ColorValue,
    scrub_handler: Option<Arc<dyn Fn(f32, ScrubPhase) + Send + Sync>>,
    key_handler: Option<Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync>>,
    interactive: bool,
) -> Node {
    use crate::slider_block::{block_grab_with_axis, block_surface_vertical, fraction_anchor_vertical};

    let vertical = spec.orientation == Orientation::Vertical;
    // Horizontal geometry mirrors in RTL; vertical never mirrors.
    let rtl = spec.direction.is_rtl() && !vertical;
    let capsule_cross = rem_to_px(capsule_height_rem(effective_size));
    let font_px = rem_to_px(font_size_rem(effective_size));
    let hit_px = SLIDER_BLOCK_HIT_PX;
    let selected_color = if visual.fill_tone == poodle_headless::slider::SliderFillTone::Negative {
        negative
    } else {
        accent
    };
    let remainder_fill = with_alpha(surface_color, surface_color.3 * 0.88);
    let selected_text_color = ctx.theme().resolve_color("color.text.inverse");
    let remainder_text_color = ctx.theme().resolve_color("color.text.primary");
    // g18.017: the block family uses the rounded-square control radius, not
    // the pill. The visible thumb stays circular.
    let control_radius = ctx.theme().resolve_radius("radius.control");
    let label = omit_empty_owned(spec.visible_label.as_deref());
    let value_text = resolved_visible_text(visual.value, spec.visible_value_text.as_deref());
    let (capsule_span, measure) = ctx.require_block_layout("Slider");
    let layout = layout_slider_block(
        capsule_span,
        label.as_deref(),
        value_text.as_deref(),
        |text| measure(text, font_px),
    );

    let paint_text = layout.label_inline || layout.value_inline;

    // g18.017/g18.022 fixed inline presentation: one stable text layout
    // painted twice. Each layer is a per-region clip container holding a
    // full-capsule layout, so glyph coordinates never depend on the current
    // value; crossing the boundary changes only the painted foreground.
    // Vertical keeps the text upright: value at the physical top, optional
    // label centered (g18.022).
    let text_layers: Option<(Node, Node)> = if paint_text {
        let selected_span = fraction.clamp(0.0, 1.0) * capsule_span;
        let remainder_span = (capsule_span - selected_span).max(0.0);
        let make_row = |role_color: ColorValue,
                        role_fill: &str,
                        role_text: &str,
                        role_id: &str| {
            let mut row = block_text_row(
                label.clone().filter(|_| layout.label_inline),
                value_text.clone().filter(|_| layout.value_inline),
                role_color,
                font_px,
                if vertical { capsule_cross } else { capsule_span },
                if vertical { capsule_span } else { capsule_cross },
                rtl,
                vertical,
                &format!("block-slider-label-{role_id}"),
                &format!("block-slider-value-{role_id}"),
            );
            stamp_forced_color(&mut row, role_fill, role_text);
            row
        };

        let selected_row = make_row(selected_text_color, "selection", "selection-text", "selected");
        let remainder_row = make_row(remainder_text_color, "canvas", "canvas-text", "remainder");
        let selected_row = if vertical {
            selected_row
        } else {
            let mut selected_row = selected_row;
            let selected_row_left = if rtl { selected_span - capsule_span } else { 0.0 };
            selected_row.position = NodePosition::Absolute {
                top: Some(0.0),
                left: Some(selected_row_left),
                right: None,
                bottom: None,
            };
            selected_row
        };
        let remainder_row = if vertical {
            remainder_row
        } else {
            let mut remainder_row = remainder_row;
            let remainder_row_left = if rtl { 0.0 } else { -selected_span };
            remainder_row.position = NodePosition::Absolute {
                top: Some(0.0),
                left: Some(remainder_row_left),
                right: None,
                bottom: None,
            };
            remainder_row
        };

        let (selected_clip, remainder_clip) = if vertical {
            // The selected fill grows from the physical bottom; the selected
            // clip is the bottom region and the remainder clip the top. The
            // row pins to the clip edge nearest the capsule's matching edge.
            let mut selected_clip = Node::container();
            selected_clip.id = Some("block-slider-clip-selected".to_owned());
            selected_clip.position = NodePosition::Absolute {
                top: None,
                left: Some(0.0),
                right: Some(0.0),
                bottom: Some(0.0),
            };
            selected_clip.style.descriptor.layout.height = LayoutSizing::Fixed(selected_span);
            selected_clip.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
            let mut pinned_row = selected_row;
            pinned_row.position = NodePosition::Absolute {
                top: Some(selected_span - capsule_span),
                left: Some(0.0),
                right: None,
                bottom: None,
            };
            selected_clip = selected_clip.child(pinned_row);

            let mut remainder_clip = Node::container();
            remainder_clip.id = Some("block-slider-clip-remainder".to_owned());
            remainder_clip.position = NodePosition::Absolute {
                top: Some(0.0),
                left: Some(0.0),
                right: Some(0.0),
                bottom: Some(remainder_span),
            };
            remainder_clip.style.descriptor.layout.height = LayoutSizing::Fixed(remainder_span);
            remainder_clip.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
            remainder_clip = remainder_clip.child(remainder_row);
            (selected_clip, remainder_clip)
        } else {
            let mut selected_clip = Node::container();
            selected_clip.id = Some("block-slider-clip-selected".to_owned());
            selected_clip.position = if rtl {
                NodePosition::Absolute {
                    top: Some(0.0),
                    left: None,
                    right: Some(0.0),
                    bottom: Some(0.0),
                }
            } else {
                NodePosition::Absolute {
                    top: Some(0.0),
                    left: Some(0.0),
                    right: None,
                    bottom: Some(0.0),
                }
            };
            selected_clip.style.descriptor.layout.width = LayoutSizing::Fixed(selected_span);
            selected_clip.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            selected_clip = selected_clip.child(selected_row);

            let mut remainder_clip = Node::container();
            remainder_clip.id = Some("block-slider-clip-remainder".to_owned());
            remainder_clip.position = if rtl {
                NodePosition::Absolute {
                    top: Some(0.0),
                    left: Some(0.0),
                    right: None,
                    bottom: Some(0.0),
                }
            } else {
                NodePosition::Absolute {
                    top: Some(0.0),
                    left: Some(selected_span),
                    right: None,
                    bottom: Some(0.0),
                }
            };
            remainder_clip.style.descriptor.layout.width = LayoutSizing::Fixed(remainder_span);
            remainder_clip.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            remainder_clip = remainder_clip.child(remainder_row);
            (selected_clip, remainder_clip)
        };
        Some((selected_clip, remainder_clip))
    } else {
        None
    };

    let mut capsule = Node::container();
    if vertical {
        capsule.style.fill_height = true;
        capsule.style.descriptor.layout.width = LayoutSizing::Fixed(capsule_cross);
        capsule.style.min_width = Some(capsule_cross);
        capsule.style.descriptor.layout.direction = LayoutDirection::Column;
    } else {
        capsule.style.fill_width = true;
        capsule.style.descriptor.layout.height = LayoutSizing::Fixed(capsule_cross);
        capsule.style.min_height = Some(capsule_cross);
        capsule.style.descriptor.layout.direction = LayoutDirection::Row;
    }
    capsule.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    capsule.style.descriptor.background = Some(remainder_fill);
    let corners = &mut capsule.style.descriptor.corner_radii;
    corners.top_left = control_radius;
    corners.top_right = control_radius;
    corners.bottom_right = control_radius;
    corners.bottom_left = control_radius;
    capsule.position = NodePosition::Relative;
    stamp_forced_color(&mut capsule, "canvas", "canvas-text");

    // Paint order: selected paint, remainder paint, then the text layers.
    // The span order flips for RTL and for vertical (selected grows from the
    // physical bottom) so the window stays at the mirrored physical edge.
    let selected = || {
        let mut node = Node::container();
        node.style.width_pct = Some(fraction.clamp(0.0, 1.0));
        node.style.fill_height = true;
        node.style.descriptor.background = Some(selected_color);
        stamp_forced_color(&mut node, "selection", "selection-text");
        node
    };
    let remainder = || {
        let mut node = Node::container();
        node.style.flex_fill = true;
        node.style.fill_height = true;
        node.style.descriptor.background = Some(remainder_fill);
        stamp_forced_color(&mut node, "canvas", "canvas-text");
        node
    };
    if vertical || rtl {
        capsule = capsule.child(remainder()).child(selected());
    } else {
        capsule = capsule.child(selected()).child(remainder());
    }
    if let Some((selected_clip, remainder_clip)) = text_layers {
        capsule = capsule.child(selected_clip).child(remainder_clip);
    }

    let thumb = visible_thumb(effective_size, elevated, border_default);
    let mut hit = block_hit(hit_px, thumb, "value");
    hit.a11y.orientation = Some(orientation_name(spec.orientation).to_owned());
    bind_slider_control(
        &mut hit,
        spec,
        visual.value,
        safe_slider_max(spec.min, spec.max),
        if interactive {
            key_handler.clone()
        } else {
            None
        },
        standard_focus_ring(ctx, spec),
    );
    if spec.is_disabled {
        stamp_disabled_roles(&mut hit);
    }

    let inset = ((hit_px - capsule_cross) * 0.5).max(0.0);
    let physical = if rtl { 1.0 - fraction } else { fraction };
    let mut surface = if vertical {
        // The capsule fills the block axis; the cross axis is centred inside
        // the hit-sized surface.
        let mut capsule = capsule;
        capsule.position = NodePosition::Absolute {
            top: Some(0.0),
            left: Some(inset),
            right: Some(inset),
            bottom: Some(0.0),
        };
        let anchor_layer = fraction_anchor_vertical(physical, hit_px, hit, hit_px * 0.5);
        let mut s = block_surface_vertical(hit_px);
        s = s.child(capsule);
        s.child(anchor_layer)
    } else {
        let mut capsule = capsule;
        capsule.position = NodePosition::Absolute {
            top: Some(inset),
            left: Some(0.0),
            right: Some(0.0),
            bottom: None,
        };
        let anchor_layer = fraction_anchor(physical, hit_px, hit, hit_px * 0.5);
        let mut s = block_surface(hit_px);
        s = s.child(capsule);
        s.child(anchor_layer)
    };
    if let Some(handler) = scrub_handler {
        surface = surface.child(block_grab_with_axis(handler, scrub_axis(spec.orientation)));
    }

    let mut root = Node::container();
    if vertical {
        root.style.fill_height = true;
        root.style.descriptor.layout.width = LayoutSizing::Fixed(hit_px);
        root.style.min_width = Some(hit_px);
        root.style.min_height = Some(rem_to_px(10.0));
        root.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    } else {
        root.style.fill_width = true;
        root.style.descriptor.layout.direction = LayoutDirection::Column;
    }
    root.roles
        .insert("appearance".to_owned(), "block".to_owned());
    root.roles.insert("orientation".to_owned(), orientation_name(spec.orientation).to_owned());
    root.roles.insert(
        "direction".to_owned(),
        if rtl { "rtl" } else { "ltr" }.to_owned(),
    );
    root = root.child(surface);
    // g18.017: single Slider block text never renders an external fallback
    // line; whole-track collision suppresses the optional label instead.
    if spec.is_disabled {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        stamp_disabled_roles(&mut root);
    }
    root
}

fn omit_empty_owned(text: Option<&str>) -> Option<String> {
    match text {
        Some(value) if !value.is_empty() => Some(value.to_owned()),
        _ => None,
    }
}

/// One slot of the stable block text row. An absent/suppressed item keeps its
/// slot as empty text so the surviving item stays pinned to its logical edge
/// (`MainAxisAlignment::SpaceBetween` moves a lone child to the start).
fn block_text_slot(content: Option<String>, color: ColorValue, size: f32, id: &str) -> Node {
    let mut node = Node::text(content.unwrap_or_default());
    node.id = Some(id.to_owned());
    node.style.descriptor.text_color = Some(color);
    node.style.text_size = Some(size);
    node.style.no_wrap = true;
    node.style.flex_none = true;
    node
}

/// The full-capsule text layout shared by both clipped text layers.
///
/// Horizontal (g18.017): optional label pinned to the logical inline start,
/// exact value pinned to the logical inline end, at every value. Vertical
/// (g18.022): the text stays upright with the exact value at the physical
/// top and the optional label centered. Padding matches the fit law's
/// content inset convention (0.5rem web / 8px native).
#[allow(clippy::too_many_arguments)]
fn block_text_row(
    label: Option<String>,
    value: Option<String>,
    color: ColorValue,
    size: f32,
    row_span: f32,
    row_height: f32,
    rtl: bool,
    vertical: bool,
    label_id: &str,
    value_id: &str,
) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.width = LayoutSizing::Fixed(row_span);
    row.style.descriptor.layout.height = LayoutSizing::Fixed(row_height);
    row.style.descriptor.layout.direction = if vertical {
        LayoutDirection::Column
    } else {
        LayoutDirection::Row
    };
    row.style.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    let inset = rem_to_px(0.5);
    row.style.descriptor.layout.spacing.padding.left = inset;
    row.style.descriptor.layout.spacing.padding.right = inset;
    let label_slot = block_text_slot(label, color, size, label_id);
    let value_slot = block_text_slot(value, color, size, value_id);
    row = if vertical {
        // Value at the physical top, label centered, and an empty bottom
        // slot completing the three-anchor law. Text stays upright; nothing
        // rotates.
        let bottom_slot = block_text_slot(None, color, size, "block-slider-spacer");
        row.child(value_slot).child(label_slot).child(bottom_slot)
    } else if rtl {
        row.child(value_slot).child(label_slot)
    } else {
        row.child(label_slot).child(value_slot)
    };
    row
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn find_scrub(node: &Node) -> Option<&Node> {
        node.find(&|n| n.interaction.on_scrub.is_some())
    }

    fn slider_control(node: &Node) -> &Node {
        node.find(&|n| n.a11y.role == Some(NodeRole::Slider))
            .expect("one slider node")
    }

    fn armed(spec: SliderSpec) -> (Node, Arc<std::sync::Mutex<Vec<(String, f64)>>>) {
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let change = Arc::clone(&seen);
        let commit = Arc::clone(&seen);
        let theme = theme();
        let root = RenderContext::new(&theme);
        let ctx = if spec.variant == SliderVariant::Block {
            use poodle_headless::slider::measure_block_advance;
            root.with_block_layout(160.0, Arc::new(measure_block_advance))
        } else {
            root
        };
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
        let spec = SliderSpec::new(0.5)
            .with_bounds(0.0, 1.0)
            .with_variant(SliderVariant::Embedded);
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
        let spec = SliderSpec::new(0.5)
            .with_bounds(0.0, 1.0)
            .with_variant(SliderVariant::Embedded);
        let node = slider(&spec, &ctx, &SliderHandlers::default());
        assert!(find_scrub(&node).is_none());
        assert!(slider_control(&node).interaction.on_key.is_none());
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
        let key = slider_control(&node).interaction.on_key.as_ref().unwrap();
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
        let control = slider_control(&node);
        assert_eq!(control.a11y.role, Some(NodeRole::Slider));
        assert_eq!(control.a11y.label.as_deref(), Some("Volume"));
        assert_eq!(control.a11y.value, Some(40.0));
        assert_eq!(control.a11y.value_min, Some(0.0));
        assert_eq!(control.a11y.value_max, Some(100.0));
        assert_eq!(control.a11y.value_text.as_deref(), Some("quiet"));
        assert_eq!(control.a11y.orientation.as_deref(), Some("horizontal"));
        assert!(control.interaction.focusable);
        assert_eq!(control.a11y.tab_index, Some(0));
        assert!(!control.interaction.disabled);
        let ring = control.style.focus_ring.expect("standard thumb ring");
        assert!((ring.width - rem_to_px(0.1875)).abs() < 1e-6);
        assert!((ring.offset - 0.0).abs() < 1e-6);
        assert!((ring.color.3 - 0.32).abs() < 1e-6);
        assert!(
            node.style.focus_ring.is_none(),
            "standard focus belongs on the thumb, not the root"
        );
        assert!(!node.interaction.focusable);
    }

    #[test]
    fn embedded_focus_is_a_root_outline() {
        let mut spec = SliderSpec::new(40.0)
            .with_bounds(0.0, 100.0)
            .with_embedded_control(poodle_headless::slider::SliderPolarity::Unipolar);
        spec.aria_label = Some("Gain".into());
        let (node, _) = armed(spec);
        let ring = node.style.focus_ring.expect("embedded root ring");
        assert_eq!(node.a11y.role, Some(NodeRole::Slider));
        assert!(node.interaction.focusable);
        assert_eq!(node.a11y.tab_index, Some(0));
        assert!((ring.width - rem_to_px(0.125)).abs() < 1e-6);
        assert!((ring.offset - rem_to_px(0.0625)).abs() < 1e-6);
        assert!((ring.color.3 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn disabled_slider_is_inert() {
        let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
        spec.is_disabled = true;
        spec.aria_label = Some("Volume".into());
        let (node, seen) = armed(spec);
        let control = slider_control(&node);
        assert!(find_scrub(&node).is_none());
        assert!(control.interaction.on_key.is_none());
        assert!(control.interaction.disabled);
        assert!(!control.interaction.focusable);
        assert_eq!(control.a11y.tab_index, None);
        assert!(control.style.focus_ring.is_none());
        assert_eq!(control.a11y.role, Some(NodeRole::Slider));
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

    fn block_hit_node(node: &Node) -> &Node {
        node.find(&|n| n.roles.get("part").map(String::as_str) == Some("hit"))
            .expect("block hit")
    }

    #[test]
    fn block_hit_is_forty_four_and_forced_colors_keep_roles() {
        let spec = SliderSpec::new(50.0)
            .with_bounds(0.0, 100.0)

            .with_visible_label("Blur")
            .with_visible_value_text("50");
        let (node, _) = armed(spec);
        let hit = block_hit_node(&node);
        assert_eq!(hit.style.descriptor.layout.width, LayoutSizing::Fixed(44.0));
        assert_eq!(
            hit.style.descriptor.layout.height,
            LayoutSizing::Fixed(44.0)
        );
        assert_eq!(hit.a11y.role, Some(NodeRole::Slider));
        let selected = node
            .find(&|n| n.roles.get("forced-color-fill").map(String::as_str) == Some("selection"))
            .expect("selected fill");
        let remainder = node
            .find(&|n| n.roles.get("forced-color-fill").map(String::as_str) == Some("canvas"))
            .expect("remainder fill");
        assert_eq!(
            selected.roles.get("forced-color-text").map(String::as_str),
            Some("selection-text")
        );
        assert_eq!(
            remainder.roles.get("forced-color-text").map(String::as_str),
            Some("canvas-text")
        );
        assert_ne!(
            selected.roles.get("forced-color-fill"),
            remainder.roles.get("forced-color-fill")
        );
        assert!(node
            .find(&|n| n.roles.get("part").map(String::as_str) == Some("fallback"))
            .is_none());
        let texts = node.texts().join(" ");
        assert!(!texts.contains("Volume"));
    }

    #[test]
    fn collision_suppresses_the_label_and_keeps_the_exact_value() {
        let spec = SliderSpec::new(10.0)
            .with_bounds(0.0, 100.0)

            .with_visible_label("Compressor makeup gain")
            .with_visible_value_text("10");
        let (node, _) = armed(spec);
        // 22-char label + 2-char value cannot coexist at the 160px context
        // span, so the optional label is suppressed while the exact value
        // keeps painting inside the capsule.
        assert!(node
            .find(&|n| matches!(&n.kind, NodeKind::Text { content } if content == "Compressor makeup gain"))
            .is_none());
        assert!(node
            .find(&|n| matches!(&n.kind, NodeKind::Text { content } if content == "10"))
            .is_some());
        // A single Slider never renders the external fallback line.
        assert!(node
            .find(&|n| n.roles.get("part").map(String::as_str) == Some("fallback"))
            .is_none());
    }

    #[test]
    fn block_text_layers_split_at_the_fill_boundary() {
        let spec = SliderSpec::new(25.0)
            .with_bounds(0.0, 100.0)

            .with_visible_label("Blur")
            .with_visible_value_text("25");
        let (node, _) = armed(spec);
        let selected_clip = node
            .find(&|n| n.id.as_deref() == Some("block-slider-clip-selected"))
            .expect("selected clip");
        let remainder_clip = node
            .find(&|n| n.id.as_deref() == Some("block-slider-clip-remainder"))
            .expect("remainder clip");
        let LayoutSizing::Fixed(selected_w) = selected_clip.style.descriptor.layout.width else {
            panic!("selected clip width");
        };
        let LayoutSizing::Fixed(remainder_w) = remainder_clip.style.descriptor.layout.width
        else {
            panic!("remainder clip width");
        };
        assert!((selected_w - 40.0).abs() < 1e-4, "clip tracks the 25% fill");
        assert!((remainder_w - 120.0).abs() < 1e-4);
        assert!(matches!(
            selected_clip.style.descriptor.layout.overflow_x,
            LayoutOverflow::Hidden
        ));
        // One stable full-capsule row per layer: same span, same glyph
        // metrics, different foreground role. Both rows stamp the same slot
        // ids, so the roles are asserted per layer row instead.
        let selected_row = &selected_clip.children[0];
        let remainder_row = &remainder_clip.children[0];
        let selected_slot = selected_row
            .find(&|n| n.id.as_deref() == Some("block-slider-value-selected"))
            .expect("selected value slot");
        let remainder_slot = remainder_row
            .find(&|n| n.id.as_deref() == Some("block-slider-value-remainder"))
            .expect("remainder value slot");
        assert_eq!(selected_slot.style.text_size, remainder_slot.style.text_size);
        let selected_row = &selected_clip.children[0];
        let remainder_row = &remainder_clip.children[0];
        assert_eq!(
            selected_row.roles.get("forced-color-text").map(String::as_str),
            Some("selection-text")
        );
        assert_eq!(
            remainder_row.roles.get("forced-color-text").map(String::as_str),
            Some("canvas-text")
        );
    }

    #[test]
    fn block_rtl_remaps_scrub_without_changing_keys() {
        let spec = SliderSpec::new(0.0)
            .with_bounds(0.0, 100.0)

            .with_direction(poodle_specs::SliderDirection::Rtl);
        let (node, seen) = armed(spec);
        let scrub = Arc::clone(
            find_scrub(&node)
                .unwrap()
                .interaction
                .on_scrub
                .as_ref()
                .unwrap(),
        );
        scrub(0.2, ScrubPhase::Press);
        assert_eq!(
            seen.lock().unwrap().last().unwrap(),
            &("change".into(), 80.0)
        );
        let key = slider_control(&node).interaction.on_key.as_ref().unwrap();
        key(NodeKey::ArrowRight, NodeModifiers::default());
        assert_eq!(seen.lock().unwrap().last().unwrap().1, 81.0);
    }

    #[test]
    fn a_second_scrub_release_is_inert() {
        let spec = SliderSpec::new(0.0)
            .with_bounds(0.0, 100.0)
            ;
        let (node, seen) = armed(spec);
        let scrub = Arc::clone(
            find_scrub(&node)
                .unwrap()
                .interaction
                .on_scrub
                .as_ref()
                .unwrap(),
        );
        scrub(0.4, ScrubPhase::Press);
        scrub(0.4, ScrubPhase::Release);
        scrub(0.4, ScrubPhase::Release);
        let events = seen.lock().unwrap();
        assert_eq!(
            events.as_slice(),
            [("change".into(), 40.0), ("commit".into(), 40.0)]
        );
    }

    #[test]
    fn block_fit_follows_context_span_and_measure() {
        let theme = theme();
        let spec = SliderSpec::new(50.0)
            .with_bounds(0.0, 100.0)

            .with_visible_label("AB")
            .with_visible_value_text("50");
        let measure: crate::context::BlockTextMeasure =
            Arc::new(|text: &str, _font| text.chars().count() as f32 * 30.0);
        let root = RenderContext::new(&theme);
        // 200px span: 184 available, 120 needed — both strings coexist.
        let wide = root.with_block_layout(200.0, Arc::clone(&measure));
        let wide_node = slider(&spec, &wide, &SliderHandlers::default());
        assert!(wide_node
            .find(&|n| matches!(&n.kind, NodeKind::Text { content } if content == "AB"))
            .is_some());
        assert!(wide_node
            .find(&|n| n.roles.get("part").map(String::as_str) == Some("fallback"))
            .is_none());

        // 100px span: 84 available — the label suppresses, the value stays.
        let narrow = root.with_block_layout(100.0, measure);
        let narrow_node = slider(&spec, &narrow, &SliderHandlers::default());
        assert!(narrow_node
            .find(&|n| matches!(&n.kind, NodeKind::Text { content } if content == "AB"))
            .is_none());
        assert!(narrow_node
            .find(&|n| matches!(&n.kind, NodeKind::Text { content } if content == "50"))
            .is_some());
        assert!(narrow_node
            .find(&|n| n.roles.get("part").map(String::as_str) == Some("fallback"))
            .is_none());
    }

    #[should_panic(
        expected = "Slider appearance=\"block\" requires RenderContext::with_block_layout"
    )]
    #[test]
    fn block_without_layout_inputs_panics() {
        let theme = theme();
        let spec = SliderSpec::new(50.0)
            .with_bounds(0.0, 100.0)
            ;
        let _node = slider(
            &spec,
            &RenderContext::new(&theme),
            &SliderHandlers::default(),
        );
    }

    #[test]
    fn vertical_block_keeps_upright_text_and_a_vertical_scrub() {
        let spec = SliderSpec::new(40.0)
            .with_bounds(0.0, 100.0)
            .with_orientation(Orientation::Vertical)
            .with_visible_label("Blur")
            .with_visible_value_text("40");
        let (node, seen) = armed(spec);
        let carrier = find_scrub(&node).unwrap();
        assert_eq!(carrier.interaction.scrub_axis, ScrubAxis::Vertical);
        // Pressing near the top drives the value toward the maximum.
        let scrub = Arc::clone(
            find_scrub(&node)
                .unwrap()
                .interaction
                .on_scrub
                .as_ref()
                .unwrap(),
        );
        scrub(0.2, ScrubPhase::Press);
        let pressed = seen.lock().unwrap().last().unwrap().1;
        assert!((pressed - 20.0).abs() < 1e-6);
        // Upright text: both strings paint inside the capsule.
        let texts = node.texts();
        assert!(texts.iter().any(|t| *t == "Blur"));
        assert!(texts.iter().any(|t| *t == "40"));
        assert!(node
            .find(&|n| n.roles.get("part").map(String::as_str) == Some("fallback"))
            .is_none());
        assert_eq!(slider_control(&node).a11y.orientation.as_deref(), Some("vertical"));
    }

    #[test]
    fn aria_label_never_becomes_visible_block_text() {
        let mut spec = SliderSpec::new(50.0)
            .with_bounds(0.0, 100.0)
            ;
        spec.aria_label = Some("Gain".into());
        let (node, _) = armed(spec);
        let texts = node.texts().join(" ");
        assert!(!texts.contains("Gain"));
        assert_eq!(slider_control(&node).a11y.label.as_deref(), Some("Gain"));
    }

    #[test]
    fn visible_label_never_becomes_block_accessible_name() {
        let spec = SliderSpec::new(50.0)
            .with_bounds(0.0, 100.0)

            .with_visible_label("Blur");
        let (node, _) = armed(spec);
        assert_eq!(slider_control(&node).a11y.label, None);
        let fallback_or_inline = node.texts().join(" ");
        assert!(
            fallback_or_inline.contains("Blur"),
            "visibleLabel still paints: {fallback_or_inline:?}"
        );
    }
}
