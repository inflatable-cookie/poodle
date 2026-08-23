use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, Field, TextInput};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, FieldSpec, TextInputSpec, ValidationState};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // Track typed values
    let name_value = state
        .specimens
        .text
        .get("text-input-name")
        .cloned()
        .unwrap_or_default();
    let email_value = state
        .specimens
        .text
        .get("text-input-email")
        .cloned()
        .unwrap_or_else(|| "invalid-email".to_string());
    let email_is_valid = email_value.contains('@');
    let validation_state = if email_is_valid {
        ValidationState::Valid
    } else {
        ValidationState::Invalid
    };
    let workspace_value = state
        .specimens
        .text
        .get("text-input-workspace")
        .cloned()
        .unwrap_or_else(|| "acme-admin".to_string());

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(384.0)) // 24rem = Svelte specimen max-width
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default field"),
                    theme,
                ))
                .child(
                    Field::from_spec(
                        FieldSpec::new("name-field", "Name")
                            .with_description("Enter your full name."),
                        theme,
                    )
                    .with_control(live_text_input(
                        TextInputSpec::new()
                            .with_id("name-field")
                            .with_placeholder("Jane Doe")
                            .with_value(&name_value),
                        theme,
                        state,
                        "text-input-name",
                    ).into_slot()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Validation and async availability"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(
                            Field::from_spec(
                                {
                                    let mut field = FieldSpec::new("email-field", "Email")
                                        .with_description("A valid email address is required.")
                                        .with_validation_state(validation_state);
                                    if !email_is_valid {
                                        field = field.with_error("Please enter a valid email address.");
                                    }
                                    field
                                },
                                theme,
                            )
                            .with_control(live_text_input(
                                TextInputSpec::new()
                                    .with_id("email-field")
                                    .with_placeholder("you@example.com")
                                    .with_value(&email_value)
                                    .with_validation_state(validation_state),
                                theme,
                                state,
                                "text-input-email",
                            ).into_slot()),
                        )
                        .child(
                            Field::from_spec(
                                FieldSpec::new("workspace-field", "Workspace")
                                    .with_description(
                                        "Check whether the workspace handle is available.",
                                    )
                                    .with_validation_state(ValidationState::Pending)
                                    .with_pending_message("Checking availability..."),
                                theme,
                            )
                            .with_control(live_text_input(
                                TextInputSpec::new()
                                    .with_id("workspace-field")
                                    .with_value(&workspace_value)
                                    .with_validation_state(ValidationState::Pending),
                                theme,
                                state,
                                "text-input-workspace",
                            ).into_slot()),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Search input"),
                    theme,
                ))
                .child(TextInput::from_spec(
                    TextInputSpec::new()
                        .with_id("search-field")
                        .with_input_type("search")
                        .with_placeholder("Search..."),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Prefix and suffix"),
                    theme,
                ))
                .child(TextInput::from_spec(
                    TextInputSpec::new()
                        .with_id("price-field")
                        .with_prefix("$")
                        .with_suffix("USD")
                        .with_placeholder("0.00")
                        .with_input_type("number"),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multiline"),
                    theme,
                ))
                .child(
                    Field::from_spec(FieldSpec::new("multiline-field", "Description"), theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("multiline-field")
                                .with_input_type("multiline")
                                .with_rows(3)
                                .with_max_length(280)
                                .with_show_char_count(true)
                                .with_placeholder("Enter a description..."),
                            theme,
                        ).into_slot()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(
                    Field::from_spec(FieldSpec::new("disabled-field", "API key"), theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("disabled-field")
                                .with_value("sk-xxxx-xxxx-xxxx")
                                .with_disabled(true),
                            theme,
                        ).into_slot()),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "text-input",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                TextInput::from_spec(TextInputSpec::new().with_placeholder("Text input"), theme)
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                TextInput::from_spec(TextInputSpec::new().with_placeholder("Text input"), theme)
                    .density(density)
                    .into_any_element()
            }),
    )
}

fn live_text_input(
    spec: TextInputSpec,
    theme: &GpuiThemeProvider,
    state: &AppState,
    key: &'static str,
) -> TextInput {
    // The caret is host state here for the same reason the value is: with no
    // native editor, nothing else survives a re-render. Storing the value but
    // not the caret leaves every keystroke inserting at index 0 — "abc" typed
    // into an empty field comes out "cba".
    let (start, end) = state.specimens.carets.get(key).copied().unwrap_or_default();
    let spec = spec.with_selection(start, end);
    let events = state.node_events.clone();
    let selection_events = state.node_events.clone();
    TextInput::from_spec(spec, theme)
        .on_change(move |value| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: key.to_string(),
                value: value.to_string(),
            });
        })
        .on_selection_change(std::sync::Arc::new(move |start: usize, end: usize| {
            selection_events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetCaret {
                    key: key.to_string(),
                    start,
                    end,
                });
        }))
}
