use std::sync::Arc;

use poodle_headless::rating::trim_rating_fraction;
use poodle_node::Node;
use poodle_render::{rating, RatingHandlers, RenderContext};
use poodle_specs::RatingSpec;

use crate::{column, control_density, control_size, readout, section, Sink, View};

pub(crate) struct State {
    half: Option<f64>,
    whole: Option<f64>,
    clear: Option<f64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            half: Some(3.5),
            whole: Some(3.0),
            clear: Some(4.0),
        }
    }
}

pub(crate) enum Event {
    Half(Option<f64>),
    Whole(Option<f64>),
    Clear(Option<f64>),
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event {
        Event::Half(value) => state.half = value,
        Event::Whole(value) => state.whole = value,
        Event::Clear(value) => state.clear = value,
    }
    state
}

fn format_whole(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("{value} / 5"),
        None => "none / 5".into(),
    }
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let mut half_spec = RatingSpec::new()
        .with_allow_clear(true)
        .with_aria_label("Half-step rating");
    if let Some(value) = state.half {
        half_spec = half_spec.with_value(value);
    }
    let half_change = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value| sink(Event::Half(value))) as Arc<dyn Fn(Option<f64>) + Send + Sync>
    };
    let half = section(
        "Half-step (default)",
        [
            rating(
                &half_spec,
                ctx,
                RatingHandlers::new("specimen-half").on_change(half_change),
            ),
            readout(
                match state.half {
                    Some(value) => format!("{} / 5", trim_rating_fraction(value)),
                    None => "none / 5".into(),
                },
                ctx,
            ),
        ],
        ctx,
    );

    let mut whole_spec = RatingSpec::new()
        .with_step(1.0)
        .with_aria_label("Whole-step rating");
    if let Some(value) = state.whole {
        whole_spec = whole_spec.with_value(value);
    }
    let whole_change = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value| sink(Event::Whole(value))) as Arc<dyn Fn(Option<f64>) + Send + Sync>
    };
    let whole = section(
        "Whole-step",
        [
            rating(
                &whole_spec,
                ctx,
                RatingHandlers::new("specimen-whole").on_change(whole_change),
            ),
            readout(format_whole(state.whole), ctx),
        ],
        ctx,
    );

    let ten_star = section(
        "10-star scale",
        [rating(
            &RatingSpec::new()
                .with_default_value(7.0)
                .with_max(10)
                .with_step(1.0)
                .with_aria_label("Score out of 10"),
            ctx,
            RatingHandlers::new("specimen-scale-10"),
        )],
        ctx,
    );

    let fraction = section(
        "Arbitrary fraction display",
        [
            rating(
                &RatingSpec::new()
                    .with_value(3.7)
                    .with_aria_label("Display fraction"),
                ctx,
                RatingHandlers::new("specimen-display-37"),
            ),
            readout("3.7 / 5 (display only)", ctx),
        ],
        ctx,
    );

    let mut clear_spec = RatingSpec::new()
        .with_step(1.0)
        .with_allow_clear(true)
        .with_aria_label("Clearable rating");
    if let Some(value) = state.clear {
        clear_spec = clear_spec.with_value(value);
    }
    let clear_change = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value| sink(Event::Clear(value))) as Arc<dyn Fn(Option<f64>) + Send + Sync>
    };
    let clearable = section(
        "Clearable",
        [
            rating(
                &clear_spec,
                ctx,
                RatingHandlers::new("specimen-clear").on_change(clear_change),
            ),
            readout(format_whole(state.clear), ctx),
        ],
        ctx,
    );

    let disabled = section(
        "Disabled",
        [rating(
            &RatingSpec::new()
                .with_default_value(2.0)
                .with_step(1.0)
                .with_disabled(true)
                .with_aria_label("Disabled rating"),
            ctx,
            RatingHandlers::new("specimen-disabled"),
        )],
        ctx,
    );

    column(
        24.0,
        vec![half, whole, ten_star, fraction, clearable, disabled],
    )
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = RatingSpec::new().with_value(3.0).with_step(1.0);
    let (id, spec) = match view {
        View::Size(value) => (
            format!("specimen-rating-size-{value}"),
            base.with_size(control_size(value)),
        ),
        View::Density(value) => (
            format!("specimen-rating-density-{value}"),
            base.with_density(control_density(value)),
        ),
        View::Examples => unreachable!("Examples uses the session"),
    };
    rating(&spec, ctx, RatingHandlers::new(id))
}
