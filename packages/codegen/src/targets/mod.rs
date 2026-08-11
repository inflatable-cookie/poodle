//! Target registry. `g13-b022` shipped the TypeScript target; this card
//! adds the four the milestone names — JSON surface, JSON Schema, registry,
//! conformance vectors, docs fragments — in stable (id-sorted) order.
//! Targets land one at a time; the registry is extended as each lands.

use crate::emit::EmitTarget;

mod json_common;

pub mod json;
pub mod ts;

/// Every registered target, in stable order.
pub fn all() -> Vec<&'static dyn EmitTarget> {
    vec![&json::JsonSurfaceTarget, &ts::TypeScriptTarget]
}

/// Looks up a target by id.
pub fn by_id(id: &str) -> Option<&'static dyn EmitTarget> {
    all().into_iter().find(|target| target.id() == id)
}
