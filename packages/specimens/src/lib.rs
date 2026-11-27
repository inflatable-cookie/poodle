//! Native catalogue page bodies. A host supplies the render context, mounts
//! the resulting nodes and drains its session between input and repaint.

mod button;
mod checkbox;
mod code_input;
mod data_table;
mod icon_button;
mod number_input;
mod radio;
mod radio_group;
mod rating;
mod segmented_control;
mod select;
mod split_button;
mod switch;
mod text_input;
mod toggle_group;
mod token_input;
mod tri_state_switch;

use std::sync::{Arc, Mutex};

use poodle_node::{LayoutDirection, Node};
use poodle_render::RenderContext;
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec};

/// Routes authored in this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Specimen {
    Button,
    Select,
    DataTable,
    Checkbox,
    Switch,
    Radio,
    RadioGroup,
    TriStateSwitch,
    SegmentedControl,
    ToggleGroup,
    TextInput,
    NumberInput,
    TokenInput,
    CodeInput,
    Rating,
    IconButton,
    SplitButton,
}

/// One row in an axis pane.
///
/// `value` is a stable component-specific axis identifier passed back in
/// [`View`]; it does not shift when another row is added, removed or reordered.
/// `label` is the display label shown beside the row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AxisRow {
    pub value: &'static str,
    pub label: &'static str,
}

impl AxisRow {
    pub const fn new(value: &'static str, label: &'static str) -> Self {
        Self { value, label }
    }

    pub const fn same(value: &'static str) -> Self {
        Self {
            value,
            label: value,
        }
    }
}

pub(crate) const CONTROL_SIZES: &[AxisRow] = &[
    AxisRow::same("xs"),
    AxisRow::same("sm"),
    AxisRow::same("md"),
    AxisRow::same("lg"),
    AxisRow::same("xl"),
];

pub(crate) const CONTROL_DENSITIES: &[AxisRow] = &[
    AxisRow::same("compact"),
    AxisRow::same("default"),
    AxisRow::same("comfortable"),
];

pub(crate) fn control_size(value: &str) -> ControlSize {
    match value {
        "xs" => ControlSize::Xs,
        "sm" => ControlSize::Sm,
        "md" => ControlSize::Md,
        "lg" => ControlSize::Lg,
        "xl" => ControlSize::Xl,
        other => panic!("unknown ControlSize axis value: {other:?}"),
    }
}

pub(crate) fn control_density(value: &str) -> ControlDensity {
    match value {
        "compact" => ControlDensity::Compact,
        "default" => ControlDensity::Default,
        "comfortable" => ControlDensity::Comfortable,
        other => panic!("unknown ControlDensity axis value: {other:?}"),
    }
}

/// A tab and, for an axis tab, its ordered public rows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pane {
    Examples,
    Sizes(&'static [AxisRow]),
    Densities(&'static [AxisRow]),
}

/// A body request. Axis panes render one representative per stable row value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    Examples,
    Size(&'static str),
    Density(&'static str),
}

const STANDARD_PANES: &[Pane] = &[
    Pane::Examples,
    Pane::Sizes(CONTROL_SIZES),
    Pane::Densities(CONTROL_DENSITIES),
];

impl Specimen {
    pub const ALL: [Self; 17] = [
        Self::Button,
        Self::Select,
        Self::DataTable,
        Self::Checkbox,
        Self::Switch,
        Self::Radio,
        Self::RadioGroup,
        Self::TriStateSwitch,
        Self::SegmentedControl,
        Self::ToggleGroup,
        Self::TextInput,
        Self::NumberInput,
        Self::TokenInput,
        Self::CodeInput,
        Self::Rating,
        Self::IconButton,
        Self::SplitButton,
    ];

    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|specimen| specimen.slug() == slug)
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Select => "select",
            Self::DataTable => "data-table",
            Self::Checkbox => "checkbox",
            Self::Switch => "switch",
            Self::Radio => "radio",
            Self::RadioGroup => "radio-group",
            Self::TriStateSwitch => "tri-state-switch",
            Self::SegmentedControl => "segmented-control",
            Self::ToggleGroup => "toggle-group",
            Self::TextInput => "text-input",
            Self::NumberInput => "number-input",
            Self::TokenInput => "token-input",
            Self::CodeInput => "code-input",
            Self::Rating => "rating",
            Self::IconButton => "icon-button",
            Self::SplitButton => "split-button",
        }
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::Select => "Select",
            Self::DataTable => "DataTable",
            Self::Checkbox => "Checkbox",
            Self::Switch => "Switch",
            Self::Radio => "Radio",
            Self::RadioGroup => "RadioGroup",
            Self::TriStateSwitch => "TriStateSwitch",
            Self::SegmentedControl => "SegmentedControl",
            Self::ToggleGroup => "ToggleGroup",
            Self::TextInput => "TextInput",
            Self::NumberInput => "NumberInput",
            Self::TokenInput => "TokenInput",
            Self::CodeInput => "CodeInput",
            Self::Rating => "Rating",
            Self::IconButton => "IconButton",
            Self::SplitButton => "SplitButton",
        }
    }

    pub const fn panes(self) -> &'static [Pane] {
        match self {
            Self::Button
            | Self::Select
            | Self::DataTable
            | Self::Checkbox
            | Self::Switch
            | Self::Radio
            | Self::RadioGroup
            | Self::TriStateSwitch
            | Self::SegmentedControl
            | Self::ToggleGroup
            | Self::TextInput
            | Self::NumberInput
            | Self::TokenInput
            | Self::CodeInput
            | Self::Rating
            | Self::IconButton
            | Self::SplitButton => STANDARD_PANES,
        }
    }
}

/// An operation the host performs against the mounted node tree.
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    Focus(String),
}

enum Event {
    Button(button::Event),
    Select(select::Event),
    DataTable(data_table::Event),
    Checkbox(checkbox::Event),
    Switch(switch::Event),
    Radio(radio::Event),
    RadioGroup(radio_group::Event),
    TriStateSwitch(tri_state_switch::Event),
    SegmentedControl(segmented_control::Event),
    ToggleGroup(toggle_group::Event),
    TextInput(text_input::Event),
    NumberInput(number_input::Event),
    CodeInput(code_input::Event),
    Rating(rating::Event),
    IconButton(icon_button::Event),
    SplitButton(split_button::Event),
}

type Sink<E> = Arc<dyn Fn(E) + Send + Sync>;

/// Retained state for one preview. Render callbacks enqueue typed page events
/// without borrowing the host. The host drains them in order before repaint.
#[derive(Default)]
pub struct Session {
    events: Arc<Mutex<Vec<Event>>>,
    button: button::State,
    select: select::State,
    data_table: data_table::State,
    checkbox: checkbox::State,
    switch: switch::State,
    radio: radio::State,
    radio_group: radio_group::State,
    tri_state_switch: tri_state_switch::State,
    segmented_control: segmented_control::State,
    toggle_group: toggle_group::State,
    text_input: text_input::State,
    number_input: number_input::State,
    code_input: code_input::State,
    rating: rating::State,
    icon_button: icon_button::State,
    split_button: split_button::State,
}

impl Session {
    pub fn render(&self, specimen: Specimen, view: View, ctx: &RenderContext<'_>) -> Node {
        if view != View::Examples {
            return match specimen {
                Specimen::Button => button::axis(view, ctx),
                Specimen::Select => select::axis(view, ctx),
                Specimen::DataTable => data_table::axis(view, ctx),
                Specimen::Checkbox => checkbox::axis(view, ctx),
                Specimen::Switch => switch::axis(view, ctx),
                Specimen::Radio => radio::axis(view, ctx),
                Specimen::RadioGroup => radio_group::axis(view, ctx),
                Specimen::TriStateSwitch => tri_state_switch::axis(view, ctx),
                Specimen::SegmentedControl => segmented_control::axis(view, ctx),
                Specimen::ToggleGroup => toggle_group::axis(view, ctx),
                Specimen::TextInput => text_input::axis(view, ctx),
                Specimen::NumberInput => number_input::axis(view, ctx),
                Specimen::TokenInput => token_input::axis(view, ctx),
                Specimen::CodeInput => code_input::axis(view, ctx),
                Specimen::Rating => rating::axis(view, ctx),
                Specimen::IconButton => icon_button::axis(view, ctx),
                Specimen::SplitButton => split_button::axis(view, ctx),
            };
        }
        let mut node = match specimen {
            Specimen::Button => button::examples(&self.button, self.sink(Event::Button), ctx),
            Specimen::Select => select::examples(&self.select, self.sink(Event::Select), ctx),
            Specimen::DataTable => {
                data_table::examples(&self.data_table, self.sink(Event::DataTable), ctx)
            }
            Specimen::Checkbox => {
                checkbox::examples(&self.checkbox, self.sink(Event::Checkbox), ctx)
            }
            Specimen::Switch => switch::examples(&self.switch, self.sink(Event::Switch), ctx),
            Specimen::Radio => radio::examples(&self.radio, self.sink(Event::Radio), ctx),
            Specimen::RadioGroup => {
                radio_group::examples(&self.radio_group, self.sink(Event::RadioGroup), ctx)
            }
            Specimen::TriStateSwitch => tri_state_switch::examples(
                &self.tri_state_switch,
                self.sink(Event::TriStateSwitch),
                ctx,
            ),
            Specimen::SegmentedControl => segmented_control::examples(
                &self.segmented_control,
                self.sink(Event::SegmentedControl),
                ctx,
            ),
            Specimen::ToggleGroup => {
                toggle_group::examples(&self.toggle_group, self.sink(Event::ToggleGroup), ctx)
            }
            Specimen::TextInput => {
                text_input::examples(&self.text_input, self.sink(Event::TextInput), ctx)
            }
            Specimen::NumberInput => {
                number_input::examples(&self.number_input, self.sink(Event::NumberInput), ctx)
            }
            Specimen::TokenInput => token_input::examples(ctx),
            Specimen::CodeInput => {
                code_input::examples(&self.code_input, self.sink(Event::CodeInput), ctx)
            }
            Specimen::Rating => rating::examples(&self.rating, self.sink(Event::Rating), ctx),
            Specimen::IconButton => {
                icon_button::examples(&self.icon_button, self.sink(Event::IconButton), ctx)
            }
            Specimen::SplitButton => {
                split_button::examples(&self.split_button, self.sink(Event::SplitButton), ctx)
            }
        };
        node.id = Some(format!("specimen-{}-examples", specimen.slug()));
        node
    }

    /// Returns whether the host should rebuild, plus the effects to apply
    /// against that rebuilt tree.
    pub fn drain(&mut self) -> (bool, Vec<Effect>) {
        let events = std::mem::take(&mut *self.events.lock().unwrap());
        let changed = !events.is_empty();
        let mut effects = Vec::new();
        for event in events {
            match event {
                Event::Button(event) => {
                    self.button = button::reduce(std::mem::take(&mut self.button), event);
                }
                Event::Select(event) => {
                    let (state, effect) = select::reduce(std::mem::take(&mut self.select), event);
                    self.select = state;
                    effects.extend(effect);
                }
                Event::DataTable(event) => {
                    self.data_table =
                        data_table::reduce(std::mem::take(&mut self.data_table), event);
                }
                Event::Checkbox(event) => {
                    self.checkbox = checkbox::reduce(std::mem::take(&mut self.checkbox), event);
                }
                Event::Switch(event) => {
                    self.switch = switch::reduce(std::mem::take(&mut self.switch), event);
                }
                Event::Radio(event) => {
                    self.radio = radio::reduce(std::mem::take(&mut self.radio), event);
                }
                Event::RadioGroup(event) => {
                    self.radio_group =
                        radio_group::reduce(std::mem::take(&mut self.radio_group), event);
                }
                Event::TriStateSwitch(event) => {
                    self.tri_state_switch =
                        tri_state_switch::reduce(std::mem::take(&mut self.tri_state_switch), event);
                }
                Event::SegmentedControl(event) => {
                    self.segmented_control = segmented_control::reduce(
                        std::mem::take(&mut self.segmented_control),
                        event,
                    );
                }
                Event::ToggleGroup(event) => {
                    self.toggle_group =
                        toggle_group::reduce(std::mem::take(&mut self.toggle_group), event);
                }
                Event::TextInput(event) => {
                    self.text_input =
                        text_input::reduce(std::mem::take(&mut self.text_input), event);
                }
                Event::NumberInput(event) => {
                    self.number_input =
                        number_input::reduce(std::mem::take(&mut self.number_input), event);
                }
                Event::CodeInput(event) => {
                    self.code_input =
                        code_input::reduce(std::mem::take(&mut self.code_input), event);
                }
                Event::Rating(event) => {
                    self.rating = rating::reduce(std::mem::take(&mut self.rating), event);
                }
                Event::IconButton(event) => {
                    self.icon_button =
                        icon_button::reduce(std::mem::take(&mut self.icon_button), event);
                }
                Event::SplitButton(event) => {
                    self.split_button =
                        split_button::reduce(std::mem::take(&mut self.split_button), event);
                }
            }
        }
        (changed, effects)
    }

    fn sink<E: 'static>(&self, wrap: fn(E) -> Event) -> Sink<E> {
        let events = Arc::clone(&self.events);
        Arc::new(move |event| events.lock().unwrap().push(wrap(event)))
    }
}

fn column(gap: f32, children: Vec<Node>) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.spacing.gap = gap;
    node.children = children;
    node
}

fn row(children: Vec<Node>) -> Node {
    let mut node = column(8.0, children);
    node.style.descriptor.layout.direction = LayoutDirection::Row;
    node.style.flex_wrap = true;
    node
}

fn section(title: &str, body: impl IntoIterator<Item = Node>, ctx: &RenderContext<'_>) -> Node {
    let heading = poodle_render::eyebrow(&EyebrowSpec::new().with_content(title), ctx);
    column(8.0, std::iter::once(heading).chain(body).collect())
}

fn readout(text: impl Into<String>, ctx: &RenderContext<'_>) -> Node {
    let mut node = Node::text(text);
    node.style.text_size = Some(14.0);
    node.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.secondary"));
    node
}

#[cfg(test)]
mod tests;
