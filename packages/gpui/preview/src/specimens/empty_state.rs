use crate::app_state::AppState;
use crate::node_compat::{EmptyState, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EmptyStateSpec, EmptyStateVariant, EyebrowSpec};

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Neutral",
            theme,
            EmptyState::from_spec(
                EmptyStateSpec::new("No projects yet")
                    .with_message("Create your first project to get started."),
                theme,
            ),
        ))
        .child(group(
            "Search",
            theme,
            EmptyState::from_spec(
                EmptyStateSpec::new("No results found")
                    .with_variant(EmptyStateVariant::Search)
                    .with_message("Try adjusting your search terms or clearing filters."),
                theme,
            ),
        ))
        .child(group(
            "First run",
            theme,
            EmptyState::from_spec(
                EmptyStateSpec::new("Welcome to your workspace")
                    .with_variant(EmptyStateVariant::FirstRun)
                    .with_message(
                        "This is where your team's components will appear once you start building.",
                    ),
                theme,
            ),
        ))
        .child(group(
            "Compact",
            theme,
            EmptyState::from_spec(
                EmptyStateSpec::new("No captured emails found")
                    .with_compact(true)
                    .with_message("Emails will appear here when sent in development mode."),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "empty-state",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|_size, theme: &GpuiThemeProvider| {
                EmptyState::from_spec(
                    EmptyStateSpec::new("No projects yet")
                        .with_message("Create your first project to get started."),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                EmptyState::from_spec(
                    EmptyStateSpec::new("No projects yet")
                        .with_message("Create your first project to get started.")
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
