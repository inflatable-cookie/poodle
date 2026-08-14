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
    /// Semantic token roles the component projects onto this node — the
    /// native counterpart of web `data-*` recipe attributes (e.g.
    /// `variant: primary`). Observers read these; nothing executes them.
    pub roles: std::collections::BTreeMap<String, String>,
    /// An input's caret and selection, when the host owns one. Ignored by
    /// every other kind, and by any backend that does not draw carets.
    pub caret: Option<NodeCaret>,
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
    /// A single-line text field seeded with its current value; shows
    /// `placeholder` when the value is empty. Same rationale as `Button`:
    /// backends give inputs intrinsic behaviour (caret, selection, IME,
    /// intrinsic min-width) a styled box does not get. Render-side the value
    /// is host-owned — the node declares the field, the host drives edits.
    Input { value: String, placeholder: String },
}

/// Which part of a scrub gesture an event belongs to.
///
/// A single fraction cannot say *which* of a two-thumb control the pointer is
/// moving. The press is where that gets decided — after it, the gesture stays
/// with whatever the press chose, or a thumb dragged past its partner would
/// hand the gesture over mid-drag.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ScrubPhase {
    /// The pointer went down (or a click landed) at this fraction.
    #[default]
    Press,
    /// The pointer moved while the gesture is held.
    Drag,
}

/// How far a pointer selection reaches out from where it landed.
///
/// A backend knows the click count; only the component knows what a "word" is,
/// so the backend names the granularity and the component resolves it.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SelectGranularity {
    /// Exactly the reported range: a single click, or a drag.
    #[default]
    Character,
    /// Expand each end to its word: a double click.
    Word,
    /// The whole value: a triple click.
    Line,
}

/// Where an input's caret is and what colour to draw it.
///
/// A separate channel rather than fields on [`NodeKind::Input`]: adding fields
/// to a struct variant breaks every `match` in every backend, and the
/// vocabulary's additions have to be additive for real — a backend that has
/// never heard of carets must keep compiling and keep rendering the value.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NodeCaret {
    /// `(start, end)` as **character** indices into the input's value. A
    /// collapsed range is a caret; a wider one is a selection.
    ///
    /// Characters, not bytes: the vocabulary is shared by backends with
    /// different string representations, and a component counting `chars()` is
    /// the only encoding-independent answer. Backends convert at their own
    /// edge, where they know what their text system counts in.
    pub selection: (usize, usize),
    /// Caret colour, resolved by the component from tokens. The backend paints
    /// it, because placing it means measuring shaped glyphs.
    pub caret_color: ColorValue,
    /// Selected-text wash, resolved by the component from tokens.
    pub selection_color: ColorValue,
    /// Whether the text being drawn is the field's *placeholder* rather than
    /// its value.
    ///
    /// The two look identical to a backend otherwise, and treating a
    /// placeholder as a value is silently wrong in both directions: a caret
    /// positions itself inside prompt text, and anything recording the value
    /// records the prompt.
    pub showing_placeholder: bool,
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
    /// Draw a one-pixel text underline using the backend's native decoration.
    pub text_underline: bool,
    /// Optional decoration tint for text underlines. When absent, the
    /// backend uses the text color, matching native GPUI defaults.
    pub text_underline_color: Option<ColorValue>,
    /// Text size in pixels. Optional so a backend can distinguish "the
    /// component chose 13px" from "use your default" — stamping a default here
    /// would change output the component never asked for.
    pub text_size: Option<f32>,
    /// Font weight, same optionality argument.
    pub text_weight: Option<u16>,
    /// Italic text (`font-style: italic`). Several contracts call for it —
    /// blockquotes, empty-state captions, unsaved-edit labels — and without a
    /// channel for it recipes reach for a tone change as a substitute.
    pub text_italic: bool,
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
    /// Style values swapped in while the node itself holds focus — the
    /// contracts' `focus-visible` state. Distinct from [`Self::active`], which
    /// is the *pressed* state: a focused-but-unpressed control showed nothing
    /// at all before this existed, so a clicked text field looked identical to
    /// an idle one. Only meaningful on a focusable node.
    pub focus: Option<StylePatch>,
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
    /// Flex-basis as a fraction of the parent's main axis (0.0..=1.0).
    ///
    /// Distinct from `flex_basis`: a ratio-allocated pane seeds at a share of
    /// the container and then shrinks to make room for siblings like a
    /// divider. Seeding at zero and growing by the ratio instead distributes
    /// the divider's thickness across both panes, which moves the split.
    /// Wins over `flex_basis` when both are set.
    pub flex_basis_pct: Option<f32>,
    /// Width as a fraction of the parent (0.0..=1.0).
    pub width_pct: Option<f32>,
    /// Wrap flex children onto multiple lines.
    pub flex_wrap: bool,
    /// Keep text on one line (white-space: nowrap).
    pub no_wrap: bool,
    /// Border-top colour override — the bright arc of a ring spinner. The
    /// other sides keep `descriptor.border.color`.
    pub border_color_top: Option<ColorValue>,
    /// Border-left colour override — the tone accent of an inline
    /// remediation. The other sides keep `descriptor.border.color`.
    pub border_color_left: Option<ColorValue>,
    /// Border-bottom colour override — an active tab's accent underline.
    pub border_color_bottom: Option<ColorValue>,
    /// Grayscale filter amount (0.0 = full colour, 1.0 = fully desaturated) —
    /// a not-live card's washed-out treatment.
    pub grayscale: f32,
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
            text_underline: false,
            text_underline_color: None,
            text_size: None,
            text_weight: None,
            text_italic: false,
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
            flex_basis_pct: None,
            width_pct: None,
            flex_wrap: false,
            no_wrap: false,
            border_color_top: None,
            border_color_left: None,
            border_color_bottom: None,
            grayscale: 0.0,
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
            focus: None,
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
    /// Opacity override — a hover-revealed affordance's 0→1.
    pub opacity: Option<f32>,
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

/// Modifier state at the moment of an interaction.
///
/// `accel` is the platform's "toggle one of many" modifier — Cmd on macOS,
/// Ctrl elsewhere. Backends collapse their platform pair onto this one flag so
/// components never branch on the host OS.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NodeModifiers {
    pub shift: bool,
    pub accel: bool,
    pub alt: bool,
}

/// A point in the backend's window space, logical px.
///
/// The vocabulary hands a component a position in exactly one place —
/// [`Interaction::on_context`] — because a context menu is anchored to the
/// pointer by definition, so the anchor *is* the semantic rather than leaked
/// layout. Nothing else exposes coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NodePoint {
    pub x: f32,
    pub y: f32,
}

/// A key the backend dispatches to the focused node.
///
/// Named physically, not semantically: ArrowDown means "next row" in a tree
/// and "next option" in a select, so the meaning belongs to the component.
/// Enter/Tab and Escape are deliberately absent — they stay on
/// [`Interaction::on_submit`] and [`Interaction::on_cancel`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKey {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    Space,
    F2,
}

/// Where a drop lands relative to the zone it is over.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropEdge {
    Before,
    #[default]
    Inside,
    After,
}

/// A drag hovering or released over a drop zone.
///
/// The backend hit-tests zones and derives `edge` from where the pointer sits
/// within the zone node's OWN bounds — geometry the backend already holds. The
/// component names its zones and receives a semantic edge, so this does not
/// reopen the delta-only rule on [`NodeDragEvent`]: no coordinate reaches the
/// component here either.
#[derive(Clone, Debug)]
pub struct NodeDropEvent {
    /// The [`Interaction::drag_payload`] of the node the gesture started on.
    pub payload: String,
    pub edge: DropEdge,
}

/// Handler for an activation (click, enter key, tap). Payload is captured by
/// the component when it builds the closure — the vocabulary carries no event
/// plumbing, and `poodle-events`' `SemanticEvent` remains the layer above.
pub type ActivateHandler = Arc<dyn Fn() + Send + Sync>;

/// Handler for a host-edited text value. The node carries the current value
/// in [`NodeKind::Input`]; the backend owns key/IME dispatch and reports the
/// replacement string through this callback.
pub type TextChangeHandler = Arc<dyn Fn(&str) + Send + Sync>;

/// Handler for a host-owned edit submission (Enter or Tab on an input).
pub type SubmitHandler = Arc<dyn Fn() + Send + Sync>;

/// Handler for a host-owned edit cancellation (Escape on an input).
pub type CancelHandler = Arc<dyn Fn() + Send + Sync>;

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
    /// Reports replacement text for an input node. Backends with a native
    /// editor use its change stream; lightweight backends may provide a
    /// smaller editing subset while preserving the same callback contract.
    pub on_text_change: Option<TextChangeHandler>,
    /// Completes the current input edit on Enter or Tab. The component
    /// captures the current value in the closure; the backend only maps the
    /// key gesture.
    pub on_submit: Option<SubmitHandler>,
    /// Cancels the current input edit on Escape.
    pub on_cancel: Option<CancelHandler>,
    /// Raw editing keys for a text field: the key name (`"a"`, `"left"`,
    /// `"backspace"`, …) and the modifiers held.
    ///
    /// Deliberately lower level than [`Self::on_text_change`]. Editing depends
    /// on the caret, which the component owns via its spec — so the component
    /// decides what a keystroke means and reports the resulting value and
    /// selection, rather than the backend guessing. That also keeps one
    /// editing model shared across targets instead of one per backend.
    pub on_edit_key: Option<Arc<dyn Fn(&str, NodeModifiers) + Send + Sync>>,
    /// Fires when the pointer sets an input's caret or selection, as
    /// **character** indices into the value.
    ///
    /// A click reports a collapsed range; a drag reports an anchored one. The
    /// backend derives both from measured text, so the component never sees a
    /// coordinate — the same division as `on_scrub`'s fraction.
    pub on_select_range: Option<Arc<dyn Fn(usize, usize, SelectGranularity) + Send + Sync>>,
    /// Insert text at the caret, replacing any selection.
    ///
    /// Paste, and eventually an IME commit or a text drop: the backend has
    /// text from somewhere the component cannot reach, and the component owns
    /// where it lands. Distinct from `on_edit_key`, which reports a keystroke
    /// rather than content.
    pub on_edit_insert: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires when this node gains or loses focus, with the state it moved to.
    ///
    /// Focus is the backend's (the vocabulary says so), but a component that
    /// draws a caret has to know. Without a *blur* report the host can only
    /// ever latch focus on, which leaves a caret sitting in a field that no
    /// longer has it — two disagreeing ideas of what is focused.
    pub on_focus_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Drag handler. Unlike activation, drags do not bubble: the exact node
    /// under the pointer must carry the handler.
    pub on_drag: Option<Arc<dyn Fn(&NodeDragEvent) + Send + Sync>>,
    /// Scrub handler for controls whose value IS a position along themselves —
    /// sliders, range sliders, seek bars.
    ///
    /// Reports where the pointer sits along this node's main axis as a
    /// fraction (0.0 = start, 1.0 = end), on press and continuously while
    /// dragging, including once the pointer leaves the node. Like
    /// [`DropEdge`], the backend derives it from bounds it already owns and the
    /// component receives a semantic value, never a coordinate.
    ///
    /// Prefer this over [`Self::on_drag`] for value controls: a delta needs the
    /// component to guess its own rendered length, which it cannot know, and
    /// that guess is wrong whenever the control is not its natural size.
    ///
    /// The [`ScrubPhase`] separates the press from the moves that follow, which
    /// is what lets a two-thumb control decide once which thumb it is moving.
    pub on_scrub: Option<Arc<dyn Fn(f32, ScrubPhase) + Send + Sync>>,
    /// Activation that needs the modifier state — multi-select lists, where
    /// Shift extends the range and the platform accel toggles one item. When
    /// set the backend calls this INSTEAD of `on_activate`, so a node wires
    /// one or the other, never both.
    pub on_activate_modified: Option<Arc<dyn Fn(NodeModifiers) + Send + Sync>>,
    /// Secondary (right) activation, carrying the pointer anchor for a
    /// context menu.
    pub on_context: Option<Arc<dyn Fn(NodePoint) + Send + Sync>>,
    /// Navigation and command keys, while this node holds focus.
    pub on_key: Option<Arc<dyn Fn(NodeKey, NodeModifiers) + Send + Sync>>,
    /// Marks this node as a drag source carrying an opaque payload id. The
    /// component chooses the id; the backend only carries it back.
    pub drag_payload: Option<String>,
    /// Marks this node as a drop zone for `drag_payload` gestures.
    pub drop_zone: bool,
    /// Fires repeatedly while a drag hovers this zone — drives the drop
    /// indicator.
    pub on_drop_hover: Option<Arc<dyn Fn(&NodeDropEvent) + Send + Sync>>,
    /// Fires once when a drag is released over this zone.
    pub on_drop: Option<Arc<dyn Fn(&NodeDropEvent) + Send + Sync>>,
}

/// Accessibility roles the ported components have needed so far. Grows with
/// the migration; backends map to their platform vocabulary (accesskit, the
/// DOM) at the edge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeRole {
    Alert,
    AlertDialog,
    Button,
    Cell,
    CheckBox,
    ComboBox,
    Dialog,
    Grid,
    Group,
    Label,
    List,
    ListItem,
    ListBox,
    ListBoxOption,
    /// Append-only output surface (a transcript, a console).
    Log,
    Image,
    Menu,
    MenuBar,
    MenuItem,
    MenuItemCheckBox,
    MenuItemRadio,
    Splitter,
    /// A value along a track (hue/alpha channel strips, range thumbs).
    Slider,
    ProgressIndicator,
    RadioGroup,
    RadioButton,
    Region,
    Row,
    SpinButton,
    Status,
    Switch,
    Tab,
    TabList,
    TextInput,
    Toolbar,
    Tooltip,
    Tree,
    TreeItem,
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
    /// Hierarchy level (1-based) — a treeitem's depth, announced as
    /// "level N" rather than inferred from indentation nobody can see.
    pub level: Option<usize>,
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

    pub fn input(value: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            kind: NodeKind::Input {
                value: value.into(),
                placeholder: placeholder.into(),
            },
            ..Self::default()
        }
    }

    /// Give an input a caret: a selection range and the colours to draw it in.
    ///
    /// Position is the *backend's* to compute — mapping a character index to an
    /// x offset means measuring shaped glyphs, which no target-independent
    /// layer can do. The component supplies the index and the colour; the
    /// backend supplies the pixels.
    pub fn with_caret(
        mut self,
        selection_range: (usize, usize),
        caret_color: ColorValue,
        selection_color: ColorValue,
    ) -> Self {
        self.caret = Some(NodeCaret {
            selection: selection_range,
            caret_color,
            selection_color,
            showing_placeholder: false,
        });
        self
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
            NodeKind::Input { value, placeholder } => {
                out.push(if value.is_empty() { placeholder } else { value });
            }
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
            NodeKind::Input { value, .. } => format!("Input({value:?})"),
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

    /// Stamp one semantic token role (the `data-*` counterpart).
    pub fn token_role(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.roles.insert(name.into(), value.into());
        self
    }

    pub fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.a11y.label = Some(label.into());
        self
    }
}
