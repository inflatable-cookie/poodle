//! SegmentedControl specimen — selection, disabled, equal-width vs content-fit,
//! plus full size and density matrices. Mirrors the contract §13 specimens and
//! the Svelte SegmentedControlSpecimen.
//!
//! NOTE: `js_segmented_control` is render-only — selection is drawn from
//! `current_value()`; click + arrow-key selection lives in the preview event
//! loop per Jetstream architecture. The "Default" group therefore shows a
//! static seeded selection rather than a live `on_change` readout (accepted
//! limit, see parity/segmented-control.md).

use crate::compat::js_segmented_control;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, SegmentedControlOption, SegmentedControlSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Contract default set: Grid / List / Table (Svelte authority).
    let view_options = vec![
        SegmentedControlOption::new("grid", "Grid"),
        SegmentedControlOption::new("list", "List"),
        SegmentedControlOption::new("table", "Table"),
    ];

    // Contract "With disabled option" set: All / Active / Archived / Draft(disabled).
    let status_options = vec![
        SegmentedControlOption::new("all", "All"),
        SegmentedControlOption::new("active", "Active"),
        SegmentedControlOption::new("archived", "Archived"),
        SegmentedControlOption::new("draft", "Draft").with_disabled(true),
    ];

    div()
        .flex_col()
        .gap(24.0)
        // Default — Grid selected (seeded; render-only, no live readout).
        .child(group(
            "Default (Grid selected)",
            secondary,
            div().w(300.0).child(js_segmented_control(
                &SegmentedControlSpec::new(view_options.clone()).with_default_value("grid"),
                theme,
            )),
        ))
        // With disabled option — Draft segment muted, All selected.
        .child(group(
            "With disabled option (Draft)",
            secondary,
            div().w(340.0).child(js_segmented_control(
                &SegmentedControlSpec::new(status_options).with_default_value("all"),
                theme,
            )),
        ))
        // Fully disabled — whole control muted, List shows selected styling.
        .child(group(
            "Fully disabled",
            secondary,
            div().w(300.0).child(js_segmented_control(
                &{
                    let mut s =
                        SegmentedControlSpec::new(view_options.clone()).with_default_value("list");
                    s.is_disabled = true;
                    s
                },
                theme,
            )),
        ))
        // Equal width (default) — segments share the track equally.
        .child(group(
            "Equal width (default)",
            secondary,
            div().w(360.0).child(js_segmented_control(
                &SegmentedControlSpec::new(vec![
                    SegmentedControlOption::new("day", "Day"),
                    SegmentedControlOption::new("week", "Week"),
                    SegmentedControlOption::new("month", "Month"),
                    SegmentedControlOption::new("year", "Year"),
                ])
                .with_default_value("week")
                .with_equal_width(true),
                theme,
            )),
        ))
        // Content fit (equalWidth=false) — segments size to label, group left-aligns.
        .child(group(
            "Content fit (equalWidth=false)",
            secondary,
            js_segmented_control(
                &SegmentedControlSpec::new(vec![
                    SegmentedControlOption::new("1h", "1h").with_aria_label("Last 1 hour"),
                    SegmentedControlOption::new("6h", "6h").with_aria_label("Last 6 hours"),
                    SegmentedControlOption::new("24h", "24h").with_aria_label("Last 24 hours"),
                ])
                .with_default_value("24h")
                .with_size(ControlSize::Xs)
                .with_equal_width(false),
                theme,
            ),
        ))
        // Sizes — full xs–xl ladder.
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(sized(theme, ControlSize::Xs))
                .child(sized(theme, ControlSize::Sm))
                .child(sized(theme, ControlSize::Md))
                .child(sized(theme, ControlSize::Lg))
                .child(sized(theme, ControlSize::Xl)),
        ))
        // Densities — compact / default / comfortable (height unchanged).
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(dense(theme, ControlDensity::Compact))
                .child(dense(theme, ControlDensity::Default))
                .child(dense(theme, ControlDensity::Comfortable)),
        ))
}

fn view_opts() -> Vec<SegmentedControlOption> {
    vec![
        SegmentedControlOption::new("grid", "Grid"),
        SegmentedControlOption::new("list", "List"),
        SegmentedControlOption::new("table", "Table"),
    ]
}

fn sized(theme: &JetstreamThemeProvider, size: ControlSize) -> El {
    div().w(300.0).child(js_segmented_control(
        &SegmentedControlSpec::new(view_opts())
            .with_default_value("grid")
            .with_size(size),
        theme,
    ))
}

fn dense(theme: &JetstreamThemeProvider, density: ControlDensity) -> El {
    div().w(300.0).child(js_segmented_control(
        &SegmentedControlSpec::new(view_opts())
            .with_default_value("grid")
            .with_density(density),
        theme,
    ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
