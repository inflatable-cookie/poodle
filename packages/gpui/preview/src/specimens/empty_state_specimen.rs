use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{EmptyState, Eyebrow};
use poodle_specs::{ButtonVariant, EyebrowSpec};
use poodle_specs::{EmptyStateSpec, EmptyStateVariant, RemediationAction};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
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
                                RemediationAction::new("import", "Import existing"),
                            ]),
                        theme,
                    )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Search"), theme))
                .child(
                    EmptyState::from_spec(
                        EmptyStateSpec::new("No results found")
                            .with_variant(EmptyStateVariant::Search)
                            .with_aria_label("Search results empty state")
                            .with_message("Try adjusting your search terms or clearing filters.")
                            .with_actions(vec![
                                RemediationAction::new("clear", "Clear filters"),
                                RemediationAction::new("browse", "Browse all"),
                            ]),
                        theme,
                    )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("First run"), theme))
                .child(
                    EmptyState::from_spec(
                        EmptyStateSpec::new("Welcome to your workspace")
                            .with_variant(EmptyStateVariant::FirstRun)
                            .with_message("This is where your team's components will appear once you start building.")
                            .with_actions(vec![
                                RemediationAction::new("tour", "Take the tour")
                                    .with_variant(ButtonVariant::Primary),
                                RemediationAction::new("sample", "Load sample data"),
                            ]),
                        theme,
                    )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Compact"), theme))
                .child(
                    div().max_w(px(420.0)).child(
                        EmptyState::from_spec(
                            EmptyStateSpec::new("No comments yet")
                                .with_variant(EmptyStateVariant::Neutral)
                                .with_message("Be the first to add one.")
                                .with_compact(true),
                            theme,
                        )
                    )
                )
        )
}
