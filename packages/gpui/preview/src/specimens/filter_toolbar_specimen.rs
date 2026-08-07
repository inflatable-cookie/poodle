use crate::node_compat::{Button, Eyebrow, FilterToolbar, IconButton, Select, TextInput};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::FilterToolbarSpec;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ChoiceOption, ControlDensity, ControlSize, EyebrowSpec,
    IconButtonSpec, SelectSpec, TextInputSpec,
};

/// Static filter option sets reused across specimen sections.
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

fn search_input(theme: &GpuiThemeProvider, id: &str) -> TextInput {
    TextInput::from_spec(
        TextInputSpec::new()
            .with_id(id)
            .with_input_type("search")
            .with_placeholder("Search…"),
        theme,
    )
}

fn select_input(theme: &GpuiThemeProvider, id: &str, options: Vec<ChoiceOption>) -> Select {
    Select::from_spec(SelectSpec::new(options).with_default_value("all"), theme)
        .with_id(id.to_string())
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Responsive grid layout ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Responsive grid layout"),
                    theme,
                ))
                .child(
                    FilterToolbar::from_spec(
                        FilterToolbarSpec::new()
                            .with_summary_text("Showing 24 of 156 items")
                            .with_aria_label("Item filters")
                            .with_collapsible(false),
                        theme,
                    )
                    .with_child(search_input(theme, "filter-search"))
                    .with_child(select_input(theme, "filter-status", status_options()))
                    .with_child(select_input(theme, "filter-type", type_options()))
                    .with_child(select_input(
                        theme,
                        "filter-owner",
                        owner_options(),
                    )),
                ),
        )
        // --- Sizes ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Sizes"),
                    theme,
                ))
                .child({
                    let sizes: &[(&str, ControlSize)] = &[
                        ("xs", ControlSize::Xs),
                        ("sm", ControlSize::Sm),
                        ("md", ControlSize::Md),
                        ("lg", ControlSize::Lg),
                        ("xl", ControlSize::Xl),
                    ];
                    let mut col = div().flex().flex_col().gap(px(12.0));
                    for &(key, size) in sizes {
                        col = col.child(
                            FilterToolbar::from_spec(
                                FilterToolbarSpec::new()
                                    .with_summary_text(format!("Toolbar at {key}"))
                                    .with_aria_label(format!("Filter toolbar at {key}"))
                                    .with_collapsible(false)
                                    .with_size(size),
                                theme,
                            )
                            .with_child(search_input(theme, &format!("size-search-{key}")))
                            .with_child(select_input(
                                theme,
                                &format!("size-status-{key}"),
                                status_options(),
                            )),
                        );
                    }
                    col
                }),
        )
        // --- Densities ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Densities"),
                    theme,
                ))
                .child({
                    let densities: &[(&str, ControlDensity)] = &[
                        ("compact", ControlDensity::Compact),
                        ("default", ControlDensity::Default),
                        ("comfortable", ControlDensity::Comfortable),
                    ];
                    let mut col = div().flex().flex_col().gap(px(12.0));
                    for &(key, density) in densities {
                        col = col.child(
                            FilterToolbar::from_spec(
                                FilterToolbarSpec::new()
                                    .with_summary_text(format!("Toolbar at {key}"))
                                    .with_aria_label(format!("Filter toolbar at {key}"))
                                    .with_collapsible(false)
                                    .with_density(density),
                                theme,
                            )
                            .with_child(search_input(theme, &format!("density-search-{key}")))
                            .with_child(select_input(
                                theme,
                                &format!("density-status-{key}"),
                                status_options(),
                            ))
                            .with_child(select_input(
                                theme,
                                &format!("density-type-{key}"),
                                type_options(),
                            )),
                        );
                    }
                    col
                }),
        )
        // --- Collapsible with actions (expanded) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Collapsible with actions"),
                    theme,
                ))
                .child(
                    FilterToolbar::from_spec(
                        FilterToolbarSpec::new()
                            .with_summary_text("Showing 24 of 156 items")
                            .with_aria_label("Collapsible filters")
                            .with_collapsible(true)
                            .with_collapsed(false),
                        theme,
                    )
                    .with_actions(
                        IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon("refresh-cw")
                                .with_aria_label("Refresh"),
                            theme,
                        )
                        .with_id("ft-refresh-1"),
                    )
                    .with_child(search_input(theme, "col-search"))
                    .with_child(select_input(theme, "col-status", status_options()))
                    .with_child(select_input(
                        theme,
                        "col-type",
                        type_options(),
                    )),
                ),
        )
        // --- Explicit collapsed state ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Explicit collapsed state"),
                    theme,
                ))
                .child(
                    FilterToolbar::from_spec(
                        FilterToolbarSpec::new()
                            .with_summary_text("3 filters active")
                            .with_aria_label("Collapsed filters")
                            .with_collapsible(true)
                            .with_collapsed(true),
                        theme,
                    )
                    .with_actions(
                        IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon("refresh-cw")
                                .with_aria_label("Refresh"),
                            theme,
                        )
                        .with_id("ft-refresh-2"),
                    )
                    .with_child(search_input(theme, "col2-search"))
                    .with_child(select_input(
                        theme,
                        "col2-status",
                        status_options(),
                    )),
                ),
        )
        // --- With secondary slot ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With secondary slot"),
                    theme,
                ))
                .child(
                    FilterToolbar::from_spec(
                        FilterToolbarSpec::new()
                            .with_aria_label("Project filters")
                            .with_collapsible(false)
                            .with_columns(3),
                        theme,
                    )
                    .with_child(search_input(theme, "proj-search"))
                    .with_child(select_input(theme, "proj-status", status_options()))
                    .with_child(select_input(theme, "proj-type", type_options()))
                    .with_secondary(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Reset all"),
                            theme,
                        )
                        .with_id("proj-reset"),
                    ),
                ),
        )
}
