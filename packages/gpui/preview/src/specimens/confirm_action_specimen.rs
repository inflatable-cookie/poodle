use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, ConfirmAction, Eyebrow};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_node::{FontFamily, LayoutDirection, Node};
use poodle_specs::ConfirmActionSpec;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec, StatusTone};
use std::sync::Arc;

fn open_click(state: &AppState, key: &'static str) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.to_string(),
            value: true,
        });
    })
}

fn finish_click(
    state: &AppState,
    open_key: &'static str,
    message: Option<&'static str>,
) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetToggle {
            key: open_key.to_string(),
            value: false,
        });
        if let Some(message) = message {
            events.push(NodeSpecimenEvent::SetText {
                key: "confirm-action-last".to_string(),
                value: message.to_string(),
            });
        }
    })
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let panel_bg = theme.resolve_color("color.background.panel");

    let danger_open = state.specimens.is_on("confirm-action-danger-open");
    let warning_open = state.specimens.is_on("confirm-action-warning-open");
    let ghost_open = state.specimens.is_on("confirm-action-ghost-open");
    let body_open = state.specimens.is_on("confirm-action-body-open");
    let last_action = state
        .specimens
        .text
        .get("confirm-action-last")
        .cloned()
        .unwrap_or_default();

    let text_secondary = theme.resolve_color("color.text.secondary");

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default trigger (danger) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default trigger (danger)"),
                    theme,
                ))
                .child(
                    ConfirmAction::from_spec(
                        ConfirmActionSpec::new(
                            "Delete this record?",
                            "This record will be permanently removed.",
                            "Delete",
                            "Cancel",
                        )
                        .with_tone(StatusTone::Danger)
                        .with_open(danger_open),
                        theme,
                    )
                    .with_trigger(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Delete record"),
                            theme,
                        )
                        .with_id("confirm-danger-trigger")
                        .on_click(open_click(state, "confirm-action-danger-open")),
                    )
                    .on_confirm(finish_click(
                        state,
                        "confirm-action-danger-open",
                        Some("Record deleted"),
                    ))
                    .on_cancel(finish_click(
                        state,
                        "confirm-action-danger-open",
                        None,
                    )),
                ),
        )
        // --- Warning tone ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Warning tone"),
                    theme,
                ))
                .child(
                    ConfirmAction::from_spec(
                        ConfirmActionSpec::new(
                            "Archive this project?",
                            "The project will be moved to the archive and can be restored later.",
                            "Archive",
                            "Cancel",
                        )
                        .with_tone(StatusTone::Warning)
                        .with_open(warning_open),
                        theme,
                    )
                    .with_trigger(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Archive project"),
                            theme,
                        )
                        .with_id("confirm-warning-trigger")
                        .on_click(open_click(state, "confirm-action-warning-open")),
                    )
                    .on_confirm(finish_click(
                        state,
                        "confirm-action-warning-open",
                        Some("Project archived"),
                    ))
                    .on_cancel(finish_click(
                        state,
                        "confirm-action-warning-open",
                        None,
                    )),
                ),
        )
        // --- Custom trigger slot (ghost button) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom trigger slot"),
                    theme,
                ))
                .child(
                    ConfirmAction::from_spec(
                        ConfirmActionSpec::new(
                            "Remove all filters?",
                            "This will clear all active filters and show all items.",
                            "Clear all",
                            "Cancel",
                        )
                        .with_tone(StatusTone::Warning)
                        .with_open(ghost_open),
                        theme,
                    )
                    .with_trigger(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Clear filters"),
                            theme,
                        )
                        .with_id("confirm-ghost-trigger")
                        .on_click(open_click(state, "confirm-action-ghost-open")),
                    )
                    .on_confirm(finish_click(
                        state,
                        "confirm-action-ghost-open",
                        Some("Filters cleared"),
                    ))
                    .on_cancel(finish_click(
                        state,
                        "confirm-action-ghost-open",
                        None,
                    )),
                ),
        )
        // --- With body content ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With body content"),
                    theme,
                ))
                .child(
                    ConfirmAction::from_spec(
                        ConfirmActionSpec::new(
                            "Revoke API key?",
                            "This key will immediately stop working.",
                            "Revoke",
                            "Cancel",
                        )
                        .with_tone(StatusTone::Danger)
                        .with_open(body_open),
                        theme,
                    )
                    .with_trigger(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Revoke API key"),
                            theme,
                        )
                        .with_id("confirm-body-trigger")
                        .on_click(open_click(state, "confirm-action-body-open")),
                    )
                    .with_content({
                        let mut code = Node::text("pk_live_abc123...xyz789");
                        code.style.descriptor.layout.direction = LayoutDirection::Row;
                        code.style.descriptor.layout.spacing.padding.left = 12.0;
                        code.style.descriptor.layout.spacing.padding.right = 12.0;
                        code.style.descriptor.layout.spacing.padding.top = 8.0;
                        code.style.descriptor.layout.spacing.padding.bottom = 8.0;
                        code.style.descriptor.corner_radii.top_left = 6.0;
                        code.style.descriptor.corner_radii.top_right = 6.0;
                        code.style.descriptor.corner_radii.bottom_left = 6.0;
                        code.style.descriptor.corner_radii.bottom_right = 6.0;
                        code.style.descriptor.background = Some(poodle_tokens::typed::ColorValue(
                            panel_bg.0,
                            panel_bg.1,
                            panel_bg.2,
                            panel_bg.3 * 0.9,
                        ));
                        code.style.text_size = Some(13.0);
                        code.style.font_family = Some(FontFamily::Mono);
                        code
                    })
                    .on_confirm(finish_click(
                        state,
                        "confirm-action-body-open",
                        Some("Key revoked"),
                    ))
                    .on_cancel(finish_click(
                        state,
                        "confirm-action-body-open",
                        None,
                    )),
                ),
        )
        // --- Last action (only when a confirm has fired) ---
        .child({
            if last_action.is_empty() {
                div()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(Eyebrow::from_spec(
                        EyebrowSpec::new().with_content("Last action"),
                        theme,
                    ))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(color_to_hsla(text_secondary))
                            .child(last_action),
                    )
            }
        })
}
