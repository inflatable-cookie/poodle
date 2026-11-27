use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{switch, RenderContext};
use poodle_specs::{SwitchSpec, SwitchTone};

use crate::{column, control_density, control_size, section, Sink, View};

#[derive(Clone, Copy)]
pub(crate) enum Item {
    DarkMode,
    AutoSave,
    Compact,
}

pub(crate) struct State {
    dark_mode: bool,
    auto_save: bool,
    compact: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            dark_mode: true,
            auto_save: false,
            compact: true,
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
        Item::DarkMode => state.dark_mode = event.checked,
        Item::AutoSave => state.auto_save = event.checked,
        Item::Compact => state.compact = event.checked,
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
    let mut spec = SwitchSpec::new().with_checked(checked);
    spec.label = Some(label.to_string());
    let sink = Arc::clone(sink);
    let mut node = switch(
        &spec,
        ctx,
        Some(Arc::new(move |checked| sink(Event { item, checked }))),
    );
    node.id = Some(id.to_string());
    node
}

fn static_switch(id: &str, spec: SwitchSpec, ctx: &RenderContext<'_>) -> Node {
    let mut node = switch(&spec, ctx, None);
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
                    "switch-dark-mode",
                    Item::DarkMode,
                    "Dark mode",
                    state.dark_mode,
                    &sink,
                    ctx,
                ),
                live(
                    "switch-auto-save",
                    Item::AutoSave,
                    "Auto-save drafts",
                    state.auto_save,
                    &sink,
                    ctx,
                ),
                live(
                    "switch-compact",
                    Item::Compact,
                    "Compact view",
                    state.compact,
                    &sink,
                    ctx,
                ),
            ],
        )],
        ctx,
    );

    let mut disabled_off = SwitchSpec::new().with_checked(false);
    disabled_off.label = Some("Disabled off".into());
    disabled_off.is_disabled = true;

    let mut disabled_on = SwitchSpec::new().with_checked(true);
    disabled_on.label = Some("Disabled on".into());
    disabled_on.is_disabled = true;

    let mut readonly_on = SwitchSpec::new().with_checked(true);
    readonly_on.label = Some("Read-only on".into());
    readonly_on.is_read_only = true;

    let states = section(
        "States",
        [column(
            10.0,
            vec![
                static_switch("sw-disabled-off", disabled_off, ctx),
                static_switch("sw-disabled-on", disabled_on, ctx),
                static_switch("sw-readonly-on", readonly_on, ctx),
            ],
        )],
        ctx,
    );

    let mut custom_green = SwitchSpec::new()
        .with_checked(true)
        .with_on_color("#22c55e")
        .with_off_color("#cbd5e1");
    custom_green.label = Some("Billing alerts".into());

    let mut custom_amber = SwitchSpec::new()
        .with_checked(false)
        .with_on_color("#f59e0b")
        .with_off_color("#94a3b8");
    custom_amber.label = Some("Quiet mode".into());

    let custom = section(
        "Custom colors",
        [column(
            10.0,
            vec![
                static_switch("sw-custom-green", custom_green, ctx),
                static_switch("sw-custom-amber", custom_amber, ctx),
            ],
        )],
        ctx,
    );

    let dual = section(
        "Dual labels and tones",
        [column(
            10.0,
            vec![
                static_switch(
                    "sw-dual-publish",
                    SwitchSpec::new()
                        .with_default_checked(true)
                        .with_left_label("Draft")
                        .with_right_label("Live")
                        .with_left_tone(SwitchTone::Danger)
                        .with_right_tone(SwitchTone::Success)
                        .with_aria_label("Publication status"),
                    ctx,
                ),
                static_switch(
                    "sw-dual-access",
                    SwitchSpec::new()
                        .with_default_checked(false)
                        .with_left_label("Restricted")
                        .with_right_label("Free")
                        .with_left_tone(SwitchTone::Warning)
                        .with_right_tone(SwitchTone::Success)
                        .with_aria_label("Access status"),
                    ctx,
                ),
            ],
        )],
        ctx,
    );

    column(24.0, vec![default, states, custom, dual])
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let (id, spec) = match view {
        View::Size(value) => {
            let size = control_size(value);
            let mut spec = SwitchSpec::new().with_checked(true).with_size(size);
            spec.label = Some(format!("{size:?}"));
            (format!("specimen-switch-size-{value}"), spec)
        }
        View::Density(value) => {
            let mut spec = SwitchSpec::new()
                .with_checked(true)
                .with_density(control_density(value));
            spec.label = Some("Option".into());
            (format!("specimen-switch-density-{value}"), spec)
        }
        View::Examples => unreachable!("Examples uses the session"),
    };
    static_switch(&id, spec, ctx)
}
