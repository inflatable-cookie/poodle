use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{IconButtonSpec, ButtonVariant, TextInputSpec};
use pug_gpui_components::{PugIconButton, PugTextInput};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let value = state.specimens.count("number-entry") as i32 + 42;

    div().flex().items_center().gap(px(2.0))
        .child(
            PugIconButton::new(
                IconButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_icon("−"),
                theme,
            )
            .with_id("num-minus")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                let val = this.state.specimens.counters.entry("number-entry".to_string()).or_insert(0);
                if *val > 0 { *val -= 1; }
                cx.notify();
            }))
        )
        .child(
            PugTextInput::new(
                TextInputSpec::new()
                    .with_value(format!("{}", value))
                    .with_disabled(true),
                theme,
            )
        )
        .child(
            PugIconButton::new(
                IconButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_icon("+"),
                theme,
            )
            .with_id("num-plus")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.increment("number-entry");
                cx.notify();
            }))
        )
}
