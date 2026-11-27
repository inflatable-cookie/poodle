use std::sync::Arc;

use poodle_headless::toggle_group::ToggleGroupValue;
use poodle_node::Node;
use poodle_render::{toggle_group, RenderContext, ToggleGroupHandlers};
use poodle_specs::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

use crate::{column, control_density, control_size, readout, section, Sink, View};

pub(crate) struct State {
    single: Option<String>,
    four: Option<String>,
    multiple: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            single: Some("grid".into()),
            four: Some("left".into()),
            multiple: vec!["design".into(), "docs".into()],
        }
    }
}

pub(crate) enum Event {
    Single(ToggleGroupValue),
    Four(ToggleGroupValue),
    Multiple(ToggleGroupValue),
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event {
        Event::Single(ToggleGroupValue::Single(value)) => state.single = value,
        Event::Four(ToggleGroupValue::Single(value)) => state.four = value,
        Event::Multiple(ToggleGroupValue::Multiple(values)) => state.multiple = values,
        _ => {}
    }
    state
}

fn single_options() -> Vec<ToggleGroupOption> {
    vec![
        ToggleGroupOption::new("grid", "Grid"),
        ToggleGroupOption::new("list", "List"),
        ToggleGroupOption::new("board", "Board"),
    ]
}

fn four_options() -> Vec<ToggleGroupOption> {
    vec![
        ToggleGroupOption::new("left", "Left"),
        ToggleGroupOption::new("center", "Center"),
        ToggleGroupOption::new("right", "Right"),
        ToggleGroupOption::new("justify", "Justify"),
    ]
}

fn multi_options() -> Vec<ToggleGroupOption> {
    vec![
        ToggleGroupOption::new("design", "Design"),
        ToggleGroupOption::new("engineering", "Engineering"),
        ToggleGroupOption::new("docs", "Docs"),
    ]
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let single_handler = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value| sink(Event::Single(value)))
            as Arc<dyn Fn(ToggleGroupValue) + Send + Sync>
    };
    let four_handler = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value| sink(Event::Four(value)))
            as Arc<dyn Fn(ToggleGroupValue) + Send + Sync>
    };
    let multi_handler = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value| sink(Event::Multiple(value)))
            as Arc<dyn Fn(ToggleGroupValue) + Send + Sync>
    };

    let single = section(
        "Single selection",
        [
            toggle_group(
                &ToggleGroupSpec::new(single_options())
                    .with_aria_label("View mode")
                    .with_value(state.single.clone().into_iter().collect()),
                ctx,
                ToggleGroupHandlers::new("toggle-group-single").on_value_change(single_handler),
            ),
            readout(
                format!("View: {}", state.single.as_deref().unwrap_or("")),
                ctx,
            ),
        ],
        ctx,
    );
    let four = section(
        "Four options",
        [toggle_group(
            &ToggleGroupSpec::new(four_options())
                .with_aria_label("Text alignment")
                .with_value(state.four.clone().into_iter().collect()),
            ctx,
            ToggleGroupHandlers::new("toggle-group-four").on_value_change(four_handler),
        )],
        ctx,
    );
    let multiple = section(
        "Multiple selection",
        [
            toggle_group(
                &ToggleGroupSpec::new(multi_options())
                    .with_aria_label("Filter tags")
                    .with_value(state.multiple.clone())
                    .with_selection_mode(ToggleGroupSelectionMode::Multiple),
                ctx,
                ToggleGroupHandlers::new("toggle-group-multiple").on_value_change(multi_handler),
            ),
            readout(
                format!(
                    "Selected: {}",
                    if state.multiple.is_empty() {
                        "none".into()
                    } else {
                        state.multiple.join(", ")
                    }
                ),
                ctx,
            ),
        ],
        ctx,
    );
    let disabled = section(
        "Disabled",
        [toggle_group(
            &ToggleGroupSpec::new(single_options())
                .with_aria_label("Disabled toggle group")
                .with_default_value(vec!["list".into()])
                .with_disabled(true),
            ctx,
            ToggleGroupHandlers::new("toggle-group-disabled"),
        )],
        ctx,
    );
    let disabled_item = section(
        "Disabled item",
        [toggle_group(
            &ToggleGroupSpec::new(vec![
                ToggleGroupOption::new("grid", "Grid"),
                ToggleGroupOption::new("list", "List").with_disabled(true),
                ToggleGroupOption::new("board", "Board"),
            ])
            .with_aria_label("Toggle group with disabled item")
            .with_default_value(vec!["grid".into()]),
            ctx,
            ToggleGroupHandlers::new("toggle-group-disabled-item"),
        )],
        ctx,
    );

    column(24.0, vec![single, four, multiple, disabled, disabled_item])
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = ToggleGroupSpec::new(single_options()).with_default_value(vec!["grid".into()]);
    let (id, spec) = match view {
        View::Size(value) => (
            format!("specimen-toggle-group-size-{value}"),
            base.with_size(control_size(value)),
        ),
        View::Density(value) => (
            format!("specimen-toggle-group-density-{value}"),
            base.with_density(control_density(value)),
        ),
        View::Examples => unreachable!("Examples uses the session"),
    };
    toggle_group(&spec, ctx, ToggleGroupHandlers::new(id))
}
