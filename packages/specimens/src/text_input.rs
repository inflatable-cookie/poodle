use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{
    apply_text_input_intent, field, text_input, text_input_with_handlers, RenderContext,
    TextInputHandlers, TextInputIntent,
};
use poodle_specs::{FieldSpec, TextInputSpec, TextInputType, ValidationState};

use crate::{column, control_density, control_size, section, Sink, View};

#[derive(Clone, Copy)]
pub(crate) enum FieldId {
    Name,
    Email,
    Workspace,
}

struct FieldState {
    value: String,
    selection: (usize, usize),
    focused: bool,
}

pub(crate) struct State {
    name: FieldState,
    email: FieldState,
    workspace: FieldState,
}

impl Default for State {
    fn default() -> Self {
        let email = "invalid-email".to_string();
        let workspace = "acme-admin".to_string();
        Self {
            name: FieldState {
                value: String::new(),
                selection: (0, 0),
                focused: false,
            },
            email: FieldState {
                selection: (0, 0),
                value: email,
                focused: false,
            },
            workspace: FieldState {
                selection: (0, 0),
                value: workspace,
                focused: false,
            },
        }
    }
}

pub(crate) struct Event {
    field: FieldId,
    intent: TextInputIntent,
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    let entry = match event.field {
        FieldId::Name => &mut state.name,
        FieldId::Email => &mut state.email,
        FieldId::Workspace => &mut state.workspace,
    };
    if let TextInputIntent::FocusChange(focused) = event.intent {
        entry.focused = focused;
    }
    apply_text_input_intent(&mut entry.value, &mut entry.selection, None, event.intent);
    state
}

fn live_input(
    field_id: FieldId,
    entry: &FieldState,
    spec: TextInputSpec,
    sink: &Sink<Event>,
    ctx: &RenderContext<'_>,
) -> Node {
    let mut spec = spec
        .with_value(&entry.value)
        .with_selection(entry.selection.0, entry.selection.1);
    spec.is_focused = entry.focused;
    let sink = Arc::clone(sink);
    text_input_with_handlers(
        &spec,
        ctx,
        TextInputHandlers {
            on_intent: Some(Arc::new(move |intent| {
                sink(Event {
                    field: field_id,
                    intent,
                });
            })),
            ..TextInputHandlers::default()
        },
    )
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let email_is_valid = state.email.value.contains('@');
    let email_validation = if email_is_valid {
        ValidationState::Valid
    } else {
        ValidationState::Invalid
    };

    let default_field = section(
        "Default field",
        [field(
            &FieldSpec::new("name-field", "Name").with_description("Enter your full name."),
            ctx,
            Some(Box::new(|slot_ctx| {
                live_input(
                    FieldId::Name,
                    &state.name,
                    TextInputSpec::new()
                        .with_id("name-field")
                        .with_placeholder("Jane Doe"),
                    &sink,
                    slot_ctx,
                )
            })),
        )],
        ctx,
    );

    let mut email_field_spec = FieldSpec::new("email-field", "Email")
        .with_description("A valid email address is required.")
        .with_validation_state(email_validation);
    if !email_is_valid {
        email_field_spec = email_field_spec.with_error("Please enter a valid email address.");
    }
    let email_field = field(
        &email_field_spec,
        ctx,
        Some(Box::new(|slot_ctx| {
            live_input(
                FieldId::Email,
                &state.email,
                TextInputSpec::new()
                    .with_id("email-field")
                    .with_placeholder("you@example.com")
                    .with_validation_state(email_validation),
                &sink,
                slot_ctx,
            )
        })),
    );

    let workspace_field = field(
        &FieldSpec::new("workspace-field", "Workspace")
            .with_description("Check whether the workspace handle is available.")
            .with_validation_state(ValidationState::Pending)
            .with_pending_message("Checking availability..."),
        ctx,
        Some(Box::new(|slot_ctx| {
            live_input(
                FieldId::Workspace,
                &state.workspace,
                TextInputSpec::new()
                    .with_id("workspace-field")
                    .with_validation_state(ValidationState::Pending),
                &sink,
                slot_ctx,
            )
        })),
    );

    let validation = section(
        "Validation and async availability",
        [column(12.0, vec![email_field, workspace_field])],
        ctx,
    );

    let search = section(
        "Search input",
        [text_input(
            &TextInputSpec::new()
                .with_id("search-field")
                .with_type(TextInputType::Search)
                .with_placeholder("Search..."),
            ctx,
            None,
        )],
        ctx,
    );

    let affixes = section(
        "Prefix and suffix",
        [text_input(
            &TextInputSpec::new()
                .with_id("price-field")
                .with_prefix("$")
                .with_suffix("USD")
                .with_placeholder("0.00")
                .with_type(TextInputType::Number),
            ctx,
            None,
        )],
        ctx,
    );

    let multiline = section(
        "Multiline",
        [field(
            &FieldSpec::new("multiline-field", "Description"),
            ctx,
            Some(Box::new(|slot_ctx| {
                text_input(
                    &TextInputSpec::new()
                        .with_id("multiline-field")
                        .with_type(TextInputType::Multiline)
                        .with_rows(3)
                        .with_max_length(280)
                        .with_show_char_count(true)
                        .with_placeholder("Enter a description..."),
                    slot_ctx,
                    None,
                )
            })),
        )],
        ctx,
    );

    let disabled = section(
        "Disabled",
        [field(
            &FieldSpec::new("disabled-field", "API key"),
            ctx,
            Some(Box::new(|slot_ctx| {
                text_input(
                    &TextInputSpec::new()
                        .with_id("disabled-field")
                        .with_value("sk-xxxx-xxxx-xxxx")
                        .with_disabled(true),
                    slot_ctx,
                    None,
                )
            })),
        )],
        ctx,
    );

    let mut body = column(
        24.0,
        vec![
            default_field,
            validation,
            search,
            affixes,
            multiline,
            disabled,
        ],
    );
    body.style.max_width = Some(384.0);
    body
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = TextInputSpec::new().with_placeholder("Text input");
    let spec = match view {
        View::Size(value) => base
            .with_id(format!("specimen-text-input-size-{value}"))
            .with_size(control_size(value)),
        View::Density(value) => base
            .with_id(format!("specimen-text-input-density-{value}"))
            .with_density(control_density(value)),
        View::Examples => unreachable!("Examples uses the session"),
    };
    text_input(&spec, ctx, None)
}
