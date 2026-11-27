use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{
    apply_code_input_intent, code_input, code_input_with_handlers, CodeInputHandlers,
    CodeInputIntent, RenderContext,
};
use poodle_specs::{CodeInputCompletion, CodeInputSpec};

use crate::{column, control_density, control_size, readout, section, Sink, View};

#[derive(Clone, Copy)]
pub(crate) enum FieldId {
    Code,
    Pin,
    Key,
}

impl FieldId {
    fn config(self) -> (usize, bool) {
        match self {
            Self::Code => (6, true),
            Self::Pin => (4, true),
            Self::Key => (20, false),
        }
    }
}

#[derive(Default)]
struct Entry {
    value: String,
    selection: (usize, usize),
}

#[derive(Default)]
pub(crate) struct State {
    code: Entry,
    pin: Entry,
    key: Entry,
    completed: bool,
}

pub(crate) struct Event {
    field: FieldId,
    intent: CodeInputIntent,
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    let (length, numbers_only) = event.field.config();
    let entry = match event.field {
        FieldId::Code => &mut state.code,
        FieldId::Pin => &mut state.pin,
        FieldId::Key => &mut state.key,
    };
    let finished = apply_code_input_intent(
        &mut entry.value,
        &mut entry.selection,
        length,
        numbers_only,
        event.intent,
    );
    if finished {
        state.completed = !state.completed;
    }
    state
}

fn six_digit_check(value: &str) -> Option<CodeInputCompletion> {
    if value.chars().count() != 6 {
        return None;
    }
    Some(if value == "123456" {
        CodeInputCompletion::Passed(value.to_string())
    } else {
        CodeInputCompletion::Failed(value.to_string())
    })
}

fn live_code(
    id: &str,
    field: FieldId,
    entry: &Entry,
    spec: CodeInputSpec,
    sink: &Sink<Event>,
    ctx: &RenderContext<'_>,
) -> Node {
    let sink = Arc::clone(sink);
    let mut node = code_input_with_handlers(
        &spec
            .with_value(&entry.value)
            .with_selection(entry.selection.0, entry.selection.1),
        ctx,
        CodeInputHandlers {
            on_intent: Some(Arc::new(move |intent| {
                sink(Event { field, intent });
            })),
            ..CodeInputHandlers::default()
        },
    );
    node.id = Some(id.to_string());
    node
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let six_digit = section(
        "6-digit code",
        [
            live_code(
                "specimen-code-input-code",
                FieldId::Code,
                &state.code,
                CodeInputSpec::new()
                    .with_completion_opt(six_digit_check(&state.code.value))
                    .with_aria_label("Verification code"),
                &sink,
                ctx,
            ),
            readout(
                if state.completed {
                    "Code complete!".into()
                } else {
                    format!("Entered: {}", state.code.value)
                },
                ctx,
            ),
        ],
        ctx,
    );

    let masked = section(
        "4-digit masked",
        [live_code(
            "specimen-code-input-pin",
            FieldId::Pin,
            &state.pin,
            CodeInputSpec::new()
                .with_length(4)
                .with_mask(true)
                .with_aria_label("PIN"),
            &sink,
            ctx,
        )],
        ctx,
    );

    let grouped = section(
        "Grouped key",
        [
            live_code(
                "specimen-code-input-key",
                FieldId::Key,
                &state.key,
                CodeInputSpec::new()
                    .with_length(20)
                    .with_groups([5, 5, 5, 5])
                    .with_separator("-")
                    .with_numbers_only(false)
                    .with_autocomplete("off")
                    .with_aria_label("Licence key"),
                &sink,
                ctx,
            ),
            readout(format!("Entered: {}", state.key.value), ctx),
        ],
        ctx,
    );

    let alphanumeric = section(
        "Alphanumeric",
        [code_input(
            &CodeInputSpec::new()
                .with_default_value("AB12")
                .with_numbers_only(false)
                .with_autocomplete("off")
                .with_aria_label("Recovery code"),
            ctx,
        )],
        ctx,
    );

    let with_error = section(
        "With error",
        [code_input(
            &CodeInputSpec::new()
                .with_value("1234")
                .with_error("Invalid code — try again.")
                .with_aria_label("Code with error"),
            ctx,
        )],
        ctx,
    );

    let disabled = section(
        "Disabled",
        [code_input(
            &CodeInputSpec::new()
                .with_default_value("123")
                .with_disabled(true),
            ctx,
        )],
        ctx,
    );

    let mut body = column(
        24.0,
        vec![
            six_digit,
            masked,
            grouped,
            alphanumeric,
            with_error,
            disabled,
        ],
    );
    body.style.max_width = Some(384.0);
    body
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = CodeInputSpec::new().with_length(4);
    let spec = match view {
        View::Size(value) => base.with_size(control_size(value)),
        View::Density(value) => base.with_density(control_density(value)),
        View::Examples => unreachable!("Examples uses the session"),
    };
    code_input(&spec, ctx)
}
