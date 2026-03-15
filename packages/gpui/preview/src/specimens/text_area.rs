use gpui::*;
use pug_gpui_primitives::TextAreaSpec;
use pug_gpui_components::PugTextArea;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().child(
        PugTextArea::new(
            TextAreaSpec::new()
                .with_placeholder("Multi-line text area...")
                .with_rows(4),
            theme,
        )
        .with_id("specimen")
    )
}
