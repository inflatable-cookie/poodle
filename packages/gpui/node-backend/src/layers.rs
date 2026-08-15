//! Dismissable-layer registry (spec 066, g14.005) — the generic backend half
//! of the overlay dismiss-stack contract.
//!
//! Renderer-neutral nodes declare dismissal intent through
//! `Interaction.on_dismiss` (the layer's reason handler) and
//! `Interaction.dismiss_layer` (the layer id every node of one containment
//! unit — an overlay's trigger and surface — shares). Each frame this module
//! rebuilds the stack from the painted tree:
//!
//! - layers are ordered by tree position (outer before inner), matching the
//!   web's registration order for nested overlays;
//! - a layer's `parent` is the innermost layer already open when the walk
//!   first meets it — the web stack records the same ancestry at
//!   registration;
//! - the containment set of a layer is the rendered bounds of every node
//!   sharing its id, recorded in the paint pass.
//!
//! Escape dismisses the innermost layer only. An outside pointer interaction
//! dismisses every layer that neither contains the position nor is an
//! ancestor of a layer that does — the shared dismiss-stack contract
//! (`packages/core/src/dom/dismiss.ts::resolveDismiss`), executed through the
//! real event tree by the mount host. No component identifier lives here.

use std::cell::RefCell;

use gpui::{App, Bounds, Pixels, Point};
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
    /// to_gpui recursion depth: the registry is frame-scoped, and the frame
    /// boundary is the outermost to_gpui call.
    static FRAME_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The rendered bounds of the element with this id, as of the last frame.
pub fn bounds_for(element_id: &str) -> Option<Bounds<Pixels>> {
    ELEMENT_BOUNDS.with(|bounds| bounds.borrow().get(element_id).copied())
}

/// How many open overlay layers are registered this frame.
pub fn open_layer_count() -> usize {
    LAYERS.with(|layers| layers.borrow().len())
}

/// Begin a frame: called at the outermost `to_gpui` call so the registry is
/// rebuilt exactly once per painted tree. Returns true when this call is the
/// outermost (the frame boundary).
pub fn begin_frame() -> bool {
    let depth = FRAME_DEPTH.with(|depth| depth.get());
    if depth == 0 {
        LAYERS.with(|layers| layers.borrow_mut().clear());
        ELEMENT_BOUNDS.with(|bounds| bounds.borrow_mut().clear());
    }
    FRAME_DEPTH.with(|depth| depth.set(depth.get() + 1));
    depth == 0
}

/// End a frame, mirroring the outermost `to_gpui` call.
pub fn end_frame() {
    FRAME_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));
}

/// Rebuild the layer stack from the painted tree, in tree order.
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
                parent: innermost
                    .filter(|parent| *parent != id)
                    .map(str::to_owned),
            });
        });
    }
    let next = node
        .interaction
        .dismiss_layer
        .as_deref()
        .or(innermost);
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
fn spared_by_ancestry(layers: &[LayerRecord], position: Point<Pixels>) -> std::collections::HashSet<String> {
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
    let handler = LAYERS
        .with(|layers| layers.borrow().last().and_then(|record| record.handler.clone()));
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
