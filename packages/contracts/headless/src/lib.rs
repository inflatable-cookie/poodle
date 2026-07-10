//! Poodle headless behavior core — Rust mirror.
//!
//! Hand-ported from `packages/core` (TypeScript). Machines are pure:
//! `transition(state?, context, event) -> (state?, context, effects)`.
//! Adapters (GPUI, Jetstream) own reactivity and execute effect intents.
//! Parity with the TS core is enforced by the shared JSON conformance
//! vectors in `vectors/`, executed by both runtimes.
//! See docs/architecture/006-headless-core-and-machine-model.md.

pub mod checkbox;
pub mod disclosure;
pub mod hover;
pub mod menu;
pub mod modal;
pub mod nav;
pub mod popover;
pub mod single_select;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod toggle_group;
