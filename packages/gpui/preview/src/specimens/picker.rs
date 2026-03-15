use gpui::*;
use pug_gpui_composites::{
    PickerShellSpec, PickerVariant, SelectionMode, BrowseState,
    SelectionSummarySpec, SelectionSummaryItem, RemediationAction,
};
use pug_gpui_components::{PugPickerShell, PugSelectionSummary};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let picker_spec = PickerShellSpec::new("Select an item")
        .with_description("Choose from the available options below")
        .with_variant(PickerVariant::Inline)
        .with_selection_mode(SelectionMode::Multiple)
        .with_state(BrowseState::Ready)
        .with_result_count(3)
        .with_selected_count(1);

    let summary_spec = SelectionSummarySpec::new(vec![
        SelectionSummaryItem::new("a", "Project Alpha").with_meta("Active"),
        SelectionSummaryItem::new("b", "Project Beta"),
    ])
    .with_clear_action(RemediationAction::new("clear", "Clear all"));

    div().flex().flex_col().gap(px(12.0))
        .child(
            PugPickerShell::new(picker_spec, theme)
                .with_results(
                    div().flex().flex_col()
                        .child(div().px(px(12.0)).py(px(8.0)).text_sm().child("Item one"))
                        .child(div().px(px(12.0)).py(px(8.0)).text_sm().child("Item two"))
                        .child(div().px(px(12.0)).py(px(8.0)).text_sm().child("Item three"))
                )
        )
        .child(PugSelectionSummary::new(summary_spec, theme))
}
