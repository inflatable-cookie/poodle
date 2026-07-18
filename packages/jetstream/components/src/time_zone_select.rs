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

pub fn js_time_zone_select(spec: &TimeZoneSelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Build the searchable `SelectSpec` exactly as the Svelte wrapper does
    // (searchable always on, timezone empty message, mapped option list,
    // placeholder + value + size/density forwarded) and delegate.
    let select_spec = spec.to_select_spec();
    js_select(&select_spec, theme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
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
}
