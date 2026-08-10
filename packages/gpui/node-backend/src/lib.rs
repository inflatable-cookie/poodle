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

use std::cell::RefCell;
use std::sync::Arc;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering, AtomicUsize};
use std::time::Duration;

use gpui::{
    div, img, linear_color_stop, linear_gradient, point, px, relative, svg, AnyElement, App,
    AppContext, ClickEvent, CursorStyle, Div, ElementId, Hsla, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement,
    SharedString, Stateful, StatefulInteractiveElement,
    StyleRefinement, Styled, StyledImage, Window,
};
use poodle_node::{
    AnimEasing, AnimLoop, AnimProperty, ColorValue, CrossAxisAlignment, CursorHint, DropEdge,
    FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    NodeAnimation, NodeDragEvent, NodeDragPhase, NodeDropEvent, NodeKey, NodeKind, NodeModifiers,
    NodePoint, NodePosition, NodeRole, ScrubPhase, SelectGranularity, StylePatch, TextAlign,
};

mod interaction;
mod style;

use interaction::apply_listeners;
use style::{
    apply_cursor, apply_layout, apply_paint, apply_patch, apply_position, apply_state_patches,
    apply_text,
};

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

/// Deterministic per-tree ids for nodes that need element state (interaction)
/// but declare none. Tree order is stable across frames for a stable tree, so
/// a counter keeps the same node on the same id between rebuilds — but ONLY if
/// the counter restarts each frame. See [`reset_element_ids`].
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

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
    NEXT_ID.store(0, Ordering::Relaxed);
    NEXT_GESTURE_ID.store(0, Ordering::Relaxed);
}

/// Per-frame counter for gesture-drag identities.
///
/// Reset with the element ids, so a node gets the same gesture id on every
/// frame: the tree is walked in the same order each time, and a drag begun on
/// one frame has to still recognise itself on the next.
static NEXT_GESTURE_ID: AtomicUsize = AtomicUsize::new(0);

fn next_gesture_id() -> String {
    format!(
        "gesture-{}",
        NEXT_GESTURE_ID.fetch_add(1, Ordering::Relaxed)
    )
}

fn element_id(node: &Node) -> ElementId {
    if let Some(id) = &node.id {
        return ElementId::Name(SharedString::from(id.clone()));
    }
    if let Some(anim) = &node.style.animation {
        // Vocabulary: an animation's key becomes the id when none is set —
        // nodes sharing a key share a clock.
        return ElementId::Name(SharedString::from(anim.key.clone()));
    }
    ElementId::Name(SharedString::from(format!(
        "poodle-node-{}",
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    )))
}

/// Interpret one node (and its subtree) as a GPUI element.
pub fn to_gpui(node: &Node) -> AnyElement {
    match &node.kind {
        NodeKind::Container => build_box(node, div()),
        NodeKind::Text { content } => {
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
                    node.caret.map(|c| color(c.caret_color)).unwrap_or(text_color),
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
        NodeKind::Icon { name, size } => {
            // Same path convention as the old tier's Icon: the app owns the
            // asset source; the name is the contract. svg() renders tinted by
            // `text_color`, which the style walk supplies.
            let el = svg()
                .path(SharedString::from(format!("assets/icons/{name}.svg")))
                .size(px(*size))
                .flex_shrink_0();
            build_svg_leaf(node, el)
        }
        NodeKind::Image { source } => {
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
        || node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
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
    let needs_wrapper = node.style.hover.is_some()
        || node.style.active.is_some()
        || node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || !node.children.is_empty();
    if needs_wrapper {
        return build_box(node, div().child(el));
    }

    let Some(anim) = &node.style.animation else {
        return el.into_any_element();
    };
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
    if needs_state(node) {
        let el = base.id(element_id(node));
        let el = apply_shared(el, node);
        let el = apply_listeners(el, node);
        maybe_animated(el, node)
    } else {
        let el = apply_shared(base, node);
        maybe_animated(el, node)
    }
}

fn needs_state(node: &Node) -> bool {
    node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_activate_modified.is_some()
        || node.interaction.on_context.is_some()
        || node.interaction.on_key.is_some()
        || node.interaction.drag_payload.is_some()
        || node.interaction.drop_zone
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || node.interaction.on_scrub.is_some()
        || node.interaction.on_select_range.is_some()
        || node.interaction.on_focus_change.is_some()
        || node.id.is_some()
        // `active` style patches and scroll overflow live on gpui 0.2.2's
        // StatefulInteractiveElement — both need element state.
        || node.style.active.is_some()
        || node.style.descriptor.layout.overflow_x == LayoutOverflow::Scroll
        || node.style.descriptor.layout.overflow_y == LayoutOverflow::Scroll
}

/// The channels every box gets, in the Jetstream walk's order: layout,
/// position, paint, text, cursor, state patches, children.
fn apply_shared<E>(el: E, node: &Node) -> E
where
    E: Styled + InteractiveElement + ParentElement + 'static,
{
    let el = apply_layout(el, node);
    let el = apply_position(el, node);
    let el = apply_paint(el, node);
    let el = apply_text(el, node);
    let el = apply_cursor(el, node);
    let el = apply_state_patches(el, node);
    apply_children(el, node)
}

// The id `element_id` would assign, as a string, for keying editor state.
// Focus handles we own, so we can ask "is this node focused?" — gpui's
// auto-created handle lives in element state it never hands back. Created
// lazily in the paint pass (which has an `App`) and used from the next build
// onward; keyed by element id, like every other per-node cache here.
thread_local! {
    // Whether the subtree currently being built sits inside a focused node.
    static FOCUS_SCOPE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    // The id of the node that currently holds focus, if any.
    static FOCUSED_FIELD: RefCell<Option<String>> = const { RefCell::new(None) };
    static FOCUS_HANDLES: RefCell<std::collections::HashMap<String, gpui::FocusHandle>> =
        RefCell::new(std::collections::HashMap::new());
    static FOCUS_STATES: RefCell<std::collections::HashMap<String, bool>> =
        RefCell::new(std::collections::HashMap::new());
}

/// The focus handle of whatever holds focus right now.
pub(crate) fn focused_handle() -> Option<gpui::FocusHandle> {
    let id = FOCUSED_FIELD.with(|f| f.borrow().clone())?;
    focus_handle_for(&id)
}

fn focus_handle_for(id: &str) -> Option<gpui::FocusHandle> {
    FOCUS_HANDLES.with(|h| h.borrow().get(id).cloned())
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

/// Whether a node wants focus tracked: it draws differently when focused, or
/// it asked to be told.
fn tracks_focus(node: &Node) -> bool {
    // Deliberately not "every input": a field's value node is an input too,
    // and gpui focuses the *innermost* focusable element under the pointer, so
    // tracking it stole focus from the field root that carries the key
    // listeners — clicks focused something that could not type. The value node
    // learns it is focused by inheritance instead (see `apply_children`).
    node.interaction.on_focus_change.is_some()
        || (node.interaction.focusable && node.style.focus.is_some())
}

fn element_id_string(node: &Node) -> String {
    match &node.id {
        Some(id) => id.clone(),
        None => match &node.style.animation {
            Some(anim) => anim.key.clone(),
            None => String::new(),
        },
    }
}

fn apply_children<E: ParentElement>(mut el: E, node: &Node) -> E {
    // A focused field's caret sits on the *value* node, several levels below
    // the focusable root that actually holds focus (affixes and icons are
    // siblings of the value). Focus is inherited down the subtree here, the
    // same way a real input's caret shows because its wrapper has focus.
    let inherited = FOCUS_SCOPE.with(|f| f.get());
    let scope = inherited || (tracks_focus(node) && is_focused(&element_id_string(node)));
    FOCUS_SCOPE.with(|f| f.set(scope));
    for child in &node.children {
        el = el.child(to_gpui(child));
    }
    FOCUS_SCOPE.with(|f| f.set(inherited));
    el
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

#[cfg(test)]
mod tests;
