use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{CodeInput, Eyebrow};
use poodle_specs::{CodeInputSpec, EyebrowSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");

    let code_value = state
        .specimens
        .text
        .get("code-input-code")
        .cloned()
        .unwrap_or_default();
    let pin_value = state
        .specimens
        .text
        .get("code-input-pin")
        .cloned()
        .unwrap_or_default();
    let completed = state.specimens.is_on("code-input-complete");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(384.0))
        // --- 6-digit code ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("6-digit code"),
                    theme,
                ))
                .child(
                    CodeInput::from_spec(
                        CodeInputSpec::new()
                            .with_value(&code_value)
                            .with_aria_label("Verification code"),
                        theme,
                    )
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("code-input-code".to_string(), val.to_string());
                        cx.notify();
                    }))
                    .on_complete(cx.listener(|this, _val: &str, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("code-input-complete".to_string(), true);
                        cx.notify();
                    })),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(if completed {
                            text_primary
                        } else {
                            text_secondary
                        }))
                        .child(if completed {
                            "Code complete!".to_string()
                        } else {
                            format!("Entered: {}", code_value)
                        }),
                ),
        )
        // --- Partial (3 of 6) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Partial (3 of 6)"),
                    theme,
                ))
                .child(CodeInput::from_spec(
                    CodeInputSpec::new()
                        .with_value("123")
                        .with_aria_label("Partial code"),
                    theme,
                )),
        )
        // --- Complete ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Complete"),
                    theme,
                ))
                .child(CodeInput::from_spec(
                    CodeInputSpec::new()
                        .with_value("123456")
                        .with_aria_label("Complete code"),
                    theme,
                )),
        )
        // --- Numbers only (default) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Numbers only"),
                    theme,
                ))
                .child(CodeInput::from_spec(
                    CodeInputSpec::new()
                        .with_value("042")
                        .with_numbers_only(true)
                        .with_aria_label("Numeric code"),
                    theme,
                )),
        )
        // --- Alphanumeric ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Alphanumeric"),
                    theme,
                ))
                .child(CodeInput::from_spec(
                    CodeInputSpec::new()
                        .with_default_value("AB12")
                        .with_numbers_only(false)
                        .with_autocomplete("off")
                        .with_aria_label("Recovery code"),
                    theme,
                )),
        )
        // --- 4-digit masked ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("4-digit masked"),
                    theme,
                ))
                .child(
                    CodeInput::from_spec(
                        CodeInputSpec::new()
                            .with_length(4)
                            .with_value(&pin_value)
                            .with_mask(true)
                            .with_aria_label("PIN"),
                        theme,
                    )
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("code-input-pin".to_string(), val.to_string());
                        cx.notify();
                    })),
                ),
        )
        // --- With error ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With error"),
                    theme,
                ))
                .child(CodeInput::from_spec(
                    CodeInputSpec::new()
                        .with_value("1234")
                        .with_error("Invalid code — try again.")
                        .with_aria_label("Code with error"),
                    theme,
                )),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(CodeInput::from_spec(
                    CodeInputSpec::new()
                        .with_default_value("123")
                        .with_disabled(true),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "code-input",
        examples,
        |size, theme: &GpuiThemeProvider| {
            CodeInput::from_spec(CodeInputSpec::new().with_length(4), theme)
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            CodeInput::from_spec(CodeInputSpec::new().with_length(4), theme)
                .density(density)
                .into_any_element()
        },
    )
}
