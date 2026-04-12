use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Button, ConfirmAction, Eyebrow};
use poodle_specs::ConfirmActionSpec;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec, StatusTone};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
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
                        .on_click(cx.listener(
                            |this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("confirm-action-danger-open".to_string(), true);
                                cx.notify();
                            },
                        )),
                    )
                    .on_confirm(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-danger-open".to_string(), false);
                        this.state.specimens.text.insert(
                            "confirm-action-last".to_string(),
                            "Record deleted".to_string(),
                        );
                        cx.notify();
                    }))
                    .on_cancel(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-danger-open".to_string(), false);
                        cx.notify();
                    })),
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
                        .on_click(cx.listener(
                            |this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("confirm-action-warning-open".to_string(), true);
                                cx.notify();
                            },
                        )),
                    )
                    .on_confirm(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-warning-open".to_string(), false);
                        this.state.specimens.text.insert(
                            "confirm-action-last".to_string(),
                            "Project archived".to_string(),
                        );
                        cx.notify();
                    }))
                    .on_cancel(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-warning-open".to_string(), false);
                        cx.notify();
                    })),
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
                        .on_click(cx.listener(
                            |this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("confirm-action-ghost-open".to_string(), true);
                                cx.notify();
                            },
                        )),
                    )
                    .on_confirm(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-ghost-open".to_string(), false);
                        this.state.specimens.text.insert(
                            "confirm-action-last".to_string(),
                            "Filters cleared".to_string(),
                        );
                        cx.notify();
                    }))
                    .on_cancel(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-ghost-open".to_string(), false);
                        cx.notify();
                    })),
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
                        .on_click(cx.listener(
                            |this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("confirm-action-body-open".to_string(), true);
                                cx.notify();
                            },
                        )),
                    )
                    .with_content({
                        let code_bg = {
                            let mut h = color_to_hsla(panel_bg);
                            h.a *= 0.9;
                            h
                        };
                        div()
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .bg(code_bg)
                            .text_size(px(13.0))
                            .font_family("Menlo")
                            .child("pk_live_abc123...xyz789")
                    })
                    .on_confirm(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-body-open".to_string(), false);
                        this.state
                            .specimens
                            .text
                            .insert("confirm-action-last".to_string(), "Key revoked".to_string());
                        cx.notify();
                    }))
                    .on_cancel(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("confirm-action-body-open".to_string(), false);
                        cx.notify();
                    })),
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
