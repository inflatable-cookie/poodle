//! Keyboard command tables — declared vocabulary, adapter-owned delivery.
//!
//! Serves `CROSS-16` (per-component keyboard command tables declared as
//! vocabulary; delivery is adapter-owned) and the per-component keyboard rows
//! (`BTN-20`, `RNG-18`, `TXT-20`), per spec 063 "Component IR" ("keyboard
//! commands"). A command may declare the [`Capability`](crate::Capability)
//! its delivery requires; `validate` flags a command whose capability is not
//! declared by the component ("undeclared capabilities", `IR-08`).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{Capability, Identifier};

/// A declared keyboard command (`CROSS-16`; `B §6`, `R §6`, `T §6`).
///
/// Keys are declarative chords; the effect is a declarative action name the
/// runtime machine implements, never executable code (`NEG-01`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardCommand {
    /// Stable identifier for the command, e.g. `increment-step`
    /// (`RNG-18`).
    pub id: Identifier,
    /// Key chords that trigger the command, e.g. `ArrowRight` (`RNG-18`),
    /// `Enter`/`Space` (`BTN-20`), `accel+z` (`TXT-25`).
    pub keys: Vec<KeyChord>,
    /// Declarative action name, e.g. `increment-step`, `submit`, `cancel`,
    /// `undo`, `paste` (`RNG-18`, `TXT-13`, `TXT-25`, `TXT-23`).
    pub action: String,
    /// Declarative effect-intent, e.g. "value += step; emit value-change;
    /// commit on release" (`RNG-18` runs INPUT then COMMIT through the
    /// machine).
    pub effect: String,
    /// Capability the delivery requires, if any — e.g. `Clipboard` for
    /// paste (`TXT-23`), `Focus` for Tab cycling (`RNG-18`). `validate`
    /// checks the component declares it.
    pub requires: Option<Capability>,
    /// What the command does, citing the contract keyboard table.
    pub description: String,
}

/// A key chord — one or more modifier keys plus a main key (`CROSS-16`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyChord {
    /// The main key, e.g. `ArrowRight`, `Enter`, `Escape`, `z`
    /// (`RNG-18`, `BTN-20`, `TXT-13`, `TXT-25`).
    pub key: String,
    /// Modifiers, serialized sorted for deterministic output (`IR-07`).
    pub modifiers: BTreeSet<Modifier>,
}

/// A modifier key (`CROSS-16`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Modifier {
    #[serde(rename = "shift")]
    Shift,
    #[serde(rename = "ctrl")]
    Control,
    #[serde(rename = "alt")]
    Alt,
    /// Platform meta/command key, e.g. `accel+z` undo (`TXT-25`).
    #[serde(rename = "meta")]
    Meta,
}
