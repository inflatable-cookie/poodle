//! Component definitions — the serializable declaration of a
//! renderer-independent component.
//!
//! Serves `CROSS-01` (component identity: name, layer, contract reference,
//! stable identifier) and assembles every other vocabulary module into one
//! definition, per spec 063 "Component IR": props, defaults, types, controlled
//! state, events, parts, state-derived attributes, axes, token and recipe
//! references, accessibility intent, keyboard commands, adapter capabilities,
//! VisualState projections, and references to shared types with their
//! permitted subsets.

use serde::{Deserialize, Serialize};

use crate::{
    Accessibility, Axes, CapabilityRequirement, ContractRef, ControlledState, Event, Extension,
    Identifier, KeyboardCommand, Layer, Part, Prop, RecipeHookRef, StateAttribute, TokenRef,
    VisualState,
};

/// A versioned, serializable component definition (`CROSS-01`; spec 063
/// "Component IR"). Data and validation only — no rendering, no framework
/// behavior, no code generation (`NEG-02`, `NEG-06`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentDefinition {
    /// Stable identifier the IR and every generated artifact cite, e.g.
    /// `button` (`CROSS-01`).
    pub id: Identifier,
    /// Component name, e.g. `Button` (`CROSS-01`).
    pub name: String,
    /// Component layer — `foundation` for the pilot components
    /// (`CROSS-01`).
    pub layer: Layer,
    /// Governing contract reference (`CROSS-01`; `B/R/T §1`).
    pub contract: ContractRef,
    /// What the component is, citing the contract.
    pub description: String,
    /// Public props with default and type (`CROSS-02`), including web-only
    /// marks (`CROSS-03`) and permitted subsets of shared types
    /// (g13-b003 R6.2).
    pub props: Vec<Prop>,
    /// Controlled/uncontrolled state pairs and their do-not-mix rule
    /// (`CROSS-04`).
    pub controlled_state: Vec<ControlledState>,
    /// Declared events with payload and firing condition (`CROSS-05`,
    /// `CROSS-06`).
    pub events: Vec<Event>,
    /// Anatomy parts with parent/child constraints and conditional/repeated
    /// nodes (`CROSS-12`).
    pub parts: Vec<Part>,
    /// State-derived `data-*` attributes with emission rules (`CROSS-13`).
    pub attributes: Vec<StateAttribute>,
    /// Axes — size, density, orientation (`CROSS-07`, `CROSS-08`,
    /// `CROSS-11`).
    pub axes: Axes,
    /// Semantic token references (`CROSS-09`).
    pub tokens: Vec<TokenRef>,
    /// Recipe-hook override chains (`CROSS-09`).
    pub recipe_hooks: Vec<RecipeHookRef>,
    /// Accessibility intent — role, name rule, ARIA mapping (`CROSS-15`).
    pub accessibility: Accessibility,
    /// Declared adapter capability requirements (`CROSS-17`, `IR-08`).
    pub capabilities: Vec<CapabilityRequirement>,
    /// Keyboard command tables (`CROSS-16`).
    pub keyboard: Vec<KeyboardCommand>,
    /// VisualState projection shapes (`CROSS-14`).
    pub visual_state: Vec<VisualState>,
    /// Conformance vector ids this component's machine semantics rely on
    /// (`CROSS-18`); each must resolve in the model.
    pub conformance: Vec<Identifier>,
    /// Explicit runtime extensions with parity consequences (`EXT` class:
    /// `BTN-26/27/29`, `RNG-26/27`, `TXT-31`).
    pub extensions: Vec<Extension>,
}
