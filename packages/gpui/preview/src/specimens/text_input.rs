use gpui::*;
use pug_gpui_primitives::TextInputSpec;
use pug_gpui_components::PugTextInput;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().flex().flex_col().gap(px(6.0))
        .child(
            PugTextInput::new(
                TextInputSpec::new().with_placeholder("Click to focus..."),
                theme,
            )
        )
        .child(
            PugTextInput::new(
                TextInputSpec::new().with_value("Filled value"),
                theme,
            )
        )
        .child(
            PugTextInput::new(
                TextInputSpec::new().with_placeholder("Disabled").with_disabled(true),
                theme,
            )
        )
}
