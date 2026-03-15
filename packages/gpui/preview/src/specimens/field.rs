use gpui::*;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant, TextInputSpec, FieldSpec, FormActionsSpec};
use pug_gpui_components::{PugButton, PugTextInput, PugField, PugFormActions};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let submitted = state.specimens.is_on("form-submitted");

    let field_spec = FieldSpec::new("email-field", "Email")
        .with_description("We'll never share your email.");

    div().flex().flex_col().gap(px(8.0))
        .child(
            PugField::new(field_spec, theme)
                .with_control(
                    PugTextInput::new(
                        TextInputSpec::new().with_placeholder("you@example.com"),
                        theme,
                    )
                )
        )
        .child(
            PugFormActions::new(FormActionsSpec::new(), theme)
                .with_action(
                    PugButton::new(
                        ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Cancel"),
                        theme,
                    )
                    .with_id("form-cancel")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        let v = this.state.specimens.toggles.entry("form-submitted".to_string()).or_insert(false);
                        *v = false;
                        cx.notify();
                    }))
                )
                .with_action(
                    PugButton::new(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label(if submitted { "Submitted ✓" } else { "Submit" }),
                        theme,
                    )
                    .with_id("form-submit")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.toggles.insert("form-submitted".to_string(), true);
                        cx.notify();
                    }))
                )
        )
}
