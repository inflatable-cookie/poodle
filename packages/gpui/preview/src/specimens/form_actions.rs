use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec, FormActionAlign, FormActionsSpec};
use poodle_gpui_components::{Button, Eyebrow, FormActions};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state.specimens.text.get("form-action-last")
        .cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0))
        // --- End-aligned (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("End-aligned (default)"), theme))
                .child(
                    FormActions::from_spec(FormActionsSpec::new(), theme)
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Cancel"),
                                theme,
                            )
                            .with_id("fa-cancel-end")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("form-action-last".to_string(), "Cancel".to_string());
                                cx.notify();
                            }))
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save changes"),
                                theme,
                            )
                            .with_id("fa-save-end")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("form-action-last".to_string(), "Save changes".to_string());
                                cx.notify();
                            }))
                        )
                )
        )
        // --- Start-aligned ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Start-aligned"), theme))
                .child(
                    FormActions::from_spec(
                        FormActionsSpec::new().with_align(FormActionAlign::Start),
                        theme,
                    )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Back"),
                                theme,
                            )
                            .with_id("fa-back-start")
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Continue"),
                                theme,
                            )
                            .with_id("fa-continue-start")
                        )
                )
        )
        // --- Space between ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Space between"), theme))
                .child(
                    FormActions::from_spec(
                        FormActionsSpec::new().with_align(FormActionAlign::Between),
                        theme,
                    )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_tone(ButtonTone::Danger)
                                    .with_label("Delete"),
                                theme,
                            )
                            .with_id("fa-delete-between")
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save"),
                                theme,
                            )
                            .with_id("fa-save-between")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("form-action-last".to_string(), "Save".to_string());
                                cx.notify();
                            }))
                        )
                )
        )
        // --- Responsive danger actions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Responsive danger actions"), theme))
                .child(
                    FormActions::from_spec(FormActionsSpec::new(), theme)
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_tone(ButtonTone::Danger)
                                    .with_label("Discard draft"),
                                theme,
                            )
                            .with_id("fa-discard-danger")
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_label("Back"),
                                theme,
                            )
                            .with_id("fa-back-danger")
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save changes"),
                                theme,
                            )
                            .with_id("fa-save-danger")
                        )
                )
        )
        // --- Last action feedback ---
        .when(!last_action.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_action))
            )
        })
}
