use std::sync::Arc;

use poodle_headless::select::SelectContext;
use poodle_node::Node;
use poodle_render::{
    apply_select_intent, select, select_search_focus_id, select_trigger_focus_id, RenderContext,
    SelectHandlers, SelectIntent, SelectTransitionResult,
};
use poodle_specs::{ChoiceOption, SelectMode, SelectSpec};

use crate::{column, control_density, control_size, readout, section, Effect, Sink, View};

#[derive(Clone, Copy)]
pub(crate) enum Instance {
    Native,
    Custom,
    Searchable,
    Freeform,
    Rich,
    Grouped,
    Clearable,
}

impl Instance {
    fn scope(self) -> &'static str {
        match self {
            Self::Native => "select-native",
            Self::Custom => "select-custom",
            Self::Searchable => "select-searchable",
            Self::Freeform => "select-freeform",
            Self::Rich => "select-rich",
            Self::Grouped => "select-native-grouped",
            Self::Clearable => "select-clearable",
        }
    }

    fn base_spec(self) -> SelectSpec {
        match self {
            Self::Native => SelectSpec::new(fruit()).with_placeholder("Choose a fruit"),
            Self::Custom => {
                custom(SelectSpec::new(countries()).with_placeholder("Choose a country"))
            }
            Self::Searchable => SelectSpec {
                searchable: true,
                ..SelectSpec::new(frameworks()).with_placeholder("Search frameworks...")
            },
            Self::Freeform => SelectSpec {
                searchable: true,
                freeform: true,
                ..SelectSpec::new(frameworks()).with_placeholder("Type or select...")
            },
            Self::Rich => custom(SelectSpec::new(countries()).with_placeholder("Choose a country")),
            Self::Grouped => SelectSpec::new(food()).with_placeholder("Choose a food"),
            Self::Clearable => custom(
                SelectSpec::new(fruit())
                    .with_placeholder("All fruits")
                    .with_clearable(true),
            ),
        }
    }
}

#[derive(Default)]
struct InstanceState {
    context: Option<SelectContext>,
    caret: (usize, usize),
}

#[derive(Default)]
pub(crate) struct State {
    instances: [InstanceState; 7],
}

impl State {
    fn instance(&self, instance: Instance) -> &InstanceState {
        &self.instances[instance as usize]
    }

    fn value(&self, instance: Instance) -> Option<&str> {
        self.instance(instance)
            .context
            .as_ref()
            .map(|context| context.value.as_str())
            .filter(|value| !value.is_empty())
    }
}

pub(crate) struct Event {
    instance: Instance,
    intent: SelectIntent,
}

pub(crate) fn reduce(mut state: State, event: Event) -> (State, Option<Effect>) {
    let entry = &mut state.instances[event.instance as usize];
    let current = entry
        .context
        .take()
        .unwrap_or_else(|| event.instance.base_spec().select_context());
    let was_open = current.open;
    let prev_query = current.query.clone();
    let SelectTransitionResult {
        context,
        search_selection,
        ..
    } = apply_select_intent(current, entry.caret, event.intent);

    let focus = if was_open != context.open {
        let scope = event.instance.scope();
        Some(Effect::Focus(if context.open && context.searchable {
            select_search_focus_id(scope)
        } else {
            select_trigger_focus_id(scope)
        }))
    } else {
        None
    };

    if let Some(caret) = search_selection {
        entry.caret = caret;
    } else if prev_query != context.query {
        let end = context.query.chars().count();
        entry.caret = (end, end);
    }
    entry.context = Some(context);
    (state, focus)
}

fn live(instance: Instance, state: &State, sink: &Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let mut spec = instance.base_spec();
    let entry = state.instance(instance);
    if let Some(context) = &entry.context {
        spec = spec.applying_context(context);
    }
    let (start, end) = entry.caret;
    spec = spec.with_search_selection(start, end);
    let sink = Arc::clone(sink);
    let handlers = SelectHandlers::new(instance.scope()).on_intent(Arc::new(move |intent| {
        sink(Event { instance, intent });
    }));
    select(&spec, ctx, &handlers)
}

fn fruit() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
        ChoiceOption::new("dragonfruit", "Dragonfruit"),
        ChoiceOption::new("elderberry", "Elderberry"),
    ]
}

fn countries() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("us", "United States").with_description("North America"),
        ChoiceOption::new("uk", "United Kingdom").with_description("Europe"),
        ChoiceOption::new("jp", "Japan").with_description("Asia"),
        ChoiceOption::new("au", "Australia").with_description("Oceania"),
        ChoiceOption::new("br", "Brazil").with_description("South America"),
    ]
}

fn frameworks() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("svelte", "Svelte"),
        ChoiceOption::new("react", "React"),
        ChoiceOption::new("vue", "Vue"),
        ChoiceOption::new("angular", "Angular"),
        ChoiceOption::new("solid", "SolidJS"),
        ChoiceOption::new("astro", "Astro"),
    ]
}

fn food() -> Vec<ChoiceOption> {
    let mut spinach = ChoiceOption::new("spinach", "Spinach").with_group("Vegetables");
    spinach.is_disabled = true;
    vec![
        ChoiceOption::new("apple", "Apple").with_group("Fruits"),
        ChoiceOption::new("banana", "Banana").with_group("Fruits"),
        ChoiceOption::new("cherry", "Cherry").with_group("Fruits"),
        ChoiceOption::new("carrot", "Carrot").with_group("Vegetables"),
        ChoiceOption::new("broccoli", "Broccoli").with_group("Vegetables"),
        spinach,
    ]
}

fn custom(spec: SelectSpec) -> SelectSpec {
    SelectSpec {
        mode: SelectMode::Custom,
        ..spec
    }
}

fn narrow(gap: f32, children: Vec<Node>) -> Node {
    let mut body = column(gap, children);
    body.style.max_width = Some(320.0);
    body
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    use Instance::{Clearable, Custom, Freeform, Grouped, Native, Rich, Searchable};

    let show = |instance| live(instance, state, &sink, ctx);
    let with_value = |instance| {
        let mut children = vec![show(instance)];
        if let Some(value) = state.value(instance) {
            children.push(readout(format!("Selected: {value}"), ctx));
        }
        narrow(6.0, children)
    };
    let native = section("Native select", [with_value(Native)], ctx);
    let dropdown = section("Custom dropdown", [with_value(Custom)], ctx);
    let freeform_context = state.instance(Freeform).context.as_ref();
    let value = state.value(Freeform).unwrap_or("—");
    let query = freeform_context.map_or("—", |context| context.query.as_str());
    let open = freeform_context.is_some_and(|context| context.open);
    let search = section(
        "Search and freeform entry",
        [narrow(
            12.0,
            vec![
                with_value(Searchable),
                column(
                    6.0,
                    vec![
                        show(Freeform),
                        readout(
                            format!("value: {value} · query: {query} · open: {open}"),
                            ctx,
                        ),
                    ],
                ),
            ],
        )],
        ctx,
    );
    let choices = section(
        "Rich and grouped options",
        [narrow(12.0, vec![show(Rich), show(Grouped)])],
        ctx,
    );
    let clearable = section(
        "Clearable selection",
        [narrow(0.0, vec![show(Clearable)])],
        ctx,
    );
    let disabled = SelectSpec {
        is_disabled: true,
        ..SelectSpec::new(fruit())
            .with_placeholder("Choose a fruit")
            .with_value("banana")
    };
    let disabled = section(
        "Disabled",
        [narrow(
            0.0,
            vec![select(
                &disabled,
                ctx,
                &SelectHandlers::new("select-static"),
            )],
        )],
        ctx,
    );
    column(
        24.0,
        vec![native, dropdown, search, choices, clearable, disabled],
    )
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let spec = SelectSpec::new(fruit()).with_placeholder("Select...");
    let (spec, axis, label) = match view {
        View::Size(value) => (spec.with_size(control_size(value)), "size", value),
        View::Density(value) => (spec.with_density(control_density(value)), "density", value),
        View::Examples => unreachable!("Examples uses the session"),
    };
    select(
        &spec.with_aria_label(format!("{label} select")),
        ctx,
        &SelectHandlers::new(format!("select-{axis}-{label}")),
    )
}
