use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{checkbox, RenderContext};
use poodle_specs::CheckboxSpec;

use crate::{column, control_density, control_size, section, Sink, View};

#[derive(Clone, Copy)]
pub(crate) enum Item {
    Email,
    Marketing,
    Terms,
}

pub(crate) struct State {
    email: bool,
    marketing: bool,
    terms: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            email: true,
            marketing: false,
            terms: false,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Event {
    item: Item,
    checked: bool,
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event.item {
        Item::Email => state.email = event.checked,
        Item::Marketing => state.marketing = event.checked,
        Item::Terms => state.terms = event.checked,
    }
    state
}

fn live(
    id: &str,
    item: Item,
    label: &str,
    checked: bool,
    sink: &Sink<Event>,
    ctx: &RenderContext<'_>,
) -> Node {
    let sink = Arc::clone(sink);
    let mut node = checkbox(
        &CheckboxSpec::new().with_checked(checked).with_label(label),
        ctx,
        Some(Arc::new(move |checked| sink(Event { item, checked }))),
    );
    node.id = Some(id.to_string());
    node
}

fn static_box(id: &str, spec: CheckboxSpec, ctx: &RenderContext<'_>) -> Node {
    let mut node = checkbox(&spec, ctx, None);
    node.id = Some(id.to_string());
    node
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let default = section(
        "Default",
        [column(
            10.0,
            vec![
                live(
                    "checkbox-email",
                    Item::Email,
                    "Enable email notifications",
                    state.email,
                    &sink,
                    ctx,
                ),
                live(
                    "checkbox-marketing",
                    Item::Marketing,
                    "Subscribe to marketing emails",
                    state.marketing,
                    &sink,
                    ctx,
                ),
                live(
                    "checkbox-terms",
                    Item::Terms,
                    "I agree to the terms and conditions",
                    state.terms,
                    &sink,
                    ctx,
                ),
            ],
        )],
        ctx,
    );
    let states = section(
        "States",
        [column(
            10.0,
            vec![
                static_box(
                    "cb-disabled-unchecked",
                    CheckboxSpec::new()
                        .with_disabled(true)
                        .with_label("Disabled unchecked"),
                    ctx,
                ),
                static_box(
                    "cb-disabled-checked",
                    CheckboxSpec::new()
                        .with_checked(true)
                        .with_disabled(true)
                        .with_label("Disabled checked"),
                    ctx,
                ),
                static_box(
                    "cb-mixed",
                    CheckboxSpec::new()
                        .with_mixed(true)
                        .with_label("Mixed / indeterminate"),
                    ctx,
                ),
                static_box(
                    "cb-readonly-checked",
                    CheckboxSpec::new()
                        .with_checked(true)
                        .with_read_only(true)
                        .with_label("Read-only checked"),
                    ctx,
                ),
            ],
        )],
        ctx,
    );
    let custom = section(
        "Custom selected color",
        [column(
            10.0,
            vec![
                static_box(
                    "cb-custom-green",
                    CheckboxSpec::new()
                        .with_checked(true)
                        .with_label("Billable feature")
                        .with_selected_color("#22c55e"),
                    ctx,
                ),
                static_box(
                    "cb-custom-amber",
                    CheckboxSpec::new()
                        .with_checked(true)
                        .with_label("Requires moderation")
                        .with_selected_color("#f59e0b"),
                    ctx,
                ),
            ],
        )],
        ctx,
    );
    column(24.0, vec![default, states, custom])
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let (id, spec) = match view {
        View::Size(value) => (
            format!("specimen-checkbox-size-{value}"),
            CheckboxSpec::new()
                .with_label("Accept terms")
                .with_size(control_size(value)),
        ),
        View::Density(value) => (
            format!("specimen-checkbox-density-{value}"),
            CheckboxSpec::new()
                .with_label("Option")
                .with_density(control_density(value)),
        ),
        View::Examples => unreachable!("Examples uses the session"),
    };
    static_box(&id, spec, ctx)
}
