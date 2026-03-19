use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::TextAreaSpec;
use pug_gpui_components::PugTextArea;
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
            PugTextArea::new(
                TextAreaSpec::new()
                    .with_placeholder("Write a note\u{2026}")
                    .with_aria_label("Note"),
                theme,
            )
            .with_id("ta-default")
        )
        // --- With initial value ---
        .child(section_label("WITH INITIAL VALUE", text_secondary))
        .child(
            PugTextArea::new(
                TextAreaSpec::new()
                    .with_default_value("A brief description about yourself.")
                    .with_rows(3)
                    .with_aria_label("Biography"),
                theme,
            )
            .with_id("ta-initial")
        )
        // --- Read-only ---
        .child(section_label("READ-ONLY", text_secondary))
        .child(
            PugTextArea::new(
                TextAreaSpec::new()
                    .with_default_value("This content cannot be modified by the user.")
                    .with_rows(2)
                    .with_read_only(true)
                    .with_aria_label("Read-only textarea"),
                theme,
            )
            .with_id("ta-readonly")
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            PugTextArea::new(
                TextAreaSpec::new()
                    .with_placeholder("Disabled")
                    .with_disabled(true)
                    .with_aria_label("Disabled textarea"),
                theme,
            )
            .with_id("ta-disabled")
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
