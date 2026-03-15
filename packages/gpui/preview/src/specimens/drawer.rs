use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::DrawerSpec;
use pug_gpui_components::PugDrawer;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let spec = DrawerSpec::new()
        .with_title("Drawer Panel")
        .with_description("Side panel content");

    div().child(
        PugDrawer::new(spec, theme)
            .with_main_content(
                div().text_xs().text_color(color_to_hsla(text_secondary)).child("Main area")
            )
    )
}
