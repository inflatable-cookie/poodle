use std::sync::Arc;

use poodle_node::{LayoutSizing, Node};
use poodle_render::{segmented_control, RenderContext};
use poodle_specs::{ControlSize, SegmentedControlOption, SegmentedControlSpec};

use crate::{column, control_density, control_size, readout, section, Sink, View};

pub(crate) struct State {
    value: String,
    icon_value: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: "grid".into(),
            icon_value: "effects".into(),
        }
    }
}

pub(crate) enum Event {
    Value(String),
    IconValue(String),
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event {
        Event::Value(value) => state.value = value,
        Event::IconValue(value) => state.icon_value = value,
    }
    state
}

fn view_options() -> Vec<SegmentedControlOption> {
    vec![
        SegmentedControlOption::new("grid", "Grid"),
        SegmentedControlOption::new("list", "List"),
        SegmentedControlOption::new("table", "Table"),
    ]
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let default_change = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value: &str| sink(Event::Value(value.to_string())))
            as Arc<dyn Fn(&str) + Send + Sync>
    };
    let icon_change = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value: &str| sink(Event::IconValue(value.to_string())))
            as Arc<dyn Fn(&str) + Send + Sync>
    };

    let default = section(
        "Default",
        [
            segmented_control(
                &SegmentedControlSpec::new("default", view_options())
                    .with_default_value(&state.value),
                ctx,
                Some(default_change),
            ),
            readout(format!("Selected: {}", state.value), ctx),
        ],
        ctx,
    );

    let disabled_opt = section(
        "With disabled option",
        [segmented_control(
            &SegmentedControlSpec::new(
                "disabled-option",
                vec![
                    SegmentedControlOption::new("all", "All"),
                    SegmentedControlOption::new("active", "Active"),
                    SegmentedControlOption::new("archived", "Archived"),
                    SegmentedControlOption::new("draft", "Draft").with_disabled(true),
                ],
            )
            .with_default_value("all"),
            ctx,
            None,
        )],
        ctx,
    );

    let mut fully_disabled_spec =
        SegmentedControlSpec::new("fully-disabled", view_options()).with_default_value("list");
    fully_disabled_spec.is_disabled = true;
    let fully_disabled = section(
        "Fully disabled",
        [segmented_control(&fully_disabled_spec, ctx, None)],
        ctx,
    );

    let mut equal_width_spec = SegmentedControlSpec::new(
        "equal-width",
        vec![
            SegmentedControlOption::new("day", "Day"),
            SegmentedControlOption::new("week", "Week"),
            SegmentedControlOption::new("month", "Month"),
            SegmentedControlOption::new("year", "Year"),
        ],
    )
    .with_default_value("week")
    .with_equal_width(true);
    equal_width_spec.aria_label = Some("Time range".into());
    let mut equal_box = column(0.0, vec![segmented_control(&equal_width_spec, ctx, None)]);
    equal_box.style.descriptor.layout.width = LayoutSizing::Fixed(360.0);
    let equal_width = section("Equal width segments", [equal_box], ctx);

    let mut content_fit_spec = SegmentedControlSpec::new(
        "content-fit",
        vec![
            SegmentedControlOption::new("1h", "1h").with_aria_label("Last 1 hour"),
            SegmentedControlOption::new("6h", "6h").with_aria_label("Last 6 hours"),
            SegmentedControlOption::new("24h", "24h").with_aria_label("Last 24 hours"),
        ],
    )
    .with_default_value("24h")
    .with_size(ControlSize::Xs)
    .with_equal_width(false);
    content_fit_spec.aria_label = Some("Timeline window".into());
    let content_fit = section(
        "Content fit (equalWidth=false)",
        [segmented_control(&content_fit_spec, ctx, None)],
        ctx,
    );

    let mut icon_only_spec = SegmentedControlSpec::new(
        "icon-only",
        vec![
            SegmentedControlOption::new("effects", "Effects")
                .with_icon("audio-waveform")
                .with_icon_only(true),
            SegmentedControlOption::new("instruments", "Instruments")
                .with_icon("piano")
                .with_icon_only(true),
        ],
    )
    .with_default_value(&state.icon_value)
    .with_size(ControlSize::Sm)
    .with_equal_width(false);
    icon_only_spec.aria_label = Some("Plugin kind".into());
    let icon_only = section(
        "Icon-only options",
        [segmented_control(&icon_only_spec, ctx, Some(icon_change))],
        ctx,
    );

    column(
        24.0,
        vec![
            default,
            disabled_opt,
            fully_disabled,
            equal_width,
            content_fit,
            icon_only,
        ],
    )
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let spec = match view {
        View::Size(value) => SegmentedControlSpec::new(format!("size-{value}"), view_options())
            .with_default_value("grid")
            .with_size(control_size(value)),
        View::Density(value) => {
            SegmentedControlSpec::new(format!("density-{value}"), view_options())
                .with_default_value("grid")
                .with_density(control_density(value))
        }
        View::Examples => unreachable!("Examples uses the session"),
    };
    segmented_control(&spec, ctx, None)
}
