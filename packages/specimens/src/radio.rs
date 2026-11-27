use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{radio, RenderContext};
use poodle_specs::RadioSpec;

use crate::{column, control_density, control_size, section, Sink, View};

pub(crate) struct State {
    shipping: &'static str,
}

impl Default for State {
    fn default() -> Self {
        Self {
            shipping: "standard",
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Event {
    value: &'static str,
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    state.shipping = event.value;
    state
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let option = |value: &'static str, label: &'static str| {
        let sink = Arc::clone(&sink);
        let mut node = radio(
            &RadioSpec::new()
                .with_name("shipping")
                .with_value(value)
                .with_label(label)
                .with_checked(state.shipping == value),
            ctx,
            Some(Arc::new(move |checked| {
                if checked {
                    sink(Event { value });
                }
            })),
        );
        node.id = Some(format!("radio-{value}"));
        node
    };

    let default = section(
        "Default — three-option native group",
        [
            option("standard", "Standard shipping"),
            option("express", "Express shipping"),
            option("overnight", "Overnight shipping"),
        ],
        ctx,
    );
    let states = section(
        "States",
        [
            radio(
                &RadioSpec::new()
                    .with_label("Disabled unchecked")
                    .with_disabled(true),
                ctx,
                None,
            ),
            radio(
                &RadioSpec::new()
                    .with_label("Disabled checked")
                    .with_checked(true)
                    .with_disabled(true),
                ctx,
                None,
            ),
            radio(
                &RadioSpec::new()
                    .with_label("Read-only checked")
                    .with_checked(true)
                    .with_read_only(true),
                ctx,
                None,
            ),
            radio(
                &RadioSpec::new()
                    .with_label("Custom selected color")
                    .with_checked(true)
                    .with_selected_color("#7c3aed"),
                ctx,
                None,
            ),
        ],
        ctx,
    );
    column(24.0, vec![default, states])
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = RadioSpec::new()
        .with_label("Standard shipping")
        .with_checked(true);
    let (id, spec) = match view {
        View::Size(value) => (
            format!("specimen-radio-size-{value}"),
            base.with_size(control_size(value)),
        ),
        View::Density(value) => (
            format!("specimen-radio-density-{value}"),
            base.with_density(control_density(value)),
        ),
        View::Examples => unreachable!("Examples uses the session"),
    };
    let mut node = radio(&spec, ctx, None);
    node.id = Some(id);
    node
}
