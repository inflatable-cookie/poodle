//! The render vocabulary: what a Poodle component emits, with no opinion about
//! who draws it.
//!
//! A [`Node`] is a pure function's output — `Spec + Theme → Node` — carrying
//! resolved values only. Token resolution happens in the component
//! implementation, where the theme lives; backends receive concrete pixels and
//! RGBA and never see a token. That split is what keeps this crate small and
//! every backend ignorant of the theme system.
//!
//! What a backend owns, and this vocabulary must never absorb: text measurement
//! and shaping, hit-testing, focus traversal, IME, scroll physics, animation
//! driving, icon rasterisation. A node *declares* ("this text, ellipsised, in a
//! row that grows"); the backend decides what that costs in pixels.
//!
//! Grown from `poodle-layout` and `poodle-style`, which already defined the
//! renderer-agnostic seed types. This crate adds the tree, the widget kinds,
//! interaction intent, and accessibility — the parts a flat style descriptor
//! could not carry.
//!
//! Scope note: this is the v0 shape, proven against the components ported so
//! far. It grows component-by-component during the migration; it does not try
//! to be complete ahead of the components that would prove it.

use std::fmt;
use std::sync::Arc;

pub use poodle_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutOverflow, LayoutSizing,
    MainAxisAlignment,
};
pub use poodle_style::{CursorHint, FontFamily, StyleDescriptor, TypographyDescriptor};
pub use poodle_tokens::typed::{ColorValue, ShadowValue};

/// One rendered element: what it is, how it looks, how it behaves, what it
/// contains. Pure data plus interaction closures.
#[derive(Clone, Default)]
pub struct Node {
    pub kind: NodeKind,
    pub style: NodeStyle,
    pub position: NodePosition,
    pub interaction: Interaction,
    pub a11y: NodeA11y,
    pub children: Vec<Node>,
}

/// What the node *is*. Everything else describes it.
#[derive(Clone, Default)]
pub enum NodeKind {
    /// A container: layout and paint, no intrinsic content.
    #[default]
    Container,
    /// A run of text. Typography comes from the style descriptor.
    Text { content: String },
    /// A named icon. The backend rasterises; the name is the contract.
    Icon { name: String, size: f32 },
}

/// Visual style: the shared descriptor plus the tree-level properties a flat
/// descriptor could not express.
#[derive(Clone)]
pub struct NodeStyle {
    /// Colors, border, radii, shadow, typography, opacity, cursor, layout —
    /// all resolved. See `poodle_style::StyleDescriptor`.
    pub descriptor: StyleDescriptor,
    /// Style values that replace the descriptor's while the pointer is over
    /// this node. Declarative — the backend drives the state, the component
    /// declares both looks.
    pub hover: Option<StylePatch>,
    /// Truncate overflowing text with an ellipsis instead of clipping.
    pub text_ellipsis: bool,
    /// Text size in pixels. Optional so a backend can distinguish "the
    /// component chose 13px" from "use your default" — stamping a default here
    /// would change output the component never asked for.
    pub text_size: Option<f32>,
    /// Font weight, same optionality argument.
    pub text_weight: Option<u16>,
    /// Fill the parent's width (width: 100%). Candidate for a
    /// `LayoutSizing::Fraction` variant once `poodle-layout` can take the
    /// breaking change; a flag here keeps v0 additive.
    pub fill_width: bool,
    /// Render above everything and escape ancestor clip rects. The overlay
    /// half of a popover/menu/dropdown; position it with
    /// [`NodePosition::Absolute`] inside a [`NodePosition::Relative`] parent.
    pub overlay: bool,
    /// Explicit size floors/ceilings, applied whatever the sizing mode.
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            // `StyleDescriptor::default()` is opacity 0.0 / not-visible — a
            // trap for a vocabulary where "said nothing" must mean "draw
            // normally". `new()` is the constructed-visible form.
            descriptor: StyleDescriptor::new(),
            hover: None,
            text_ellipsis: false,
            text_size: None,
            text_weight: None,
            fill_width: false,
            overlay: false,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }
}

/// The subset of style a state change may swap in. Fields are `None` to keep
/// the base value.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct StylePatch {
    pub background: Option<ColorValue>,
    pub border_color: Option<ColorValue>,
    pub text_color: Option<ColorValue>,
}

/// How the node participates in layout flow.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum NodePosition {
    /// Ordinary flow child.
    #[default]
    InFlow,
    /// Flow child that anchors `Absolute` descendants.
    Relative,
    /// Removed from flow; positioned by insets from the nearest `Relative`
    /// ancestor.
    Absolute {
        top: Option<f32>,
        left: Option<f32>,
        right: Option<f32>,
        bottom: Option<f32>,
    },
}

/// Handler for an activation (click, enter key, tap). Payload is captured by
/// the component when it builds the closure — the vocabulary carries no event
/// plumbing, and `poodle-events`' `SemanticEvent` remains the layer above.
pub type ActivateHandler = Arc<dyn Fn() + Send + Sync>;

/// Interaction intent. Dispatching — hit-testing, ordering, focus — is the
/// backend's job; this declares what the node wants when dispatch reaches it.
#[derive(Clone, Default)]
pub struct Interaction {
    /// Participates in focus traversal.
    pub focusable: bool,
    /// Ignores activation and renders in the disabled state the style already
    /// describes (components bake disabled opacity into the descriptor).
    pub disabled: bool,
    pub on_activate: Option<ActivateHandler>,
}

/// Accessibility roles the ported components have needed so far. Grows with
/// the migration; backends map to their platform vocabulary (accesskit, the
/// DOM) at the edge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeRole {
    Button,
    CheckBox,
    ComboBox,
    Dialog,
    Group,
    Label,
    List,
    ListItem,
    ListBox,
    ListBoxOption,
    Menu,
    MenuItem,
    RadioGroup,
    RadioButton,
    Tab,
    TabList,
    TextInput,
    Tooltip,
}

/// What the node declares about itself to assistive technology.
#[derive(Clone, Default)]
pub struct NodeA11y {
    pub role: Option<NodeRole>,
    pub label: Option<String>,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
}

impl Node {
    pub fn container() -> Self {
        Self::default()
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self {
            kind: NodeKind::Text {
                content: content.into(),
            },
            ..Self::default()
        }
    }

    pub fn icon(name: impl Into<String>, size: f32) -> Self {
        Self {
            kind: NodeKind::Icon {
                name: name.into(),
                size,
            },
            ..Self::default()
        }
    }

    pub fn child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Node>) -> Self {
        self.children.extend(children);
        self
    }

    /// Every text content in the subtree, in tree order. The assertion surface
    /// for component tests: pure, no backend, no layout.
    pub fn texts(&self) -> Vec<&str> {
        let mut out = Vec::new();
        self.collect_texts(&mut out);
        out
    }

    fn collect_texts<'a>(&'a self, out: &mut Vec<&'a str>) {
        match &self.kind {
            NodeKind::Text { content } => out.push(content.as_str()),
            NodeKind::Icon { name, .. } => out.push(name.as_str()),
            NodeKind::Container => {}
        }
        for child in &self.children {
            child.collect_texts(out);
        }
    }

    pub fn has_text(&self, text: &str) -> bool {
        self.texts().contains(&text)
    }

    /// Depth-first search for the first node satisfying the predicate.
    pub fn find(&self, predicate: &dyn Fn(&Node) -> bool) -> Option<&Node> {
        if predicate(self) {
            return Some(self);
        }
        self.children.iter().find_map(|c| c.find(predicate))
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match &self.kind {
            NodeKind::Container => "Container".to_string(),
            NodeKind::Text { content } => format!("Text({content:?})"),
            NodeKind::Icon { name, .. } => format!("Icon({name:?})"),
        };
        f.debug_struct("Node")
            .field("kind", &kind)
            .field("children", &self.children.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_walk_the_tree_in_order() {
        let tree = Node::container()
            .child(Node::text("first"))
            .child(Node::container().child(Node::icon("chevron-down", 12.0)))
            .child(Node::text("last"));
        assert_eq!(tree.texts(), ["first", "chevron-down", "last"]);
        assert!(tree.has_text("chevron-down"));
        assert!(!tree.has_text("absent"));
    }

    #[test]
    fn find_locates_the_activatable_node() {
        let tree = Node::container().child(Node::text("go").interaction_on_activate(|| {}));
        let found = tree.find(&|n| n.interaction.on_activate.is_some());
        assert!(found.is_some());
        assert!(matches!(&found.unwrap().kind, NodeKind::Text { content } if content == "go"));
    }
}

// ── Builder sugar ──────────────────────────────────────────────────────────
//
// Thin, deliberately incomplete: components mostly assemble NodeStyle /
// descriptor structs directly. Sugar exists only where a call site pattern
// repeats across every component (activation, a11y).

impl Node {
    pub fn interaction_on_activate(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.interaction.on_activate = Some(Arc::new(handler));
        self
    }

    pub fn role(mut self, role: NodeRole) -> Self {
        self.a11y.role = Some(role);
        self
    }

    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.a11y.label = Some(label.into());
        self
    }
}
