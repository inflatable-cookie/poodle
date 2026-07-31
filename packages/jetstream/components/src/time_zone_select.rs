//! TimeZoneSelect — Jetstream timezone select backed by TimeZoneSelectSpec.
//!
//! Contract: `docs/contracts/components/time-zone-select.md`
//! Reference: `packages/svelte/components/src/TimeZoneSelect.svelte`
//!
//! Thin wrapper over `Select`: the Svelte component delegates rendering and
//! interaction entirely to `Select` in always-searchable mode. The Jetstream
//! target mirrors that by mapping the timezone options into a `SelectSpec`
//! (via `spec.to_select_spec()`) and delegating to `js_select`, so the trigger,
//! search input, option list, grouping, selected indicator, empty state, and
//! size/density all come from the shared `Select` implementation for free.

use jetstream_ui::ui_element::JsEl;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::TimeZoneSelectSpec;

use crate::select::js_select;

/// TimeZoneSelect — a searchable list of zones.
///
/// Mirrors the GPUI target's `on_toggle`, forwarded to the composed `Select`
/// rather than re-implemented.
///
/// No `on_search_change`: the search row is a text field and this runtime
/// raises no key events.
pub struct TimeZoneSelect {
    spec: TimeZoneSelectSpec,
    theme: JetstreamThemeProvider,
    on_toggle: Option<crate::element::ActionHandler>,
    on_change: Option<crate::element::Handler>,
}

impl TimeZoneSelect {
    pub fn from_spec(spec: TimeZoneSelectSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_toggle: None, on_change: None }
    }

    /// Fires when the trigger is pressed.
    pub fn on_toggle(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_toggle = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires with the chosen zone's id.
    pub fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for TimeZoneSelect {
    fn into_js_el(self) -> JsEl {
        let mut select = crate::select::Select::from_spec(self.spec.to_select_spec(), &self.theme);
        if let Some(handler) = self.on_toggle {
            select = select.on_toggle(move || handler());
        }
        if let Some(handler) = self.on_change {
            select = select.on_change(move |value| handler(value));
        }

        let root = crate::element::IntoJsEl::into_js_el(select);
        crate::aria::with_aria_label(root, self.spec.aria_label.as_deref())
    }
}

pub fn js_time_zone_select(spec: &TimeZoneSelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Build the searchable `SelectSpec` exactly as the Svelte wrapper does
    // (searchable always on, timezone empty message, mapped option list,
    // placeholder + value + size/density forwarded) and delegate.
    let select_spec = spec.to_select_spec();
    let root = js_select(&select_spec, theme);
    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn closed_trigger_shows_placeholder() {
        // Default placeholder is the Svelte "Search time zones..." string.
        let spec = TimeZoneSelectSpec::new();
        let tree = probe(&js_time_zone_select(&spec, &theme()), 320.0, 200.0);
        assert!(
            tree.has_text("Search time zones..."),
            "default placeholder missing: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("chevron-down"), "chevron missing: {:?}", tree.texts());
    }

    #[test]
    fn preselected_zone_shows_value_label() {
        let spec = TimeZoneSelectSpec::new().with_value("America/New_York");
        let tree = probe(&js_time_zone_select(&spec, &theme()), 320.0, 200.0);
        // Label is `_`→space formatted (matches Svelte formatTimeZoneLabel).
        assert!(
            tree.has_text("America/New York"),
            "selected zone label missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn open_renders_search_and_default_zones() {
        let spec = TimeZoneSelectSpec::new().with_open(true);
        let tree = probe(&js_time_zone_select(&spec, &theme()), 320.0, 400.0);
        // Searchable always on → search input row present.
        assert!(tree.has_text("search"), "search row missing: {:?}", tree.texts());
        // Default option set surfaces (UTC + a formatted zone label).
        assert!(tree.has_text("UTC"), "UTC option missing: {:?}", tree.texts());
        assert!(
            tree.has_text("America/New York"),
            "default zone option missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn open_with_query_filters_zones() {
        let spec = TimeZoneSelectSpec::new()
            .with_open(true)
            .with_search_query("tokyo");
        let tree = probe(&js_time_zone_select(&spec, &theme()), 320.0, 400.0);
        assert!(tree.has_text("Asia/Tokyo"), "matching zone missing: {:?}", tree.texts());
        assert!(!tree.has_text("UTC"), "non-matching zone not filtered: {:?}", tree.texts());
    }

    #[test]
    fn no_match_renders_timezone_empty_message() {
        let spec = TimeZoneSelectSpec::new()
            .with_open(true)
            .with_search_query("zzzz");
        let tree = probe(&js_time_zone_select(&spec, &theme()), 320.0, 400.0);
        assert!(
            tree.has_text(poodle_specs::TIME_ZONE_EMPTY_MESSAGE),
            "timezone empty message missing: {:?}",
            tree.texts()
        );
    }

    /// The zone list is a composed `Select`, so this also proves the wrapper
    /// forwards rather than re-implementing.
    #[test]
    fn the_trigger_reports_a_toggle() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = TimeZoneSelect::from_spec(TimeZoneSelectSpec::new(), &theme())
            .on_toggle(move || { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 320.0, 200.0, "Search time zones...");

        assert_eq!(hits.load(Ordering::SeqCst), 1, "on_toggle fired exactly once");
    }


    #[test]
    fn choosing_a_zone_reports_it() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let values = Arc::clone(&seen);

        let el = TimeZoneSelect::from_spec(TimeZoneSelectSpec::new().with_open(true), &theme())
            .on_change(move |value| values.lock().unwrap().push(value.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 320.0, 600.0, "UTC");

        assert_eq!(seen.lock().unwrap().len(), 1, "one zone, one event");
    }

}
