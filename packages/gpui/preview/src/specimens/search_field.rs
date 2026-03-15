use gpui::*;
use pug_gpui_primitives::SearchFieldSpec;
use pug_gpui_components::PugSearchField;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().child(
        PugSearchField::new(
            SearchFieldSpec::new().with_placeholder("Search..."),
            theme,
        )
        .with_id("specimen")
    )
}
