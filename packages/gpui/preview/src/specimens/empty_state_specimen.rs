use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{EmptyState, Eyebrow};
use poodle_specs::{ButtonVariant, EyebrowSpec};
use poodle_specs::{EmptyStateSpec, EmptyStateVariant, RemediationAction};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Neutral ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Neutral"), theme))
                .child(
                    EmptyState::from_spec(
                        EmptyStateSpec::new("No projects yet")
                            .with_message("Create your first project to get started.")
                            .with_actions(vec![
                                RemediationAction::new("create", "Create project")
                                    .with_variant(ButtonVariant::Primary),
                            ]),
                        theme,
                    )
                )
        )
        // --- Search ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Search"), theme))
                .child(
                    EmptyState::from_spec(
                        EmptyStateSpec::new("No results found")
                            .with_variant(EmptyStateVariant::Search)
                            .with_message("Try adjusting your search terms or clearing filters.")
                            .with_actions(vec![
                                RemediationAction::new("clear", "Clear filters"),
                            ]),
                        theme,
                    )
                )
        )
        // --- First run ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("First run"), theme))
                .child(
                    EmptyState::from_spec(
                        EmptyStateSpec::new("Welcome to your workspace")
                            .with_variant(EmptyStateVariant::FirstRun)
                            .with_message("This is where your team's components will appear once you start building."),
                        theme,
                    )
                )
        )
        // --- Compact (embedded in a list) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Compact"), theme))
                .child(
                    EmptyState::from_spec(
                        EmptyStateSpec::new("No comments yet")
                            .with_variant(EmptyStateVariant::Neutral)
                            .with_message("Be the first to add one.")
                            .with_compact(true),
                        theme,
                    )
                )
        )
}
