//! Native catalogue page bodies. A host supplies the render context, mounts
//! the resulting nodes and drains its session between input and repaint.

mod button;
mod data_table;
mod select;

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
}

const SIZES: &[(ControlSize, &str)] = &[
    (ControlSize::Xs, "xs"),
    (ControlSize::Sm, "sm"),
    (ControlSize::Md, "md"),
    (ControlSize::Lg, "lg"),
    (ControlSize::Xl, "xl"),
];
const DENSITIES: &[(ControlDensity, &str)] = &[
    (ControlDensity::Compact, "compact"),
    (ControlDensity::Default, "default"),
    (ControlDensity::Comfortable, "comfortable"),
];

/// A tab and, for an axis tab, its ordered public values and display labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pane {
    Examples,
    Sizes(&'static [(ControlSize, &'static str)]),
    Densities(&'static [(ControlDensity, &'static str)]),
}

/// A body request. Axis panes render one representative per requested value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    Examples,
    Size(ControlSize),
    Density(ControlDensity),
}

impl Specimen {
    pub const ALL: [Self; 3] = [Self::Button, Self::Select, Self::DataTable];

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
        }
    }

    pub const fn panes(self) -> &'static [Pane] {
        match self {
            Self::Button | Self::Select | Self::DataTable => &[
                Pane::Examples,
                Pane::Sizes(SIZES),
                Pane::Densities(DENSITIES),
            ],
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
}

impl Session {
    pub fn render(&self, specimen: Specimen, view: View, ctx: &RenderContext<'_>) -> Node {
        if view != View::Examples {
            return match specimen {
                Specimen::Button => button::axis(view, ctx),
                Specimen::Select => select::axis(view, ctx),
                Specimen::DataTable => data_table::axis(view, ctx),
            };
        }
        let mut node = match specimen {
            Specimen::Button => button::examples(&self.button, self.sink(Event::Button), ctx),
            Specimen::Select => select::examples(&self.select, self.sink(Event::Select), ctx),
            Specimen::DataTable => {
                data_table::examples(&self.data_table, self.sink(Event::DataTable), ctx)
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
