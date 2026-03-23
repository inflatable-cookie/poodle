use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::TextAreaSpec;
use pug_gpui_components::TextArea;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let default_value = state.specimens.text.get("textarea-default").cloned()
        .unwrap_or_default();
    let bio_value = state.specimens.text.get("textarea-bio").cloned()
        .unwrap_or_else(|| "A brief description about yourself.".to_string());

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            TextArea::from_spec(
                TextAreaSpec::new()
                    .with_placeholder("Write a note\u{2026}")
                    .with_value(&default_value)
                    .with_aria_label("Note"),
                theme,
            )
            .with_id("ta-default")
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("textarea-default".to_string(), val.to_string());
                cx.notify();
            }))
        )
        // --- With initial value ---
        .child(section_label("WITH INITIAL VALUE", text_secondary))
        .child(
            TextArea::from_spec(
                TextAreaSpec::new()
                    .with_value(&bio_value)
                    .with_rows(3)
                    .with_aria_label("Biography"),
                theme,
            )
            .with_id("ta-initial")
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("textarea-bio".to_string(), val.to_string());
                cx.notify();
            }))
        )
        // --- Read-only ---
        .child(section_label("READ-ONLY", text_secondary))
        .child(
            TextArea::from_spec(
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
            TextArea::from_spec(
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
