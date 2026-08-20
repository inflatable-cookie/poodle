use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{CodeInput, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CodeInputCompletion, CodeInputSpec, EyebrowSpec};

/// The specimen's completion validator (host-owned, like the web specimen's):
/// the 6-digit code is accepted only as `123456`; any other full value fails
/// and shows the danger cross.
fn six_digit_check(value: &str) -> Option<CodeInputCompletion> {
    if value.chars().count() != 6 {
        return None;
    }
    let result = if value == "123456" {
        CodeInputCompletion::Passed(value.to_string())
    } else {
        CodeInputCompletion::Failed(value.to_string())
    };
    Some(result)
}

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
    let key_value = state
        .specimens
        .text
        .get("code-input-key")
        .cloned()
        .unwrap_or_default();
    let completed = state.specimens.is_on("code-input-complete");

    // The caret is host state on the Rust targets, exactly like the value: the
    // web target's hidden `<input>` owns it and there is none here.
    let caret = |key: &str| state.specimens.carets.get(key).copied().unwrap_or_default();

    /// Wire a code input's three channels onto the specimen event queue.
    macro_rules! live_code {
        ($builder:expr, $key:literal) => {{
            let queue = std::sync::Arc::clone(&state.node_events);
            let selection_queue = std::sync::Arc::clone(&state.node_events);
            let complete_queue = std::sync::Arc::clone(&state.node_events);
            $builder
                .on_change(std::sync::Arc::new(move |val: &str| {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: $key.to_string(),
                        value: val.to_string(),
                    });
                }))
                .on_selection_change(std::sync::Arc::new(move |start: usize, end: usize| {
                    selection_queue
                        .lock()
                        .unwrap()
                        .push(NodeSpecimenEvent::SetCaret {
                            key: $key.to_string(),
                            start,
                            end,
                        });
                }))
                .on_complete(std::sync::Arc::new(move |_val: &str| {
                    complete_queue
                        .lock()
                        .unwrap()
                        .push(NodeSpecimenEvent::Toggle("code-input-complete".to_string()));
                }))
        }};
    }

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
                .child(live_code!(
                    CodeInput::from_spec(
                        CodeInputSpec::new()
                            .with_value(&code_value)
                            .with_selection(caret("code-input-code").0, caret("code-input-code").1)
                            .with_completion_opt(six_digit_check(&code_value))
                            .with_aria_label("Verification code"),
                        theme,
                    ),
                    "code-input-code"
                ))
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
                .child(live_code!(
                    CodeInput::from_spec(
                        CodeInputSpec::new()
                            .with_length(4)
                            .with_value(&pin_value)
                            .with_selection(caret("code-input-pin").0, caret("code-input-pin").1)
                            .with_mask(true)
                            .with_aria_label("PIN"),
                        theme,
                    ),
                    "code-input-pin"
                )),
        )
        // --- Grouped key (explicit partition + separator, alphanumeric) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Grouped key"),
                    theme,
                ))
                .child(live_code!(
                    CodeInput::from_spec(
                        CodeInputSpec::new()
                            .with_length(20)
                            .with_value(&key_value)
                            .with_selection(caret("code-input-key").0, caret("code-input-key").1)
                            .with_groups([5, 5, 5, 5])
                            .with_separator("-")
                            .with_numbers_only(false)
                            .with_autocomplete("off")
                            .with_aria_label("Licence key"),
                        theme,
                    ),
                    "code-input-key"
                ))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Entered: {}", key_value)),
                ),
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
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                CodeInput::from_spec(CodeInputSpec::new().with_length(4), theme)
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                CodeInput::from_spec(CodeInputSpec::new().with_length(4), theme)
                    .density(density)
                    .into_any_element()
            }),
    )
}
