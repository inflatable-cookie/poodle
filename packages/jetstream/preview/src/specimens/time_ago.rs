//! TimeAgo specimen — relative time labels computed from real ISO timestamps.
//!
//! Every value is a real ISO 8601 timestamp, so `js_time_ago` actually runs the
//! parse → diff → threshold-table formatting (not hand-typed relative strings).
//! Live ticking is a preview-loop concern and is not wired here.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::*;
use poodle_jetstream_components::time_ago::js_time_ago;
use poodle_specs::{InlineTypographyMode, TimeAgoSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Recent timestamps — exercises the seconds/minutes/hours/days tiers.
        .child(group(
            "Recent timestamps",
            secondary,
            div()
                .flex_col()
                .gap(6.0)
                .child(row("2026-03-23T10:58:00Z", "2 minutes ago", secondary, theme))
                .child(row("2026-03-23T08:00:00Z", "3 hours ago", secondary, theme))
                .child(row("2026-03-21T11:00:00Z", "2 days ago", secondary, theme)),
        ))
        // Future — exercises the "in …" prefix.
        .child(group(
            "Future timestamp",
            secondary,
            row("2099-01-01T00:00:00Z", "in the future", secondary, theme),
        ))
        // Long format — long-form words + "yesterday" special case.
        .child(group(
            "Long format",
            secondary,
            div()
                .flex_col()
                .gap(6.0)
                .child(row_long("2026-03-23T10:58:00Z", "2 minutes", secondary, theme))
                .child(row_long("2026-03-21T11:00:00Z", "2 days", secondary, theme)),
        ))
        // Static — live updates off (computed once).
        .child(group(
            "Static (live off)",
            secondary,
            js_time_ago(
                &TimeAgoSpec::new()
                    .with_timestamp("2026-03-23T10:58:00Z")
                    .with_live(false),
                theme,
            ),
        ))
        // Inherit typography — font flows from the surrounding inline context.
        .child(group(
            "Inherit typography",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .text_size(20.0)
                .child(label("Updated"))
                .child(js_time_ago(
                    &TimeAgoSpec::new()
                        .with_timestamp("2026-03-23T10:58:00Z")
                        .with_typography(InlineTypographyMode::Inherit),
                    theme,
                ))
                .child(label("by Clay")),
        ))
}

/// A short-form TimeAgo with a caption describing the expected relative text.
fn row(timestamp: &str, caption: &str, text_secondary: glam::Vec4, theme: &JetstreamThemeProvider) -> JsEl {
    div()
        .flex_row()
        .gap(8.0)
        .items_center()
        .child(js_time_ago(&TimeAgoSpec::new().with_timestamp(timestamp), theme))
        .child(
            label(format!("({caption})"))
                .text_color(text_secondary)
                .text_size(11.0),
        )
}

/// A long-form (`short=false`) TimeAgo row with a caption.
fn row_long(
    timestamp: &str,
    caption: &str,
    text_secondary: glam::Vec4,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    div()
        .flex_row()
        .gap(8.0)
        .items_center()
        .child(js_time_ago(
            &TimeAgoSpec::new().with_timestamp(timestamp).with_short(false),
            theme,
        ))
        .child(
            label(format!("({caption})"))
                .text_color(text_secondary)
                .text_size(11.0),
        )
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
