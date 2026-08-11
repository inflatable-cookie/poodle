//! Adapter capabilities — named, typed environment work a runtime owns.
//!
//! Serves `CROSS-17` (the adapter capability inventory: focus, measurement/
//! shaping, pointer capture, scrub fraction, text editing/IME, clipboard,
//! portal placement, timers, announcements) and the per-component capability
//! rows (`RNG-13`, `RNG-20`, `TXT-21`–`TXT-24`), per spec 063 `IR-05`
//! ("Adapter ownership") and `IR-08` ("Typed capability gaps"): capabilities
//! are named, typed, and visible in the definition, and a missing capability
//! may degrade or disable a component but may not silently drop behavior.
//! Environment work never moves into drawing (`NEG-03`).

use serde::{Deserialize, Serialize};

/// The named adapter capability inventory (`CROSS-17`; spec 063 "adapter
/// capability" list: focus, measurement, pointer capture, text editing,
/// portal placement, timers, announcements).
///
/// These are vocabulary names for environment work, not runtime types: each
/// runtime's adapter owns the implementation (`IR-05`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Capability {
    /// Focus management, e.g. per-thumb focus stops (`RNG-20`), caret focus
    /// ownership (`TXT-21`), `Tab`/`Shift+Tab` (`BTN-20`).
    #[serde(rename = "focus")]
    Focus,
    /// Measurement/shaping, e.g. glyph measurement for caret placement
    /// (`TXT-21` `shape_line`/`x_for_index`).
    #[serde(rename = "measurement")]
    Measurement,
    /// Pointer capture, e.g. RangeSlider shared-root pointer capture
    /// (`RNG-15`).
    #[serde(rename = "pointer-capture")]
    PointerCapture,
    /// Scrub fraction reporting, e.g. `on_scrub` pointer fraction on the
    /// grab overlay (`RNG-13`).
    #[serde(rename = "scrub-fraction")]
    ScrubFraction,
    /// Text editing, e.g. the shared edit model per target (`TXT-21`).
    #[serde(rename = "text-editing")]
    TextEditing,
    /// Platform IME and input handlers (`TXT-24`).
    #[serde(rename = "ime")]
    Ime,
    /// Platform clipboard for copy/cut/paste (`TXT-23`).
    #[serde(rename = "clipboard")]
    Clipboard,
    /// Portal placement (overlays out of flow) (`CROSS-17`; `NEG-03`).
    #[serde(rename = "portal-placement")]
    PortalPlacement,
    /// Timers, e.g. debounce and validation timing owned by the component
    /// (`TXT-11`, `TXT-28`).
    #[serde(rename = "timers")]
    Timers,
    /// Announcements, e.g. live-region announcements (`CROSS-17`).
    #[serde(rename = "announcements")]
    Announcements,
}

/// A declared capability requirement of a component (`CROSS-17`; `IR-08`
/// "capabilities are named, typed, and visible in the definition").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    /// The capability the component requires.
    pub capability: Capability,
    /// Why the component requires it, naming the corpus requirement and
    /// contract section, e.g. "caret/selection ownership — `T §6` (`TXT-21`)".
    pub purpose: String,
}
