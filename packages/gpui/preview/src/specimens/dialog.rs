use gpui::*;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant, DialogSpec};
use pug_gpui_components::{PugButton, PugDialog};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let confirmed = state.specimens.is_on("dialog-confirmed");

    let spec = DialogSpec::new()
        .with_title("Confirm Action")
        .with_description(
            if confirmed { "Action confirmed!" } else { "Are you sure you want to proceed?" }
        );

    let actions = div().flex().gap(px(8.0))
        .child(
            PugButton::new(
                ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Cancel"),
                theme,
            )
            .with_id("dialog-cancel")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.toggles.insert("dialog-confirmed".to_string(), false);
                cx.notify();
            }))
        )
        .child(
            PugButton::new(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label(if confirmed { "Done ✓" } else { "Confirm" }),
                theme,
            )
            .with_id("dialog-confirm")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.toggle("dialog-confirmed");
                cx.notify();
            }))
        );

    div().child(
        PugDialog::new(spec, theme)
            .with_actions(actions)
    )
}
