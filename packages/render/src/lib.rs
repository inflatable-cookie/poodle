//! The single Rust component implementation.
//!
//! Each component here is a pure function `Spec + Theme → poodle_node::Node`.
//! No backend types, no measurement, no window: what a Button *is*, decided
//! once. Per-backend adapters (GPUI's in this repo, Jetstream's in its own)
//! interpret the node tree; the parity evidence for each backend lives with
//! that backend, against fixtures this crate can generate headlessly.
//!
//! Started 2026-08-04 with `Select` — deliberately the hardest component
//! (overlay anchoring, filtering, groups, a trigger with nested clickables) —
//! so the vocabulary was proven where it was most likely to fail. The rest of
//! the tier migrates component-by-component; each port deletes its
//! `packages/jetstream/components` and `packages/gpui/components`
//! predecessors once its parity fixtures are green on both backends.

pub mod color;
pub mod presentation;
pub mod select;

pub use select::{select, SelectHandlers};
