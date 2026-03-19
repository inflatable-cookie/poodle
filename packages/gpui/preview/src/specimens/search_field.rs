use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::SearchFieldSpec;
use pug_gpui_components::PugSearchField;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            PugSearchField::new(
                SearchFieldSpec::new()
                    .with_placeholder("Search components...")
                    .with_aria_label("Search components"),
                theme,
            )
            .with_id("sf-default")
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            PugSearchField::new(
                SearchFieldSpec::new()
                    .with_value("locked query")
                    .with_disabled(true)
                    .with_aria_label("Disabled search"),
                theme,
            )
            .with_id("sf-disabled")
        )
        // --- Read-only ---
        .child(section_label("READ-ONLY", text_secondary))
        .child(
            PugSearchField::new(
                SearchFieldSpec::new()
                    .with_value("active filter")
                    .with_read_only(true)
                    .with_aria_label("Read-only search"),
                theme,
            )
            .with_id("sf-readonly")
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
