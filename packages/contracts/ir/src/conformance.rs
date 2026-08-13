//! Conformance vectors and explicit runtime extensions.
//!
//! Together with [`crate::capabilities`] these are two of the four
//! representation slots spec 063's "Hard Boundary" allows for cross-runtime
//! behavior: shared conformance vectors implemented by each runtime machine
//! (`CROSS-18`), and explicit runtime extensions with a documented parity
//! consequence (`EXT` class rows: `BTN-26/27/29`, `RNG-26/27`, `TXT-31`).
//! Declarative transitions/guards/effect-intents appear in
//! [`crate::events`] and [`crate::keyboard`].

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// The runtime machines a conformance vector or extension targets. These are
/// vocabulary names for the four pilot shells (the corpus's `SHELL-*` rows),
/// not framework dependencies — no framework type crosses the boundary
/// (`NEG-01`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RuntimeTarget {
    #[serde(rename = "svelte")]
    Svelte,
    #[serde(rename = "react")]
    React,
    #[serde(rename = "gpui")]
    Gpui,
    #[serde(rename = "jetstream")]
    Jetstream,
}

/// A shared conformance vector — machine semantics executed by each runtime
/// machine from one vector file (`CROSS-18`; spec 063 "shared conformance
/// vectors implemented by each runtime machine"; evidence
/// `contracts/headless/vectors/machines.json`).
///
/// Vectors are declarative step intents, never executable code (`NEG-01`).
/// `GAP-01` (no range/text vectors yet) and `RNG-29` are served by this
/// type: the RangeSlider and TextInput vectors will be authored against it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceVector {
    /// Stable identifier for the vector, e.g. `range-slider`.
    pub id: Identifier,
    /// Vector name, e.g. `slider` (`CROSS-18`).
    pub name: String,
    /// Runtimes that implement this vector (`CROSS-18` "both runtimes").
    pub applies_to: Vec<RuntimeTarget>,
    /// Declarative steps, in execution order.
    pub steps: Vec<VectorStep>,
    /// What the vector proves, citing the machine semantics and contract
    /// sections.
    pub description: String,
}

/// One declarative step of a conformance vector (`CROSS-18`; spec 063
/// "declarative transition, guard, or effect-intent expression").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorStep {
    /// Stable identifier for the step, e.g. `snap-to-step` (`CROSS-19`).
    pub id: Identifier,
    /// Step name.
    pub name: String,
    /// Step kind.
    pub kind: VectorStepKind,
    /// The machine behavior this step pins, citing the contract note, e.g.
    /// `R §3` step snapping anchored at `min` (`CROSS-19`). Input
    /// conditions were guard expressions (g13.017 R1 bucket 1: dead weight
    /// — no vector step in any authored model carried one) and are gone:
    /// guards over machine state are vector machines (`CROSS-19`, `RNG-02`,
    /// `TXT-09`), never expressions.
    pub description: String,
}

/// Kind of a conformance-vector step (spec 063 "Hard Boundary" vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VectorStepKind {
    /// A declarative state transition, e.g. INPUT → value-change
    /// (`RNG-11`).
    #[serde(rename = "transition")]
    Transition,
    /// A guard on a transition, e.g. the degenerate-range guard
    /// (`CROSS-19`, `RNG-02`).
    #[serde(rename = "guard")]
    Guard,
    /// An effect-intent emitted at a transition, e.g. commit on release
    /// (`RNG-11`).
    #[serde(rename = "effect-intent")]
    EffectIntent,
    /// An invariant that must hold, e.g. lower ≤ upper always
    /// (`RNG-12`), a thumb never crossing its sibling (`RNG-12`).
    #[serde(rename = "invariant")]
    Invariant,
}

/// An explicit runtime extension — the escape hatch of spec 063's
/// "Capability And Escape-hatch Rules", with the required owning runtime,
/// reason, parity effect, evidence surface, and removal condition.
///
/// Serves the `EXT` class rows: `BTN-26` (GPUI deltas), `BTN-27` (Jetstream
/// deltas), `BTN-29` (Rust enum superset), `RNG-26` (GPUI native vertical),
/// `RNG-27` (Jetstream pair reporting), `TXT-31` (Jetstream clear-only
/// events).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Extension {
    /// Stable identifier for the extension, e.g. `jetstream-clear-only`.
    pub id: Identifier,
    /// Owning runtime and reason (spec 063 "owning runtime and reason").
    pub owning_runtime: RuntimeTarget,
    /// Why the extension exists and cannot be expressed without damaging
    /// runtime-native semantics.
    pub reason: String,
    /// Semantic effect on parity (spec 063 "semantic effect on parity"),
    /// e.g. "loading treated as disabled; `on_click` carries no payload"
    /// (`BTN-27`).
    pub parity_effect: String,
    /// Test or evidence surface (spec 063 "test or evidence surface"),
    /// e.g. `docs/parity/button.md` (`BTN-26`).
    pub evidence_surface: String,
    /// Removal condition, or a statement that the difference is intentional
    /// (spec 063 "removal condition").
    pub removal_condition: String,
    /// The corpus requirement this extension records, e.g. `TXT-31`.
    pub description: String,
}
