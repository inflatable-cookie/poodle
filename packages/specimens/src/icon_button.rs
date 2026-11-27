use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{icon_button, icon_button_with_handlers, IconButtonHandlers, RenderContext};
use poodle_specs::{ButtonTone, ButtonVariant, IconButtonSpec};

use crate::{column, control_density, control_size, readout, row, section, Sink, View};

#[derive(Default)]
pub(crate) struct State {
    last_action: Option<&'static str>,
    bold: bool,
    italic: bool,
    underline: bool,
    pinned: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum Event {
    Action(&'static str),
    Bold(bool),
    Italic(bool),
    Underline(bool),
    Pinned(bool),
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event {
        Event::Action(label) => state.last_action = Some(label),
        Event::Bold(value) => state.bold = value,
        Event::Italic(value) => state.italic = value,
        Event::Underline(value) => state.underline = value,
        Event::Pinned(value) => state.pinned = value,
    }
    state
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let action_btn = |id: &str, spec: IconButtonSpec, label: &'static str| {
        let sink = Arc::clone(&sink);
        let mut node = icon_button(
            &spec,
            ctx,
            Some(Arc::new(move || sink(Event::Action(label)))),
        );
        node.id = Some(format!("specimen-icon-button-{id}"));
        node
    };
    let toggle_btn = |id: &str, icon: &str, label: &str, pressed: bool, wrap: fn(bool) -> Event| {
        let sink = Arc::clone(&sink);
        let mut node = icon_button_with_handlers(
            &IconButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_icon(icon)
                .with_pressed(pressed)
                .with_aria_label(label),
            ctx,
            IconButtonHandlers {
                on_click: None,
                on_pressed_change: Some(Arc::new(move |value| sink(wrap(value)))),
            },
        );
        node.id = Some(format!("specimen-icon-button-{id}"));
        node
    };

    let variants = section(
        "Variants",
        [row(vec![
            action_btn(
                "add",
                IconButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_icon("plus")
                    .with_aria_label("Add"),
                "Add",
            ),
            action_btn(
                "settings",
                IconButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_icon("settings")
                    .with_aria_label("Settings"),
                "Settings",
            ),
            action_btn(
                "close",
                IconButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_icon("x")
                    .with_aria_label("Close"),
                "Close",
            ),
        ])],
        ctx,
    );

    let tone_row = |tone: ButtonTone, icon: &str, label: &str| {
        row([
            ButtonVariant::Primary,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
        ]
        .into_iter()
        .map(|variant| {
            icon_button(
                &IconButtonSpec::new()
                    .with_variant(variant)
                    .with_tone(tone)
                    .with_icon(icon)
                    .with_aria_label(label),
                ctx,
                None,
            )
        })
        .collect())
    };
    let danger = section(
        "Danger tone",
        [tone_row(ButtonTone::Danger, "trash-2", "Delete")],
        ctx,
    );
    let success = section(
        "Success tone",
        [tone_row(ButtonTone::Success, "check", "Approve")],
        ctx,
    );

    let toggle = section(
        "Toggle (text editor toolbar)",
        [row(vec![
            toggle_btn("bold", "bold", "Bold", state.bold, Event::Bold),
            toggle_btn("italic", "italic", "Italic", state.italic, Event::Italic),
            toggle_btn(
                "underline",
                "underline",
                "Underline",
                state.underline,
                Event::Underline,
            ),
        ])],
        ctx,
    );

    let pin_click = Arc::clone(&sink);
    let pin_toggle = Arc::clone(&sink);
    let mut pin_node = icon_button_with_handlers(
        &IconButtonSpec::new()
            .with_icon("map-pin")
            .with_pressed(state.pinned)
            .with_aria_label("Pin"),
        ctx,
        IconButtonHandlers {
            on_click: Some(Arc::new(move || pin_click(Event::Action("Pin toggled")))),
            on_pressed_change: Some(Arc::new(move |value| pin_toggle(Event::Pinned(value)))),
        },
    );
    pin_node.id = Some("specimen-icon-button-pin".into());

    let states = section(
        "States",
        [row(vec![
            pin_node,
            icon_button(
                &IconButtonSpec::new()
                    .with_icon("settings")
                    .with_disabled(true)
                    .with_aria_label("Settings"),
                ctx,
                None,
            ),
            icon_button(
                &IconButtonSpec::new()
                    .with_icon("loader")
                    .with_loading(true)
                    .with_aria_label("Loading"),
                ctx,
                None,
            ),
        ])],
        ctx,
    );

    let string_names = section(
        "String name (built-in internals)",
        [row(vec![
            icon_button(
                &IconButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_icon("plus")
                    .with_aria_label("Add"),
                ctx,
                None,
            ),
            icon_button(
                &IconButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_icon("search")
                    .with_aria_label("Search"),
                ctx,
                None,
            ),
            icon_button(
                &IconButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_icon("x")
                    .with_aria_label("Close"),
                ctx,
                None,
            ),
        ])],
        ctx,
    );

    let mut children = vec![variants, danger, success, toggle, states, string_names];
    if let Some(last) = state.last_action {
        children.push(readout(format!("Last action: {last}"), ctx));
    }
    column(24.0, children)
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = IconButtonSpec::new()
        .with_icon("star")
        .with_aria_label("Star");
    let spec = match view {
        View::Size(value) => base.with_size(control_size(value)),
        View::Density(value) => base.with_density(control_density(value)),
        View::Examples => unreachable!("Examples uses the session"),
    };
    icon_button(&spec, ctx, None)
}
