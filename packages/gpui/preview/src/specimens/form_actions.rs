use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonSpec, ButtonTone, ButtonVariant, FormActionAlign, FormActionsSpec};
use pug_gpui_components::{Button, FormActions};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

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
                )
                .with_action(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Save changes"),
                        theme,
                    )
                    .with_id("fa-save-end")
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
