use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonSpec, ButtonTone, ButtonVariant, FormActionAlign, FormActionsSpec};
use pug_gpui_components::{Button, FormActions};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let last_action = state.specimens.text.get("form-action-last")
        .cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0))
        // --- End-aligned (default) ---
        .child(section_label("END-ALIGNED (DEFAULT)", text_secondary))
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
        // --- Start-aligned ---
        .child(section_label("START-ALIGNED", text_secondary))
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
        // --- Space between ---
        .child(section_label("SPACE BETWEEN", text_secondary))
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
        // --- Last action feedback ---
        .when(!last_action.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_action))
            )
        })
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
