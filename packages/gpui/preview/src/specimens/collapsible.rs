use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::CollapsibleSpec;
use pug_gpui_components::PugCollapsible;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let is_open = state.specimens.is_on("collapsible-open");
    let spec = CollapsibleSpec::new()
        .with_title("Collapsible Section")
        .with_open(is_open);

    div().child(
        PugCollapsible::new(spec, theme)
            .with_id("specimen")
            .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                this.state.specimens.toggle("collapsible-open");
                cx.notify();
            }))
            .with_content(
                div().text_xs().text_color(color_to_hsla(text_secondary))
                    .child("This content is revealed when expanded.")
            )
    )
}
