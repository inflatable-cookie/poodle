//! Rust-authored models — content instances built from `poodle-ir` types,
//! serialized to the JSON fixtures the existing pipeline consumes (spec 063
//! "Authoring Form": ordinary Rust types and constructor helpers).
//!
//! Placement is **pilot-scoped** (g13-b003 R1): an authored *instance* is
//! content, not schema, so it lives in `poodle-codegen`, not `poodle-ir`.
//! Where production models are authored is a `g13.008` decision.

pub mod button;
pub mod preview_shell;
