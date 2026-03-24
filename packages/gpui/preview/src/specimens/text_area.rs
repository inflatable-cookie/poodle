use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{TextAreaSpec, EyebrowSpec};
use poodle_gpui_components::{TextArea, Eyebrow};
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

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
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
                        .when(!default_value.is_empty(), |d| {
                            d.child(
                                div().text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("{} characters", default_value.len()))
                            )
                        })
                )
        )
        // --- With initial value ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With initial value"), theme))
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
        )
        // --- Read-only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Read-only"), theme))
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
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
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
        )
}
