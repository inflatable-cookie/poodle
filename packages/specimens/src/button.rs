use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{button, RenderContext};
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant};

use crate::{column, readout, row, section, Sink, View};

#[derive(Default)]
pub(crate) struct State {
    clicks: usize,
    last_clicked: Option<&'static str>,
    disclosure_open: bool,
    bookmarked: bool,
    submission: Option<&'static str>,
}

#[derive(Clone, Copy)]
pub(crate) enum Event {
    Clicked(&'static str),
    ToggleDisclosure,
    ToggleBookmark,
    Submitted(&'static str),
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event {
        Event::Clicked(label) => {
            state.clicks += 1;
            state.last_clicked = Some(label);
        }
        Event::ToggleDisclosure => state.disclosure_open = !state.disclosure_open,
        Event::ToggleBookmark => state.bookmarked = !state.bookmarked,
        Event::Submitted(action) => state.submission = Some(action),
    }
    state
}

fn spec(label: &str, variant: ButtonVariant) -> ButtonSpec {
    ButtonSpec::new().with_variant(variant).with_label(label)
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    use ButtonVariant::{Ghost, Primary, Secondary};

    let show = |spec| button(&spec, ctx, None);
    let live = |id: &str, spec, event| {
        let sink = Arc::clone(&sink);
        let mut node = button(&spec, ctx, Some(Arc::new(move || sink(event))));
        node.id = Some(format!("specimen-button-{id}"));
        node
    };
    let count = |id, label, variant| live(id, spec(label, variant), Event::Clicked(label));

    let normal = section(
        "A normal action row — the primary action, then the way out",
        [row(vec![
            count("save-changes", "Save changes", Primary),
            count("cancel", "Cancel", Ghost),
        ])],
        ctx,
    );
    let variants = section(
        "Variants — how much weight the action carries",
        [row(vec![
            count("primary", "Primary", Primary),
            count("secondary", "Secondary", Secondary),
            count("ghost", "Ghost", Ghost),
        ])],
        ctx,
    );
    let tones = [
        ("Default", ButtonTone::Default),
        ("Delete", ButtonTone::Danger),
        ("Approve", ButtonTone::Success),
        ("Override", ButtonTone::Warning),
    ]
    .into_iter()
    .map(|(label, tone)| show(spec(label, Secondary).with_tone(tone)))
    .collect();
    let tones = section("Tones — what kind of action it is", [row(tones)], ctx);
    let icons = section(
        "Icons, disclosure, and icon-only",
        [
            row(vec![
                show(spec("Create", Secondary).with_leading_icon("plus")),
                show(spec("Open", Secondary).with_trailing_icon("external-link")),
                show(
                    spec("Filter", Secondary)
                        .with_leading_icon("filter")
                        .with_chevron(true),
                ),
                show(
                    ButtonSpec::new()
                        .with_variant(Secondary)
                        .with_leading_icon("settings")
                        .with_aria_label("Settings"),
                ),
            ]),
            live(
                "sections",
                ButtonSpec::new()
                    .with_label("Sections")
                    .with_chevron(true)
                    .with_aria_expanded(state.disclosure_open),
                Event::ToggleDisclosure,
            ),
            readout(
                format!(
                    "ButtonSpec.aria_expanded = {:?}",
                    Some(state.disclosure_open)
                ),
                ctx,
            ),
        ],
        ctx,
    );
    let bookmark = if state.bookmarked {
        "Bookmarked"
    } else {
        "Bookmark"
    };
    let states = section(
        "States — unavailable, working, and held down",
        [row(vec![
            show(spec("Disabled", Primary).with_disabled(true)),
            show(spec("Loading", Primary).with_loading(true)),
            live(
                "bookmark",
                spec(bookmark, Secondary)
                    .with_leading_icon("star")
                    .with_pressed(state.bookmarked),
                Event::ToggleBookmark,
            ),
        ])],
        ctx,
    );
    let forms = section(
        "Inside a form — each button can submit somewhere else",
        [row(vec![
            live(
                "save",
                spec("Save", Secondary),
                Event::Submitted("Save (submit)"),
            ),
            live(
                "publish",
                spec("Publish", Primary),
                Event::Submitted("Publish (formaction)"),
            ),
        ])]
        .into_iter()
        .chain(state.submission.map(|action| readout(action, ctx))),
        ctx,
    );
    let mut clicks = column(2.0, vec![readout(format!("Clicks: {}", state.clicks), ctx)]);
    if let Some(label) = state.last_clicked {
        clicks
            .children
            .push(readout(format!("Last clicked: {label}"), ctx));
    }
    column(
        24.0,
        vec![normal, variants, tones, icons, states, forms, clicks],
    )
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let spec = match view {
        View::Size(size) => spec("Enabled", ButtonVariant::Primary).with_size(size),
        View::Density(density) => spec("Toggle", ButtonVariant::Primary).with_density(density),
        View::Examples => unreachable!("Examples uses the session"),
    };
    button(&spec, ctx, None)
}
