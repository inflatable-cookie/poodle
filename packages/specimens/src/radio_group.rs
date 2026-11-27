use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{radio_group, RadioGroupHandlers, RenderContext};
use poodle_specs::{ChoiceOption, Orientation, RadioGroupSpec};

use crate::{column, control_density, control_size, readout, section, Sink, View};

pub(crate) struct State {
    plan: String,
    size: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            plan: "pro".into(),
            size: "md".into(),
        }
    }
}

pub(crate) enum Event {
    Plan(String),
    Size(String),
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event {
        Event::Plan(value) => state.plan = value,
        Event::Size(value) => state.size = value,
    }
    state
}

fn plan_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("free", "Free"),
        ChoiceOption::new("pro", "Pro"),
        ChoiceOption::new("enterprise", "Enterprise"),
    ]
}

fn size_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("sm", "Small"),
        ChoiceOption::new("md", "Medium"),
        ChoiceOption::new("lg", "Large"),
        ChoiceOption::new("xl", "Extra large"),
    ]
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let plan_change = |sink: &Sink<Event>| {
        let sink = Arc::clone(sink);
        Arc::new(move |value: &str| sink(Event::Plan(value.to_string())))
            as Arc<dyn Fn(&str) + Send + Sync>
    };
    let size_change = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value: &str| sink(Event::Size(value.to_string())))
            as Arc<dyn Fn(&str) + Send + Sync>
    };

    let vertical = section(
        "Vertical (default)",
        [
            radio_group(
                &RadioGroupSpec::new(plan_options()).with_value(state.plan.clone()),
                ctx,
                RadioGroupHandlers::new("radio-plan").on_change(plan_change(&sink)),
            ),
            readout(format!("Selected: {}", state.plan), ctx),
        ],
        ctx,
    );
    let horizontal = section(
        "Horizontal",
        [
            radio_group(
                &RadioGroupSpec::new(size_options())
                    .with_value(state.size.clone())
                    .with_orientation(Orientation::Horizontal),
                ctx,
                RadioGroupHandlers::new("radio-size").on_change(size_change),
            ),
            readout(format!("Selected: {}", state.size), ctx),
        ],
        ctx,
    );
    let mut disabled_spec = RadioGroupSpec::new(plan_options()).with_value("free");
    disabled_spec.is_disabled = true;
    let disabled = section(
        "Disabled",
        [radio_group(
            &disabled_spec,
            ctx,
            RadioGroupHandlers::new("radio-disabled"),
        )],
        ctx,
    );
    let disabled_option = section(
        "Disabled option",
        [radio_group(
            &RadioGroupSpec::new(vec![
                ChoiceOption::new("free", "Free"),
                ChoiceOption::new("pro", "Pro"),
                ChoiceOption::new("enterprise", "Enterprise").with_disabled(true),
            ])
            .with_value("pro"),
            ctx,
            RadioGroupHandlers::new("radio-disabled-option"),
        )],
        ctx,
    );
    let custom_color = section(
        "Custom selected color",
        [radio_group(
            &RadioGroupSpec::new(plan_options())
                .with_value(state.plan.clone())
                .with_selected_color("#22c55e"),
            ctx,
            RadioGroupHandlers::new("radio-custom-color").on_change(plan_change(&sink)),
        )],
        ctx,
    );

    column(
        24.0,
        vec![
            vertical,
            horizontal,
            disabled,
            disabled_option,
            custom_color,
        ],
    )
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = RadioGroupSpec::new(plan_options())
        .with_value("pro")
        .with_orientation(Orientation::Horizontal);
    let (id, spec) = match view {
        View::Size(value) => (
            format!("specimen-radio-group-size-{value}"),
            base.with_size(control_size(value)),
        ),
        View::Density(value) => (
            format!("specimen-radio-group-density-{value}"),
            base.with_density(control_density(value)),
        ),
        View::Examples => unreachable!("Examples uses the session"),
    };
    radio_group(&spec, ctx, RadioGroupHandlers::new(id))
}
