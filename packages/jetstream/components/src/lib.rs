//! Pug component implementations for Jetstream.
//!
//! # CONTRACT ADHERENCE IS MANDATORY
//!
//! Every component in this crate MUST faithfully implement its contract from
//! `docs/contracts/foundation/<component>.md`. Before writing or modifying any
//! component, read the full contract. The contract defines:
//!
//! - **Anatomy**: which parts exist and how they nest
//! - **Token targets**: which semantic token controls each visual property
//! - **Props**: every prop, its type, and default
//! - **States**: hover, active, focus, disabled, loading
//! - **Accessibility**: ARIA roles, attributes, keyboard behavior
//!
//! ## Rules
//!
//! 1. **ZERO hardcoded pixel values.** Every dimension (height, padding, gap,
//!    radius, font-size, icon-size) must resolve from a token via the Spec's
//!    token methods. If you write `.h(16.0)` instead of
//!    `.h(resolve_px(theme, spec.some_token()))`, it is wrong.
//!
//! 2. **ZERO hardcoded colors.** Every color must resolve from a token.
//!
//! 3. **All anatomy parts present.** If the contract says the component has
//!    a root, indicator, label, and panel — all four must exist in the output.
//!
//! 4. **Svelte is the visual reference.** When the contract is ambiguous,
//!    check `packages/svelte/primitives/src/` for the correct behavior.
//!
//! ```rust,ignore
//! use pug_jetstream_components::button::js_button;
//! use pug_primitives::ButtonSpec;
//!
//! let el = js_button(&ButtonSpec::new().with_label("Save"), &theme);
//! ```

pub mod theme_ext;

// Existing components
pub mod accordion;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod progress;
pub mod separator;
pub mod status_indicator;
pub mod switch;

// Structural primitives (g10.003)
pub mod banner;
pub mod bx;
pub mod callout;
pub mod card;
pub mod detail_row;
pub mod eyebrow;
pub mod field;
pub mod form_actions;
pub mod grid;
pub mod pill;
pub mod region;
pub mod skeleton;
pub mod stack;
pub mod surface;
pub mod toolbar;
