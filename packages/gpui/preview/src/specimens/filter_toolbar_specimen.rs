use crate::app_state::AppState;
use crate::node_compat::{Button, Eyebrow, FilterToolbar, IconButton, Select, TextInput};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::FilterToolbarSpec;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ChoiceOption, EyebrowSpec, IconButtonSpec, SelectSpec, TextInputSpec,
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
    Select::from_spec(
        SelectSpec::new(options).with_default_value("all"),
        theme,
        id.to_string(),
    )
    .with_id(id.to_string())
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
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
                    .with_child(search_input(theme, "filter-search").into_slot())
                    .with_child(select_input(theme, "filter-status", status_options()).into_slot())
                    .with_child(select_input(theme, "filter-type", type_options()).into_slot())
                    .with_child(select_input(theme, "filter-owner", owner_options()).into_slot()),
                ),
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
                        .with_id("ft-refresh-1")
                        .into_slot(),
                    )
                    .with_child(search_input(theme, "col-search").into_slot())
                    .with_child(select_input(theme, "col-status", status_options()).into_slot())
                    .with_child(select_input(theme, "col-type", type_options()).into_slot()),
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
                        .with_id("ft-refresh-2")
                        .into_slot(),
                    )
                    .with_child(search_input(theme, "col2-search").into_slot())
                    .with_child(select_input(theme, "col2-status", status_options()).into_slot()),
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
                    .with_child(search_input(theme, "proj-search").into_slot())
                    .with_child(select_input(theme, "proj-status", status_options()).into_slot())
                    .with_child(select_input(theme, "proj-type", type_options()).into_slot())
                    .with_secondary(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Reset all"),
                            theme,
                        )
                        .with_id("proj-reset")
                        .into_slot(),
                    ),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "filter-toolbar",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                FilterToolbar::from_spec(
                    FilterToolbarSpec::new()
                        .with_summary_text("Filters")
                        .with_aria_label("Filter toolbar")
                        .with_collapsible(false)
                        .with_size(size),
                    theme,
                )
                .with_child(
                    search_input(theme, &format!("size-search-{}", size_key(size))).into_slot(),
                )
                .with_child(
                    select_input(
                        theme,
                        &format!("size-status-{}", size_key(size)),
                        status_options(),
                    )
                    .into_slot(),
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                FilterToolbar::from_spec(
                    FilterToolbarSpec::new()
                        .with_summary_text("Filters")
                        .with_aria_label("Filter toolbar")
                        .with_collapsible(false)
                        .with_density(density),
                    theme,
                )
                .with_child(
                    search_input(theme, &format!("density-search-{}", density_key(density)))
                        .into_slot(),
                )
                .with_child(
                    select_input(
                        theme,
                        &format!("density-status-{}", density_key(density)),
                        status_options(),
                    )
                    .into_slot(),
                )
                .into_any_element()
            }),
    )
}
