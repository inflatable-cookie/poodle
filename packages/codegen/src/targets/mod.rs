//! Target registry. This card ships exactly one target — TypeScript. The
//! remaining four (JSON schema, registry, conformance vectors, docs
//! fragments) are a follow-up card and are deliberately absent.

use crate::emit::EmitTarget;

pub mod ts;

/// Every registered target, in stable order.
pub fn all() -> Vec<&'static dyn EmitTarget> {
    vec![&ts::TypeScriptTarget]
}

/// Looks up a target by id.
pub fn by_id(id: &str) -> Option<&'static dyn EmitTarget> {
    all().into_iter().find(|target| target.id() == id)
}
