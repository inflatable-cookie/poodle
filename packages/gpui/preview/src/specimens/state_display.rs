use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_composites::{EmptyStateSpec, EmptyStateVariant, RemediationAction};
use pug_gpui_components::PugEmptyState;
use pug_gpui_primitives::ButtonVariant;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Neutral ---
    let neutral = EmptyStateSpec::new("No projects yet")
        .with_message("Create your first project to get started.")
        .with_actions(vec![
            RemediationAction::new("create", "Create project").with_variant(ButtonVariant::Primary),
        ]);

    // --- Search ---
    let search = EmptyStateSpec::new("No results found")
        .with_variant(EmptyStateVariant::Search)
        .with_message("Try adjusting your search terms or clearing filters.")
        .with_actions(vec![
            RemediationAction::new("clear", "Clear filters"),
        ]);

    // --- First run ---
    let first_run = EmptyStateSpec::new("Welcome to your workspace")
        .with_variant(EmptyStateVariant::FirstRun)
        .with_message("This is where your team's components will appear once you start building.");

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("NEUTRAL", text_secondary))
        .child(PugEmptyState::new(neutral, theme))
        .child(section_label("SEARCH", text_secondary))
        .child(PugEmptyState::new(search, theme))
        .child(section_label("FIRST RUN", text_secondary))
        .child(PugEmptyState::new(first_run, theme))
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
