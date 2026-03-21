//! pug-gpui-components — Real renderable GPUI components backed by Pug spec structs.
//!
//! Each component wraps a spec from `pug_primitives`, resolves tokens through
//! `GpuiThemeProvider`, and implements `IntoElement` to produce interactive gpui elements.

pub mod theme_ext;
pub mod primitives;
pub mod composites;

// Re-export all primitives
pub use primitives::*;

// Re-export all composites
pub use composites::*;

// Re-export common types from pug_primitives for ergonomic imports
pub use pug_primitives::{
    ButtonTone, ButtonVariant, ControlSize, IconSize, IconSpec, StatusTone, ValidationState,
};
