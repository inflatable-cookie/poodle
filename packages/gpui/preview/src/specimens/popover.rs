use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::PopoverSpec;
use pug_gpui_components::PugPopover;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let border = theme.resolve_color("semantic.color.border.default");
    let popover_open = state.specimens.is_on("popover-open");

    let popover_spec = PopoverSpec::new()
        .with_open(popover_open);

    div().child(
        PugPopover::new(popover_spec, theme)
            .with_trigger(
                div().id("popover-trigger")
                    .px(px(12.0)).py(px(6.0)).rounded(px(6.0))
                    .border_1().border_color(color_to_hsla(border)).text_sm()
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.06)))
                    .child(if popover_open { "Close Popover" } else { "Open Popover" })
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.toggle("popover-open");
                        cx.notify();
                    }))
            )
            .with_content(
                div().text_sm().child("Popover content")
            )
    )
}
