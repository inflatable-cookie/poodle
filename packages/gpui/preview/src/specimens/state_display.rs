use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{
    EmptyStateSpec, EmptyStateVariant, RemediationAction,
    MetricTileSpec,
    Toast, ToastTone, ToastStackSpec,
};
use pug_gpui_components::{EmptyState, MetricTile, ToastStack};
use pug_primitives::ButtonVariant;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // Build toast list, filtering out dismissed ones
    let toasts: Vec<Toast> = vec![
        ("t1", "Changes saved", Some("Your settings have been updated."), ToastTone::Success, None),
        ("t2", "Build failed", Some("Check the logs for more details."), ToastTone::Danger, Some("View logs")),
        ("t3", "New version available", Some("v2.4.0 is ready to install."), ToastTone::Info, Some("Update now")),
    ].into_iter()
    .filter(|(id, _, _, _, _)| !state.specimens.is_on(&format!("toast-dismissed-{}", id)))
    .map(|(id, title, msg, tone, action)| {
        let mut t = Toast::new(id, title).with_tone(tone);
        if let Some(m) = msg { t = t.with_message(m); }
        if let Some(a) = action { t = t.with_action_label(a); }
        t
    })
    .collect();

    let last_action = state.specimens.text.get("toast-last-action").cloned();

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
        .child(section_label("TOAST STACK (INTERACTIVE)", text_secondary))
        .child(
            div().relative().h(px(220.0)).w_full()
                .overflow_hidden()
                .border_1()
                .border_color(color_to_hsla(theme.resolve_color("semantic.color.border.subtle")))
                .rounded(px(6.0))
                .child(
                    ToastStack::from_spec(
                        ToastStackSpec::new()
                            .with_toasts(toasts),
                        theme,
                    )
                    .on_dismiss(cx.listener(|this, id: &str, _w, cx| {
                        this.state.specimens.toggles.insert(
                            format!("toast-dismissed-{}", id),
                            true,
                        );
                        cx.notify();
                    }))
                    .on_action(cx.listener(|this, id: &str, _w, cx| {
                        this.state.specimens.text.insert(
                            "toast-last-action".to_string(),
                            format!("Action on toast: {}", id),
                        );
                        cx.notify();
                    }))
                )
        )
        .child(
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child(last_action.unwrap_or_else(|| "Click dismiss (x) or action buttons on toasts".to_string()))
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
