//! TimeZoneSelect — a searchable list of zones.
//!
//! Contract: `docs/contracts/components/time-zone-select.md`
//! Ported from: `packages/jetstream/components/src/time_zone_select.rs`.
//!
//! Thin wrapper over `select`: the timezone options map into a `SelectSpec`
//! (via `spec.to_select_spec()`), so the trigger, search input, option list,
//! grouping, selected indicator, empty state and size/density all come from
//! the shared select implementation for free.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::TimeZoneSelectSpec;

use crate::select::{select, SelectHandlers};

/// Host callbacks: `on_toggle` (trigger) and `on_change` (chosen zone id),
/// forwarded to the composed select.
#[derive(Default)]
pub struct TimeZoneSelectHandlers {
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn time_zone_select(
    spec: &TimeZoneSelectSpec,
    theme: &dyn ThemeProvider,
    handlers: TimeZoneSelectHandlers,
) -> Node {
    // Build the searchable `SelectSpec` exactly as the Svelte wrapper does
    // (searchable always on, timezone empty message, mapped option list,
    // placeholder + value + size/density forwarded) and delegate.
    let select_spec = spec.to_select_spec();
    let mut root = select(
        &select_spec,
        theme,
        &SelectHandlers {
            toggle: handlers.on_toggle,
            change: handlers.on_change,
            clear: None,
        },
    );
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}
