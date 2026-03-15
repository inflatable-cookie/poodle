use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant};
use pug_gpui_components::PugButton;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let click_count = state.specimens.count("btn-clicks");

    div().flex().flex_col().gap(px(12.0))
        .child(
            div().flex().gap(px(8.0)).flex_wrap()
                .child(
                    PugButton::new(
                        ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Primary"),
                        theme,
                    )
                    .with_id("primary")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.increment("btn-clicks");
                        cx.notify();
                    }))
                )
                .child(
                    PugButton::new(
                        ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Secondary"),
                        theme,
                    )
                    .with_id("secondary")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.increment("btn-clicks");
                        cx.notify();
                    }))
                )
                .child(
                    PugButton::new(
                        ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Ghost"),
                        theme,
                    )
                    .with_id("ghost")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.increment("btn-clicks");
                        cx.notify();
                    }))
                )
                .child(
                    PugButton::new(
                        ButtonSpec::new().with_variant(ButtonVariant::Danger).with_label("Danger"),
                        theme,
                    )
                    .with_id("danger")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.increment("btn-clicks");
                        cx.notify();
                    }))
                )
                .child(
                    PugButton::new(
                        ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Disabled").with_disabled(true),
                        theme,
                    )
                    .with_id("disabled")
                )
                .child(
                    PugButton::new(
                        ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Loading").with_loading(true),
                        theme,
                    )
                    .with_id("loading")
                )
        )
        .child(
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child(format!("Clicks: {}", click_count))
        )
}
