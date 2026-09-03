//! Dismissable-layer registry (spec 066, g14.005) — the generic backend half
//! of the overlay dismiss-stack contract, plus the reusable production
//! overlay host.
//!
//! Renderer-neutral nodes declare dismissal intent through
//! `Interaction.on_dismiss` (the layer's reason handler) and
//! `Interaction.dismiss_layer` (the layer id every node of one containment
//! unit — an overlay's trigger and surface — shares). Every render frame this
//! module rebuilds the stack from the converted tree:
//!
//! - layers are ordered by tree position (outer before inner), matching the
//!   web's registration order for nested overlays;
//! - a layer's `parent` is the innermost layer already registered when the
//!   walk first meets it — the web stack records the same ancestry at
//!   registration;
//! - the containment set of a layer is the rendered bounds of every node
//!   sharing its id, recorded in the paint pass.
//!
//! Escape dismisses the innermost layer only. An outside pointer interaction
//! dismisses every layer that neither contains the position nor is an
//! ancestor of a layer that does — the shared dismiss-stack contract
//! (`packages/core/src/dom/dismiss.ts::resolveDismiss`), executed through the
//! real event tree. No component identifier lives here.
//!
//! The registry is frame-scoped, and the frame boundary is the host's render
//! pass — not an individual tree conversion, because a real page converts
//! many components independently per frame. [`overlay_frame_begin_for`] is
//! called once at the start of each rendered frame by both the production
//! preview root and the headless conformance driver; [`attach_overlay_host`]
//! wires the window-level key and pointer listeners — dismissal, payload
//! cleanup, Tab traversal, and that window's tooltip overlay — onto a root
//! element for the same two hosts. Tooltip prepare, sweep, and paint are
//! keyed by `AnyWindowHandle`; a frame in one window cannot cancel or paint
//! another window's tooltip.

use std::cell::RefCell;

use gpui::{
    AnyWindowHandle, App, Bounds, KeyDownEvent, MouseButton, MouseDownEvent, Pixels, Point,
};
use poodle_node::{DismissHandler, DismissReason};

/// One registered overlay layer for the current frame.
#[derive(Clone)]
pub struct LayerRecord {
    /// The layer id the renderer stamped on its nodes.
    pub id: String,
    /// The layer's reason handler (the first node with the id carries it).
    pub handler: Option<DismissHandler>,
    /// Rendered bounds of every node sharing this id (the containment set).
    pub bounds: Vec<Bounds<Pixels>>,
    /// The innermost layer this one sits inside, when any (tree ancestry).
    pub parent: Option<String>,
}

thread_local! {
    /// Layers in tree order, rebuilt each frame.
    static LAYERS: RefCell<Vec<LayerRecord>> = const { RefCell::new(Vec::new()) };
    /// Rendered bounds per element id, rebuilt each frame.
    static ELEMENT_BOUNDS: RefCell<std::collections::HashMap<String, Bounds<Pixels>>> =
        RefCell::new(std::collections::HashMap::new());
    /// Which dismiss layer last painted each element. Deferred overlay rows
    /// record here even when their rect has not yet been folded into the
    /// layer's containment vec.
    static ELEMENT_LAYERS: RefCell<std::collections::HashMap<String, String>> =
        RefCell::new(std::collections::HashMap::new());
    /// Focus requests queued by component hosts (machine focus effects):
    /// applied by the target element's paint-time focus canvas, once.
    static FOCUS_REQUESTS: RefCell<std::collections::HashSet<String>> =
        RefCell::new(std::collections::HashSet::new());
}

/// Begin a rendered frame: the layer registry and bounds are rebuilt per
/// frame, and the boundary is the host's render pass — not a `to_gpui` call,
/// because a real page converts many components independently per frame.
///
/// Prepares this paint: layer/bounds registries restart, and the open
/// continuous-value session is marked unpainted so a host that is not rebuilt
/// can cancel after paint. [`overlay_frame_end`] is the same-cycle lost-host
/// sweep; this begin-time sweep is only a safety net for a previous cycle that
/// never ended.
///
/// The focus queue is NOT cleared here: focus requests made between frames
/// (machine effects from event dispatch) must survive until the next frame's
/// paint applies them. [`overlay_frame_end`] drops whatever was never
/// applied.
pub fn overlay_frame_begin() {
    // Safety net for a previous cycle that never called overlay_frame_end.
    // Same-frame lost-host cancel is overlay_frame_end after this paint.
    crate::interaction::sweep_lost_continuous_host();
    LAYERS.with(|layers| layers.borrow_mut().clear());
    ELEMENT_BOUNDS.with(|bounds| bounds.borrow_mut().clear());
    ELEMENT_LAYERS.with(|layers| layers.borrow_mut().clear());
    // The ring registry is frame observation with the same lifetime: a
    // focused node that vanished paints nothing this frame, and its entry
    // must not survive it.
    crate::clear_painted_rings();
    super::clear_painted_inset_shadows();
    crate::interaction::prepare_continuous_value_frame();
}

/// Begin a rendered frame for one window. Production roots call this instead
/// of the unscoped overlay begin so tooltip prepare and close-binding belong
/// to that window only.
pub fn overlay_frame_begin_for(handle: AnyWindowHandle, cx: &mut App) {
    overlay_frame_begin();
    crate::tooltip::prepare_tooltip_frame(handle);
    crate::tooltip::bind_window_teardown(handle, cx);
}

/// End a rendered frame: drop unused focus requests, then cancel a
/// continuous-value gesture whose owner was not rebuilt in this paint.
/// Production hosts defer [`overlay_frame_end_for`] to the end of the same
/// effect cycle as [`overlay_frame_begin_for`] so removal cancels without a
/// next-frame delay.
pub fn overlay_frame_end() {
    FOCUS_REQUESTS.with(|requests| requests.borrow_mut().clear());
    crate::interaction::sweep_lost_continuous_host();
}

/// End a rendered frame for one window. Sweeps only that window's unpainted
/// tooltip so another window's pending or visible state survives this paint.
pub fn overlay_frame_end_for(handle: AnyWindowHandle) {
    overlay_frame_end();
    crate::tooltip::sweep_unpainted_tooltips(handle);
}

/// Queue a focus request for the element with this id. The target element's
/// focus canvas applies it at paint time (the element must exist and be
/// tracked), so requests made during event dispatch land after the frame
/// that mounts the target.
pub fn request_focus(element_id: &str) {
    FOCUS_REQUESTS.with(|requests| {
        requests.borrow_mut().insert(element_id.to_owned());
    });
}

/// Claim a pending focus request for the element, if any. Called by the
/// element's focus canvas in the paint pass, which owns the window.
pub fn take_focus_request(element_id: &str) -> bool {
    FOCUS_REQUESTS.with(|requests| requests.borrow_mut().remove(element_id))
}

/// The rendered bounds of the element with this id, as of the last frame.
pub fn bounds_for(element_id: &str) -> Option<Bounds<Pixels>> {
    ELEMENT_BOUNDS.with(|bounds| bounds.borrow().get(element_id).copied())
}

/// The dismiss layer the element last painted into, if any.
pub fn layer_for_element(element_id: &str) -> Option<String> {
    ELEMENT_LAYERS.with(|layers| layers.borrow().get(element_id).cloned())
}

/// Record paint bounds for a named element. Overlay members also go through
/// [`record_bounds`]; this covers ordinary identified parts (buttons, fields).
pub fn record_element_bounds(element_id: &str, bounds: Bounds<Pixels>) {
    ELEMENT_BOUNDS.with(|all| {
        all.borrow_mut().insert(element_id.to_owned(), bounds);
    });
}

/// How many open overlay layers are registered this frame.
pub fn open_layer_count() -> usize {
    LAYERS.with(|layers| layers.borrow().len())
}

/// Rebuild the layer stack from the converted tree, in tree order. Runs for
/// every independently converted root within the frame; the registry
/// dedupes by layer id.
pub fn collect_layers(node: &poodle_node::Node, innermost: Option<&str>) {
    if let Some(id) = node.interaction.dismiss_layer.as_deref() {
        LAYERS.with(|layers| {
            let mut layers = layers.borrow_mut();
            if layers.iter().any(|record| record.id == id) {
                return;
            }
            layers.push(LayerRecord {
                id: id.to_owned(),
                handler: node.interaction.on_dismiss.clone(),
                bounds: Vec::new(),
                parent: innermost.filter(|parent| *parent != id).map(str::to_owned),
            });
        });
    }
    let next = node.interaction.dismiss_layer.as_deref().or(innermost);
    for child in &node.children {
        collect_layers(child, next);
    }
}

/// Paint-time bounds record for one element of a layer.
pub fn record_bounds(element_id: &str, layer_id: &str, bounds: Bounds<Pixels>) {
    ELEMENT_BOUNDS.with(|all| {
        all.borrow_mut().insert(element_id.to_owned(), bounds);
    });
    ELEMENT_LAYERS.with(|layers| {
        layers
            .borrow_mut()
            .insert(element_id.to_owned(), layer_id.to_owned());
    });
    LAYERS.with(|layers| {
        let mut layers = layers.borrow_mut();
        if let Some(record) = layers.iter_mut().find(|record| record.id == layer_id) {
            record.bounds.push(bounds);
        }
    });
}

fn contains(position: Point<Pixels>, bounds: &Bounds<Pixels>) -> bool {
    bounds.contains(&position)
}

/// The layers an outside interaction spares: every layer containing the
/// position, plus every ancestor of those layers (walking the parent chain
/// recorded at registration — the web stack's `sparedByAncestry`).
fn spared_by_ancestry(
    layers: &[LayerRecord],
    position: Point<Pixels>,
) -> std::collections::HashSet<String> {
    let mut spared = std::collections::HashSet::new();
    let mut hit_layers: std::collections::HashSet<String> = layers
        .iter()
        .filter(|layer| layer.bounds.iter().any(|bounds| contains(position, bounds)))
        .map(|layer| layer.id.clone())
        .collect();
    ELEMENT_LAYERS.with(|element_layers| {
        ELEMENT_BOUNDS.with(|bounds| {
            let element_layers = element_layers.borrow();
            let bounds = bounds.borrow();
            for (element_id, layer_id) in element_layers.iter() {
                if bounds
                    .get(element_id)
                    .is_some_and(|rect| contains(position, rect))
                {
                    hit_layers.insert(layer_id.clone());
                }
            }
        });
    });
    for layer_id in hit_layers {
        let mut current = layers.iter().find(|layer| layer.id == layer_id);
        while let Some(record) = current {
            if !spared.insert(record.id.clone()) {
                break;
            }
            current = layers
                .iter()
                .find(|candidate| candidate.id == record.parent.as_deref().unwrap_or(""));
        }
    }
    spared
}

/// Test helper: which layers a pointer at this position would spare.
pub fn spared_layer_ids_at(position: Point<Pixels>) -> Vec<String> {
    LAYERS.with(|layers| {
        let layers = layers.borrow();
        let mut ids: Vec<String> = spared_by_ancestry(&layers, position).into_iter().collect();
        ids.sort();
        ids
    })
}

/// Escape: the innermost (last-registered) layer dismisses.
pub fn dismiss_innermost(cx: &mut App) {
    let handler = LAYERS.with(|layers| {
        layers
            .borrow()
            .last()
            .and_then(|record| record.handler.clone())
    });
    if let Some(handler) = handler {
        handler(DismissReason::Escape);
        cx.refresh_windows();
    }
}

/// An outside pointer interaction at a position: every layer that neither
/// contains the position nor is an ancestor of a layer that does, innermost
/// first (the shared dismiss-stack contract).
pub fn dismiss_layers_at(position: Point<Pixels>, cx: &mut App) {
    let to_dismiss: Vec<Option<DismissHandler>> = LAYERS.with(|layers| {
        let layers = layers.borrow();
        let spared = spared_by_ancestry(&layers, position);

        layers
            .iter()
            .rev()
            .filter(|record| !spared.contains(&record.id))
            .map(|record| record.handler.clone())
            .collect()
    });
    for handler in to_dismiss {
        if let Some(handler) = handler {
            handler(DismissReason::Outside);
        }
    }
    cx.refresh_windows();
}

/// Attach the window-level host listeners to a root element: every
/// pointer-down is routed through the layer registry (outside dismissal) and
/// Escape dismisses the innermost layer. The production preview root and the
/// conformance mount host use this wiring, so overlay dismissal behaves
/// identically in the real runtime and the headless driver.
///
/// Drag-and-drop release and cancellation are NOT here. They belong to the
/// [`crate::DragDropController`] that owns the session, because a
/// window-global handler cannot tell two providers' sessions apart — which is
/// exactly the collision the old backend-global payload session had.
///
/// Tab lives here too, for the same reason Escape does: it is a document-level
/// default action, not something a focused control decides. gpui ships the
/// traversal itself (`focus_next`/`focus_prev` over the frame's tab stops) but
/// binds no key to it, so the window host is where the gesture meets the
/// machinery — the one place a browser also puts it.
pub fn attach_overlay_host<E>(el: E, window_handle: AnyWindowHandle) -> E
where
    E: gpui::InteractiveElement + gpui::ParentElement + 'static,
{
    let mut el = el
        .on_mouse_down(
            MouseButton::Left,
            move |event: &MouseDownEvent, window, cx| {
                crate::tooltip::dismiss_tooltip(window, cx);
                dismiss_layers_at(event.position, cx);
            },
        )
        .on_key_down(
            move |event: &KeyDownEvent, window, cx| match event.keystroke.key.as_str() {
                "escape" => {
                    crate::tooltip::dismiss_tooltip(window, cx);
                    dismiss_innermost(cx);
                }
                "tab" => {
                    if event.keystroke.modifiers.shift {
                        window.focus_prev();
                    } else {
                        window.focus_next();
                    }
                    cx.refresh_windows();
                }
                _ => {}
            },
        );

    if let Some(tooltip_node) = crate::tooltip::render_active_tooltip(window_handle) {
        el = el.child(tooltip_node);
    }

    el
}
