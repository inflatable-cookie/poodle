use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{
    apply_number_input_intent, number_input, NumberInputHandlers, NumberInputIntent, RenderContext,
};
use poodle_specs::{NumberInputSpec, ValidationState};

use crate::{column, control_density, control_size, readout, section, Sink, View};

#[derive(Clone, Copy)]
pub(crate) enum FieldId {
    Quantity,
    Price,
}

impl FieldId {
    fn base_spec(self, value: Option<f64>) -> NumberInputSpec {
        match self {
            Self::Quantity => NumberInputSpec::new(value)
                .with_id("quantity")
                .with_min(Some(0.0))
                .with_max(Some(100.0))
                .with_steppers(true)
                .with_aria_label("Quantity"),
            Self::Price => NumberInputSpec::new(value)
                .with_id("price")
                .with_min(Some(0.0))
                .with_step(Some(0.01))
                .with_steppers(true)
                .with_aria_label("Price"),
        }
    }
}

struct FieldState {
    value: Option<f64>,
    draft: Option<String>,
    selection: (usize, usize),
    focused: bool,
}

impl FieldState {
    fn new(value: f64) -> Self {
        Self {
            value: Some(value),
            draft: None,
            selection: (0, 0),
            focused: false,
        }
    }

    fn spec(&self, field: FieldId) -> NumberInputSpec {
        let mut spec = field
            .base_spec(self.value)
            .with_selection(self.selection.0, self.selection.1);
        if let Some(draft) = &self.draft {
            spec = spec.with_draft_value(Some(draft.clone()));
        }
        if self.focused {
            spec = spec.with_focused(true);
        }
        spec
    }
}

pub(crate) struct State {
    quantity: FieldState,
    price: FieldState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            quantity: FieldState::new(1.0),
            price: FieldState::new(29.99),
        }
    }
}

pub(crate) struct Event {
    field: FieldId,
    intent: NumberInputIntent,
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    let entry = match event.field {
        FieldId::Quantity => &mut state.quantity,
        FieldId::Price => &mut state.price,
    };
    if let NumberInputIntent::FocusChange(focused) = event.intent {
        entry.focused = focused;
    }
    let context = entry.spec(event.field).to_context();
    let (next, selection, _) = apply_number_input_intent(context, entry.selection, event.intent);
    entry.value = next.committed;
    entry.draft = next.draft;
    entry.selection = selection;
    state
}

fn live_number(
    field: FieldId,
    entry: &FieldState,
    sink: &Sink<Event>,
    ctx: &RenderContext<'_>,
) -> Node {
    let sink = Arc::clone(sink);
    number_input(
        &entry.spec(field),
        ctx,
        NumberInputHandlers {
            on_intent: Some(Arc::new(move |intent| {
                sink(Event { field, intent });
            })),
            ..NumberInputHandlers::default()
        },
    )
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let default = section(
        "Default",
        [
            live_number(FieldId::Quantity, &state.quantity, &sink, ctx),
            readout(
                format!(
                    "Quantity: {}",
                    state
                        .quantity
                        .value
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "empty".into())
                ),
                ctx,
            ),
        ],
        ctx,
    );

    let with_steppers = section(
        "With steppers",
        [
            live_number(FieldId::Price, &state.price, &sink, ctx),
            readout(
                match state.price.value {
                    Some(v) => format!("Price: ${v:.2}"),
                    None => "Price: empty".into(),
                },
                ctx,
            ),
        ],
        ctx,
    );

    let disabled = section(
        "Disabled",
        [number_input(
            &NumberInputSpec::new(Some(42.0)).with_disabled(true),
            ctx,
            NumberInputHandlers::default(),
        )],
        ctx,
    );

    let invalid = section(
        "Invalid",
        [number_input(
            &NumberInputSpec::new(Some(-5.0))
                .with_min(Some(0.0))
                .with_validation_state(ValidationState::Invalid),
            ctx,
            NumberInputHandlers::default(),
        )],
        ctx,
    );

    let prefix = section(
        "Prefix (currency)",
        [number_input(
            &NumberInputSpec::new(Some(29.99))
                .with_min(Some(0.0))
                .with_step(Some(0.01))
                .with_prefix("$")
                .with_precision(2),
            ctx,
            NumberInputHandlers::default(),
        )],
        ctx,
    );

    let suffix = section(
        "Suffix (unit)",
        [number_input(
            &NumberInputSpec::new(Some(72.0))
                .with_min(Some(0.0))
                .with_suffix("kg"),
            ctx,
            NumberInputHandlers::default(),
        )],
        ctx,
    );

    let precision = section(
        "Precision (3 decimal places)",
        [number_input(
            &NumberInputSpec::new(Some(std::f64::consts::PI))
                .with_step(Some(0.001))
                .with_precision(3),
            ctx,
            NumberInputHandlers::default(),
        )],
        ctx,
    );

    let mut body = column(
        24.0,
        vec![
            default,
            with_steppers,
            disabled,
            invalid,
            prefix,
            suffix,
            precision,
        ],
    );
    body.style.max_width = Some(224.0);
    body
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = NumberInputSpec::new(Some(1.0));
    let spec = match view {
        View::Size(value) => base
            .with_id(format!("specimen-number-input-size-{value}"))
            .with_size(control_size(value)),
        View::Density(value) => base
            .with_id(format!("specimen-number-input-density-{value}"))
            .with_density(control_density(value)),
        View::Examples => unreachable!("Examples uses the session"),
    };
    number_input(&spec, ctx, NumberInputHandlers::default())
}
