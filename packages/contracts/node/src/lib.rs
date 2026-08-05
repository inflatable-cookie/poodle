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
    /// Stable identity for host targeting (event loops address nodes by id)
    /// and animation clocks. An animation's `key` also becomes the id when no
    /// explicit one is set.
    pub id: Option<String>,
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
    /// An image by source path/URL; the backend owns decode and upload.
    /// Fits by covering the box (object-fit: cover) — the one mode a
    /// component has needed so far.
    Image { source: String },
    /// A determinate progress bar: the backend fills `fraction` of the track.
    Progress { fraction: f32 },
    /// A button widget: the backend's native pressable, carrying its label.
    /// Distinct from `Container` + `Text` because backends give buttons
    /// intrinsic behaviour (pressed visuals, activation semantics) that a
    /// styled box does not get.
    Button { label: String },
}

/// Horizontal text alignment.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// One layer of a shadow stack.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShadowLayer {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: ColorValue,
    /// Inner shadow (highlight/inset) rather than a drop.
    pub inset: bool,
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
    /// Font family request (`poodle_style::FontFamily`), same optionality.
    pub font_family: Option<FontFamily>,
    /// Line height as a multiple of the font size.
    pub line_height: Option<f32>,
    /// Soft-wrap text to the container width (white-space: normal).
    pub text_wrap: bool,
    /// Fill the parent's height (height: 100%), the vertical `fill_width`.
    pub fill_height: bool,
    /// Fill the parent's width (width: 100%). Candidate for a
    /// `LayoutSizing::Fraction` variant once `poodle-layout` can take the
    /// breaking change; a flag here keeps v0 additive.
    pub fill_width: bool,
    /// Letter spacing in em, applied to this node's text.
    pub letter_spacing_em: Option<f32>,
    /// Horizontal text alignment. `None` = the backend's default (left).
    pub text_align: Option<TextAlign>,
    /// Style values swapped in while the node is pressed. Same contract as
    /// `hover`.
    pub active: Option<StylePatch>,
    /// Multi-layer shadow stack (inset highlights, outset drops). When
    /// non-empty it wins over `descriptor.shadow`, which stays the one-token
    /// single-shadow convenience.
    pub shadow_layers: Vec<ShadowLayer>,
    /// Never grow or shrink in flex layout.
    pub flex_none: bool,
    /// Stretch across the parent's cross axis (align-self: stretch).
    pub self_stretch: bool,
    /// Explicit flex-grow factor. Fractional splits (progress's 40/60 bar)
    /// need a raw factor; `LayoutSizing::Grow` stays the common 1.0 case.
    pub flex_grow: Option<f32>,
    /// Grow + shrink with a zero min-size, WITHOUT cross-axis stretch —
    /// distinct from `LayoutSizing::Grow`, which stretches. Skeleton's text
    /// columns are the proving call site.
    pub flex_fill: bool,
    /// Never shrink below natural size (flex-shrink: 0), still growable.
    pub flex_shrink_zero: bool,
    /// Flex-basis in pixels (seed size before grow/shrink distribute).
    pub flex_basis: Option<f32>,
    /// Width as a fraction of the parent (0.0..=1.0).
    pub width_pct: Option<f32>,
    /// Wrap flex children onto multiple lines.
    pub flex_wrap: bool,
    /// Keep text on one line (white-space: nowrap).
    pub no_wrap: bool,
    /// Border-top colour override — the bright arc of a ring spinner. The
    /// other sides keep `descriptor.border.color`.
    pub border_color_top: Option<ColorValue>,
    /// Dashed border (empty states). Solid when false.
    pub border_dashed: bool,
    /// Bottom-only border width (toolbar separators). Overrides nothing —
    /// composes with the uniform `descriptor.border` when both are set.
    pub border_bottom_width: Option<f32>,
    /// Right-only border width (vertical tab lists' edge rule).
    pub border_right_width: Option<f32>,
    /// Top-only border width (vertical block-tab separators).
    pub border_top_width: Option<f32>,
    /// Left-only border width (horizontal block-tab separators).
    pub border_left_width: Option<f32>,
    /// Linear-gradient background: angle in degrees, sRGB stops at 0..=1.
    /// Wins over `descriptor.background` when set.
    pub gradient: Option<(f32, Vec<(ColorValue, f32)>)>,
    /// A declared keyframe animation on this node.
    pub animation: Option<NodeAnimation>,
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
            font_family: None,
            line_height: None,
            text_wrap: false,
            fill_height: false,
            fill_width: false,
            flex_none: false,
            self_stretch: false,
            flex_grow: None,
            flex_fill: false,
            flex_shrink_zero: false,
            flex_basis: None,
            width_pct: None,
            flex_wrap: false,
            no_wrap: false,
            border_color_top: None,
            border_dashed: false,
            border_bottom_width: None,
            border_right_width: None,
            border_top_width: None,
            border_left_width: None,
            gradient: None,
            animation: None,
            letter_spacing_em: None,
            text_align: None,
            active: None,
            shadow_layers: Vec::new(),
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

/// Phase of a drag interaction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeDragPhase {
    Start,
    Move,
    End,
}

/// A drag event, in the vocabulary's terms: per-frame deltas only. Absolute
/// positions are a backend concern (they depend on layout, which the
/// component never sees).
#[derive(Clone, Copy, Debug)]
pub struct NodeDragEvent {
    pub phase: NodeDragPhase,
    pub delta_x: f32,
    pub delta_y: f32,
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
    /// Drag handler. Unlike activation, drags do not bubble: the exact node
    /// under the pointer must carry the handler.
    pub on_drag: Option<Arc<dyn Fn(&NodeDragEvent) + Send + Sync>>,
}

/// Accessibility roles the ported components have needed so far. Grows with
/// the migration; backends map to their platform vocabulary (accesskit, the
/// DOM) at the edge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeRole {
    Alert,
    AlertDialog,
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
    Image,
    Menu,
    MenuBar,
    MenuItem,
    MenuItemCheckBox,
    MenuItemRadio,
    Splitter,
    ProgressIndicator,
    RadioGroup,
    RadioButton,
    Region,
    Status,
    Switch,
    Tab,
    TabList,
    TextInput,
    Toolbar,
    Tooltip,
}

/// A property an animation may drive. Backends map to their own channels.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnimProperty {
    Opacity,
    Rotate,
    TranslateX,
    TranslateY,
    ScaleX,
    ScaleY,
}

/// Animation easing.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AnimEasing {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// Loop behaviour.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AnimLoop {
    #[default]
    Once,
    Loop,
    PingPong,
}

/// One keyframe: property values at a position in the cycle (0.0..=1.0).
#[derive(Clone, Debug)]
pub struct AnimKeyframe {
    pub at: f32,
    pub values: Vec<(AnimProperty, f32)>,
}

/// A declared keyframe animation. The node declares; the backend drives the
/// clock. `key` is the stable identity that lets an immediate-mode backend
/// keep the clock running across tree rebuilds — nodes sharing a key share a
/// clock and stay in phase, like CSS keyframes.
#[derive(Clone, Debug)]
pub struct NodeAnimation {
    pub key: String,
    pub keyframes: Vec<AnimKeyframe>,
    pub duration_secs: f32,
    pub easing: AnimEasing,
    pub loop_mode: AnimLoop,
}

impl NodeAnimation {
    /// The classic: continuous rotation, `duration_secs` per revolution.
    pub fn spin(key: impl Into<String>, duration_secs: f32) -> Self {
        let tau = std::f32::consts::TAU;
        Self {
            key: key.into(),
            keyframes: vec![
                AnimKeyframe {
                    at: 0.0,
                    values: vec![(AnimProperty::Rotate, 0.0)],
                },
                AnimKeyframe {
                    at: 1.0,
                    values: vec![(AnimProperty::Rotate, tau)],
                },
            ],
            duration_secs,
            easing: AnimEasing::Linear,
            loop_mode: AnimLoop::Loop,
        }
    }
}

/// Tri-state checked value, for checkboxes and switches.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeToggled {
    True,
    False,
    Mixed,
}

/// What the node declares about itself to assistive technology.
#[derive(Clone, Default)]
pub struct NodeA11y {
    pub role: Option<NodeRole>,
    pub label: Option<String>,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub toggled: Option<NodeToggled>,
}

impl Node {
    pub fn container() -> Self {
        Self::default()
    }

    pub fn button(label: impl Into<String>) -> Self {
        Self {
            kind: NodeKind::Button {
                label: label.into(),
            },
            ..Self::default()
        }
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
            NodeKind::Button { label } => {
                if !label.is_empty() {
                    out.push(label.as_str());
                }
            }
            NodeKind::Image { .. } | NodeKind::Progress { .. } | NodeKind::Container => {}
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
            NodeKind::Button { label } => format!("Button({label:?})"),
            NodeKind::Image { source } => format!("Image({source:?})"),
            NodeKind::Progress { fraction } => format!("Progress({fraction})"),
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
