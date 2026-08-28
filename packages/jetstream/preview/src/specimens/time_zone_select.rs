//! TimeZoneSelect specimen — searchable timezone picker (Select-backed).
//!
//! Mirrors the GPUI specimen: default trigger, pre-selected zone, an
//! always-open searchable state (search field + filtered zones + selected
//! indicator), disabled, and a size sweep. Every node is a real
//! `js_time_zone_select` delegating to the shared `Select`.

use crate::compat::js_time_zone_select;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlSize, TimeZoneSelectSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // --- Default (placeholder trigger) ---
        .child(group(
            "Default",
            secondary,
            div().w(280.0).child(js_time_zone_select(
                &TimeZoneSelectSpec::new()
                    .with_aria_label("Time zone 1")
                    .with_placeholder("Select time zone…"),
                theme,
                "tz-select-1",
            )),
        ))
        // --- With pre-selected zone ---
        .child(group(
            "With pre-selected zone",
            secondary,
            div().w(280.0).child(js_time_zone_select(
                &TimeZoneSelectSpec::new()
                    .with_aria_label("Time zone 2")
                    .with_value("America/New_York"),
                theme,
                "tz-select-2",
            )),
        ))
        // --- Open (searchable, selected): trigger expanded, live query filters
        //     the zone list, selected zone highlighted. Rendered statically open
        //     so the dropdown anatomy (search field + options + selected check)
        //     is visible without interaction. ---
        .child(group(
            "Open (searchable, selected)",
            secondary,
            div().w(280.0).child(js_time_zone_select(
                &TimeZoneSelectSpec::new()
                    .with_aria_label("Time zone 3")
                    .with_value("America/New_York")
                    .with_open(true)
                    .with_search_query("amer"),
                theme,
                "tz-select-3",
            )),
        ))
        // --- Disabled ---
        .child(group(
            "Disabled",
            secondary,
            div().w(280.0).child(js_time_zone_select(
                &TimeZoneSelectSpec::new()
                    .with_aria_label("Time zone 4")
                    .with_value("Europe/London")
                    .with_disabled(true),
                theme,
                "tz-select-4",
            )),
        ))
        // --- Sizes (sm / md / lg) ---
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(size_row("sm", secondary, ControlSize::Sm, theme))
                .child(size_row("md", secondary, ControlSize::Md, theme))
                .child(size_row("lg", secondary, ControlSize::Lg, theme)),
        ))
}

fn size_row(
    size_label: &str,
    text_secondary: ColorValue,
    size: ControlSize,
    theme: &JetstreamThemeProvider,
) -> El {
    div()
        .flex_row()
        .gap(12.0)
        .items_center()
        .child(
            label(size_label)
                .text_color(text_secondary)
                .text_size(11.0)
                .w(24.0),
        )
        .child(
            div().w(280.0).child(js_time_zone_select(
                &TimeZoneSelectSpec::new()
                    .with_aria_label("Time zone 5")
                    .with_value("America/New_York")
                    .with_size(size),
                theme,
                "tz-select-5",
            )),
        )
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
