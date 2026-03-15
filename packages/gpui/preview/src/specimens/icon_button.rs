use gpui::*;
use pug_gpui_primitives::{IconButtonSpec, ButtonVariant};
use pug_gpui_components::PugIconButton;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().flex().gap(px(6.0))
        .child(
            PugIconButton::new(
                IconButtonSpec::new().with_variant(ButtonVariant::Secondary).with_icon("+"),
                theme,
            )
            .with_id("plus")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.increment("icon-btn-clicks");
                cx.notify();
            }))
        )
        .child(
            PugIconButton::new(
                IconButtonSpec::new().with_variant(ButtonVariant::Primary).with_icon("×"),
                theme,
            )
            .with_id("close")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.increment("icon-btn-clicks");
                cx.notify();
            }))
        )
        .child(
            PugIconButton::new(
                IconButtonSpec::new().with_variant(ButtonVariant::Ghost).with_icon("⋯"),
                theme,
            )
            .with_id("more")
            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.increment("icon-btn-clicks");
                cx.notify();
            }))
        )
        .child(
            PugIconButton::new(
                IconButtonSpec::new().with_variant(ButtonVariant::Danger).with_icon("🗑").with_disabled(true),
                theme,
            )
            .with_id("del-disabled")
        )
}
