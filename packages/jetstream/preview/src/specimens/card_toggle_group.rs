//! CardToggleGroup specimen — multi-select card group across styled cards.
//!
//! Mirrors the GPUI specimen scope: the shared `CardToggleGroupSpec` is a
//! multi-select model (`values: Vec<String>`), so each group demonstrates
//! single- and multi-selection, per-item + group disabled, and the size/density
//! variants the spec exposes. Each option composes the real `Card` primitive via
//! `js_card_toggle_group`. The spec carries no `columns`/`count` fields, so those
//! are not exercised here (shared-spec gap noted in the parity doc).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::card_toggle_group::js_card_toggle_group;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CardToggleGroupSpec, CardToggleOption, ControlDensity, ControlSize};

fn view_options() -> Vec<CardToggleOption> {
    vec![
        CardToggleOption::new("grid", "Grid view").with_description("Show records as cards."),
        CardToggleOption::new("list", "List view").with_description("Use dense rows."),
        CardToggleOption::new("board", "Board view")
            .with_description("Group records into columns."),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let mut root = div().flex_col().gap(24.0);

    // --- Single selection ---
    root = root.child(group(
        "View mode (single selection)",
        secondary,
        js_card_toggle_group(
            &CardToggleGroupSpec::new(view_options()).with_values(vec!["grid".into()]),
            theme,
        ),
    ));

    // --- Multiple selection ---
    root = root.child(group(
        "Multiple selection",
        secondary,
        js_card_toggle_group(
            &CardToggleGroupSpec::new(view_options())
                .with_values(vec!["grid".into(), "board".into()]),
            theme,
        ),
    ));

    // --- Per-item disabled ---
    root = root.child(group(
        "Disabled option",
        secondary,
        js_card_toggle_group(
            &CardToggleGroupSpec::new(vec![
                CardToggleOption::new("draft", "Draft").with_description("In progress."),
                CardToggleOption::new("live", "Live").with_description("Published."),
                CardToggleOption::new("archived", "Archived")
                    .with_description("Read-only snapshot.")
                    .with_disabled(true),
            ])
            .with_values(vec!["live".into()]),
            theme,
        ),
    ));

    // --- Group disabled ---
    root = root.child(group(
        "Disabled group",
        secondary,
        js_card_toggle_group(
            &CardToggleGroupSpec::new(view_options())
                .with_values(vec!["list".into()])
                .with_disabled(true),
            theme,
        ),
    ));

    // --- Sizes (xs → xl) ---
    let mut sizes_row = div().flex_col().gap(12.0);
    for size in [
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ] {
        sizes_row = sizes_row.child(js_card_toggle_group(
            &CardToggleGroupSpec::new(view_options())
                .with_values(vec!["grid".into()])
                .with_size(size),
            theme,
        ));
    }
    root = root.child(group("Sizes (xs / sm / md / lg / xl)", secondary, sizes_row));

    // --- Densities (compact / default / comfortable) ---
    let mut densities_row = div().flex_col().gap(12.0);
    for density in [
        ControlDensity::Compact,
        ControlDensity::Default,
        ControlDensity::Comfortable,
    ] {
        densities_row = densities_row.child(js_card_toggle_group(
            &CardToggleGroupSpec::new(view_options())
                .with_values(vec!["grid".into()])
                .with_density(density),
            theme,
        ));
    }
    root = root.child(group(
        "Densities (compact / default / comfortable)",
        secondary,
        densities_row,
    ));

    root
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
