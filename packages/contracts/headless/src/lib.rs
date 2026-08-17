//! Poodle headless behavior core — Rust mirror.
//!
//! Hand-ported from `packages/core` (TypeScript). Machines are pure:
//! `transition(state?, context, event) -> (state?, context, effects)`.
//! Adapters (GPUI, Jetstream) own reactivity and execute effect intents.
//! Parity with the TS core is enforced by the shared JSON conformance
//! vectors in `vectors/`, executed by both runtimes.
//! See docs/architecture/006-headless-core-and-machine-model.md.

pub mod agent_plan;
pub mod agent_question;
pub mod agent_subagent;
pub mod agent_transcript;
pub mod audio;
pub mod checkbox;
pub mod color;
pub mod date;
pub mod disclosure;
pub mod duration;
pub mod file_upload;
pub mod history_center;
pub mod hover;
pub mod licence;
pub mod menu;
pub mod modal;
pub mod nav;
pub mod pagination;
pub mod popover;
pub mod single_select;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod text_input;
pub mod toggle_group;
pub mod tree;
