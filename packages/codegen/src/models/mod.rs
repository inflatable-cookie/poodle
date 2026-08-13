//! Rust-authored models — content instances built from `poodle-ir` types,
//! serialized to the JSON fixtures the existing pipeline consumes (spec 063
//! "Authoring Form": ordinary Rust types and constructor helpers).
//!
//! Placement is **pilot-scoped** (g13-b003 R1): an authored *instance* is
//! content, not schema, so it lives in `poodle-codegen`, not `poodle-ir`.
//! The three component models were unwound in g13-053; only the preview
//! shell scene remains (R1-keep).

pub mod preview_shell;
