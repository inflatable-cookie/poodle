//! HistoryCenter — the native composition: a titlebar trigger cluster
//! (undo / list / redo) plus a popover surface rendering the flat history
//! list, with node-owned fork disclosure, a per-anchor fork picker, and the
//! inline branch rename.
//!
//! Contract: `docs/contracts/components/history-center.md`
//!
//! The composition renders rows the behaviour core derived
//! ([`poodle_headless::history_center::history_center_visible_rows`]) and
//! knows nothing about topology beyond the depth number each row carries.
//! There is no recursion here and no depth cap: a row at level 9 renders the
//! same way a row at level 1 does, one inset step further in.
//!
//! Open state, the disclosure tree, the pick and the rename buffer are all
//! host-owned, exactly as `MessageCenter`'s open state is. The host runs the
//! machine, hands the resulting [`HistoryCenterView`] here, and rebuilds when
//! it changes. This module emits the intent; the backend turns it into real
//! listeners.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::history_center::{
    HistoryCenterRow, HistoryCenterRowId, HistoryCenterRowKind, HistoryContinuation,
};
use poodle_node::{
    CrossAxisAlignment, DismissReason, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeKey, NodeRole, StylePatch,
};
use poodle_specs::{
    ControlDensity, ControlSize, EmptyStateSpec, HistoryCenterSpec, HistoryCenterStatus,
    IconButtonSpec, PopoverSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant,
};

use crate::empty_state::empty_state;
use crate::floating_overlay::floating_overlay;
use crate::icon_button::icon_button;
use crate::popover::popover_surface;
use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size};
use crate::spinner::spinner;

/// Stable semantic part ids. Backends key per-instance state on `runtime_id`;
/// the semantic ids stay readable, and accessibility relationships point at
/// them.
pub const HISTORY_CENTER_UNDO_ID: &str = "history-center:undo";
pub const HISTORY_CENTER_REDO_ID: &str = "history-center:redo";
pub const HISTORY_CENTER_LIST_TRIGGER_ID: &str = "history-center:list-trigger";
pub const HISTORY_CENTER_SURFACE_ID: &str = "history-center:surface";
pub const HISTORY_CENTER_LIST_ID: &str = "history-center:list";
pub const HISTORY_CENTER_EMPTY_ID: &str = "history-center:empty";
pub const HISTORY_CENTER_STATUS_ID: &str = "history-center:status";
pub const HISTORY_CENTER_REJECTION_ID: &str = "history-center:rejection";

/// One inset step per depth level. Depth drives padding and nothing else, and
/// it is never saturated — the metric is internal, not recipe-themable.
const DEPTH_INSET_REM: f32 = 0.875;

/// The list's own height cap. It scrolls inside this rather than growing the
/// surface to the height of the history; 28rem matches the web's
/// `min(28rem, 60vh)` at the harness's pinned root size.
const LIST_MAX_HEIGHT_REM: f32 = 28.0;

fn part_id(instance: Option<&str>, semantic: &str) -> Option<String> {
    instance.map(|scope| format!("{scope}:{semantic}"))
}

pub fn history_center_row_id(entry_id: &str) -> String {
    format!("history-center:row:{entry_id}")
}

pub fn history_center_entry_id(entry_id: &str) -> String {
    format!("history-center:entry:{entry_id}")
}

pub fn history_center_disclosure_id(entry_id: &str) -> String {
    format!("history-center:disclosure:{entry_id}")
}

pub fn history_center_picker_id(anchor_entry_id: &str) -> String {
    format!("history-center:picker:{anchor_entry_id}")
}

pub fn history_center_picker_select_id(anchor_entry_id: &str) -> String {
    format!("history-center:picker-select:{anchor_entry_id}")
}

pub fn history_center_picker_option_id(fork_entry_id: &str) -> String {
    format!("history-center:picker-option:{fork_entry_id}")
}

pub fn history_center_picker_actions_id(anchor_entry_id: &str) -> String {
    format!("history-center:picker-actions:{anchor_entry_id}")
}

/// One actions menu is open at a time, so its items are singletons.
pub const HISTORY_CENTER_ACTION_RENAME_ID: &str = "history-center:action-rename";
pub const HISTORY_CENTER_ACTION_CHECKOUT_ID: &str = "history-center:action-checkout";

pub fn history_center_rename_input_id(anchor_entry_id: &str) -> String {
    format!("history-center:rename-input:{anchor_entry_id}")
}

pub fn history_center_not_yet_loaded_id(anchor_entry_id: &str) -> String {
    format!("history-center:not-yet-loaded:{anchor_entry_id}")
}

/// The layer the open surface registers on the backend dismiss stack.
pub fn history_center_layer_id(instance_id: Option<&str>) -> String {
    instance_id
        .map(|scope| format!("history-center-layer:{scope}"))
        .unwrap_or_else(|| "history-center-layer".to_owned())
}

/// An open inline rename: which picker it belongs to, which branch it commits
/// to, and the buffer the operator has typed so far. The buffer is host-owned
/// — a rename enforces no protocol rule, so nothing here validates it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryCenterRename {
    pub anchor_entry_id: String,
    pub branch_id: String,
    pub value: String,
}

/// Everything the host resolved before this frame: the derived rows plus the
/// UI state the machine and the shells own. The composition reads it and
/// decides nothing.
#[derive(Clone, Debug, Default)]
pub struct HistoryCenterView {
    /// Whether the surface is open. Host-owned: the machine holds this state,
    /// and the spec's `open`/`defaultOpen` is only what a host seeds from. A
    /// frame that read the spec would render closed after every toggle.
    pub is_open: bool,
    /// The flat visible rows, in display order.
    pub rows: Vec<HistoryCenterRow>,
    /// Roving focus identity.
    pub focus_row: Option<HistoryCenterRowId>,
    /// Anchors with an open fork, for the disclosure's expanded projection.
    pub open_anchors: Vec<String>,
    /// The currently displayed rejection message.
    pub rejection: Option<String>,
    /// The picker whose select listbox is open, if any.
    pub open_select_anchor: Option<String>,
    /// The picker whose actions menu is open, if any.
    pub open_actions_anchor: Option<String>,
    /// The open inline rename, if any.
    pub rename: Option<HistoryCenterRename>,
}

impl HistoryCenterView {
    fn is_open_at(&self, entry_id: &str) -> bool {
        self.open_anchors.iter().any(|id| id == entry_id)
    }

    fn is_focused(&self, row: &HistoryCenterRow) -> bool {
        self.focus_row.as_ref() == Some(&row.id())
    }

    fn renaming_at(&self, anchor_entry_id: &str) -> Option<&HistoryCenterRename> {
        self.rename
            .as_ref()
            .filter(|rename| rename.anchor_entry_id == anchor_entry_id)
    }
}

type Command = Arc<dyn Fn() + Send + Sync>;
type EntryCommand = Arc<dyn Fn(&str) + Send + Sync>;

/// Host-owned interaction intent. The backend turns these into real listeners;
/// none is ever invoked speculatively.
#[derive(Clone, Default)]
pub struct HistoryCenterHandlers {
    pub on_undo: Option<Command>,
    pub on_redo: Option<Command>,
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// A row was activated. Only entry rows navigate; the machine decides.
    pub on_activate_row: Option<Arc<dyn Fn(&HistoryCenterRowId) + Send + Sync>>,
    /// The fork disclosure at an entry was toggled.
    pub on_disclose: Option<EntryCommand>,
    /// A fork was picked in a picker. The pick previews; it commits nothing.
    pub on_pick: Option<EntryCommand>,
    /// The picker's select trigger was activated (open or close its listbox).
    pub on_select_toggle: Option<EntryCommand>,
    /// The picker's actions menu trigger was activated.
    pub on_actions_toggle: Option<EntryCommand>,
    /// The actions menu's checkout item was chosen for an anchor.
    pub on_checkout: Option<EntryCommand>,
    /// The actions menu's rename item was chosen for an anchor.
    pub on_rename_open: Option<EntryCommand>,
    /// A keystroke reached the open rename input. The host owns the buffer;
    /// the component enforces no protocol rule on the name.
    pub on_rename_key: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Content was inserted into the open rename input.
    pub on_rename_insert: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// A navigation key reached a focused row. The host moves roving focus;
    /// returning no target leaves the platform focus move to the host's own
    /// focus effect, which is the one the machine emitted.
    pub on_row_key: Option<Arc<dyn Fn(NodeKey) + Send + Sync>>,
    /// Document-level dismissal. The component applies its own guards.
    pub on_dismiss: Option<Arc<dyn Fn(DismissReason) + Send + Sync>>,
    /// Stable native instance scope.
    pub instance_id: Option<String>,
}

/// The full composition: trigger cluster plus the conditional surface.
pub fn history_center(
    spec: &HistoryCenterSpec,
    theme: &dyn ThemeProvider,
    view: &HistoryCenterView,
    handlers: &HistoryCenterHandlers,
) -> Node {
    let instance = handlers.instance_id.as_deref();
    let size = resolve_semantic_size(spec.size.unwrap_or(ControlSize::Md), spec.size_role);
    let density = spec.density.unwrap_or(ControlDensity::Default);
    let anchor = rem_to_px(control_height_rem(size));
    let open = view.is_open;

    let cluster = trigger_cluster(spec, theme, handlers, view, size, density, open, instance);

    let surface = open.then(|| {
        let popover_spec = PopoverSpec::new()
            .with_open(true)
            .with_placement(spec.placement)
            .with_aria_label(spec.surface_label())
            .with_surface_min_width(poodle_specs::Dimension::new("28rem"))
            .with_surface_max_width(poodle_specs::Dimension::new("38rem"));
        let mut node = popover_surface(
            &popover_spec,
            theme,
            Some(surface_content(spec, theme, view, handlers, size, density)),
        );
        node.id = Some(HISTORY_CENTER_SURFACE_ID.to_owned());
        node.runtime_id = part_id(instance, HISTORY_CENTER_SURFACE_ID);
        node.interaction.dismiss_layer = Some(history_center_layer_id(instance));
        node.interaction.on_dismiss = handlers.on_dismiss.clone();
        node
    });

    let mut wrapper = floating_overlay(
        cluster,
        surface,
        spec.placement,
        anchor,
        anchor * 3.0,
        crate::floating_overlay::OVERLAY_GAP_PX,
    );
    // Token roles project onto the composition root, as the web root's
    // `data-*` attributes do. Values are kebab-cased like the web's.
    wrapper
        .roles
        .insert("placement".to_owned(), kebab_case_debug(spec.placement));
    wrapper
        .roles
        .insert("status".to_owned(), status_role(spec.status));
    wrapper
        .roles
        .insert("size".to_owned(), kebab_case_debug(size));
    wrapper
        .roles
        .insert("density".to_owned(), kebab_case_debug(density));
    wrapper
}

// ── Trigger cluster ────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn trigger_cluster(
    spec: &HistoryCenterSpec,
    theme: &dyn ThemeProvider,
    handlers: &HistoryCenterHandlers,
    _view: &HistoryCenterView,
    size: ControlSize,
    density: ControlDensity,
    open: bool,
    instance: Option<&str>,
) -> Node {
    let mut cluster = Node::container();
    {
        let s = &mut cluster.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        // Titlebar space is premium: the cluster adds nothing but the gap.
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
    }

    let mut undo = icon_button(
        &IconButtonSpec::new()
            .with_icon("undo")
            .with_aria_label(&spec.undo_label)
            .with_tooltip(&spec.undo_label)
            .with_disabled(spec.undo_is_disabled())
            .with_size(size)
            .with_density(density),
        theme,
        handlers.on_undo.clone(),
    );
    undo.id = Some(HISTORY_CENTER_UNDO_ID.to_owned());
    undo.runtime_id = part_id(instance, HISTORY_CENTER_UNDO_ID);
    undo.a11y.role = Some(NodeRole::Button);
    undo.a11y.label = Some(spec.undo_label.clone());
    undo.interaction.disabled = spec.undo_is_disabled();
    // The focus ring is also what makes the control reachable: the backend
    // creates a focus handle only for a focusable node that carries a focus
    // patch, so a control without one can never be focused or activated by
    // keyboard. `icon_button` supplies neither today.
    undo.style.focus = Some(focus_ring(theme));

    let mut redo = icon_button(
        &IconButtonSpec::new()
            .with_icon("redo")
            .with_aria_label(&spec.redo_label)
            .with_tooltip(&spec.redo_label)
            .with_disabled(spec.redo_is_disabled())
            .with_size(size)
            .with_density(density),
        theme,
        handlers.on_redo.clone(),
    );
    redo.id = Some(HISTORY_CENTER_REDO_ID.to_owned());
    redo.runtime_id = part_id(instance, HISTORY_CENTER_REDO_ID);
    redo.a11y.role = Some(NodeRole::Button);
    redo.a11y.label = Some(spec.redo_label.clone());
    redo.interaction.disabled = spec.redo_is_disabled();
    redo.style.focus = Some(focus_ring(theme));

    cluster
        .child(undo)
        .child(list_trigger(spec, theme, handlers, size, open, instance))
        .child(redo)
}

/// A bare glyph, not an icon button: undo and redo carry the cluster's weight
/// and the disclosure reads narrower between them. It stays a real button, so
/// the keyboard and assistive tech still reach it — only the chrome goes away.
fn list_trigger(
    spec: &HistoryCenterSpec,
    theme: &dyn ThemeProvider,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    open: bool,
    instance: Option<&str>,
) -> Node {
    let mut node = Node::container();
    node.id = Some(HISTORY_CENTER_LIST_TRIGGER_ID.to_owned());
    node.runtime_id = part_id(instance, HISTORY_CENTER_LIST_TRIGGER_ID);
    node.a11y.role = Some(NodeRole::Button);
    node.a11y.label = Some(spec.list_label.clone());
    node.a11y.expanded = Some(open);
    node.a11y.tab_index = Some(0);
    node.a11y.controls = open.then(|| {
        part_id(instance, HISTORY_CENTER_SURFACE_ID)
            .unwrap_or_else(|| HISTORY_CENTER_SURFACE_ID.to_owned())
    });
    node.interaction.focusable = true;
    if let Some(handler) = handlers.on_open_change.clone() {
        node.interaction.on_activate = Some(Arc::new(move || handler(!open)));
    }
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        // A full-height hit area, narrower than the buttons beside it.
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(control_height_rem(size)));
        s.min_width = Some(rem_to_px(1.0));
        s.focus = Some(StylePatch {
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            ..StylePatch::default()
        });
    }
    node.child(icon_glyph("chevron-down", theme, size))
}

// ── Surface ────────────────────────────────────────────────────────────────

fn surface_content(
    spec: &HistoryCenterSpec,
    theme: &dyn ThemeProvider,
    view: &HistoryCenterView,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    density: ControlDensity,
) -> Node {
    let instance = handlers.instance_id.as_deref();
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.sm");
        s.fill_width = true;
    }

    root = root.child(header(spec, theme, view));

    if let Some(message) = view.rejection.as_deref() {
        root = root.child(rejection_notice(message, theme, instance));
    }

    if let Some(line) = spec.status_line() {
        root = root.child(status_row(line, theme, size, instance));
    }

    if view.rows.is_empty() {
        // Absence is the signal: no rows means the empty state, not an empty
        // list region pretending to be a history.
        let mut empty = empty_state(
            &EmptyStateSpec::new(&spec.title)
                .with_message(&spec.empty_message)
                .with_compact(true)
                .with_density(density),
            theme,
        );
        empty.id = Some(HISTORY_CENTER_EMPTY_ID.to_owned());
        empty.runtime_id = part_id(instance, HISTORY_CENTER_EMPTY_ID);
        return root.child(empty);
    }

    let mut list = Node::container();
    list.id = Some(HISTORY_CENTER_LIST_ID.to_owned());
    list.runtime_id = part_id(instance, HISTORY_CENTER_LIST_ID);
    list.a11y.role = Some(NodeRole::List);
    list.a11y.label = Some(spec.list_label.clone());
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        // Bounded and scrolling, rather than a column that grows with the
        // history.
        s.max_height = Some(rem_to_px(LIST_MAX_HEIGHT_REM));
        s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
        s.fill_width = true;
    }
    for row in &view.rows {
        list = list.child(row_node(row, theme, view, handlers, size, density));
    }
    root.child(list)
}

fn header(spec: &HistoryCenterSpec, theme: &dyn ThemeProvider, view: &HistoryCenterView) -> Node {
    let entry_rows = view
        .rows
        .iter()
        .filter(|row| matches!(row, HistoryCenterRow::Entry { .. }))
        .count();
    let mut node = Node::container();
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.md");
        s.fill_width = true;
    }
    let mut title = Node::text(spec.title.clone());
    title.style.text_weight = Some(600);
    let mut count = Node::text(entry_rows.to_string());
    count.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
    node.child(title).child(count)
}

/// A polite live region, never an alert: a rejection is information, not an
/// interruption.
fn rejection_notice(message: &str, theme: &dyn ThemeProvider, instance: Option<&str>) -> Node {
    let mut node = Node::container();
    node.id = Some(HISTORY_CENTER_REJECTION_ID.to_owned());
    node.runtime_id = part_id(instance, HISTORY_CENTER_REJECTION_ID);
    node.a11y.role = Some(NodeRole::Status);
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = theme.resolve_space("space.inline.sm");
        pad.right = theme.resolve_space("space.inline.sm");
        pad.top = rem_to_px(0.25);
        pad.bottom = rem_to_px(0.25);
        s.descriptor.border.width = theme.resolve_space("border.width.default");
        s.descriptor.border.color = theme.resolve_color("color.status.danger");
        s.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));
        s.fill_width = true;
    }
    node.child(Node::text(message.to_owned()))
}

fn status_row(
    line: &str,
    theme: &dyn ThemeProvider,
    size: ControlSize,
    instance: Option<&str>,
) -> Node {
    let mut node = Node::container();
    node.id = Some(HISTORY_CENTER_STATUS_ID.to_owned());
    node.runtime_id = part_id(instance, HISTORY_CENTER_STATUS_ID);
    node.a11y.role = Some(NodeRole::Status);
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        s.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
        s.fill_width = true;
    }
    node.child(spinner(
        &SpinnerSpec::new()
            .with_variant(SpinnerVariant::Ring)
            .with_size(spinner_size(size))
            .with_tone(SpinnerTone::Muted),
        theme,
    ))
    .child(Node::text(line.to_owned()))
}

// ── Rows ───────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn row_node(
    row: &HistoryCenterRow,
    theme: &dyn ThemeProvider,
    view: &HistoryCenterView,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    density: ControlDensity,
) -> Node {
    let instance = handlers.instance_id.as_deref();
    let id = row.id();
    let semantic = match id.kind {
        HistoryCenterRowKind::Entry => history_center_row_id(&id.entry_id),
        HistoryCenterRowKind::Picker => history_center_picker_id(&id.entry_id),
        HistoryCenterRowKind::NotYetLoaded => history_center_not_yet_loaded_id(&id.entry_id),
    };

    let mut node = Node::container();
    node.id = Some(semantic.clone());
    node.runtime_id = part_id(instance, &semantic);
    node.a11y.role = Some(NodeRole::ListItem);
    // Depth reaches assistive technology as a level, not as indentation
    // nobody can hear. It is 1-based and never clamped.
    node.a11y.level = Some(row.depth() + 1);
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.25);
        // Depth drives padding and nothing else.
        s.descriptor.layout.spacing.padding.left = rem_to_px(DEPTH_INSET_REM) * row.depth() as f32;
        s.fill_width = true;
    }

    match row {
        HistoryCenterRow::Entry {
            entry, fork_count, ..
        } => {
            let is_open = view.is_open_at(&entry.id);
            let mut node = node.child(entry_button(
                entry.id.clone(),
                entry.label.clone(),
                entry.position,
                entry.is_checkpoint,
                is_open,
                view.is_focused(row),
                theme,
                handlers,
                size,
                instance,
            ));
            if *fork_count > 0 {
                node = node.child(disclosure_button(
                    &entry.id,
                    *fork_count,
                    is_open,
                    view.is_focused(row),
                    theme,
                    handlers,
                    size,
                    instance,
                ));
            }
            node
        }
        HistoryCenterRow::Picker {
            anchor_entry_id,
            continuations,
            picked_entry_id,
            is_disabled,
            ..
        } => node.child(picker(
            anchor_entry_id,
            continuations,
            picked_entry_id.as_deref(),
            *is_disabled,
            theme,
            view,
            handlers,
            size,
            density,
            instance,
        )),
        HistoryCenterRow::NotYetLoaded { .. } => {
            // Never an empty gap, never a dropped entry.
            let mut body = Node::container();
            {
                let s = &mut body.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
                s.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
            }
            node.a11y.tab_index = Some(if view.is_focused(row) { 0 } else { -1 });
            node.interaction.focusable = true;
            node.style.focus = Some(focus_ring(theme));
            node.child(
                body.child(spinner(
                    &SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(SpinnerSize::Xs)
                        .with_tone(SpinnerTone::Muted),
                    theme,
                ))
                .child(Node::text("Loading…")),
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn entry_button(
    entry_id: String,
    label: String,
    position: poodle_specs::HistoryEntryPosition,
    is_checkpoint: bool,
    is_open: bool,
    is_focused: bool,
    theme: &dyn ThemeProvider,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    instance: Option<&str>,
) -> Node {
    let semantic = history_center_entry_id(&entry_id);
    let mut node = Node::container();
    node.id = Some(semantic.clone());
    node.runtime_id = part_id(instance, &semantic);
    node.a11y.role = Some(NodeRole::Button);
    node.a11y.label = Some(label.clone());
    node.a11y.expanded = is_open.then_some(true);
    node.a11y.tab_index = Some(if is_focused { 0 } else { -1 });
    node.interaction.focusable = true;
    if let Some(handler) = handlers.on_activate_row.clone() {
        let row = HistoryCenterRowId::new(HistoryCenterRowKind::Entry, entry_id);
        node.interaction.on_activate = Some(Arc::new(move || handler(&row)));
    }
    if let Some(handler) = handlers.on_row_key.clone() {
        node.interaction.on_key = Some(Arc::new(move |key, _modifiers| {
            handler(key);
            None
        }));
    }
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        s.descriptor.layout.spacing.padding.left = rem_to_px(0.25);
        s.descriptor.layout.spacing.padding.right = rem_to_px(0.25);
        if position == poodle_specs::HistoryEntryPosition::Current {
            s.descriptor.background = Some(theme.resolve_color("color.accent.subtle"));
        }
        s.focus = Some(StylePatch {
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            ..StylePatch::default()
        });
        s.fill_width = true;
    }
    // The pin and the marker are decorative; the label carries the meaning.
    let marker = if is_checkpoint {
        icon_glyph("git-commit-horizontal", theme, size)
    } else {
        position_marker(position, theme)
    };
    node.child(marker).child(Node::text(label))
}

fn position_marker(
    position: poodle_specs::HistoryEntryPosition,
    theme: &dyn ThemeProvider,
) -> Node {
    let mut node = Node::container();
    let dot = rem_to_px(0.375);
    {
        let s = &mut node.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(dot);
        s.descriptor.layout.height = LayoutSizing::Fixed(dot);
        let radius = dot / 2.0;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(match position {
            poodle_specs::HistoryEntryPosition::Current => {
                theme.resolve_color("color.accent.base")
            }
            _ => theme.resolve_color("color.border.subtle"),
        });
    }
    node
}

/// The fork disclosure: the entry row's secondary control and a sibling of the
/// entry button, never nested inside it — no interactive element inside
/// another interactive element.
#[allow(clippy::too_many_arguments)]
fn disclosure_button(
    entry_id: &str,
    fork_count: usize,
    is_open: bool,
    is_focused: bool,
    theme: &dyn ThemeProvider,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    instance: Option<&str>,
) -> Node {
    let semantic = history_center_disclosure_id(entry_id);
    let noun = if fork_count == 1 {
        "continuation"
    } else {
        "continuations"
    };
    let verb = if is_open { "Hide" } else { "Show" };

    let mut node = Node::container();
    node.id = Some(semantic.clone());
    node.runtime_id = part_id(instance, &semantic);
    node.a11y.role = Some(NodeRole::Button);
    node.a11y.label = Some(format!("{verb} {fork_count} {noun}"));
    node.a11y.expanded = Some(is_open);
    node.a11y.tab_index = Some(if is_focused { 0 } else { -1 });
    node.interaction.focusable = true;
    if let Some(handler) = handlers.on_disclose.clone() {
        let entry_id = entry_id.to_owned();
        node.interaction.on_activate = Some(Arc::new(move || handler(&entry_id)));
    }
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
        s.focus = Some(StylePatch {
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            ..StylePatch::default()
        });
    }
    let mut node = node.child(icon_glyph("git-branch", theme, size));
    if fork_count > 1 {
        // The badge is inside the button's label scope and reads as part of
        // its accessible name.
        let mut badge = Node::text(fork_count.to_string());
        badge.style.descriptor.text_color = Some(theme.resolve_color("color.accent.base"));
        badge.style.text_size = Some(rem_to_px(0.625));
        node = node.child(badge);
    }
    node.child(icon_glyph(
        if is_open {
            "chevron-down"
        } else {
            "chevron-right"
        },
        theme,
        size,
    ))
}

// ── Picker ─────────────────────────────────────────────────────────────────

/// The picker serves every open level, the single fork included, and persists
/// for as long as the level is open: the current selection stays visible and a
/// second fork is one interaction away, never a close-and-reopen.
#[allow(clippy::too_many_arguments)]
fn picker(
    anchor_entry_id: &str,
    continuations: &[HistoryContinuation],
    picked_entry_id: Option<&str>,
    is_disabled: bool,
    theme: &dyn ThemeProvider,
    view: &HistoryCenterView,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    density: ControlDensity,
    instance: Option<&str>,
) -> Node {
    let mut controls = Node::container();
    {
        let s = &mut controls.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        s.fill_width = true;
    }

    // While a rename is open the inline input takes the select's place.
    let controls = match view.renaming_at(anchor_entry_id) {
        Some(rename) => controls.child(rename_input(rename, theme, handlers, instance)),
        None => controls.child(picker_select(
            anchor_entry_id,
            continuations,
            picked_entry_id,
            is_disabled,
            theme,
            view,
            handlers,
            size,
            instance,
        )),
    };

    controls.child(picker_actions(
        anchor_entry_id,
        picked_entry_id,
        theme,
        view,
        handlers,
        size,
        density,
        instance,
    ))
}

#[allow(clippy::too_many_arguments)]
fn picker_select(
    anchor_entry_id: &str,
    continuations: &[HistoryContinuation],
    picked_entry_id: Option<&str>,
    is_disabled: bool,
    theme: &dyn ThemeProvider,
    view: &HistoryCenterView,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    instance: Option<&str>,
) -> Node {
    let semantic = history_center_picker_select_id(anchor_entry_id);
    let is_open = view.open_select_anchor.as_deref() == Some(anchor_entry_id);
    let picked = picked_entry_id
        .and_then(|id| continuations.iter().find(|fork| fork.entry_id == id));

    let mut trigger = Node::container();
    trigger.id = Some(semantic.clone());
    trigger.runtime_id = part_id(instance, &semantic);
    trigger.a11y.role = Some(NodeRole::ComboBox);
    trigger.a11y.label = Some("Continuations".to_owned());
    trigger.a11y.expanded = Some(is_open);
    // Disabled from the row's own signal: one fork leaves nothing to choose
    // between. The actions menu never inherits it.
    trigger.interaction.disabled = is_disabled;
    trigger.interaction.focusable = true;
    trigger.a11y.tab_index = Some(if is_disabled { -1 } else { 0 });
    if !is_disabled {
        if let Some(handler) = handlers.on_select_toggle.clone() {
            let anchor = anchor_entry_id.to_owned();
            trigger.interaction.on_activate = Some(Arc::new(move || handler(&anchor)));
        }
    }
    {
        let s = &mut trigger.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        s.descriptor.border.width = theme.resolve_space("border.width.default");
        s.descriptor.border.color = theme.resolve_color("color.border.subtle");
        s.descriptor.layout.spacing.padding.left = rem_to_px(0.375);
        s.descriptor.layout.spacing.padding.right = rem_to_px(0.375);
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(control_height_rem(size)));
        s.descriptor.opacity = if is_disabled { 0.6 } else { 1.0 };
        s.focus = Some(StylePatch {
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            ..StylePatch::default()
        });
        s.fill_width = true;
    }
    // The trigger carries the fork's own facts: its label and the branch it
    // lands on.
    let trigger = trigger.child(icon_glyph("git-branch", theme, ControlSize::Xs)).child(
        Node::text(match picked {
            Some(fork) => format!("{} · {}", fork.label, fork.display_branch()),
            None => "Choose a fork…".to_owned(),
        }),
    );

    if !is_open {
        return trigger;
    }

    let mut listbox = Node::container();
    listbox.a11y.role = Some(NodeRole::ListBox);
    {
        let s = &mut listbox.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.background = Some(theme.resolve_color("color.background.elevated"));
        s.descriptor.border.width = theme.resolve_space("border.width.default");
        s.descriptor.border.color = theme.resolve_color("color.border.subtle");
        s.overlay = true;
    }
    for fork in continuations {
        listbox = listbox.child(picker_option(fork, picked_entry_id, theme, handlers, instance));
    }

    let mut wrapper = Node::container();
    wrapper.style.descriptor.layout.direction = LayoutDirection::Column;
    wrapper.style.fill_width = true;
    wrapper.child(trigger).child(listbox)
}

fn picker_option(
    fork: &HistoryContinuation,
    picked_entry_id: Option<&str>,
    theme: &dyn ThemeProvider,
    handlers: &HistoryCenterHandlers,
    instance: Option<&str>,
) -> Node {
    let semantic = history_center_picker_option_id(&fork.entry_id);
    let mut node = Node::container();
    node.id = Some(semantic.clone());
    node.runtime_id = part_id(instance, &semantic);
    node.a11y.role = Some(NodeRole::ListBoxOption);
    node.a11y.label = Some(fork.label.clone());
    node.a11y.selected = Some(picked_entry_id == Some(fork.entry_id.as_str()));
    node.a11y.tab_index = Some(-1);
    node.interaction.focusable = true;
    if let Some(handler) = handlers.on_pick.clone() {
        let entry_id = fork.entry_id.clone();
        node.interaction.on_activate = Some(Arc::new(move || handler(&entry_id)));
    }
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.padding.left = rem_to_px(0.375);
        s.descriptor.layout.spacing.padding.right = rem_to_px(0.375);
        s.focus = Some(focus_ring(theme));
        s.fill_width = true;
    }
    let mut branch = Node::text(fork.display_branch().to_owned());
    branch.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
    branch.style.text_size = Some(rem_to_px(0.6875));
    node.child(Node::text(fork.label.clone())).child(branch)
}

/// One actions menu, not three buttons. Activating any entry inside a fork's
/// run already navigates to it, so none of these is the row's primary action —
/// checkout exists to make a fork primary *without* moving the position.
#[allow(clippy::too_many_arguments)]
fn picker_actions(
    anchor_entry_id: &str,
    picked_entry_id: Option<&str>,
    theme: &dyn ThemeProvider,
    view: &HistoryCenterView,
    handlers: &HistoryCenterHandlers,
    size: ControlSize,
    density: ControlDensity,
    instance: Option<&str>,
) -> Node {
    let semantic = history_center_picker_actions_id(anchor_entry_id);
    let is_open = view.open_actions_anchor.as_deref() == Some(anchor_entry_id);
    let is_renaming = view.renaming_at(anchor_entry_id).is_some();

    let mut trigger = icon_button(
        &IconButtonSpec::new()
            .with_icon("ellipsis")
            .with_aria_label("Fork actions")
            .with_size(size)
            .with_density(density),
        theme,
        handlers.on_actions_toggle.clone().map(|handler| {
            let anchor = anchor_entry_id.to_owned();
            Arc::new(move || handler(&anchor)) as Command
        }),
    );
    trigger.id = Some(semantic.clone());
    trigger.runtime_id = part_id(instance, &semantic);
    trigger.a11y.role = Some(NodeRole::Button);
    trigger.a11y.label = Some("Fork actions".to_owned());
    trigger.a11y.expanded = Some(is_open);
    trigger.style.focus = Some(focus_ring(theme));

    if !is_open {
        return trigger;
    }

    let mut menu = Node::container();
    menu.a11y.role = Some(NodeRole::Menu);
    {
        let s = &mut menu.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.background = Some(theme.resolve_color("color.background.elevated"));
        s.descriptor.border.width = theme.resolve_space("border.width.default");
        s.descriptor.border.color = theme.resolve_color("color.border.subtle");
        s.overlay = true;
    }
    menu = menu.child(menu_item(
        HISTORY_CENTER_ACTION_RENAME_ID,
        "Rename",
        false,
        handlers.on_rename_open.clone().map(|handler| {
            let anchor = anchor_entry_id.to_owned();
            Arc::new(move || handler(&anchor)) as Command
        }),
        theme,
        instance,
    ));
    menu = menu.child(menu_item(
        HISTORY_CENTER_ACTION_CHECKOUT_ID,
        "Checkout",
        // Disabled with no fork picked, and while a rename is open.
        picked_entry_id.is_none() || is_renaming,
        handlers.on_checkout.clone().map(|handler| {
            let anchor = anchor_entry_id.to_owned();
            Arc::new(move || handler(&anchor)) as Command
        }),
        theme,
        instance,
    ));

    let mut wrapper = Node::container();
    wrapper.style.descriptor.layout.direction = LayoutDirection::Column;
    wrapper.child(trigger).child(menu)
}

fn menu_item(
    semantic: &str,
    label: &str,
    is_disabled: bool,
    on_activate: Option<Command>,
    theme: &dyn ThemeProvider,
    instance: Option<&str>,
) -> Node {
    let mut node = Node::container();
    node.id = Some(semantic.to_owned());
    node.runtime_id = part_id(instance, semantic);
    node.a11y.role = Some(NodeRole::MenuItem);
    node.a11y.label = Some(label.to_owned());
    node.a11y.tab_index = Some(-1);
    node.interaction.focusable = true;
    node.interaction.disabled = is_disabled;
    if !is_disabled {
        node.interaction.on_activate = on_activate;
    }
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.padding.left = rem_to_px(0.5);
        s.descriptor.layout.spacing.padding.right = rem_to_px(0.5);
        s.descriptor.opacity = if is_disabled { 0.5 } else { 1.0 };
        s.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));
        s.focus = Some(focus_ring(theme));
        s.fill_width = true;
    }
    node.child(Node::text(label.to_owned()))
}

/// The inline input that takes the select's place while a rename is open,
/// seeded with the shown fork's current name. `maxBranchNameBytes` caps the
/// buffer as a client-side affordance; it enforces no protocol rule.
fn rename_input(
    rename: &HistoryCenterRename,
    theme: &dyn ThemeProvider,
    handlers: &HistoryCenterHandlers,
    instance: Option<&str>,
) -> Node {
    let semantic = history_center_rename_input_id(&rename.anchor_entry_id);
    let mut node = Node::input(rename.value.clone(), "");
    node.id = Some(semantic.clone());
    node.runtime_id = part_id(instance, &semantic);
    node.a11y.role = Some(NodeRole::TextInput);
    node.a11y.label = Some(format!("Rename branch {}", rename.branch_id));
    node.a11y.tab_index = Some(0);
    node.interaction.focusable = true;
    // The real editing path: keystrokes and inserted content reach the host's
    // buffer the same way any text field's do.
    if let Some(handler) = handlers.on_rename_key.clone() {
        node.interaction.on_edit_key = Some(Arc::new(move |key, modifiers| {
            // A key *name* is not a character: "space" is one keystroke and
            // one space, and a handler that appends the name types the word.
            let key = if key == "space" { " " } else { key };
            // Shift is what makes a capital letter. A handler that drops it
            // silently lower-cases everything the operator typed.
            if modifiers.shift && key.chars().count() == 1 {
                handler(&key.to_uppercase());
            } else {
                handler(key);
            }
        }));
    }
    if let Some(handler) = handlers.on_rename_insert.clone() {
        node.interaction.on_edit_insert = Some(Arc::new(move |text| handler(text)));
    }
    {
        let s = &mut node.style;
        s.descriptor.border.width = theme.resolve_space("border.width.default");
        s.descriptor.border.color = theme.resolve_color("color.border.strong");
        s.descriptor.layout.spacing.padding.left = rem_to_px(0.375);
        s.descriptor.layout.spacing.padding.right = rem_to_px(0.375);
        s.focus = Some(StylePatch {
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            ..StylePatch::default()
        });
        s.fill_width = true;
    }
    node
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// The accent focus ring every focusable part carries. It is not decoration:
/// the backend keys its focus handle on the presence of this patch.
fn focus_ring(theme: &dyn ThemeProvider) -> StylePatch {
    StylePatch {
        border_color: Some(theme.resolve_color("color.accent.focusRing")),
        ..StylePatch::default()
    }
}

fn icon_glyph(name: &str, theme: &dyn ThemeProvider, size: ControlSize) -> Node {
    let mut node = Node::icon(name, rem_to_px(icon_rem(size)));
    node.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
    node
}

fn spinner_size(size: ControlSize) -> SpinnerSize {
    match size {
        ControlSize::Xs => SpinnerSize::Xs,
        ControlSize::Sm => SpinnerSize::Sm,
        ControlSize::Md => SpinnerSize::Md,
        ControlSize::Lg => SpinnerSize::Lg,
        ControlSize::Xl => SpinnerSize::Xl,
    }
}

fn icon_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

fn status_role(status: HistoryCenterStatus) -> String {
    match status {
        HistoryCenterStatus::Idle => "idle",
        HistoryCenterStatus::Loading => "loading",
        HistoryCenterStatus::Failed => "failed",
    }
    .to_owned()
}

/// Debug-name → kebab-case, matching the web's `data-*` values.
fn kebab_case_debug<T: std::fmt::Debug>(value: T) -> String {
    let debug = format!("{value:?}");
    let mut out = String::with_capacity(debug.len() + 4);
    for ch in debug.chars() {
        if ch.is_uppercase() {
            if !out.is_empty() {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::history_center::{
        history_center_visible_rows, HistoryCenterOpenFork, HistoryEntry, HistoryPathPage,
    };

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn entry(id: &str, label: &str, continuation_count: usize) -> HistoryEntry {
        HistoryEntry::new(id, label).with_continuation_count(continuation_count)
    }

    fn spine() -> Vec<HistoryPathPage> {
        vec![HistoryPathPage::new(vec![
            entry("e3", "Raise gain", 0),
            entry("e2", "Trim tail", 3),
            entry("e1", "Import stems", 1),
        ])]
    }

    fn view_for(pages: &[HistoryPathPage], open: &[HistoryCenterOpenFork]) -> HistoryCenterView {
        HistoryCenterView {
            is_open: true,
            rows: history_center_visible_rows(Some(&pages.to_vec()), open),
            open_anchors: open.iter().map(|l| l.anchor_entry_id.clone()).collect(),
            ..HistoryCenterView::default()
        }
    }

    fn find<'a>(node: &'a Node, id: &str) -> Option<&'a Node> {
        node.find(&|candidate| candidate.id.as_deref() == Some(id))
    }

    fn open_spec() -> HistoryCenterSpec {
        HistoryCenterSpec::new().with_open(true)
    }

    fn open_view() -> HistoryCenterView {
        HistoryCenterView {
            is_open: true,
            ..HistoryCenterView::default()
        }
    }

    /// Every row is addressable by its entry's own id, and the hierarchy the
    /// derivation computed reaches assistive technology as a level.
    #[test]
    fn rows_carry_stable_identity_and_an_announced_level() {
        let pages = spine();
        let node = history_center(
            &open_spec(),
            &theme(),
            &view_for(&pages, &[]),
            &HistoryCenterHandlers::default(),
        );
        for id in ["e1", "e2", "e3"] {
            let row = find(&node, &history_center_row_id(id))
                .unwrap_or_else(|| panic!("row {id} renders"));
            assert_eq!(row.a11y.role, Some(NodeRole::ListItem));
            assert_eq!(row.a11y.level, Some(1));
        }
        let entry = find(&node, &history_center_entry_id("e2")).expect("entry button renders");
        assert_eq!(entry.a11y.role, Some(NodeRole::Button));
        assert_eq!(entry.a11y.label.as_deref(), Some("Trim tail"));
    }

    /// A disclosed run sits one level below its anchor, and the anchor's own
    /// rows do not move.
    #[test]
    fn a_disclosed_run_nests_one_level_without_moving_the_spine() {
        let pages = spine();
        let level = HistoryCenterOpenFork {
            anchor_entry_id: "e2".to_owned(),
            continuations: Some(vec![HistoryContinuation::new("f1", "Widen", "wide")]),
            pick: Some(HistoryContinuation::new("f1", "Widen", "wide")),
            chosen: None,
            run_pages: vec![HistoryPathPage::new(vec![entry("f1", "Widen stereo", 0)])],
            inner: Vec::new(),
        };
        let node = history_center(
            &open_spec(),
            &theme(),
            &view_for(&pages, &[level]),
            &HistoryCenterHandlers::default(),
        );
        assert_eq!(
            find(&node, &history_center_row_id("f1")).and_then(|row| row.a11y.level),
            Some(2),
        );
        assert_eq!(
            find(&node, &history_center_picker_id("e2")).and_then(|row| row.a11y.level),
            Some(2),
        );
        assert_eq!(
            find(&node, &history_center_row_id("e3")).and_then(|row| row.a11y.level),
            Some(1),
        );
    }

    /// The disclosure names its fork count and its direction, so the control
    /// says what it will do rather than only that it exists.
    #[test]
    fn the_disclosure_names_its_forks_and_its_direction() {
        let pages = spine();
        let closed = history_center(
            &open_spec(),
            &theme(),
            &view_for(&pages, &[]),
            &HistoryCenterHandlers::default(),
        );
        let disclosure =
            find(&closed, &history_center_disclosure_id("e2")).expect("e2 has two forks");
        assert_eq!(disclosure.a11y.label.as_deref(), Some("Show 2 continuations"));
        assert_eq!(disclosure.a11y.expanded, Some(false));

        // A terminal entry has no forks, so it renders no disclosure at all —
        // never a disabled stand-in for "nothing to disclose".
        assert!(find(&closed, &history_center_disclosure_id("e3")).is_none());
    }

    /// The list is a bounded scroll region rather than a column that grows
    /// with the history.
    #[test]
    fn the_list_is_bounded_and_scrolls() {
        let pages = spine();
        let node = history_center(
            &open_spec(),
            &theme(),
            &view_for(&pages, &[]),
            &HistoryCenterHandlers::default(),
        );
        let list = find(&node, HISTORY_CENTER_LIST_ID).expect("the list renders");
        assert_eq!(list.a11y.role, Some(NodeRole::List));
        assert_eq!(
            list.style.descriptor.layout.overflow_y,
            LayoutOverflow::Scroll
        );
        assert_eq!(list.style.max_height, Some(rem_to_px(LIST_MAX_HEIGHT_REM)));
    }

    /// Absence is the signal: no rows means the empty state, not an empty list
    /// region pretending to be a history.
    #[test]
    fn no_rows_renders_the_empty_state_and_no_list() {
        let node = history_center(
            &open_spec(),
            &theme(),
            &open_view(),
            &HistoryCenterHandlers::default(),
        );
        assert!(find(&node, HISTORY_CENTER_EMPTY_ID).is_some());
        assert!(find(&node, HISTORY_CENTER_LIST_ID).is_none());
    }

    /// Undo and redo are inert while an authority operation runs — disabled,
    /// not merely dimmed.
    #[test]
    fn busy_makes_both_triggers_inert() {
        let spec = open_spec().with_can_undo(true).with_can_redo(true).with_busy(true);
        let node = history_center(
            &spec,
            &theme(),
            &open_view(),
            &HistoryCenterHandlers::default(),
        );
        assert!(
            find(&node, HISTORY_CENTER_UNDO_ID)
                .expect("undo renders")
                .interaction
                .disabled
        );
        assert!(
            find(&node, HISTORY_CENTER_REDO_ID)
                .expect("redo renders")
                .interaction
                .disabled
        );
    }

    /// One fork leaves nothing to choose between, so the select alone is
    /// disabled — the actions menu still reaches the auto-chosen fork.
    #[test]
    fn a_single_fork_disables_the_select_but_not_its_actions() {
        let pages = vec![HistoryPathPage::new(vec![
            entry("s2", "Trim tail", 2),
            entry("s1", "Import stems", 1),
        ])];
        let level = HistoryCenterOpenFork {
            anchor_entry_id: "s2".to_owned(),
            continuations: Some(vec![HistoryContinuation::new("g1", "Only", "only")]),
            pick: None,
            chosen: Some(HistoryContinuation::new("g1", "Only", "only")),
            run_pages: vec![HistoryPathPage::new(vec![entry("g1", "Only fork", 0)])],
            inner: Vec::new(),
        };
        let node = history_center(
            &open_spec(),
            &theme(),
            &view_for(&pages, &[level]),
            &HistoryCenterHandlers::default(),
        );
        let select = find(&node, &history_center_picker_select_id("s2")).expect("select renders");
        assert!(select.interaction.disabled);
        let actions =
            find(&node, &history_center_picker_actions_id("s2")).expect("actions render");
        assert!(!actions.interaction.disabled);
    }

    /// The rename input takes the select's place while a rename is open, and
    /// the select is gone rather than merely hidden behind it.
    #[test]
    fn an_open_rename_replaces_the_select() {
        let pages = spine();
        let level = HistoryCenterOpenFork {
            anchor_entry_id: "e2".to_owned(),
            continuations: Some(vec![HistoryContinuation::new("f1", "Widen", "wide")]),
            pick: Some(HistoryContinuation::new("f1", "Widen", "wide")),
            chosen: None,
            run_pages: Vec::new(),
            inner: Vec::new(),
        };
        let mut view = view_for(&pages, &[level]);
        view.rename = Some(HistoryCenterRename {
            anchor_entry_id: "e2".to_owned(),
            branch_id: "wide".to_owned(),
            value: "Wide mix".to_owned(),
        });
        let node = history_center(
            &open_spec(),
            &theme(),
            &view,
            &HistoryCenterHandlers::default(),
        );
        let input =
            find(&node, &history_center_rename_input_id("e2")).expect("the rename input renders");
        assert_eq!(input.a11y.role, Some(NodeRole::TextInput));
        assert!(find(&node, &history_center_picker_select_id("e2")).is_none());
    }

    /// Token roles project onto the composition root, as the web root's data
    /// attributes do.
    #[test]
    fn token_roles_project_onto_the_root() {
        let spec = open_spec().with_status(HistoryCenterStatus::Loading);
        let node = history_center(
            &spec,
            &theme(),
            &open_view(),
            &HistoryCenterHandlers::default(),
        );
        assert_eq!(node.roles.get("status").map(String::as_str), Some("loading"));
        assert_eq!(
            node.roles.get("placement").map(String::as_str),
            Some("bottom-end"),
        );
    }

    /// A closed centre renders its cluster and no surface: the trigger says so
    /// through `expanded`, and nothing of the history is in the tree.
    #[test]
    fn a_closed_centre_has_a_trigger_and_no_surface() {
        let node = history_center(
            &HistoryCenterSpec::new(),
            &theme(),
            &HistoryCenterView::default(),
            &HistoryCenterHandlers::default(),
        );
        let trigger = find(&node, HISTORY_CENTER_LIST_TRIGGER_ID).expect("the trigger renders");
        assert_eq!(trigger.a11y.expanded, Some(false));
        assert!(find(&node, HISTORY_CENTER_SURFACE_ID).is_none());
    }

    /// g14.007 retained regression. `icon_button` carries no focus patch, and
    /// the GPUI backend creates a focus handle only for a focusable node that
    /// has one — so undo, redo and the picker's actions trigger were
    /// unreachable by keyboard and unfocusable by the backend. The composition
    /// stamps its own ring. The `icon_button` gap itself is tracked in
    /// PAPERCUTS.
    #[test]
    fn every_focusable_control_carries_the_ring_the_backend_keys_handles_on() {
        let pages = spine();
        let node = history_center(
            &open_spec(),
            &theme(),
            &view_for(&pages, &[]),
            &HistoryCenterHandlers::default(),
        );

        for id in [
            HISTORY_CENTER_UNDO_ID,
            HISTORY_CENTER_REDO_ID,
            HISTORY_CENTER_LIST_TRIGGER_ID,
        ] {
            let control = find(&node, id).unwrap_or_else(|| panic!("{id} renders"));
            assert!(
                control.style.focus.is_some(),
                "{id} has no focus ring, so the backend never creates a handle for it",
            );
        }

        let entry = find(&node, &history_center_entry_id("e2")).expect("entry button renders");
        assert!(entry.interaction.focusable);
        assert!(entry.style.focus.is_some());
    }

    /// g14.007 retained regression. Roving focus moved the tab stop without
    /// moving backend focus: the tab stop has to follow the machine's
    /// `focus_row`, not the list order, or arrow navigation is invisible to
    /// the platform.
    #[test]
    fn the_tab_stop_follows_the_machines_roving_focus() {
        let pages = spine();
        let mut view = view_for(&pages, &[]);
        view.focus_row = Some(HistoryCenterRowId::new(
            HistoryCenterRowKind::Entry,
            "e2".to_owned(),
        ));

        let node = history_center(
            &open_spec(),
            &theme(),
            &view,
            &HistoryCenterHandlers::default(),
        );

        let tab_index_of = |entry_id: &str| {
            find(&node, &history_center_entry_id(entry_id))
                .unwrap_or_else(|| panic!("entry {entry_id} renders"))
                .a11y
                .tab_index
        };
        assert_eq!(tab_index_of("e2"), Some(0));
        assert_eq!(tab_index_of("e1"), Some(-1));
        assert_eq!(tab_index_of("e3"), Some(-1));
    }

    /// g14.007 retained regression: the open rename is a real editing surface.
    /// Keystrokes reach the host's buffer through the same path any text field
    /// uses — a key *name* is not content ("space" is one space, not the word)
    /// and Shift is what makes a capital letter. Dropping either committed
    /// "wide mix v2" or "Widespacemixspacev2" for "Wide mix v2".
    #[test]
    fn the_rename_input_reports_keys_as_content_with_shift_and_space_intact() {
        use std::sync::Mutex;

        let typed: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&typed);
        let pages = spine();
        let level = HistoryCenterOpenFork {
            anchor_entry_id: "e2".to_owned(),
            continuations: Some(vec![HistoryContinuation::new("f1", "Widen", "wide")]),
            pick: Some(HistoryContinuation::new("f1", "Widen", "wide")),
            chosen: None,
            run_pages: Vec::new(),
            inner: Vec::new(),
        };
        let mut view = view_for(&pages, &[level]);
        view.rename = Some(HistoryCenterRename {
            anchor_entry_id: "e2".to_owned(),
            branch_id: "wide".to_owned(),
            value: "Wide".to_owned(),
        });

        let node = history_center(
            &open_spec(),
            &theme(),
            &view,
            &HistoryCenterHandlers {
                on_rename_key: Some(Arc::new(move |key| {
                    sink.lock().expect("typed").push(key.to_owned())
                })),
                ..HistoryCenterHandlers::default()
            },
        );

        let input = find(&node, &history_center_rename_input_id("e2")).expect("rename input");
        assert!(input.interaction.focusable);
        let on_key = input.interaction.on_edit_key.clone().expect("editing path");

        on_key("space", poodle_node::NodeModifiers::default());
        on_key(
            "m",
            poodle_node::NodeModifiers {
                shift: true,
                ..poodle_node::NodeModifiers::default()
            },
        );
        on_key("i", poodle_node::NodeModifiers::default());

        assert_eq!(
            typed.lock().expect("typed").as_slice(),
            [" ".to_owned(), "M".to_owned(), "i".to_owned()],
        );
    }
}
