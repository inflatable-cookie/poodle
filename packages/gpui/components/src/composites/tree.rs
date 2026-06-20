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

use std::collections::HashMap;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CheckState, CheckboxSpec, ControlDensity, ControlSize, DropPosition, SemanticControlSizeRole,
    SpinnerSpec, TreeNode, TreeSpec,
};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::primitives::{Checkbox, EditableLabel, Icon, Spinner};
use crate::theme_ext::{focus_ring_shadow, resolve_color, resolve_opacity, resolve_radius};

type Handler = Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>;
type ContextFn = Rc<dyn Fn(&TreeContextRequest, &mut Window, &mut App) + 'static>;

type ReorderFn = Rc<dyn Fn(&TreeReorderRequest, &mut Window, &mut App) + 'static>;

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

        let m = TreeMetrics {
            row_height: px(row_height),
            row_font: px(row_font),
            twisty_size: px(row_font * 1.5),
            chevron_font: px(row_font * 0.85),
            icon_font: row_font,
            indent: px(indent),
            row_gap: px(row_gap),
            row_pad_inline: px(row_pad_inline),
            row_radius: control_radius - px(2.0),
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
        let interactive = self.on_focus_change.is_some()
            || self.on_toggle_expand.is_some()
            || self.on_activate.is_some();
        if interactive {
            root.id("poodle-tree")
                .focusable()
                .on_key_down(self.key_handler())
                .into_any_element()
        } else {
            root.into_any_element()
        }
    }
}

impl Tree {
    /// Build the root key handler from the current visible rows + callbacks.
    fn key_handler(&self) -> impl Fn(&KeyDownEvent, &mut Window, &mut App) + 'static {
        let nav = self.spec.visible_rows();
        let focused = self.spec.focused_value.clone();
        let on_focus = self.on_focus_change.clone();
        let on_toggle = self.on_toggle_expand.clone();
        let on_activate = self.on_activate.clone();
        let on_select = self.on_select.clone();
        let on_rename = self.on_rename_start.clone();
        let on_reorder = self.on_reorder.clone();
        let reorderable = self.spec.reorderable;

        // value -> (prev sibling, next sibling), for Alt+Up/Down reorder.
        fn build_siblings(
            nodes: &[TreeNode],
            map: &mut HashMap<String, (Option<String>, Option<String>)>,
        ) {
            for (i, n) in nodes.iter().enumerate() {
                let prev = if i > 0 { Some(nodes[i - 1].value.clone()) } else { None };
                let next = nodes.get(i + 1).map(|n| n.value.clone());
                map.insert(n.value.clone(), (prev, next));
                build_siblings(&n.children, map);
            }
        }
        let mut sib_map: HashMap<String, (Option<String>, Option<String>)> = HashMap::new();
        build_siblings(&self.spec.nodes, &mut sib_map);

        move |event: &KeyDownEvent, window, cx| {
            let key = event.keystroke.key.as_str();
            let idx = focused
                .as_deref()
                .and_then(|f| nav.iter().position(|r| r.value == f));
            let focus = |value: &str, window: &mut Window, cx: &mut App| {
                if let Some(h) = &on_focus {
                    h(value, window, cx);
                }
            };
            match key {
                "down" => {
                    if event.keystroke.modifiers.alt && reorderable {
                        if let (Some(f), Some(h)) = (focused.as_deref(), &on_reorder) {
                            if let Some((_, Some(next))) = sib_map.get(f) {
                                h(
                                    &TreeReorderRequest {
                                        from: f.to_string(),
                                        to: next.clone(),
                                        position: DropPosition::After,
                                    },
                                    window,
                                    cx,
                                );
                            }
                        }
                    } else {
                        let target = match idx {
                            Some(i) => nav.get(i + 1),
                            None => nav.first(),
                        };
                        if let Some(r) = target {
                            focus(&r.value, window, cx);
                        }
                    }
                }
                "up" => {
                    if event.keystroke.modifiers.alt && reorderable {
                        if let (Some(f), Some(h)) = (focused.as_deref(), &on_reorder) {
                            if let Some((Some(prev), _)) = sib_map.get(f) {
                                h(
                                    &TreeReorderRequest {
                                        from: f.to_string(),
                                        to: prev.clone(),
                                        position: DropPosition::Before,
                                    },
                                    window,
                                    cx,
                                );
                            }
                        }
                    } else {
                        let target = match idx {
                            Some(i) if i > 0 => nav.get(i - 1),
                            Some(_) => None,
                            None => nav.last(),
                        };
                        if let Some(r) = target {
                            focus(&r.value, window, cx);
                        }
                    }
                }
                "right" => {
                    if let Some(i) = idx {
                        let r = &nav[i];
                        if r.is_branch && !r.is_expanded {
                            if let Some(h) = &on_toggle {
                                h(&r.value, window, cx);
                            }
                        } else if r.is_branch {
                            if let Some(n) = nav.get(i + 1) {
                                focus(&n.value, window, cx);
                            }
                        }
                    }
                }
                "left" => {
                    if let Some(i) = idx {
                        let r = &nav[i];
                        if r.is_branch && r.is_expanded {
                            if let Some(h) = &on_toggle {
                                h(&r.value, window, cx);
                            }
                        } else if let Some(p) = &r.parent {
                            focus(p, window, cx);
                        }
                    }
                }
                "home" => {
                    if let Some(r) = nav.first() {
                        focus(&r.value, window, cx);
                    }
                }
                "end" => {
                    if let Some(r) = nav.last() {
                        focus(&r.value, window, cx);
                    }
                }
                "enter" => {
                    if let Some(i) = idx {
                        if let Some(h) = &on_select {
                            h(&nav[i].value, window, cx);
                        }
                        if let Some(h) = &on_activate {
                            h(&nav[i].value, window, cx);
                        }
                    }
                }
                "space" | " " => {
                    if let Some(i) = idx {
                        if let Some(h) = &on_select {
                            h(&nav[i].value, window, cx);
                        }
                    }
                }
                "f2" => {
                    if let Some(i) = idx {
                        if let Some(h) = &on_rename {
                            h(&nav[i].value, window, cx);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn push_rows(
        &self,
        nodes: &[TreeNode],
        depth: usize,
        m: &TreeMetrics,
        out: &mut Vec<AnyElement>,
    ) {
        for node in nodes {
            out.push(self.render_row(node, depth, m));
            if self.spec.is_branch(node) && self.spec.is_expanded(&node.value) {
                if node.children.is_empty() {
                    if self.spec.is_loading(&node.value) {
                        out.push(self.render_loading_row(depth + 1, m));
                    }
                } else {
                    self.push_rows(&node.children, depth + 1, m, out);
                }
            }
        }
    }

    /// A non-interactive "Loading…" row with a spinner, shown under a lazy branch.
    fn render_loading_row(&self, depth: usize, m: &TreeMetrics) -> AnyElement {
        let mut row = div()
            .w_full()
            .min_h(m.row_height)
            .px(m.row_pad_inline)
            .flex()
            .items_center()
            .gap(m.row_gap)
            .text_size(m.row_font)
            .text_color(m.row_color);
        for _ in 0..depth {
            row = row.child(div().w(m.indent).h(m.row_height).flex_none());
        }
        row = row.child(
            div()
                .w(m.twisty_size)
                .h(m.row_height)
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .child(Spinner::from_spec(SpinnerSpec::new(), &self.theme)),
        );
        row.child(div().child("Loading…")).into_any_element()
    }

    fn render_row(&self, node: &TreeNode, depth: usize, m: &TreeMetrics) -> AnyElement {
        let spec = &self.spec;
        let is_branch = spec.is_branch(node);
        let is_expanded = is_branch && spec.is_expanded(&node.value);
        let is_selected = spec.is_selected(&node.value);
        let is_focused = m.focused.as_deref() == Some(node.value.as_str());

        let row_id = SharedString::from(format!("tree-row-{}", node.value));

        let (text_color, weight) = if is_selected {
            (m.selected_color, FontWeight::SEMIBOLD)
        } else {
            (m.row_color, FontWeight::MEDIUM)
        };

        let mut row = div()
            .id(row_id)
            .w_full()
            .min_w(px(0.0))
            .min_h(m.row_height)
            .px(m.row_pad_inline)
            .rounded(m.row_radius)
            .flex()
            .items_center()
            .gap(m.row_gap)
            .text_size(m.row_font)
            .text_color(text_color)
            .font_weight(weight)
            .line_height(relative(1.3));

        // App-driven focus ring on the focused node (keyboard focus is tracked in
        // app state via focused_value, not GPUI's per-element focus).
        if is_focused {
            row = row.shadow(focus_ring_shadow(m.focus_ring));
        }

        // Indent cells (left border draws the ancestor guide line).
        for _ in 0..depth {
            let mut cell = div().w(m.indent).h(m.row_height).flex_none();
            if m.show_guides {
                cell = cell.border_l_1().border_color(m.guide_color);
            }
            row = row.child(cell);
        }

        // Twisty: chevron glyph for branches, empty spacer for leaves. Clicking it
        // toggles expansion without changing selection.
        let mut twisty = div()
            .id(SharedString::from(format!("tree-twisty-{}", node.value)))
            .w(m.twisty_size)
            .h(m.row_height)
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .text_size(m.chevron_font)
            .text_color(m.twisty_color);
        if is_branch {
            twisty = twisty.child(if is_expanded { "▾" } else { "▸" });
            if !node.is_disabled {
                if let Some(ref handler) = self.on_toggle_expand {
                    let handler = Rc::clone(handler);
                    let val = node.value.clone();
                    twisty = twisty.cursor_pointer().on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }
        }
        row = row.child(twisty);

        // Optional cascade checkbox (leading, before the icon).
        if spec.show_checkboxes {
            let cs = spec.check_state(node);
            let mut checkbox = Checkbox::from_spec(
                CheckboxSpec::new()
                    .with_checked(matches!(cs, CheckState::Checked))
                    .with_mixed(matches!(cs, CheckState::Mixed))
                    .with_disabled(node.is_disabled)
                    .with_size(ControlSize::Xs),
                &self.theme,
            )
            .with_id(format!("tree-check-{}", node.value));
            if !node.is_disabled {
                if let Some(ref handler) = self.on_check {
                    let handler = Rc::clone(handler);
                    let val = node.value.clone();
                    checkbox = checkbox.on_change(move |_checked: &bool, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }
            row = row.child(div().flex_none().flex().items_center().child(checkbox));
        }

        // Optional leading icon (reserve the slot even when the node has none).
        if m.show_icons {
            match &node.icon {
                Some(name) => {
                    row = row.child(
                        Icon::new(name.clone(), &self.theme)
                            .with_px_size(m.icon_font)
                            .with_color(m.icon_color),
                    );
                }
                None => {
                    row = row.child(div().w(px(m.icon_font)).flex_none());
                }
            }
        }

        // Label, or an inline-rename editor when this node is being edited.
        if spec.is_editing(&node.value) {
            let mut editor = EditableLabel::new(&self.theme)
                .value(spec.editing_text.clone())
                .editing(true)
                .with_id(format!("tree-edit-{}", node.value))
                .size(ControlSize::Xs);
            if let Some(ref change) = self.on_rename_change {
                let change = Rc::clone(change);
                editor = editor.on_change(move |text, window, cx| change(text, window, cx));
            }
            if let Some(ref commit) = self.on_rename_commit {
                let commit = Rc::clone(commit);
                editor = editor.on_commit(move |text, window, cx| commit(text, window, cx));
            }
            if let Some(ref cancel) = self.on_rename_cancel {
                let cancel = Rc::clone(cancel);
                let val = node.value.clone();
                editor = editor.on_cancel(move |window, cx| cancel(&val, window, cx));
            }
            row = row.child(div().flex_1().min_w(px(0.0)).child(editor));
        } else {
            row = row.child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(SharedString::from(node.label.clone())),
            );
        }

        // Selected fill (alpha-only). The contract's inset ring is a Svelte-only
        // refinement; a layout-affecting border would shift unselected rows.
        if is_selected {
            row = row.bg(m.selected_fill);
        }

        if node.is_disabled {
            row = row
                .opacity(m.disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            let hover_bg = m.hover_bg;
            let hover_text = m.selected_color;
            row = row.cursor_pointer().hover(move |s| s.text_color(hover_text).bg(hover_bg));

            // Clicking a row focuses + selects it.
            let on_focus = self.on_focus_change.clone();
            let on_select = self.on_select.clone();
            if on_focus.is_some() || on_select.is_some() {
                let val = node.value.clone();
                row = row.on_click(move |_event, window, cx| {
                    if let Some(h) = &on_focus {
                        h(&val, window, cx);
                    }
                    if let Some(h) = &on_select {
                        h(&val, window, cx);
                    }
                });
            }

            // Right-click requests a context menu at the pointer.
            if let Some(ref ctx) = self.on_context_menu {
                let ctx = Rc::clone(ctx);
                let val = node.value.clone();
                row = row.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                    ctx(
                        &TreeContextRequest {
                            value: val.clone(),
                            position: event.position,
                        },
                        window,
                        cx,
                    );
                });
            }

            // Drag-and-drop reorder.
            if spec.reorderable {
                let preview_bg = Hsla { a: 1.0, ..m.hover_bg };
                let preview_fg = m.selected_color;
                row = row.on_drag(
                    NodeDragPayload {
                        value: node.value.clone(),
                        label: node.label.clone(),
                    },
                    move |payload, _offset, _window, cx| {
                        cx.new(|_| TreeDragPreview {
                            label: payload.label.clone(),
                            bg: preview_bg,
                            fg: preview_fg,
                        })
                    },
                );
                let hl = m.selected_fill;
                row = row.drag_over::<NodeDragPayload>(move |style, _payload, _w, _cx| style.bg(hl));
                if let Some(ref reorder) = self.on_reorder {
                    let reorder = Rc::clone(reorder);
                    let to = node.value.clone();
                    let to_branch = is_branch;
                    row = row.on_drop::<NodeDragPayload>(move |payload, window, cx| {
                        let position = if to_branch {
                            DropPosition::Inside
                        } else {
                            DropPosition::After
                        };
                        reorder(
                            &TreeReorderRequest {
                                from: payload.value.clone(),
                                to: to.clone(),
                                position,
                            },
                            window,
                            cx,
                        );
                    });
                }
            }
        }

        row.into_any_element()
    }
}
