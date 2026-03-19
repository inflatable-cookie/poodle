use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_composites::{
    PickerShellSpec, PickerVariant, BrowseState,
};
use pug_gpui_components::PugPickerShell;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Inline variant (ready) ---
    let inline_spec = PickerShellSpec::new("Select a component")
        .with_description("Browse and select from available components.")
        .with_variant(PickerVariant::Inline)
        .with_state(BrowseState::Ready)
        .with_result_count(12);

    // --- No results ---
    let no_results_spec = PickerShellSpec::new("Select an item")
        .with_variant(PickerVariant::Inline)
        .with_state(BrowseState::NoResults);

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("INLINE VARIANT (READY)", text_secondary))
        .child(
            PugPickerShell::new(inline_spec, theme)
                .with_results(
                    div().flex().flex_col()
                        .child(div().px(px(12.0)).py(px(8.0)).text_sm().child("Button"))
                        .child(div().px(px(12.0)).py(px(8.0)).text_sm().child("Checkbox"))
                        .child(div().px(px(12.0)).py(px(8.0)).text_sm().child("Select"))
                )
        )
        .child(section_label("NO RESULTS", text_secondary))
        .child(
            PugPickerShell::new(no_results_spec, theme)
                .with_results(div())
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
