use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let accent = theme.resolve_color("semantic.color.accent.base");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let rating = state.specimens.selected("rating");
    let current = if state.specimens.selections.contains_key("rating") { rating } else { 3 };

    let mut row = div().flex().gap(px(2.0));

    for i in 0..5 {
        let filled = i <= current;
        let star = div()
            .id(SharedString::from(format!("star-{}", i)))
            .text_base()
            .cursor_pointer()
            .text_color(if filled { color_to_hsla(accent) } else { color_to_hsla(text_secondary) })
            .hover(|s| s.text_color(color_to_hsla(accent).opacity(0.7)))
            .child("★")
            .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.select("rating", i);
                cx.notify();
            }));
        row = row.child(star);
    }

    div().flex().flex_col().gap(px(4.0))
        .child(row)
        .child(div().text_xs().text_color(color_to_hsla(text_secondary))
            .child(format!("{} / 5 stars", current + 1)))
}
