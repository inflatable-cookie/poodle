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
//! many components independently per frame. [`overlay_frame_begin`] is called
//! once at the start of each rendered frame by both the production preview
//! root and the headless conformance driver; [`attach_overlay_host`] wires
//! the window-level dismissal listeners onto a root element for the same two
//! hosts.

use std::cell::RefCell;

use gpui::{App, Bounds, KeyDownEvent, MouseButton, MouseDownEvent, MouseUpEvent, Pixels, Point};
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
    /// Focus requests queued by component hosts (machine focus effects):
    /// applied by the target element's paint-time focus canvas, once.
    static FOCUS_REQUESTS: RefCell<std::collections::HashSet<String>> =
        RefCell::new(std::collections::HashSet::new());
}

/// Begin a rendered frame: the layer registry and bounds are rebuilt per
/// frame, and the boundary is the host's render pass — not a `to_gpui` call,
/// because a real page converts many components independently per frame.
///
/// The focus queue is NOT cleared here: focus requests made between frames
/// (machine effects from event dispatch) must survive until the next frame's
/// paint applies them. [`overlay_frame_end`] drops whatever was never
/// applied.
pub fn overlay_frame_begin() {
    LAYERS.with(|layers| layers.borrow_mut().clear());
    ELEMENT_BOUNDS.with(|bounds| bounds.borrow_mut().clear());
    // The ring registry is frame observation with the same lifetime: a
    // focused node that vanished paints nothing this frame, and its entry
    // must not survive it.
    crate::clear_painted_rings();
    super::clear_painted_inset_shadows();
}

/// End a rendered frame: drop focus requests the frame's paint never applied
/// (the target element never appeared). Called by the headless driver after
/// each draw; the production host's requests always target painted elements.
pub fn overlay_frame_end() {
    FOCUS_REQUESTS.with(|requests| requests.borrow_mut().clear());
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
    for layer in layers {
        if !layer.bounds.iter().any(|bounds| contains(position, bounds)) {
            continue;
        }
        let mut current = Some(layer);
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

/// Attach the window-level overlay host listeners to a root element: every
/// pointer-down is routed through the layer registry (outside dismissal) and
/// Escape dismisses the innermost layer. The same root also ends an unfinished
/// payload-drag session on mouse-up (after a zone `on_drop` has already taken
/// a successful drop) and on Escape. The production preview root and the
/// conformance mount host use this wiring, so overlay dismissal and payload
/// cleanup behave identically in the real runtime and the headless driver.
pub fn attach_overlay_host<E>(el: E) -> E
where
    E: gpui::InteractiveElement + 'static,
{
    el.on_mouse_down(
        MouseButton::Left,
        move |event: &MouseDownEvent, _window, cx| {
            dismiss_layers_at(event.position, cx);
        },
    )
    .on_mouse_up(
        MouseButton::Left,
        move |_event: &MouseUpEvent, _window, cx| {
            crate::interaction::release_payload_session(cx);
        },
    )
    .on_key_down(move |event: &KeyDownEvent, window, cx| {
        if event.keystroke.key.as_str() == "escape" {
            crate::interaction::cancel_payload_session(window, cx);
            dismiss_innermost(cx);
        }
    })
}
