use std::sync::Arc;

use poodle_node::Node;
use poodle_render::{split_button, RenderContext, SplitButtonHandlers};
use poodle_specs::{ButtonTone, ButtonVariant, SplitButtonSpec, SplitMenuItem};

use crate::{column, control_density, control_size, readout, row, section, Sink, View};

#[derive(Default)]
pub(crate) struct State {
    last_action: Option<String>,
}

pub(crate) struct Event {
    action: String,
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    state.last_action = Some(event.action);
    state
}

fn action_cb(sink: &Sink<Event>, label: &'static str) -> Arc<dyn Fn() + Send + Sync> {
    let sink = Arc::clone(sink);
    Arc::new(move || {
        sink(Event {
            action: label.to_string(),
        });
    })
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let on_menu_action = {
        let sink = Arc::clone(&sink);
        Arc::new(move |value: &str| {
            sink(Event {
                action: value.to_string(),
            });
        }) as Arc<dyn Fn(&str) + Send + Sync>
    };
    let mut save_node = split_button(
        &SplitButtonSpec::new()
            .with_variant(ButtonVariant::Primary)
            .with_label("Save")
            .with_items(vec![
                SplitMenuItem::action("save-draft", "Save as draft"),
                SplitMenuItem::action("save-template", "Save as template"),
                SplitMenuItem::Separator,
                SplitMenuItem::action("discard", "Discard changes"),
            ]),
        ctx,
        SplitButtonHandlers {
            on_click: Some(action_cb(&sink, "Save")),
            on_dropdown: Some(action_cb(&sink, "dropdown: toggle")),
            on_action: Some(on_menu_action),
            ..SplitButtonHandlers::default()
        },
    );
    save_node.id = Some("specimen-split-button-save".into());
    let save = section("Save split action", [save_node], ctx);

    let export = section(
        "Secondary export",
        [split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label("Export")
                .with_items(vec![
                    SplitMenuItem::action("csv", "Export as CSV"),
                    SplitMenuItem::action("json", "Export as JSON"),
                    SplitMenuItem::action("pdf", "Export as PDF"),
                ]),
            ctx,
            SplitButtonHandlers {
                on_click: Some(action_cb(&sink, "Export")),
                on_dropdown: Some(action_cb(&sink, "dropdown: toggle")),
                ..SplitButtonHandlers::default()
            },
        )],
        ctx,
    );

    let mut tones_row = row(vec![
        split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_tone(ButtonTone::Danger)
                .with_label("Delete")
                .with_items(vec![
                    SplitMenuItem::action("delete-selected", "Delete selected"),
                    SplitMenuItem::action("delete-all", "Delete all"),
                ]),
            ctx,
            SplitButtonHandlers {
                on_click: Some(action_cb(&sink, "Delete")),
                ..SplitButtonHandlers::default()
            },
        ),
        split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_tone(ButtonTone::Success)
                .with_label("Publish")
                .with_items(vec![
                    SplitMenuItem::action("publish-now", "Publish now"),
                    SplitMenuItem::action("schedule-publish", "Schedule"),
                ]),
            ctx,
            SplitButtonHandlers {
                on_click: Some(action_cb(&sink, "Publish")),
                ..SplitButtonHandlers::default()
            },
        ),
        split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_tone(ButtonTone::Warning)
                .with_label("Archive")
                .with_items(vec![
                    SplitMenuItem::action("archive-selected", "Archive selected"),
                    SplitMenuItem::action("archive-all", "Archive all"),
                ]),
            ctx,
            SplitButtonHandlers {
                on_click: Some(action_cb(&sink, "Archive")),
                ..SplitButtonHandlers::default()
            },
        ),
    ]);
    tones_row.style.descriptor.layout.spacing.gap = 12.0;
    let tones = section("Intent tones", [tones_row], ctx);

    let mut states_row = row(vec![
        split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_label("Saving...")
                .with_loading(true),
            ctx,
            SplitButtonHandlers::default(),
        ),
        split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label("Save")
                .with_disabled(true),
            ctx,
            SplitButtonHandlers::default(),
        ),
    ]);
    states_row.style.descriptor.layout.spacing.gap = 12.0;
    let states = section("Loading and disabled states", [states_row], ctx);

    let last = state.last_action.as_deref().unwrap_or("(none)");
    column(
        24.0,
        vec![
            save,
            export,
            tones,
            states,
            readout(format!("Last action: {last}"), ctx),
        ],
    )
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let base = SplitButtonSpec::new()
        .with_variant(ButtonVariant::Secondary)
        .with_label("Action")
        .with_items(vec![
            SplitMenuItem::action("a", "Action A"),
            SplitMenuItem::action("b", "Action B"),
        ]);
    let spec = match view {
        View::Size(value) => base.with_size(control_size(value)),
        View::Density(value) => base.with_density(control_density(value)),
        View::Examples => unreachable!("Examples uses the session"),
    };
    split_button(&spec, ctx, SplitButtonHandlers::default())
}
