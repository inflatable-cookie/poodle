//! Deterministic emission from validated [`poodle_ir`] models to committed
//! artifacts — the compiler boundary of `docs/specs/063` (`IR-07`
//! deterministic generation, `IR-11` pilot gate).
//!
//! The crate is a standalone `lib` + `[[bin]]` tool (g13-b003 R1): nothing
//! depends on it, it owns every byte it emits (no external formatter, R2),
//! and its check mode is structurally read-only (R3) — the emitter renders
//! into memory, then either writes (write mode) or byte-compares (check
//! mode); the check path contains no write call.
//!
//! # Design
//!
//! - [`emit`] — the pure emission core: [`GeneratedFile`], the
//!   [`EmitTarget`] abstraction, the generated header, and deterministic
//!   ordering. Rendering never touches the filesystem.
//! - [`check`] — read-only drift gate: byte-exact comparison, whitespace-only
//!   classification, and stale-orphan detection. Structurally incapable of
//!   writing.
//! - [`write`] — write mode: materializes [`GeneratedFile`]s and deletes
//!   stale orphans.
//! - [`model`] — load and validate a serialized `IrModel`.
//! - [`machine_interfaces`] — load and validate the machine-interface schema
//!   (spec 064 mechanism 1). Parallel to [`model`], not an `IrModel`.
//! - [`targets`] — the target registry (TypeScript is the one target this
//!   card ships; JSON schema, registry, conformance vectors, and docs
//!   fragments are a follow-up card). Machine-interface targets are
//!   select-only, like the shell.
//!
//! Determinism is the acceptance criterion: every ordering decision is fixed
//! (top-level collections sorted by id; prop and member order preserved as
//! authored, the way `build-tokens.ts` sorts inputs and keeps token paths),
//! the header is a pure function of the source path with no timestamp or
//! machine value, and float/string rendering is locale-independent.

pub mod check;
pub mod emit;
pub mod error;
pub mod machine_interfaces;
pub mod model;
pub mod models;
pub mod targets;
pub mod write;

pub use check::{check_outputs, CheckReport, DriftKind};
pub use emit::{generate, header, machine_header, EmitTarget, GeneratedFile};
pub use error::{CodegenError, Result};
pub use model::load_and_validate;
pub use write::write_outputs;

/// Generator version carried in every emitted header (`IR-07` "generator
/// version"). Compile-time from this crate's manifest — deterministic and
/// pinned by the same commit that pins the source.
pub const GENERATOR_VERSION: &str = env!("CARGO_PKG_VERSION");
