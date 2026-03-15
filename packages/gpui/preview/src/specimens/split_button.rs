use gpui::*;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant, IconButtonSpec, SeparatorSpec, SeparatorOrientation, RuleTone};
use pug_gpui_components::{PugButton, PugIconButton, PugSeparator};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().flex().items_center()
        .child(
            PugButton::new(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Save"),
                theme,
            )
            .with_id("split-btn-main")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.increment("split-btn");
                cx.notify();
            }))
        )
        .child(
            PugSeparator::new(
                SeparatorSpec::new()
                    .with_orientation(SeparatorOrientation::Vertical)
                    .with_tone(RuleTone::Subtle),
                theme,
            )
        )
        .child(
            PugIconButton::new(
                IconButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_icon("▾"),
                theme,
            )
            .with_id("split-btn-drop")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.toggle("split-btn-open");
                cx.notify();
            }))
        )
}
