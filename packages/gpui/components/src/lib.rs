//! poodle-gpui-components — Real renderable GPUI components backed by Poodle spec structs.
//!
//! Each component wraps a spec from `poodle_specs`, resolves tokens through
//! `GpuiThemeProvider`, and implements `IntoElement` to produce interactive gpui elements.

pub mod presentation;
pub mod theme_ext;
pub mod primitives;
pub mod composites;

// Re-export all primitives
pub use primitives::*;

// Re-export all composites
pub use composites::*;

// Re-export common types from poodle_specs for ergonomic imports
pub use poodle_specs::{
    ButtonTone, ButtonVariant, ControlDensity, ControlSize, IconSize, IconSpec,
    SemanticControlSizeRole, StatusTone, ValidationState,
};
