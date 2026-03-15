use gpui::*;
use pug_gpui_composites::{EmptyStateSpec, EmptyStateVariant, RemediationAction};
use pug_gpui_components::PugEmptyState;
use pug_gpui_primitives::ButtonVariant;
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let empty_neutral = EmptyStateSpec::new("No items found")
        .with_message("Try adjusting your filters or adding new items.")
        .with_actions(vec![
            RemediationAction::new("add", "Add Item").with_variant(ButtonVariant::Primary),
        ]);

    let empty_first_run = EmptyStateSpec::new("Welcome to your workspace")
        .with_variant(EmptyStateVariant::FirstRun)
        .with_message("Get started by creating your first project.")
        .with_actions(vec![
            RemediationAction::new("create", "Create Project").with_variant(ButtonVariant::Primary),
            RemediationAction::new("import", "Import"),
        ]);

    let empty_search = EmptyStateSpec::new("No results")
        .with_variant(EmptyStateVariant::Search)
        .with_message("No items match your search criteria.");

    div().flex().flex_col().gap(px(12.0))
        .child(PugEmptyState::new(empty_neutral, theme))
        .child(PugEmptyState::new(empty_first_run, theme))
        .child(PugEmptyState::new(empty_search, theme))
}
