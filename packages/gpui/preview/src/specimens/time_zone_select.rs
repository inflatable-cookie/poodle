use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, TimeZoneSelect};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, TimeZoneSelectSpec};
use std::sync::{Arc, Mutex};

fn toggle_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
) -> Arc<dyn Fn() + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move || {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::Toggle(key.to_string()));
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let is_open = state.specimens.is_on("tz-select-open");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(320.0))
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(
                    TimeZoneSelect::from_spec(
                        TimeZoneSelectSpec::new()
                            .with_placeholder("Select time zone\u{2026}")
                            .with_open(is_open),
                        theme,
                    )
                    .on_toggle(toggle_handler(&state.node_events, "tz-select-open")),
                ),
        )
        // --- With pre-selected zone ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With pre-selected zone"),
                    theme,
                ))
                .child(TimeZoneSelect::from_spec(
                    TimeZoneSelectSpec::new().with_value("America/New_York"),
                    theme,
                )),
        )
        // --- Open (searchable): trigger expanded, live query filters the zone
        //     list, selected zone highlighted with a check. Rendered statically
        //     open so the dropdown anatomy (search field + filtered options +
        //     selected indicator) is visible without interaction. ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                // Reserve vertical room for the absolutely-positioned dropdown
                // so it does not overlap the following examples.
                .pb(px(240.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Open (searchable, selected)"),
                    theme,
                ))
                .child(TimeZoneSelect::from_spec(
                    TimeZoneSelectSpec::new()
                        .with_value("America/New_York")
                        .with_open(true)
                        .with_search_query("amer"),
                    theme,
                )),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(TimeZoneSelect::from_spec(
                    TimeZoneSelectSpec::new()
                        .with_value("Europe/London")
                        .with_disabled(true),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "time-zone-select",
        examples,
        |size, theme: &GpuiThemeProvider| {
            TimeZoneSelect::from_spec(
                TimeZoneSelectSpec::new().with_value("America/New_York"),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            TimeZoneSelect::from_spec(
                TimeZoneSelectSpec::new().with_value("America/New_York"),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
