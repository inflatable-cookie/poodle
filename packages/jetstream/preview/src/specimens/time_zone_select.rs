//! TimeZoneSelect specimen — searchable timezone picker (Select-backed).
//!
//! Mirrors the GPUI specimen: default trigger, pre-selected zone, an
//! always-open searchable state (search field + filtered zones + selected
//! indicator), disabled, and a size sweep. Every node is a real
//! `js_time_zone_select` delegating to the shared `Select`.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::time_zone_select::js_time_zone_select;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, TimeZoneSelectSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // --- Default (placeholder trigger) ---
        .child(group("Default", secondary,
            div().w(280.0)
                .child(js_time_zone_select(
                    &TimeZoneSelectSpec::new().with_placeholder("Select time zone…"),
                    theme,
                ))
        ))
        // --- With pre-selected zone ---
        .child(group("With pre-selected zone", secondary,
            div().w(280.0)
                .child(js_time_zone_select(
                    &TimeZoneSelectSpec::new().with_value("America/New_York"),
                    theme,
                ))
        ))
        // --- Open (searchable, selected): trigger expanded, live query filters
        //     the zone list, selected zone highlighted. Rendered statically open
        //     so the dropdown anatomy (search field + options + selected check)
        //     is visible without interaction. ---
        .child(group("Open (searchable, selected)", secondary,
            div().w(280.0)
                .child(js_time_zone_select(
                    &TimeZoneSelectSpec::new()
                        .with_value("America/New_York")
                        .with_open(true)
                        .with_search_query("amer"),
                    theme,
                ))
        ))
        // --- Disabled ---
        .child(group("Disabled", secondary,
            div().w(280.0)
                .child(js_time_zone_select(
                    &TimeZoneSelectSpec::new().with_value("Europe/London").with_disabled(true),
                    theme,
                ))
        ))
        // --- Sizes (sm / md / lg) ---
        .child(group("Sizes", secondary,
            div().flex_col().gap(8.0)
                .child(size_row("sm", secondary, ControlSize::Sm, theme))
                .child(size_row("md", secondary, ControlSize::Md, theme))
                .child(size_row("lg", secondary, ControlSize::Lg, theme))
        ))
}

fn size_row(
    size_label: &str,
    text_secondary: glam::Vec4,
    size: ControlSize,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    div().flex_row().gap(12.0).items_center()
        .child(label(size_label).text_color(text_secondary).text_size(11.0).w(24.0))
        .child(div().w(280.0).child(js_time_zone_select(
            &TimeZoneSelectSpec::new().with_value("America/New_York").with_size(size),
            theme,
        )))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
