//! GPUI backend for Poodle's render vocabulary: interpret a [`poodle_node::Node`]
//! tree as GPUI elements.
//!
//! This crate is the GPUI half of the inversion `g12.019` completes. Poodle's
//! components (`poodle-render`) emit `Spec + Theme → Node` trees and know
//! nothing of GPUI; this adapter translates that vocabulary into GPUI 0.2.2's
//! fluent element API. The transcription source is the Jetstream backend
//! (`jetstream-poodle/src/lib.rs`), whose channel walk this mirrors channel by
//! channel; where GPUI has no equivalent channel the gap is documented inline
//! and in the crate's channel table (see `docs/roadmaps/g12/019-gpui-node-backend.md`).
//!
//! What this backend owns (and the vocabulary correctly does not): text
//! measurement and shaping (GPUI's text system), hit-testing and event
//! dispatch, icon rasterisation (SVG via the app's asset source), animation
//! clocks.
//!
//! Color: the vocabulary is sRGB and GPUI's `Hsla`/`Rgba` are sRGB, so the
//! conversion at this edge is a raw passthrough — the same path the old GPUI
//! tier used. No transfer function applies;
//! alpha is coverage and passes through. All mixing happened render-side
//! (`poodle-render::color`); nodes carry final values.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use gpui::{
    canvas, deferred, div, img, linear_color_stop, linear_gradient, point, px, relative, size, svg,
    AnyElement, App, AppContext, Bounds, ClickEvent, CursorStyle, Div, ElementId, Hsla,
    InteractiveElement, IntoElement, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, ParentElement, PathBuilder, Pixels, ScrollDelta, ScrollWheelEvent, SharedString,
    Stateful, StatefulInteractiveElement, StyleRefinement, Styled, StyledImage, Window,
};
use poodle_node::{
    AnimEasing, AnimLoop, AnimProperty, ColorValue, ContinuousValuePhase, CrossAxisAlignment,
    CursorHint, FocusRing, FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeAnimation, NodeContinuousValueEvent, NodeDragEvent, NodeDragPhase,
    NodeKey, NodeKind, NodeModifiers, NodePoint, NodePosition, NodeWheelEvent,
    ResolvedIconGeometryFrame, ScrubAxis, ScrubPhase, SelectGranularity, StylePatch, TextAlign,
};

mod drag;
mod inset_shadow;
mod interaction;
mod layers;
mod measured_node;
mod style;
mod tooltip;
mod tracked_scroll;

pub mod file_capability;

pub use tooltip::{
    is_tooltip_pending, is_tooltip_visible, painted_tooltip, painted_tooltip_for,
    reset_tooltip_registry, teardown_window_tooltips, tooltip_runtime_owns_window, PaintedTooltip,
    TOOLTIP_DELAY,
};
pub use tracked_scroll::{tracked_vertical_scroll, TrackedScrollOptions, TrackedScrollState};

pub use drag::{
    drag_drop_provider, drag_drop_window_host, DragAnnouncementEvent, DragDropController,
    DragDropSnapshot, DragDropTargetPosture, DragDropWindowHost, DragPreviewSnapshot,
    NativeDragPayload, ANNOUNCEMENT_LOG_LIMIT, GPUI_DRAG_CAPABILITIES,
};
use interaction::apply_listeners;
pub use layers::{
    attach_overlay_host, bounds_for, dismiss_innermost, dismiss_layers_at, layer_for_element,
    open_layer_count, overlay_frame_begin, overlay_frame_begin_for, overlay_frame_end,
    overlay_frame_end_for, request_focus, spared_layer_ids_at,
};
use style::{
    apply_cursor, apply_layout, apply_paint, apply_patch, apply_position, apply_state_patches,
    apply_text,
};

thread_local! {
    static PROBE_ACTIVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static PROBE_CHANNELS: RefCell<std::collections::BTreeSet<&'static str>> =
        RefCell::new(std::collections::BTreeSet::new());
}

/// Begin a bounded receipt for the real backend channel walk.
pub fn begin_probe_capture() {
    PROBE_CHANNELS.with(|channels| channels.borrow_mut().clear());
    PROBE_ACTIVE.with(|active| active.set(true));
}

/// Finish the current receipt. Markers sit inside the mapping branches, so a
/// removed backend emission also removes its evidence.
pub fn take_probe_capture() -> Vec<&'static str> {
    PROBE_ACTIVE.with(|active| active.set(false));
    PROBE_CHANNELS.with(|channels| channels.borrow().iter().copied().collect())
}

pub(crate) fn record_probe_channel(channel: &'static str) {
    if PROBE_ACTIVE.with(|active| active.get()) {
        PROBE_CHANNELS.with(|channels| {
            channels.borrow_mut().insert(channel);
        });
    }
}

/// sRGB passthrough — the exact conversion the old GPUI tier performed.
/// gpui's `Rgba` channels are sRGB; the round trip through `Hsla` is what
/// `theme_ext::resolve_color` did for every token the old tier resolved.
pub fn color(c: ColorValue) -> Hsla {
    gpui::Rgba {
        r: c.0,
        g: c.1,
        b: c.2,
        a: c.3,
    }
    .into()
}

thread_local! {
    /// Deterministic per-tree ids for nodes that need element state
    /// (interaction) but declare none. Tree order is stable across frames for
    /// a stable tree, so a counter keeps the same node on the same id between
    /// rebuilds — but ONLY if the counter restarts each frame. The counter is
    /// local to the UI thread, matching GPUI's render model and the
    /// thread-local focus/ring registries it keys. Independent headless apps
    /// must not reset one another while Rust runs their tests in parallel.
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };

    /// Per-frame counter for gesture-drag identities. It shares the element
    /// counter's thread boundary for the same reason.
    static NEXT_GESTURE_ID: Cell<usize> = const { Cell::new(0) };
}

/// Restart the generated-id counter. Call once per frame, before building.
///
/// gpui stores a click's `pending_mouse_down` in the element state it keys by
/// `ElementId`. A real click spans many frames, so if a node's id changes
/// between the press and the release, the release reads a fresh state, finds
/// no pending press, and the click is silently dropped. Without this reset the
/// counter runs monotonically forever and every generated id is new on every
/// frame — so every node that does not declare an id becomes unclickable.
///
/// This is invisible to both existing gates: the visual gate compares static
/// frames, and the in-process click driver posts press and release inside a
/// single frame, so it never crosses a rebuild.
pub fn reset_element_ids() {
    NEXT_ID.with(|next| next.set(0));
    NEXT_GESTURE_ID.with(|next| next.set(0));
}

/// Drop backend-owned focus handles. Call at the start of a new headless
/// window so a previous mount's ids cannot short-circuit wait_for_focus_handle.
pub fn reset_focus_registry() {
    FOCUS_HANDLES.with(|handles| handles.borrow_mut().clear());
    FOCUS_STATES.with(|states| states.borrow_mut().clear());
    FOCUSED_FIELD.with(|field| *field.borrow_mut() = None);
    interaction::reset_continuous_value_session();
    tooltip::reset_tooltip_registry();
}

/// Per-frame counter for gesture-drag identities.
///
/// Reset with the element ids, so a node gets the same gesture id on every
/// frame: the tree is walked in the same order each time, and a drag begun on
/// one frame has to still recognise itself on the next.
fn next_gesture_id() -> String {
    NEXT_GESTURE_ID.with(|next| {
        let id = next.get();
        next.set(id + 1);
        format!("gesture-{id}")
    })
}

fn element_id(node: &Node) -> ElementId {
    if let Some(id) = &node.runtime_id {
        return ElementId::Name(SharedString::from(id.clone()));
    }
    if let Some(id) = &node.id {
        return ElementId::Name(SharedString::from(id.clone()));
    }
    if let Some(anim) = &node.style.animation {
        // Vocabulary: an animation's key becomes the id when none is set —
        // nodes sharing a key share a clock.
        return ElementId::Name(SharedString::from(anim.key.clone()));
    }
    NEXT_ID.with(|next| {
        let id = next.get();
        next.set(id + 1);
        ElementId::Name(SharedString::from(format!("poodle-node-{id}")))
    })
}

/// The string form of a resolved element id. `element_id` only ever mints
/// `Name` ids, so this is lossless.
fn element_id_text(id: &ElementId) -> String {
    match id {
        ElementId::Name(name) => name.to_string(),
        other => unreachable!("the node backend only mints Name element ids, got {other:?}"),
    }
}

/// Interpret one node (and its subtree) as a GPUI element.
pub fn to_gpui(node: &Node) -> AnyElement {
    // Layer registration runs for every independently converted root; the
    // frame-scoped registries are cleared by the host's overlay_frame_begin
    // once per rendered frame (the production preview and the conformance
    // driver both call it), so a real page's multiple conversions all land
    // in the same frame's registry. overlay_frame_begin also cancels a
    // continuous-value gesture whose owner was not rebuilt last frame.
    layers::collect_layers(node, None);
    // Drop-target nesting depth is a fact about the tree, so it is read from
    // the tree — before the build, while the parent/child relation still
    // exists. Paint order and measured rectangles cannot recover it.
    drag::collect_drop_depths(node);
    to_gpui_impl(node)
}

fn to_gpui_impl(node: &Node) -> AnyElement {
    if !node.roles.is_empty() {
        record_probe_channel("semantic.token-roles.received");
    }
    if node.a11y.role.is_some() && node.a11y.label.is_some() {
        record_probe_channel("accessibility.projection.received");
    }
    if node.a11y.toggled.is_some() {
        record_probe_channel("toggle.received");
    }
    if node.interaction.disabled {
        record_probe_channel("semantic.disabled.received");
    }
    match &node.kind {
        NodeKind::Container => {
            record_probe_channel("structure.identity.container");
            build_box(node, div())
        }
        NodeKind::Text { content } => {
            record_probe_channel("content.text-icon.text");
            // A text node carrying a caret is a field's value: the component
            // draws it as text so it stays a single node in the accessibility
            // tree (an input nested inside an input is one control announced
            // twice), and the caret channel is what asks for measurement.
            if node.caret.is_some() && !content.contains('\n') {
                let text_color = node
                    .style
                    .descriptor
                    .text_color
                    .map(color)
                    .unwrap_or_else(gpui::white);
                let id = element_id_string(node);
                let focused = is_focused(&id) || FOCUS_SCOPE.with(|f| f.get());
                let caret = node.caret.expect("checked above");
                let value_for_ime = if caret.showing_placeholder {
                    String::new()
                } else {
                    content.clone()
                };
                let ime = crate::ime::NodeInputHandler {
                    id: id.clone(),
                    value: value_for_ime,
                    selection: caret.selection,
                    insert: node.interaction.on_edit_insert.clone(),
                    select: node.interaction.on_select_range.clone().map(|handler| {
                        Arc::new(move |a: usize, b: usize| {
                            handler(a, b, SelectGranularity::Character)
                        }) as Arc<dyn Fn(usize, usize) + Send + Sync>
                    }),
                };
                let mut element = input_text::input_text(
                    id,
                    content.clone(),
                    // The value is empty whenever the placeholder is what is on
                    // screen; the caret and the history both count into the
                    // value, never the prompt.
                    if caret.showing_placeholder {
                        String::new()
                    } else {
                        content.clone()
                    },
                    text_color,
                    Some(caret.selection),
                    color(caret.caret_color),
                    color(caret.selection_color),
                    focused,
                );
                element.ime = Some(ime);
                return build_box(node, div().child(element));
            }
            build_box(node, div().child(content.clone()))
        }
        // GPUI has no native button element; the old tier's buttons are styled
        // divs too, so the label-child div is the faithful mapping. Same for
        // Input: a real GPUI text field is an `Editor` entity, which a pure
        // `&Node -> element` function cannot create. A childless input renders
        // its intrinsic value/placeholder; composite inputs supply styled
        // children (affixes, icons, count) and the backend avoids duplicating
        // the value. Caret/selection/IME remain a backend gap.
        NodeKind::Button { label } => {
            record_probe_channel("structure.identity.button");
            let el = if label.is_empty() {
                div()
            } else if matches!(
                node.a11y.role,
                Some(poodle_node::NodeRole::Button | poodle_node::NodeRole::RadioButton)
            ) {
                // The old GPUI button and radio segment place their labels
                // directly in the styled control. The generic wrapper changes
                // intrinsic text measurement and centering.
                div().child(label.clone())
            } else {
                div().child(
                    div()
                        .whitespace_nowrap()
                        .min_w(px(0.0))
                        .child(label.clone()),
                )
            };
            build_box(node, el)
        }
        NodeKind::Input { value, placeholder } => {
            record_probe_channel("structure.identity.input");
            // A childless input renders its own value; composite inputs supply
            // styled children (affixes, count) and the backend must not
            // duplicate the value underneath them.
            let el = if node.children.is_empty() {
                let display = if value.is_empty() {
                    placeholder.clone()
                } else {
                    value.clone()
                };
                // A multi-line value is not this element's job: `shape_line`
                // shapes exactly one line and *panics* on a newline, and a
                // markdown body wants wrapping anyway. Fall back to the plain
                // wrapped text child, which is what these fields rendered
                // before the caret existed — no caret, but no lost content.
                if display.contains('\n') {
                    return build_box(node, div().child(display));
                }
                let text_color = node
                    .style
                    .descriptor
                    .text_color
                    .map(color)
                    .unwrap_or_else(gpui::white);
                let id = element_id_string(node);
                let focused = is_focused(&id) || FOCUS_SCOPE.with(|f| f.get());
                div().child(input_text::input_text(
                    id,
                    display,
                    value.clone(),
                    text_color,
                    node.caret.map(|c| c.selection),
                    node.caret
                        .map(|c| color(c.caret_color))
                        .unwrap_or(text_color),
                    node.caret
                        .map(|c| color(c.selection_color))
                        .unwrap_or(text_color),
                    focused,
                ))
            } else {
                div()
            };
            build_box(node, el)
        }
        NodeKind::Progress { fraction } => {
            record_probe_channel("structure.identity.progress");
            // The node styles the track; the backend fills `fraction` of it.
            // Fill colour comes from `text_color` when the component set one —
            // the vocabulary carries no dedicated fill channel (Jetstream's
            // progress widget supplies its own). UNPROVEN: no gated specimen
            // exercises this yet (progress is skipped as non-deterministic).
            let fill_color = node
                .style
                .descriptor
                .text_color
                .map(color)
                .unwrap_or_else(|| gpui::white());
            let fill = div()
                .h_full()
                .w(relative(fraction.clamp(0.0, 1.0)))
                .rounded_full()
                .bg(fill_color);
            build_box(node, div().child(fill))
        }
        NodeKind::ProgressRing { fraction } => {
            record_probe_channel("structure.identity.progress-ring");
            let track_color = color(node.style.descriptor.border.color);
            let fill_color = node
                .style
                .descriptor
                .text_color
                .map(color)
                .unwrap_or_else(|| gpui::white());
            let fraction = fraction.clamp(0.0, 1.0);
            let ring = canvas(
                move |_, _, _| {},
                move |bounds, _, window, _| {
                    let width = f32::from(bounds.size.width);
                    let height = f32::from(bounds.size.height);
                    let diameter = width.min(height);
                    let stroke = px(2.0);
                    let radius = (diameter - 2.0) / 2.0;
                    let center = point(
                        bounds.origin.x + px(width / 2.0),
                        bounds.origin.y + px(height / 2.0),
                    );
                    let radii = point(px(radius), px(radius));
                    let top = point(center.x, center.y - px(radius));
                    let bottom = point(center.x, center.y + px(radius));

                    let mut track = PathBuilder::stroke(stroke);
                    track.move_to(top);
                    track.arc_to(radii, px(0.0), false, true, bottom);
                    track.arc_to(radii, px(0.0), false, true, top);
                    if let Ok(path) = track.build() {
                        window.paint_path(path, track_color);
                    }

                    if fraction > 0.0 {
                        let angle = -std::f32::consts::FRAC_PI_2 + std::f32::consts::TAU * fraction;
                        let end = point(
                            center.x + px(radius * angle.cos()),
                            center.y + px(radius * angle.sin()),
                        );
                        let mut fill = PathBuilder::stroke(stroke);
                        fill.move_to(top);
                        if fraction >= 1.0 {
                            fill.arc_to(radii, px(0.0), false, true, bottom);
                            fill.arc_to(radii, px(0.0), false, true, top);
                        } else {
                            fill.arc_to(radii, px(0.0), fraction > 0.5, true, end);
                        }
                        if let Ok(path) = fill.build() {
                            window.paint_path(path, fill_color);
                        }
                    }
                },
            )
            .size_full();
            build_box(node, div().child(ring))
        }
        NodeKind::Icon { name, size } => {
            record_probe_channel("content.text-icon.icon");
            // Same path convention as the old tier's Icon: the app owns the
            // asset source; the name is the contract. svg() renders tinted by
            // `text_color`, which the style walk supplies.
            let el = svg()
                .path(SharedString::from(format!("assets/icons/{name}.svg")))
                .size(px(*size))
                .flex_shrink_0();
            build_svg_leaf(node, el)
        }
        NodeKind::ResolvedIconGeometry { size, frame } => {
            record_probe_channel("content.text-icon.resolved-geometry");
            let paint_color = node
                .style
                .descriptor
                .text_color
                .map(color)
                .unwrap_or_else(gpui::white);
            let size = *size;
            let frame = frame.clone();
            let glyph = canvas(
                move |_, _, _| {},
                move |bounds, _, window, _| {
                    record_probe_channel("content.text-icon.resolved-geometry.paint");
                    paint_resolved_icon_geometry(window, bounds, &frame, paint_color);
                },
            )
            .size(px(size))
            .flex_shrink_0();
            build_box(node, div().child(glyph).size(px(size)).flex_shrink_0())
        }
        NodeKind::Image { source } => {
            record_probe_channel("structure.identity.image");
            // Vocabulary: fits by covering the box (object-fit: cover).
            let el = img(source.clone()).object_fit(gpui::ObjectFit::Cover);
            build_leaf(node, el)
        }
    }
}

/// Leaves (svg, img) implement `Styled` but not `InteractiveElement`/
/// `ParentElement` in gpui 0.2.2. A leaf node that declares interaction,
/// state patches, or children is wrapped in a div that carries them — the
/// leaf keeps its own sizing and colour.
fn build_leaf<E>(node: &Node, el: E) -> AnyElement
where
    E: Styled + IntoElement + 'static,
{
    let el = apply_layout(el, node);
    let el = apply_paint(el, node);
    let el = apply_text(el, node);
    let el = apply_cursor(el, node);
    let needs_wrapper = node.style.hover.is_some()
        || node.style.active.is_some()
        || node.style.focus_ring.is_some()
        || node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || node.interaction.on_continuous_value.is_some()
        || node.interaction.on_wheel.is_some()
        || node.interaction.on_double_activate.is_some()
        || !node.children.is_empty();
    if !needs_wrapper {
        return maybe_animated(el, node);
    }
    let wrapped = div().child(el);
    build_box(node, wrapped)
}

/// SVG leaves can carry the vocabulary's rotation channel directly. GPUI
/// exposes transforms on SVG elements, but not on generic Styled elements.
fn build_svg_leaf(node: &Node, el: gpui::Svg) -> AnyElement {
    use gpui::{AnimationExt, Transformation};

    let el = apply_layout(el, node);
    let el = apply_paint(el, node);
    let el = apply_text(el, node);
    let el = apply_cursor(el, node);
    // `node.id` forces the wrapper: an identified leaf must go through
    // `build_box` so its paint bounds are recorded for `bounds_for`
    // observation (the g15.047 capture seam). The wrapper carries the
    // animation channels then — opacity only, so an identified spinning leaf
    // keeps its clock but not its rotation; no production tree identifies an
    // animated icon, and the capture host freezes motion regardless.
    let needs_wrapper = node.style.hover.is_some()
        || node.style.active.is_some()
        || node.style.focus_ring.is_some()
        || node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || node.interaction.on_continuous_value.is_some()
        || node.interaction.on_wheel.is_some()
        || node.interaction.on_double_activate.is_some()
        || !node.children.is_empty()
        || node.id.is_some();
    if needs_wrapper {
        return build_box(node, div().child(el));
    }

    let Some(anim) = &node.style.animation else {
        return el.into_any_element();
    };
    record_probe_channel("surface.animation.scheduled");
    if sample_property(anim, AnimProperty::Rotate, 0.0).is_none() {
        return maybe_animated(el, node);
    }

    let anim = anim.clone();
    let id = element_id(node);
    el.with_animation(id, gpui_animation(&anim), move |svg, t| {
        let radians = sample_property(&anim, AnimProperty::Rotate, t).unwrap_or(0.0);
        svg.with_transformation(Transformation::rotate(gpui::radians(radians)))
    })
    .into_any_element()
}

/// Container-shaped nodes: the full channel walk. Interaction that needs
/// element state (click, drag, focus) forces a stateful div — gpui 0.2.2
/// gates its listener model behind `Stateful`.
fn build_box(node: &Node, base: Div) -> AnyElement {
    // Nested overlay surfaces (a popover inside a popover) draw inside the
    // enclosing deferred element; only the outermost overlay defers.
    let was_deferred = DEFERRED_SCOPE.with(|scope| scope.get());
    if node.style.overlay {
        DEFERRED_SCOPE.with(|scope| scope.set(true));
    }
    let previous_layer = CURRENT_DISMISS_LAYER.with(|layer| layer.borrow().clone());
    if let Some(layer_id) = node.interaction.dismiss_layer.clone() {
        CURRENT_DISMISS_LAYER.with(|layer| *layer.borrow_mut() = Some(layer_id));
    }
    let element = if needs_state(node) {
        // Resolve the element id ONCE and use the same identity for the
        // element and every focus/ring registry: `element_id` mints a fresh
        // generated name per call, and keying the registries by
        // `element_id_string` ("" for an id-less node) made every unstamped
        // control share one focus handle.
        let id = element_id(node);
        let id_string = element_id_text(&id);
        let el = base.id(id);
        let el = apply_shared(el, node, &id_string);
        let el = apply_listeners(el, node, &id_string);
        // Deferred overlays paint later; without occlude, pointer events fall
        // through to in-flow widgets that share the same window point.
        let el = if node.style.overlay { el.occlude() } else { el };
        maybe_animated(el, node)
    } else {
        let el = apply_shared(base, node, "");
        maybe_animated(el, node)
    };
    CURRENT_DISMISS_LAYER.with(|layer| *layer.borrow_mut() = previous_layer);
    if node.style.overlay {
        DEFERRED_SCOPE.with(|scope| scope.set(was_deferred));
        record_probe_channel("overlay.intent.painted");
        if was_deferred {
            element
        } else {
            deferred(element).with_priority(1).into_any_element()
        }
    } else {
        element
    }
}

fn needs_state(node: &Node) -> bool {
    node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_activate_modified.is_some()
        || node.interaction.on_context.is_some()
        || node.interaction.on_key.is_some()
        || node.interaction.drag_source.is_some()
        || node.interaction.drop_target.is_some()
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || node.interaction.on_scrub.is_some()
        || node.interaction.on_continuous_value.is_some()
        || node.interaction.on_wheel.is_some()
        || node.interaction.on_double_activate.is_some()
        || node.interaction.on_select_range.is_some()
        || node.interaction.on_focus_change.is_some()
        || node.id.is_some()
        // A declared focus ring paints through a canvas child and implies
        // focus tracking — both need element state.
        || node.style.focus_ring.is_some()
        // `active` style patches and scroll overflow live on gpui 0.2.2's
        // StatefulInteractiveElement — both need element state.
        || node.style.active.is_some()
        || node.style.descriptor.layout.overflow_x == LayoutOverflow::Scroll
        || node.style.descriptor.layout.overflow_y == LayoutOverflow::Scroll
        // Node tooltips need element state for bounds, hover/focus lifecycle, and overlay.
        || node.tooltip.as_deref().is_some_and(|text| !text.is_empty())
        // Overlay surfaces must be stateful so they can occlude hit-testing
        // and record containment bounds. `runtime_id` is a stable identity
        // even when `Node.id` is unset.
        || node.style.overlay
        || node.runtime_id.is_some()
}

pub(crate) fn current_dismiss_layer() -> Option<String> {
    CURRENT_DISMISS_LAYER.with(|layer| layer.borrow().clone())
}

/// The channels every box gets, in the Jetstream walk's order: layout,
/// position, paint, text, cursor, state patches, children. `id` is the
/// resolved element identity (see `build_box`) — "" only on the stateless
/// path, which never tracks focus.
fn apply_shared<E>(el: E, node: &Node, id: &str) -> E
where
    E: Styled + InteractiveElement + ParentElement + 'static,
{
    let el = apply_layout(el, node);
    let el = apply_position(el, node);
    let el = apply_paint(el, node);
    let el = apply_text(el, node);
    let el = apply_cursor(el, node);
    let el = apply_state_patches(el, node, id);
    // Inset shadow bands paint under the node's own children, so the painter
    // goes in before them (g16.005: crates.io `BoxShadow` has no inset flag,
    // so the backend paints these layers itself).
    let el = inset_shadow::apply(el, node, id);
    apply_children(el, node, id)
}

// The id `element_id` would assign, as a string, for keying editor state.
// Focus handles we own, so we can ask "is this node focused?" — gpui's
// auto-created handle lives in element state it never hands back. Created
// lazily in the paint pass (which has an `App`) and used from the next build
// onward; keyed by element id, like every other per-node cache here.
thread_local! {
    /// Whether the subtree currently being built sits inside a focused node.
    static FOCUS_SCOPE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// Whether the subtree currently being built sits inside an overlay
    /// (deferred) node. gpui 0.2.2 forbids calling `defer_draw` during its
    /// deferred pass, so a nested overlay surface (a menu inside a popover)
    /// draws within the enclosing deferred element instead of deferring
    /// again.
    static DEFERRED_SCOPE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// Dismiss-layer id inherited by descendants that do not declare their
    /// own. Painted child bounds join the containment set so a pointer on a
    /// deferred option row is inside, not an outside dismiss.
    static CURRENT_DISMISS_LAYER: RefCell<Option<String>> = const { RefCell::new(None) };
    // The id of the node that currently holds focus, if any.
    static FOCUSED_FIELD: RefCell<Option<String>> = const { RefCell::new(None) };
    static FOCUS_HANDLES: RefCell<std::collections::HashMap<String, gpui::FocusHandle>> =
        RefCell::new(std::collections::HashMap::new());
    static FOCUS_STATES: RefCell<std::collections::HashMap<String, bool>> =
        RefCell::new(std::collections::HashMap::new());
    // What the ring paint pass last painted per element id. Written only from
    // the real paint pass; absent means no ring is on screen.
    static PAINTED_RINGS: RefCell<std::collections::HashMap<String, PaintedRing>> =
        RefCell::new(std::collections::HashMap::new());
    // What the inset-shadow paint pass last painted per element id, in
    // declaration order. Same discipline as PAINTED_RINGS: written only from
    // the real paint pass, so an assertion against it is evidence that pixels
    // were emitted rather than that a style was declared.
    static PAINTED_INSET_SHADOWS: RefCell<std::collections::HashMap<String, Vec<PaintedInsetShadow>>> =
        RefCell::new(std::collections::HashMap::new());
}

/// One inset shadow band as the paint pass actually drew it: the per-side
/// widths and the padding box they were clipped to, in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaintedInsetShadow {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub color: ColorValue,
    /// The padding box the band was painted inside: `[x, y, width, height]`.
    pub bounds: [f32; 4],
}

/// The inset shadow bands painted for this element id as of the last paint
/// pass. Empty or absent means nothing was drawn.
pub fn painted_inset_shadows_for(id: &str) -> Vec<PaintedInsetShadow> {
    PAINTED_INSET_SHADOWS.with(|r| r.borrow().get(id).cloned().unwrap_or_default())
}

pub(crate) fn record_painted_inset_shadows(id: &str, painted: Vec<PaintedInsetShadow>) {
    PAINTED_INSET_SHADOWS.with(|r| {
        if painted.is_empty() {
            r.borrow_mut().remove(id);
        } else {
            r.borrow_mut().insert(id.to_owned(), painted);
        }
    });
}

/// Frame boundary for the inset-shadow registry, called beside the ring one.
pub(crate) fn clear_painted_inset_shadows() {
    PAINTED_INSET_SHADOWS.with(|r| r.borrow_mut().clear());
}

/// What the focus-ring paint pass last painted for one tracked element: the
/// declared ring values and the outer-edge bounds it drew, in logical pixels.
/// Same observation posture as [`bounds_for`]: the paint pass records, tests
/// and capture hosts read — nothing here steers what is painted. The registry
/// is frame-scoped: [`overlay_frame_begin`] clears it and the frame's paint
/// repopulates it, so an entry can never outlive the node that painted it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaintedRing {
    pub ring: FocusRing,
    /// Outer edge of the painted ring: `[x, y, width, height]`.
    pub bounds: [f32; 4],
}

/// The ring painted for this element id as of the last paint pass, or `None`
/// when no ring is on screen (the node is not focused, declares none, or is
/// gone from the tree).
pub fn painted_ring_for(id: &str) -> Option<PaintedRing> {
    PAINTED_RINGS.with(|r| r.borrow().get(id).copied())
}

/// Every ring painted in the current frame, keyed by element id. The
/// observation surface for "exactly one ring is on screen, and it is this
/// one" claims where the element id is backend-generated.
pub fn painted_rings() -> Vec<(String, PaintedRing)> {
    PAINTED_RINGS.with(|r| {
        r.borrow()
            .iter()
            .map(|(id, painted)| (id.clone(), *painted))
            .collect()
    })
}

pub(crate) fn record_painted_ring(id: &str, painted: PaintedRing) {
    PAINTED_RINGS.with(|r| r.borrow_mut().insert(id.to_owned(), painted));
}

pub(crate) fn clear_painted_ring(id: &str) {
    PAINTED_RINGS.with(|r| r.borrow_mut().remove(id));
}

/// Frame boundary for the ring registry, called from
/// [`layers::overlay_frame_begin`] beside `ELEMENT_BOUNDS`.
pub(crate) fn clear_painted_rings() {
    PAINTED_RINGS.with(|r| r.borrow_mut().clear());
}

/// The focus handle of whatever holds focus right now.
pub(crate) fn focused_handle() -> Option<gpui::FocusHandle> {
    let id = FOCUSED_FIELD.with(|f| f.borrow().clone())?;
    focus_handle_for(&id)
}

/// The focus handle for a tracked node's element id, if one has been
/// created. The conformance runner focuses through this — the real backend
/// focus API, observed both ways.
pub fn focus_handle_for(id: &str) -> Option<gpui::FocusHandle> {
    FOCUS_HANDLES.with(|h| h.borrow().get(id).cloned())
}

/// Whether the node with this element id held focus as of the last frame.
/// The conformance observer reads this for the `backend-focus` state — real
/// window focus, observed both ways, never a latched flag.
pub fn focus_state_for(id: &str) -> Option<bool> {
    FOCUS_STATES.with(|s| s.borrow().get(id).copied())
}

/// Whether this node held focus as of the last frame.
///
/// One source of truth, and it is gpui's: an earlier pass latched a spec flag
/// on click, which could only ever turn focus *on* — a field kept its caret
/// forever once clicked. Reading the real handle means blur is just as
/// observable as focus.
fn is_focused(id: &str) -> bool {
    FOCUS_STATES.with(|s| s.borrow().get(id).copied().unwrap_or(false))
}

/// Whether a node wants focus tracked: it draws differently when focused, it
/// asked to be told, or it declares a focus ring — a ring is painted only
/// while the real handle holds focus, so declaring one is meaningless without
/// a tracked handle. A bare `focusable` stays untracked: most focusable nodes
/// never draw a focus treatment of their own.
fn tracks_focus(node: &Node) -> bool {
    // Deliberately not "every input": a field's value node is an input too,
    // and gpui focuses the *innermost* focusable element under the pointer, so
    // tracking it stole focus from the field root that carries the key
    // listeners — clicks focused something that could not type. The value node
    // learns it is focused by inheritance instead (see `apply_children`).
    node.interaction.on_focus_change.is_some()
        || node.style.focus_ring.is_some()
        || (node.interaction.focusable && node.style.focus.is_some())
        || (node.interaction.focusable
            && node.tooltip.as_deref().is_some_and(|text| !text.is_empty()))
        // A source that opted into keyboard pickup must be observably focused,
        // or the controller can never tell which source a Space or Enter
        // belongs to and the keyboard route silently does nothing.
        || node
            .interaction
            .drag_source
            .as_ref()
            .is_some_and(|source| source.keyboard_order.is_some())
}

pub(crate) fn element_id_string(node: &Node) -> String {
    match node.runtime_id.as_ref().or(node.id.as_ref()) {
        Some(id) => id.clone(),
        None => match &node.style.animation {
            Some(anim) => anim.key.clone(),
            None => String::new(),
        },
    }
}

fn apply_children<E: ParentElement>(mut el: E, node: &Node, id: &str) -> E {
    // A focused field's caret sits on the *value* node, several levels below
    // the focusable root that actually holds focus (affixes and icons are
    // siblings of the value). Focus is inherited down the subtree here, the
    // same way a real input's caret shows because its wrapper has focus.
    let inherited = FOCUS_SCOPE.with(|f| f.get());
    let scope = inherited || (tracks_focus(node) && is_focused(id));
    FOCUS_SCOPE.with(|f| f.set(scope));
    for child in &node.children {
        el = el.child(to_gpui(child));
    }
    FOCUS_SCOPE.with(|f| f.set(inherited));
    el
}

fn paint_resolved_icon_geometry(
    window: &mut Window,
    bounds: Bounds<Pixels>,
    frame: &ResolvedIconGeometryFrame,
    paint_color: Hsla,
) {
    let width = f32::from(bounds.size.width);
    let height = f32::from(bounds.size.height);
    let side = width.min(height);
    if side <= 0.0 || frame.contours.is_empty() {
        return;
    }
    let stroke = px((2.0 / 24.0) * side);
    let origin = bounds.origin;
    let to_point = |x: i32, y: i32| {
        point(
            origin.x + px((x as f32 / 10_000.0) / 24.0 * side),
            origin.y + px((y as f32 / 10_000.0) / 24.0 * side),
        )
    };
    for contour in &frame.contours {
        if contour.points.len() < 2 {
            continue;
        }
        let mut path = PathBuilder::stroke(stroke);
        path.move_to(to_point(contour.points[0].0, contour.points[0].1));
        for &(x, y) in &contour.points[1..] {
            path.line_to(to_point(x, y));
        }
        if contour.closed {
            path.close();
        }
        if let Ok(built) = path.build() {
            window.paint_path(built, paint_color);
        }
    }
}

// ── Animation ───────────────────────────────────────────────────────

/// Sample one animated property at cycle position `t` (0.0..=1.0):
/// piecewise-linear between the keyframes that declare it, clamped at the
/// ends. Pure — the unit-tested half of the animation channel.
fn sample_property(anim: &NodeAnimation, prop: AnimProperty, t: f32) -> Option<f32> {
    let mut keys: Vec<(f32, f32)> = anim
        .keyframes
        .iter()
        .filter_map(|k| {
            k.values
                .iter()
                .find(|(p, _)| *p == prop)
                .map(|(_, v)| (k.at, *v))
        })
        .collect();
    if keys.is_empty() {
        return None;
    }
    keys.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let t = t.clamp(0.0, 1.0);
    if t <= keys[0].0 {
        return Some(keys[0].1);
    }
    if t >= keys[keys.len() - 1].0 {
        return Some(keys[keys.len() - 1].1);
    }
    for pair in keys.windows(2) {
        let (t0, v0) = pair[0];
        let (t1, v1) = pair[1];
        if t >= t0 && t <= t1 {
            let span = t1 - t0;
            let f = if span > 0.0 { (t - t0) / span } else { 0.0 };
            return Some(v0 + (v1 - v0) * f);
        }
    }
    Some(keys[keys.len() - 1].1)
}

fn gpui_animation(anim: &NodeAnimation) -> gpui::Animation {
    let a = gpui::Animation::new(Duration::from_secs_f32(anim.duration_secs));
    // APPROXIMATION: gpui 0.2.2 animations repeat or run once; there is no
    // ping-pong mode, so PingPong degrades to Loop.
    let a = match anim.loop_mode {
        AnimLoop::Once => a,
        AnimLoop::Loop | AnimLoop::PingPong => a.repeat(),
    };
    match anim.easing {
        AnimEasing::Linear => a,
        AnimEasing::EaseIn => a.with_easing(|t| t * t),
        AnimEasing::EaseOut => a.with_easing(|t| 1.0 - (1.0 - t) * (1.0 - t)),
        AnimEasing::EaseInOut => a.with_easing(|t| {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }),
    }
}

/// Opacity is the one property gpui 0.2.2 can animate on any Styled element.
/// SVG rotation is handled above; other transform channels remain unavailable
/// on generic elements.
fn maybe_animated<E>(el: E, node: &Node) -> AnyElement
where
    E: Styled + IntoElement + 'static,
{
    use gpui::AnimationExt;
    let Some(anim) = &node.style.animation else {
        return el.into_any_element();
    };
    record_probe_channel("surface.animation.scheduled");
    if sample_property(anim, AnimProperty::TranslateX, 0.0).is_some()
        || sample_property(anim, AnimProperty::TranslateY, 0.0).is_some()
        || sample_property(anim, AnimProperty::ScaleX, 0.0).is_some()
        || sample_property(anim, AnimProperty::ScaleY, 0.0).is_some()
    {
        record_probe_channel("surface.animation.approximation.opacity-stand-in");
    }
    let anim = anim.clone();
    let id = element_id(node);
    el.with_animation(id, gpui_animation(&anim), move |el, t| {
        let mut el = el;
        if let Some(v) = sample_property(&anim, AnimProperty::Opacity, t) {
            el = el.opacity(v);
        }
        el
    })
    .into_any_element()
}

// ── Accessibility ───────────────────────────────────────────────────
//
// NodeA11y (role, label, expanded, selected, toggled, level) is intentionally
// NOT mapped: gpui 0.2.2's fluent element API exposes no accessibility
// attributes. `docs/contracts/003-native-accessibility.md` records the same
// accepted runtime gap, and g12.015 holds GPUI accessibility upstream work
// deliberately. The channels are walked (read)
// here so the omission is a decision, not a drift.

mod ime;
mod input_text;

pub use ime::{mark_composing, take_composing};
pub use input_text::{painted_text_state_for, PaintedTextState};
pub use measured_node::{measured_node_element, shaped_block_advance, ShapedAdvance};

#[cfg(test)]
mod tests;
