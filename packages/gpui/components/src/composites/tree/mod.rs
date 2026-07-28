//! Tree — real GPUI component backed by TreeSpec.
//!
//! Renders the currently-visible rows (a node is visible when every ancestor
//! branch is expanded) as a flat column. Depth is expressed with indent cells
//! whose left border draws the ancestor guide lines. Branches show a chevron
//! glyph twisty; leaves reserve a twisty-sized spacer for alignment.
//!
//! Keyboard: when any interaction callback is set, the root becomes a single
//! focusable element owning an `on_key_down` handler. Keys route to the nearest
//! focusable ancestor, so clicking a (clickable, non-focusable) row focuses the
//! root and arrow/Home/End/Enter/Space drive navigation against
//! `TreeSpec::visible_rows`. The focus ring is drawn app-driven on
//! `focused_value` rather than via GPUI's native element focus.

use std::rc::Rc;


use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    compute_selection, ControlDensity, ControlSize, DropPosition,
    SemanticControlSizeRole, TreeNode, TreeSelectionMode, TreeSpec,
};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

type Handler = Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>;
type ContextFn = Rc<dyn Fn(&TreeContextRequest, &mut Window, &mut App) + 'static>;

type ReorderFn = Rc<dyn Fn(&TreeReorderRequest, &mut Window, &mut App) + 'static>;

/// Whether the node with `value` is disabled (searched across the whole tree).
fn find_disabled(nodes: &[TreeNode], value: &str) -> Option<bool> {
    for n in nodes {
        if n.value == value {
            return Some(n.is_disabled);
        }
        if let Some(d) = find_disabled(&n.children, value) {
            return Some(d);
        }
    }
    None
}
type SelectionFn = Rc<dyn Fn(&TreeSelectionUpdate, &mut Window, &mut App) + 'static>;
type DragOverFn = Rc<dyn Fn(&TreeDragOver, &mut Window, &mut App) + 'static>;

/// The next selection state computed by the component (multi-select aware).
pub struct TreeSelectionUpdate {
    pub values: Vec<String>,
    /// New range anchor (the host must round-trip this back via `selection_anchor`).
    pub anchor: Option<String>,
    /// The node that should hold keyboard focus after the update.
    pub focused: String,
}

/// A drag hovering over a row: the target value and where it would land.
pub struct TreeDragOver {
    pub value: String,
    pub position: DropPosition,
}

/// A right-click on a tree row: the node value and the pointer position.
pub struct TreeContextRequest {
    pub value: String,
    pub position: Point<Pixels>,
}

/// A reorder request from drag-drop or keyboard: move `from` relative to `to`.
pub struct TreeReorderRequest {
    pub from: String,
    pub to: String,
    pub position: DropPosition,
}

/// Drag payload carried while reordering a tree row.
#[derive(Clone)]
struct NodeDragPayload {
    value: String,
    label: String,
}

/// The floating preview rendered next to the cursor during a drag.
struct TreeDragPreview {
    label: String,
    bg: Hsla,
    fg: Hsla,
}

impl Render for TreeDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px(px(8.0))
            .py(px(2.0))
            .rounded(px(4.0))
            .bg(self.bg)
            .text_color(self.fg)
            .child(SharedString::from(self.label.clone()))
    }
}

/// Shared selection context for computing the next selection set on click /
/// Space / Shift+Arrow, mirroring the Svelte reference (replace / toggle / range).
struct SelectionCtx {
    /// All visible values, in render order.
    order: Vec<String>,
    /// Visible, non-disabled values (range selection skips disabled).
    selectable: Vec<String>,
    selected: Vec<String>,
    anchor: Option<String>,
    handler: Option<SelectionFn>,
}

impl SelectionCtx {
    /// Compute the next selection via the shared, unit-tested `compute_selection`.
    fn build(&self, value: &str, mode: TreeSelectionMode) -> TreeSelectionUpdate {
        let r = compute_selection(
            &self.order,
            &self.selectable,
            &self.selected,
            self.anchor.as_deref(),
            value,
            mode,
        );
        TreeSelectionUpdate {
            values: r.values,
            anchor: r.anchor,
            focused: value.to_string(),
        }
    }

    /// Replace selection with a single value (plain click / Enter).
    fn replace(&self, value: &str) -> TreeSelectionUpdate {
        self.build(value, TreeSelectionMode::Replace)
    }

    /// Toggle a value in the selection set (Ctrl/Cmd+click, Space).
    fn toggle(&self, value: &str) -> TreeSelectionUpdate {
        self.build(value, TreeSelectionMode::Toggle)
    }

    /// Extend the selection range from the anchor to `value` (Shift+click/arrow).
    fn extend(&self, value: &str) -> TreeSelectionUpdate {
        self.build(value, TreeSelectionMode::Range)
    }

    fn emit(&self, update: TreeSelectionUpdate, window: &mut Window, cx: &mut App) {
        if let Some(h) = &self.handler {
            h(&update, window, cx);
        }
    }
}

/// A real GPUI tree component backed by `TreeSpec`.
pub struct Tree {
    spec: TreeSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Handler>,
    on_focus_change: Option<Handler>,
    on_toggle_expand: Option<Handler>,
    on_activate: Option<Handler>,
    on_check: Option<Handler>,
    on_rename_start: Option<Handler>,
    on_rename_change: Option<Handler>,
    on_rename_commit: Option<Handler>,
    on_rename_cancel: Option<Handler>,
    on_context_menu: Option<ContextFn>,
    on_reorder: Option<ReorderFn>,
    on_selection_change: Option<SelectionFn>,
    on_drag_over: Option<DragOverFn>,
    selection_anchor: Option<String>,
}

impl std::ops::Deref for Tree {
    type Target = TreeSpec;
    fn deref(&self) -> &TreeSpec {
        &self.spec
    }
}

impl Tree {
    pub fn new(nodes: Vec<TreeNode>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(TreeSpec::new(nodes), theme)
    }

    pub fn from_spec(spec: TreeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
            on_focus_change: None,
            on_toggle_expand: None,
            on_activate: None,
            on_check: None,
            on_rename_start: None,
            on_rename_change: None,
            on_rename_commit: None,
            on_rename_cancel: None,
            on_context_menu: None,
            on_reorder: None,
            on_selection_change: None,
            on_drag_over: None,
            selection_anchor: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn selected_values(mut self, v: Vec<String>) -> Self {
        self.spec.selected_values = v;
        self
    }
    pub fn expanded_values(mut self, v: Vec<String>) -> Self {
        self.spec.expanded_values = Some(v);
        self
    }
    pub fn focused_value(mut self, v: impl Into<String>) -> Self {
        self.spec.focused_value = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn show_guides(mut self, v: bool) -> Self {
        self.spec.show_guides = v;
        self
    }
    pub fn show_icons(mut self, v: bool) -> Self {
        self.spec.show_icons = v;
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
    /// Called when keyboard navigation moves the focused node.
    pub fn on_focus_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_focus_change = Some(Rc::new(handler));
        self
    }
    /// Called to toggle a branch's expansion (twisty click or Right/Left key).
    pub fn on_toggle_expand(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle_expand = Some(Rc::new(handler));
        self
    }
    /// Called on Enter / activation (open intent).
    pub fn on_activate(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_activate = Some(Rc::new(handler));
        self
    }
    /// Called when a row's cascade checkbox is toggled.
    pub fn on_check(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_check = Some(Rc::new(handler));
        self
    }
    /// Called when rename is requested (F2) on the focused node.
    pub fn on_rename_start(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_rename_start = Some(Rc::new(handler));
        self
    }
    /// Called on each keystroke while editing, with the draft text. The host
    /// must update `editing_text` so the controlled editor reflects input. The
    /// edited node is the host's current `editing_value`.
    pub fn on_rename_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_rename_change = Some(Rc::new(handler));
        self
    }
    /// Called when an inline rename commits, with the final text.
    pub fn on_rename_commit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_rename_commit = Some(Rc::new(handler));
        self
    }
    /// Called when an inline rename is cancelled (Escape). The argument is the
    /// edited node value.
    pub fn on_rename_cancel(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_rename_cancel = Some(Rc::new(handler));
        self
    }
    /// Called on right-click of a row (value + pointer position).
    pub fn on_context_menu(
        mut self,
        handler: impl Fn(&TreeContextRequest, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_context_menu = Some(Rc::new(handler));
        self
    }
    /// Called when a row is reordered via drag-drop or Alt+Up/Down.
    pub fn on_reorder(
        mut self,
        handler: impl Fn(&TreeReorderRequest, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reorder = Some(Rc::new(handler));
        self
    }
    /// Range anchor for Shift selection (host round-trips it from updates).
    pub fn selection_anchor(mut self, v: Option<String>) -> Self {
        self.selection_anchor = v;
        self
    }
    /// Multi-select aware selection change (replace / toggle / range). When set,
    /// click + Space + Shift+Arrow compute the next set; prefer this over
    /// `on_select` for multi-selectable trees.
    pub fn on_selection_change(
        mut self,
        handler: impl Fn(&TreeSelectionUpdate, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selection_change = Some(Rc::new(handler));
        self
    }
    /// Called while a drag hovers a row, with the computed drop position.
    pub fn on_drag_over(
        mut self,
        handler: impl Fn(&TreeDragOver, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_drag_over = Some(Rc::new(handler));
        self
    }
}

/// Resolved metrics + colors shared across the recursion.
struct TreeMetrics {
    row_height: Pixels,
    row_font: Pixels,
    twisty_size: Pixels,
    chevron_font: Pixels,
    icon_font: f32,
    indent: Pixels,
    row_gap: Pixels,
    row_pad_inline: Pixels,
    row_radius: Pixels,
    show_guides: bool,
    show_icons: bool,
    row_color: Hsla,
    selected_color: Hsla,
    selected_fill: Hsla,
    guide_color: Hsla,
    twisty_color: Hsla,
    icon_color: Hsla,
    hover_bg: Hsla,
    focus_ring: Hsla,
    disabled_opacity: f32,
    focused: Option<String>,
    drag_accent: Hsla,
    drop_target: Option<String>,
    drop_position: DropPosition,
    sel: Rc<SelectionCtx>,
}

impl IntoElement for Tree {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size-dependent metrics (tree rows are denser than SidebarNav) ──
        let (row_height, row_font) = match effective_size {
            ControlSize::Xs => (rem_to_px(1.375), rem_to_px(0.6875)),
            ControlSize::Sm => (rem_to_px(1.5), rem_to_px(0.75)),
            ControlSize::Md => (rem_to_px(1.75), rem_to_px(0.8125)),
            ControlSize::Lg => (rem_to_px(2.0), rem_to_px(0.875)),
            ControlSize::Xl => (rem_to_px(2.25), rem_to_px(0.9375)),
        };

        // ── Density-dependent horizontal rhythm ──
        let (indent, row_gap, row_pad_inline) = match spec.density {
            ControlDensity::Compact => (rem_to_px(0.75), rem_to_px(0.1875), rem_to_px(0.25)),
            ControlDensity::Default => (rem_to_px(1.0), rem_to_px(0.25), rem_to_px(0.375)),
            ControlDensity::Comfortable => (rem_to_px(1.25), rem_to_px(0.375), rem_to_px(0.5)),
        };

        let control_radius = resolve_radius(theme, "radius.control");
        let selected_fill_base = resolve_color(theme, spec.selected_fill_token());
        let guide_base = resolve_color(theme, spec.guide_color_token());
        let elevated_bg = resolve_color(theme, "color.background.elevated");

        // Multi-select context: visible order + selectable (non-disabled) values.
        let visible = spec.visible_rows();
        let order: Vec<String> = visible.iter().map(|r| r.value.clone()).collect();
        let selectable: Vec<String> = order
            .iter()
            .filter(|v| !find_disabled(&spec.nodes, v).unwrap_or(false))
            .cloned()
            .collect();
        let sel = Rc::new(SelectionCtx {
            order,
            selectable,
            selected: spec.selected_values.clone(),
            anchor: self.selection_anchor.clone(),
            handler: self.on_selection_change.clone(),
        });

        let m = TreeMetrics {
            row_height: px(row_height),
            row_font: px(row_font),
            // Zeroed on a flat tree so the gutter collapses at every render
            // site from one decision. Contract §7: the twisty aligns leaf
            // labels with branch labels, and where nothing can expand it
            // aligns them with nothing.
            twisty_size: px(if spec.is_flat() { 0.0 } else { row_font * 1.5 }),
            chevron_font: px(row_font * 0.85),
            icon_font: row_font,
            indent: px(indent),
            row_gap: px(row_gap),
            row_pad_inline: px(row_pad_inline),
            // Contract: row radius = control radius − 0.125rem (2px at root). Use
            // rem_to_px so it tracks the root font size, not a raw px literal.
            row_radius: control_radius - px(rem_to_px(0.125)),
            show_guides: spec.show_guides,
            show_icons: spec.show_icons,
            row_color: resolve_color(theme, spec.row_color_token()),
            selected_color: resolve_color(theme, spec.row_selected_color_token()),
            // Accent at 10% (alpha reduction only) for the selected fill.
            selected_fill: Hsla { a: selected_fill_base.a * 0.10, ..selected_fill_base },
            guide_color: Hsla { a: guide_base.a * 0.54, ..guide_base },
            twisty_color: resolve_color(theme, spec.twisty_color_token()),
            icon_color: resolve_color(theme, spec.icon_color_token()),
            hover_bg: Hsla { a: elevated_bg.a * 0.60, ..elevated_bg },
            focus_ring: resolve_color(theme, spec.focus_ring_color_token()),
            disabled_opacity: resolve_opacity(theme, spec.disabled_opacity_token()),
            focused: spec.focused_value.clone(),
            drag_accent: resolve_color(theme, spec.selected_fill_token()),
            drop_target: spec.drop_target_value.clone(),
            drop_position: spec.drop_position,
            sel: sel.clone(),
        };

        let panel_y = rem_to_px(match spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        });

        let mut rows: Vec<AnyElement> = Vec::new();
        self.push_rows(&spec.nodes, 0, &m, &mut rows);

        let root = div()
            .flex()
            .flex_col()
            .min_w(px(0.0))
            .py(px(panel_y))
            .px(px(rem_to_px(0.25)))
            .children(rows);

        // Keyboard: only the interactive tree (with callbacks) becomes a single
        // focusable key-owner, so the static size/density trees don't collide on
        // a shared element id.
        // Any keyboard-served behavior must make the tree a focusable key-owner.
        let interactive = self.on_focus_change.is_some()
            || self.on_toggle_expand.is_some()
            || self.on_activate.is_some()
            || self.on_selection_change.is_some()
            || self.on_reorder.is_some();
        if interactive {
            root.id("poodle-tree")
                .focusable()
                .on_key_down(self.key_handler(sel))
                .into_any_element()
        } else {
            root.into_any_element()
        }
    }
}


mod render;
