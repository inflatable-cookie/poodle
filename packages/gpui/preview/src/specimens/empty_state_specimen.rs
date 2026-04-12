use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{EmptyState, Eyebrow};
use poodle_specs::EyebrowSpec;
use poodle_specs::{EmptyStateSpec, EmptyStateVariant, RemediationAction};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Neutral"),
                    theme,
                ))
                .child(EmptyState::from_spec(
                    EmptyStateSpec::new("No projects yet")
                        .with_message("Create your first project to get started.")
                        .with_actions(vec![RemediationAction::new("create", "Create project")]),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Search"),
                    theme,
                ))
                .child(EmptyState::from_spec(
                    EmptyStateSpec::new("No results found")
                        .with_variant(EmptyStateVariant::Search)
                        .with_aria_label("Search results empty state")
                        .with_message("Try adjusting your search terms or clearing filters.")
                        .with_actions(vec![RemediationAction::new("clear", "Clear filters")]),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("First run"),
                    theme,
                ))
                .child(EmptyState::from_spec(
                    EmptyStateSpec::new("Welcome to your workspace")
                        .with_variant(EmptyStateVariant::FirstRun)
                        .with_message(
                            "This is where your team's components will appear once you start building.",
                        ),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Compact"),
                    theme,
                ))
                .child(div().max_w(px(420.0)).child(EmptyState::from_spec(
                    EmptyStateSpec::new("No captured emails found")
                        .with_message("Emails will appear here when sent in development mode.")
                        .with_compact(true),
                    theme,
                ))),
        )
}
