//! Poodle component implementations for Jetstream.
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
//! use poodle_jetstream_components::button::js_button;
//! use poodle_primitives::ButtonSpec;
//!
//! let el = js_button(&ButtonSpec::new().with_label("Save"), &theme);
//! ```

pub mod presentation;
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

// Interactive primitives (g10.004)
pub mod code;
pub mod collapse_toggle;
pub mod collapsible;
pub mod icon;
pub mod icon_button;
pub mod meter;
pub mod number_entry;
pub mod radio_group;
pub mod range_slider;
pub mod rating;
pub mod search_field;
pub mod segmented_control;
pub mod slider;
pub mod split_button;
pub mod tab_strip;
pub mod tabs;
pub mod text_area;
pub mod text_input;
pub mod time_ago;

// Complex primitives (g10.005)
pub mod alert_dialog;
pub mod breadcrumbs_comp;
pub mod bulk_action_bar;
pub mod calendar;
pub mod combobox;
pub mod context_menu;
pub mod date_picker;
pub mod dialog;
pub mod drawer;
pub mod editable_label;
pub mod hover_card;
pub mod menu;
pub mod menubar;
pub mod navigation_menu;
pub mod pagination_comp;
pub mod pin_input;
pub mod popover;
pub mod resize_handle;
pub mod scroll_shell;
pub mod select;
pub mod tooltip;

// Composites (g10.006)
pub mod action_discovery_panel;
pub mod app_header;
pub mod command_palette;
pub mod data_table;
pub mod detail_section;
pub mod detail_shell;
pub mod dock_region;
pub mod empty_state;
pub mod filter_toolbar;
pub mod list_card;
pub mod media_preview;
pub mod media_thumbnail;
pub mod metric_tile;
pub mod nav_card;
pub mod nav_card_grid;
pub mod order_by;
pub mod page_header;
pub mod pagination_summary;
pub mod picker_shell;
pub mod relation_picker;
pub mod selection_summary;
pub mod split_view;
pub mod toast_stack;
