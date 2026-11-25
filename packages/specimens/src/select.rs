use std::sync::Arc;

use poodle_headless::select::SelectContext;
use poodle_node::Node;
use poodle_render::{
    select, select_search_focus_id, select_trigger_focus_id, RenderContext, SelectHandlers,
    SelectTransitionResult,
};
use poodle_specs::{ChoiceOption, SelectMode, SelectSpec};

use crate::{column, readout, section, Effect, Sink, View, DENSITIES, SIZES};

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
    transition: SelectTransitionResult,
}

pub(crate) fn reduce(mut state: State, event: Event) -> (State, Option<Effect>) {
    let entry = &mut state.instances[event.instance as usize];
    let previous = entry.context.as_ref();
    let SelectTransitionResult {
        context,
        search_selection,
        ..
    } = event.transition;

    let focus = if previous.is_some_and(|previous| previous.open) != context.open {
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
    } else if previous.map_or("", |previous| previous.query.as_str()) != context.query {
        let end = context.query.chars().count();
        entry.caret = (end, end);
    }
    entry.context = Some(context);
    (state, focus)
}

fn live(
    instance: Instance,
    mut spec: SelectSpec,
    state: &State,
    sink: &Sink<Event>,
    ctx: &RenderContext<'_>,
) -> Node {
    let entry = state.instance(instance);
    if let Some(context) = &entry.context {
        spec = spec.applying_context(context);
    }
    let (start, end) = entry.caret;
    spec = spec.with_search_selection(start, end);
    let sink = Arc::clone(sink);
    let handlers =
        SelectHandlers::new(instance.scope()).on_transition(Arc::new(move |transition| {
            sink(Event {
                instance,
                transition,
            });
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

    let show = |instance, spec| live(instance, spec, state, &sink, ctx);
    let with_value = |instance, spec| {
        let mut children = vec![show(instance, spec)];
        if let Some(value) = state.value(instance) {
            children.push(readout(format!("Selected: {value}"), ctx));
        }
        narrow(6.0, children)
    };
    let native = section(
        "Native select",
        [with_value(
            Native,
            SelectSpec::new(fruit()).with_placeholder("Choose a fruit"),
        )],
        ctx,
    );
    let dropdown = section(
        "Custom dropdown",
        [with_value(
            Custom,
            custom(SelectSpec::new(countries()).with_placeholder("Choose a country")),
        )],
        ctx,
    );
    let search = SelectSpec {
        searchable: true,
        ..SelectSpec::new(frameworks()).with_placeholder("Search frameworks...")
    };
    let freeform = SelectSpec {
        searchable: true,
        freeform: true,
        ..SelectSpec::new(frameworks()).with_placeholder("Type or select...")
    };
    let freeform_context = state.instance(Freeform).context.as_ref();
    let value = state.value(Freeform).unwrap_or("—");
    let query = freeform_context.map_or("—", |context| context.query.as_str());
    let open = freeform_context.is_some_and(|context| context.open);
    let search = section(
        "Search and freeform entry",
        [narrow(
            12.0,
            vec![
                with_value(Searchable, search),
                column(
                    6.0,
                    vec![
                        show(Freeform, freeform),
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
        [narrow(
            12.0,
            vec![
                show(
                    Rich,
                    custom(SelectSpec::new(countries()).with_placeholder("Choose a country")),
                ),
                show(
                    Grouped,
                    SelectSpec::new(food()).with_placeholder("Choose a food"),
                ),
            ],
        )],
        ctx,
    );
    let clearable = section(
        "Clearable selection",
        [narrow(
            0.0,
            vec![show(
                Clearable,
                custom(
                    SelectSpec::new(fruit())
                        .with_placeholder("All fruits")
                        .with_clearable(true),
                ),
            )],
        )],
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
        View::Size(size) => (
            spec.with_size(size),
            "size",
            SIZES.iter().find(|(value, _)| *value == size).unwrap().1,
        ),
        View::Density(density) => (
            spec.with_density(density),
            "density",
            DENSITIES
                .iter()
                .find(|(value, _)| *value == density)
                .unwrap()
                .1,
        ),
        View::Examples => unreachable!("Examples uses the session"),
    };
    select(
        &spec.with_aria_label(format!("{label} select")),
        ctx,
        &SelectHandlers::new(format!("select-{axis}-{label}")),
    )
}
