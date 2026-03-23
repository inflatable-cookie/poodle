use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{
    EmptyStateSpec, EmptyStateVariant, RemediationAction,
    MetricTileSpec,
    Toast, ToastTone, ToastStackSpec,
};
use pug_gpui_components::{EmptyState, MetricTile, ToastStack};
use pug_primitives::ButtonVariant;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // ── MetricTile ──────────────────────────────────────────
        .child(section_label("METRIC TILES", text_secondary))
        .child(
            div().flex().gap(px(12.0)).flex_wrap()
                .child(MetricTile::from_spec(
                    MetricTileSpec::new("Total Users", "12,847"),
                    theme,
                ))
                .child(MetricTile::from_spec(
                    MetricTileSpec::new("Active Sessions", "342"),
                    theme,
                ))
                .child(MetricTile::from_spec(
                    MetricTileSpec::new("Conversion Rate", "3.2%"),
                    theme,
                ))
                .child(MetricTile::from_spec(
                    MetricTileSpec::new("Revenue", "$48,290"),
                    theme,
                ))
        )

        // ── EmptyState: Neutral ─────────────────────────────────
        .child(section_label("EMPTY STATE: NEUTRAL", text_secondary))
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

        // ── EmptyState: Search ──────────────────────────────────
        .child(section_label("EMPTY STATE: SEARCH", text_secondary))
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

        // ── EmptyState: First Run ───────────────────────────────
        .child(section_label("EMPTY STATE: FIRST RUN", text_secondary))
        .child(
            EmptyState::from_spec(
                EmptyStateSpec::new("Welcome to your workspace")
                    .with_variant(EmptyStateVariant::FirstRun)
                    .with_message("This is where your team's components will appear once you start building."),
                theme,
            )
        )

        // ── ToastStack ──────────────────────────────────────────
        .child(section_label("TOAST STACK (INLINE DEMO)", text_secondary))
        .child(
            div().relative().h(px(200.0)).w_full()
                .overflow_hidden()
                .border_1()
                .border_color(color_to_hsla(theme.resolve_color("semantic.color.border.subtle")))
                .rounded(px(6.0))
                .child(
                    ToastStack::from_spec(
                        ToastStackSpec::new()
                            .with_toasts(vec![
                                Toast::new("t1", "Changes saved")
                                    .with_message("Your settings have been updated.")
                                    .with_tone(ToastTone::Success),
                                Toast::new("t2", "Build failed")
                                    .with_message("Check the logs for more details.")
                                    .with_tone(ToastTone::Danger)
                                    .with_action_label("View logs"),
                                Toast::new("t3", "New version available")
                                    .with_message("v2.4.0 is ready to install.")
                                    .with_tone(ToastTone::Info)
                                    .with_action_label("Update now"),
                            ]),
                        theme,
                    )
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
