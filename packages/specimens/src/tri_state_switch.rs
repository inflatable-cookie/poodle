use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, Node};
use poodle_render::{tri_state_switch, RenderContext, TriStateSwitchHandlers};
use poodle_specs::{ControlSize, TriStateSwitchSpec, TriStateValue};

use crate::{column, control_density, control_size, readout, row, section, Sink, View};

pub(crate) struct State {
    filter: TriStateValue,
}

impl Default for State {
    fn default() -> Self {
        Self {
            filter: TriStateValue::Default,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Event {
    value: TriStateValue,
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    state.filter = event.value;
    state
}

fn centered_row(children: Vec<Node>) -> Node {
    let mut node = row(children);
    node.style.descriptor.layout.spacing.gap = 12.0;
    node.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    node
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let on_change = |sink: &Sink<Event>| {
        let sink = Arc::clone(sink);
        Arc::new(move |value| sink(Event { value })) as Arc<dyn Fn(TriStateValue) + Send + Sync>
    };

    let default = section(
        "Default",
        [
            tri_state_switch(
                &TriStateSwitchSpec::new()
                    .with_value(state.filter)
                    .with_aria_label("Filter mode"),
                ctx,
                TriStateSwitchHandlers::new("tri-state-filter").on_value_change(on_change(&sink)),
            ),
            readout(format!("Value: {}", state.filter.as_str()), ctx),
        ],
        ctx,
    );
    let custom_labels = section(
        "Custom labels",
        [tri_state_switch(
            &TriStateSwitchSpec::new()
                .with_excluded_label("Hide")
                .with_default_label("All")
                .with_included_label("Show")
                .with_aria_label("Visibility filter"),
            ctx,
            TriStateSwitchHandlers::new("tri-state-custom-labels"),
        )],
        ctx,
    );
    let semantic_sizes = section(
        "Semantic sizes",
        [centered_row(
            [
                (ControlSize::Xs, "tri-state-size-xs", "Extra small filter"),
                (ControlSize::Sm, "tri-state-size-sm", "Small filter"),
                (ControlSize::Md, "tri-state-size-md", "Medium filter"),
                (ControlSize::Lg, "tri-state-size-lg", "Large filter"),
                (ControlSize::Xl, "tri-state-size-xl", "Extra large filter"),
            ]
            .into_iter()
            .map(|(size, id, label)| {
                tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Default)
                        .with_size(size)
                        .with_aria_label(label),
                    ctx,
                    TriStateSwitchHandlers::new(id),
                )
            })
            .collect(),
        )],
        ctx,
    );
    let role_offset = section(
        "Chrome vs prominent role offset",
        [centered_row(vec![
            tri_state_switch(
                &TriStateSwitchSpec::new()
                    .with_value(TriStateValue::Excluded)
                    .with_size(ControlSize::Xs)
                    .with_aria_label("Chrome extra small filter"),
                ctx,
                TriStateSwitchHandlers::new("tri-state-role-xs"),
            ),
            tri_state_switch(
                &TriStateSwitchSpec::new()
                    .with_value(TriStateValue::Included)
                    .with_size(ControlSize::Sm)
                    .with_aria_label("Prominent small filter"),
                ctx,
                TriStateSwitchHandlers::new("tri-state-role-sm"),
            ),
            tri_state_switch(
                &TriStateSwitchSpec::new()
                    .with_value(TriStateValue::Default)
                    .with_size(ControlSize::Md)
                    .with_aria_label("Control medium filter"),
                ctx,
                TriStateSwitchHandlers::new("tri-state-role-md"),
            ),
        ])],
        ctx,
    );
    let disabled = section(
        "Disabled",
        [tri_state_switch(
            &TriStateSwitchSpec::new()
                .with_value(TriStateValue::Included)
                .with_disabled(true)
                .with_aria_label("Disabled switch"),
            ctx,
            TriStateSwitchHandlers::new("tri-state-disabled"),
        )],
        ctx,
    );
    let custom_colors = section(
        "Custom semantic colors",
        [tri_state_switch(
            &TriStateSwitchSpec::new()
                .with_value(state.filter)
                .with_excluded_color("#ef4444")
                .with_default_color("#64748b")
                .with_included_color("#22c55e")
                .with_aria_label("Filter mode"),
            ctx,
            TriStateSwitchHandlers::new("tri-state-custom-colors")
                .on_value_change(on_change(&sink)),
        )],
        ctx,
    );

    column(
        24.0,
        vec![
            default,
            custom_labels,
            semantic_sizes,
            role_offset,
            disabled,
            custom_colors,
        ],
    )
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let (id, spec) = match view {
        View::Size(value) => (
            format!("tri-state-axis-size-{value}"),
            TriStateSwitchSpec::new()
                .with_value(TriStateValue::Default)
                .with_size(control_size(value))
                .with_aria_label(format!("TriStateSwitch size {value}")),
        ),
        View::Density(value) => (
            format!("tri-state-axis-density-{value}"),
            TriStateSwitchSpec::new()
                .with_value(TriStateValue::Default)
                .with_density(control_density(value))
                .with_aria_label(format!("TriStateSwitch density {value}")),
        ),
        View::Examples => unreachable!("Examples uses the session"),
    };
    tri_state_switch(&spec, ctx, TriStateSwitchHandlers::new(id))
}
