//! Events — the event/effect vocabulary and event timing.
//!
//! Serves `CROSS-05` (activation `onClick`, value-change vs value-commit,
//! focus/blur, pressed-change, submit/cancel/clear — each with payload and
//! firing condition), `CROSS-06` (event timing: change during interaction,
//! commit on release, debounce, Enter/Escape, blur flush, and ordering such
//! as `onPressedChange` before `onClick`), and the per-component event rows
//! (`BTN-14`, `RNG-11`, `TXT-08`, `TXT-13`, `TXT-21`, `TXT-28`).

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A declared event on a component (`CROSS-05`; spec 063 "declarative
/// transition, guard, or effect-intent" and "events").
///
/// Events are intent, not wiring: the payload and firing condition are
/// declared, and each runtime machine executes them (`CV` semantics,
/// `CROSS-06`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Stable identifier for the event, e.g. `value-change` (`CROSS-05`).
    pub id: Identifier,
    /// Public event name, e.g. `onValueChange` (`RNG-11`, `TXT-13`).
    pub name: String,
    /// What kind of event this is (`CROSS-05`).
    pub kind: EventKind,
    /// Payload carried by the event, if any (`CROSS-05` "each with payload
    /// and firing condition").
    pub payload: Option<EventPayload>,
    /// Declared firing timing (`CROSS-06`).
    pub timing: EventTiming,
    /// What the event means and when it fires, citing the contract section.
    pub description: String,
}

/// The event/effect vocabulary (`CROSS-05`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    /// Activation, e.g. `onClick` (`CROSS-05`, `BTN-14`).
    #[serde(rename = "activation")]
    Activation,
    /// Value change during interaction, e.g. `onValueChange` (`CROSS-05`,
    /// `RNG-11`, `TXT-13`).
    #[serde(rename = "value-change")]
    ValueChange,
    /// Value commit on release, e.g. `onValueCommit` (`CROSS-05`, `RNG-11`).
    #[serde(rename = "value-commit")]
    ValueCommit,
    /// Focus change — focus/blur (`CROSS-05`, `TXT-13`).
    #[serde(rename = "focus-change")]
    FocusChange,
    /// Pressed-state change on a toggle, e.g. `onPressedChange`
    /// (`CROSS-05`, `BTN-14`).
    #[serde(rename = "pressed-change")]
    PressedChange,
    /// Submit, e.g. `onSubmit` on Enter / Cmd+Ctrl+Enter (`CROSS-05`,
    /// `TXT-13`, `TXT-07`).
    #[serde(rename = "submit")]
    Submit,
    /// Cancel, e.g. `onCancel` on Escape (`CROSS-05`, `TXT-13`).
    #[serde(rename = "cancel")]
    Cancel,
    /// Clear, e.g. the search-mode `clear` event (`CROSS-05`, `TXT-08`).
    #[serde(rename = "clear")]
    Clear,
    /// Selection change, e.g. `onSelectionChange` for caret/selection
    /// ownership (`TXT-21`).
    #[serde(rename = "selection-change")]
    SelectionChange,
}

/// Payload of an event (`CROSS-05` "each with payload and firing condition").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventPayload {
    /// Payload name, e.g. `value` for `onValueChange(string)` (`TXT-13`) or
    /// `[lower, upper]` for the RangeSlider pair (`RNG-11`).
    pub name: String,
    /// Payload value kind.
    pub kind: PayloadKind,
}

/// The value kind of an event payload (`CROSS-05`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayloadKind {
    /// No payload, e.g. `onClick` and Jetstream `on_click` (`BTN-27`).
    #[serde(rename = "none")]
    None,
    /// A string, e.g. `onValueChange(string)` (`TXT-13`).
    #[serde(rename = "string")]
    String,
    /// A number, e.g. a scrub fraction (`RNG-13` `on_scrub`).
    #[serde(rename = "number")]
    Number,
    /// A boolean, e.g. `onPressedChange(bool)` (`BTN-14`).
    #[serde(rename = "boolean")]
    Bool,
    /// A pair, e.g. `onValueChange([lower, upper])` reported together
    /// (`RNG-11`, `RNG-27` "report `(low, high)` together").
    #[serde(rename = "pair")]
    Pair,
    /// A validation status object `{status, valid, message}`
    /// (`TXT-13` `onValidationChange`).
    #[serde(rename = "validation-status")]
    ValidationStatus,
}

/// Declared firing timing of an event (`CROSS-06`; `CV` semantics executed by
/// each runtime machine).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventTiming {
    /// When the event fires relative to the interaction.
    pub phase: FiringPhase,
    /// Debounce before firing, in milliseconds (`CROSS-06`, `TXT-11` —
    /// `debounce` delays `onValueChange`).
    pub debounce_ms: Option<u32>,
    /// Whether a pending debounce flushes on blur (`CROSS-06`, `TXT-11`
    /// "flush on blur", `TXT-28` "blur flush").
    pub flush_on_blur: bool,
    /// Declared ordering constraints against sibling events, e.g.
    /// `onPressedChange` fires before `onClick` (`CROSS-06`, `BTN-14`).
    pub ordering: Vec<OrderingConstraint>,
}

impl Default for EventTiming {
    fn default() -> Self {
        Self {
            phase: FiringPhase::DuringInteraction,
            debounce_ms: None,
            flush_on_blur: false,
            ordering: Vec::new(),
        }
    }
}

/// Firing phase of an event (`CROSS-06`; `RNG-11` change during interaction
/// on `input`, commit on release on `change`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiringPhase {
    /// Fires during the interaction, e.g. value change per `input` event
    /// (`CROSS-06`, `RNG-11`).
    #[serde(rename = "during-interaction")]
    DuringInteraction,
    /// Fires on release/commit, e.g. value commit on `change` event
    /// (`CROSS-06`, `RNG-11`).
    #[serde(rename = "on-release")]
    OnRelease,
    /// Fires on blur flush (`CROSS-06`, `TXT-11`).
    #[serde(rename = "on-blur")]
    OnBlur,
    /// Fires immediately, e.g. clear and slug source regeneration
    /// (`TXT-11` "immediate for clear and slug source regeneration").
    #[serde(rename = "immediate")]
    Immediate,
}

/// Declared ordering between two sibling events (`CROSS-06`; the contract
/// ordering note in `B §5` — `onPressedChange` before `onClick`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderingConstraint {
    /// Event id that must fire first, e.g. `pressed-change` (`BTN-14`).
    pub before: Identifier,
    /// Event id that must fire after it, e.g. `activation` (`BTN-14`).
    pub after: Identifier,
    /// Why the order is required, citing the contract note.
    pub reason: String,
}
