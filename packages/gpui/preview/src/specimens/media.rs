use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant, ProgressSpec};
use pug_gpui_components::{PugButton, PugProgress};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let border = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let playing = state.specimens.is_on("media-playing");

    div().flex().gap(px(8.0))
        .child(
            div().id("media-play")
                .w(px(160.0)).h(px(80.0)).rounded(px(6.0))
                .border_1().border_color(color_to_hsla(border))
                .flex().items_center().justify_center()
                .cursor_pointer()
                .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)))
                .child(
                    PugButton::new(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label(if playing { "⏸" } else { "▶" }),
                        theme,
                    )
                    .with_id("media-btn")
                )
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggle("media-playing");
                    cx.notify();
                }))
        )
        .child(
            div().flex_1().flex().flex_col().gap(px(4.0))
                .child(div().text_sm().child(if playing { "Now Playing..." } else { "Media Player" }))
                .child(
                    PugProgress::new(ProgressSpec::new().with_value(0.3), theme)
                )
                .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("1:23 / 4:56"))
        )
}
