//! Controlled-state model — controlled/uncontrolled pairs and the
//! do-not-mix rule.
//!
//! Serves `CROSS-04` (per-component controlled-state model: Button
//! `pressed`/`defaultPressed` toggle; RangeSlider bindable pair; TextInput
//! `value`/`defaultValue` with a "do not mix modes" rule) and `TXT-02`
//! (controlled `value` + `onValueChange`; `defaultValue` seeds uncontrolled;
//! do not mix modes).

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A controlled/uncontrolled state pair on a component (`CROSS-04`).
///
/// `controlled` names the prop carrying the controlled value (e.g. `pressed`,
/// `value`); `seed` names the prop seeding the uncontrolled mode (e.g.
/// `defaultPressed`, `defaultValue`). The pair is the declarative form of the
/// runtime's `$bindable`/`isControlled` logic (`B/R/T §3` evidence).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlledState {
    /// Stable identifier for the state, e.g. `pressed` (`CROSS-04`).
    pub id: Identifier,
    /// Prop id that carries the controlled value, e.g. `pressed`
    /// (`BTN-14`), `value` (`RNG-01`, `TXT-02`).
    pub controlled: Identifier,
    /// Prop id that seeds the uncontrolled mode, e.g. `defaultPressed`
    /// (`BTN-14`), `defaultValue` (`RNG-01`, `TXT-02`).
    pub seed: Identifier,
    /// Rule governing the pair — always the do-not-mix rule for the pilot
    /// vocabulary (`CROSS-04`; `T §3`).
    pub rule: ControlRule,
    /// What the pair expresses, citing the contract section.
    pub description: String,
}

/// Rule applied to a controlled/uncontrolled pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlRule {
    /// Controlled and seed must never both be bound — "do not mix modes"
    /// (`CROSS-04`; `B §3`, `R §3`, `T §3`). [`validate`](crate::validate)
    /// reports an `ImpossibleBinding` finding when a scene binds both props
    /// of a `DoNotMix` pair.
    #[serde(rename = "do-not-mix")]
    DoNotMix,
}
