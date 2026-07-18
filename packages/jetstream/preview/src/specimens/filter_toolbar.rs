//! FilterToolbar specimen — layout container for filter controls.
//!
//! Grid slots, header actions, and the secondary slot are REAL controls
//! (js_text_input / js_select / js_icon_button / js_button), mirroring the
//! Svelte FilterToolbarSpecimen — no stand-in label chips.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::filter_toolbar::js_filter_toolbar;
use poodle_jetstream_components::icon_button::js_icon_button;
use poodle_jetstream_components::select::js_select;
use poodle_jetstream_components::text_input::js_text_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ChoiceOption, ControlDensity, ControlSize, FilterToolbarSpec,
    IconButtonSpec, SelectSpec, TextInputSpec,
};

fn status_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("all", "All statuses"),
        ChoiceOption::new("active", "Active"),
        ChoiceOption::new("archived", "Archived"),
        ChoiceOption::new("draft", "Draft"),
    ]
}

fn type_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("all", "All types"),
        ChoiceOption::new("document", "Document"),
        ChoiceOption::new("spreadsheet", "Spreadsheet"),
        ChoiceOption::new("presentation", "Presentation"),
    ]
}

fn owner_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("all", "All owners"),
        ChoiceOption::new("me", "Me"),
        ChoiceOption::new("team", "My team"),
    ]
}

fn search(theme: &JetstreamThemeProvider, size: Option<ControlSize>, density: Option<ControlDensity>) -> JsEl {
    let mut spec = TextInputSpec::new()
        .with_input_type("search")
        .with_placeholder("Search…")
        .with_aria_label("Search items");
    if let Some(size) = size {
        spec = spec.with_size(size);
    }
    if let Some(density) = density {
        spec = spec.with_density(density);
    }
    js_text_input(&spec, theme)
}

fn select(
    theme: &JetstreamThemeProvider,
    options: Vec<ChoiceOption>,
    size: Option<ControlSize>,
    density: Option<ControlDensity>,
) -> JsEl {
    let mut spec = SelectSpec::new(options).with_value("all");
    if let Some(size) = size {
        spec = spec.with_size(size);
    }
    if let Some(density) = density {
        spec = spec.with_density(density);
    }
    js_select(&spec, theme)
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Refresh icon-button for the header actions slot.
    let refresh = |theme: &JetstreamThemeProvider| {
        js_icon_button(
            &IconButtonSpec::new()
                .with_icon("refresh-cw")
                .with_aria_label("Refresh"),
            theme,
        )
    };

    let sizes: &[(&str, ControlSize)] = &[
        ("xs", ControlSize::Xs),
        ("sm", ControlSize::Sm),
        ("md", ControlSize::Md),
        ("lg", ControlSize::Lg),
        ("xl", ControlSize::Xl),
    ];
    let densities: &[(&str, ControlDensity)] = &[
        ("compact", ControlDensity::Compact),
        ("default", ControlDensity::Default),
        ("comfortable", ControlDensity::Comfortable),
    ];

    let mut root = div().flex_col().gap(24.0);

    // Responsive grid layout: search + 3 selects, summary text, no collapse.
    root = root.child(group(
        "Responsive grid layout",
        secondary,
        js_filter_toolbar(
            &FilterToolbarSpec::new()
                .with_summary_text("Showing 24 of 156 items")
                .with_aria_label("Item filters")
                .with_collapsible(false),
            theme,
            vec![
                search(theme, None, None),
                select(theme, status_options(), None, None),
                select(theme, type_options(), None, None),
                select(theme, owner_options(), None, None),
            ],
            None,
            None,
        ),
    ));

    // Collapsible with actions (expanded): collapse toggle + refresh action.
    root = root.child(group(
        "Collapsible with actions",
        secondary,
        js_filter_toolbar(
            &FilterToolbarSpec::new()
                .with_summary_text("Showing 24 of 156 items")
                .with_aria_label("Collapsible filters")
                .with_collapsible(true)
                .with_collapsed(false),
            theme,
            vec![
                search(theme, None, None),
                select(theme, status_options(), None, None),
                select(theme, type_options(), None, None),
            ],
            Some(refresh(theme)),
            None,
        ),
    ));

    // Collapsed by default: compact header row, controls hidden.
    root = root.child(group(
        "Collapsed by default",
        secondary,
        js_filter_toolbar(
            &FilterToolbarSpec::new()
                .with_summary_text("3 filters active")
                .with_aria_label("Collapsed filters")
                .with_collapsible(true)
                .with_collapsed(true),
            theme,
            vec![
                search(theme, None, None),
                select(theme, status_options(), None, None),
            ],
            Some(refresh(theme)),
            None,
        ),
    ));

    // With secondary slot: 3-column grid + trailing Reset all button.
    root = root.child(group(
        "With secondary slot",
        secondary,
        js_filter_toolbar(
            &FilterToolbarSpec::new()
                .with_aria_label("Project filters")
                .with_collapsible(false)
                .with_columns(3),
            theme,
            vec![
                search(theme, None, None),
                select(theme, status_options(), None, None),
                select(theme, type_options(), None, None),
            ],
            None,
            Some(js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Reset all")
                    .with_aria_label("Reset all"),
                theme,
            )),
        ),
    ));

    // Sizes: summary font + nested controls scale.
    let mut size_col = div().flex_col().gap(12.0);
    for &(key, size) in sizes {
        size_col = size_col.child(js_filter_toolbar(
            &FilterToolbarSpec::new()
                .with_summary_text(format!("Toolbar at {key}"))
                .with_aria_label(format!("Filter toolbar at {key}"))
                .with_collapsible(false)
                .with_size(size),
            theme,
            vec![
                search(theme, Some(size), None),
                select(theme, status_options(), Some(size), None),
                select(theme, type_options(), Some(size), None),
            ],
            None,
            None,
        ));
    }
    root = root.child(group("Sizes", secondary, size_col));

    // Densities: root + controls gap/padding scale (panel-internal exception).
    let mut density_col = div().flex_col().gap(12.0);
    for &(key, density) in densities {
        density_col = density_col.child(js_filter_toolbar(
            &FilterToolbarSpec::new()
                .with_summary_text(format!("Toolbar at {key}"))
                .with_aria_label(format!("Filter toolbar at {key}"))
                .with_collapsible(false)
                .with_density(density),
            theme,
            vec![
                search(theme, None, Some(density)),
                select(theme, status_options(), None, Some(density)),
                select(theme, type_options(), None, Some(density)),
            ],
            None,
            None,
        ));
    }
    root = root.child(group("Densities", secondary, density_col));

    root
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
