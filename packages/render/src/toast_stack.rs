//! ToastStack — toast notification stack.
//!
//! Contract: `docs/contracts/components/toast-stack.md`
//! Ported from: `packages/jetstream/components/src/toast_stack.rs`.
//!
//! Each toast: leading tone accent bar, title + optional message, optional
//! Button action, Icon-backed dismiss button, tone-tinted gradient fill, and
//! elevation-overlay shadow. Authored rows paint at the settled endpoint.

use std::sync::Arc;

use poodle_headless::motion_policy::{
    motion_key, MotionPolicy, MOTION_DURATION_STANDARD_MS, MOTION_ROLE_TOAST_ENTER,
    MOTION_ROLE_TOAST_EXIT,
};
use poodle_node::{
    AnimEasing, AnimKeyframe, AnimLoop, AnimProperty, CrossAxisAlignment, CursorHint, FocusRing,
    LayoutDirection, LayoutSizing, Node, NodeAnimation, NodePosition, NodeRole, StylePatch,
};
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, IconSpec, Toast, ToastPosition,
    ToastStackSpec, ToastTone,
};

use crate::button::button;
use crate::color::{mix_srgb, with_alpha, TRANSPARENT, WHITE};
use crate::context::RenderContext;
use crate::icon::icon;
use crate::presentation::rem_to_px;

/// Host callbacks: dismiss and action, each carrying the toast's id.
#[derive(Default)]
pub struct ToastStackHandlers {
    pub on_dismiss: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Stable native instance scope. Toast ids are queue-local, so duplicate
    /// hosts may legitimately render the same id without sharing backend
    /// focus, hit-test, or element state.
    pub instance_id: Option<String>,
}

/// The animation phase of one toast row (contract §8a).
///
/// Renderer-owned: the renderer decides the phase from the semantic item list
/// and the effective motion policy. The backend drives the phase's visual
/// clock; it never decides what the phase is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToastVisualPhase {
    /// A newly joined item: it owns live-region and accessibility semantics at
    /// once, and may add a bounded enter treatment.
    Enter,
    /// The settled endpoint: no enter clock runs.
    Settled,
    /// The item left the semantic list; the row stays only as inert paint
    /// until its leave treatment finishes or the host drops it.
    Exit,
}

impl ToastVisualPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enter => "enter",
            Self::Settled => "settled",
            Self::Exit => "exit",
        }
    }
}

/// One row's presence phase, keyed by the semantic item id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastVisual {
    pub id: String,
    pub phase: ToastVisualPhase,
}

/// The framework-free presence law (contract §8a), mirroring
/// `packages/core/src/dom/motion-runtime.ts` so Svelte, React, and native
/// share one decision.
///
/// Authored initial items are settled. A new id enters; a kept id keeps its
/// phase; a removed id leaves an exit remnant at the tail. Reusing a key
/// before its remnant is dropped retargets that row back to enter rather than
/// queueing a second leave.
pub fn next_toast_visuals(
    previous: &[ToastVisual],
    live_ids: &[String],
    initial: bool,
) -> Vec<ToastVisual> {
    if initial {
        return live_ids
            .iter()
            .map(|id| ToastVisual {
                id: id.clone(),
                phase: ToastVisualPhase::Settled,
            })
            .collect();
    }
    let live: std::collections::HashSet<&str> = live_ids.iter().map(String::as_str).collect();
    let mut next: Vec<ToastVisual> = live_ids
        .iter()
        .map(|id| {
            let phase = match previous.iter().find(|visual| &visual.id == id) {
                Some(prior) if prior.phase != ToastVisualPhase::Exit => prior.phase,
                _ => ToastVisualPhase::Enter,
            };
            ToastVisual {
                id: id.clone(),
                phase,
            }
        })
        .collect();
    for prior in previous {
        if !live.contains(prior.id.as_str()) {
            next.push(ToastVisual {
                id: prior.id.clone(),
                phase: ToastVisualPhase::Exit,
            });
        }
    }
    next
}

/// An action affordance that disappeared during one reconcile. The renderer
/// resolves focus for each before its control unmounts (contract §6).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemovedToastAction {
    /// The row id whose action affordance left.
    pub id: String,
    /// Whether the row survives. A surviving row's dismiss control is the
    /// first transfer stop; a removed row's focus falls through the surviving
    /// order to the entered-from control.
    pub row_survives: bool,
}

/// What one reconcile changed, including the follow-up effects presence does
/// not itself carry.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ToastPresenceChange {
    /// The visual list or its phases changed.
    pub visuals_changed: bool,
    /// Action affordances that disappeared this reconcile.
    pub removed_actions: Vec<RemovedToastAction>,
}

/// One stack's renderer-owned presence ledger.
///
/// The host owns one per stack and threads it through
/// [`toast_stack_with_presence`]; the renderer reconciles phases, retains the
/// removed item's copy for the inert remnant, and settles or drops a row on
/// the host's completion report (the browser's `onComplete`). The host still
/// owns the item list and timers: presence never owns expiry or dismissal.
#[derive(Clone, Debug, Default)]
pub struct ToastStackPresence {
    visuals: Vec<ToastVisual>,
    retained: Vec<Toast>,
    /// Ids whose last reconcile still carried an action affordance. A
    /// disappearing id is detected against this set once, not against the
    /// retained copy every frame (a remnant would re-trigger forever).
    action_ids: Vec<String>,
    initialised: bool,
}

impl ToastStackPresence {
    pub fn new() -> Self {
        Self::default()
    }

    /// Every row in render order, including exit remnants at the tail.
    pub fn visuals(&self) -> &[ToastVisual] {
        &self.visuals
    }

    pub fn phase(&self, id: &str) -> Option<ToastVisualPhase> {
        self.visuals
            .iter()
            .find(|visual| visual.id == id)
            .map(|visual| visual.phase)
    }

    /// The item copy backing a row, live or remnant.
    pub fn retained(&self, id: &str) -> Option<&Toast> {
        self.retained.iter().find(|toast| toast.id == id)
    }

    pub fn has_exiting(&self) -> bool {
        self.visuals
            .iter()
            .any(|visual| visual.phase == ToastVisualPhase::Exit)
    }

    /// Reconcile the ledger against the live items under the effective motion
    /// policy. Initial items settle; new ids enter; kept ids keep their phase;
    /// removed ids leave an exit remnant. `frozen` has no clock: new rows
    /// settle and removed rows drop immediately (contract §8a). The returned
    /// change also names every action affordance that disappeared, so the
    /// renderer can hand focus on before the control unmounts (contract §6).
    pub fn reconcile(&mut self, toasts: &[Toast], policy: MotionPolicy) -> ToastPresenceChange {
        let live_ids: Vec<String> = toasts.iter().map(|toast| toast.id.clone()).collect();
        let previous = self.visuals.clone();
        let mut visuals = next_toast_visuals(&self.visuals, &live_ids, !self.initialised);
        self.initialised = true;
        if policy == MotionPolicy::Frozen {
            visuals.retain(|visual| visual.phase != ToastVisualPhase::Exit);
            for visual in &mut visuals {
                if visual.phase == ToastVisualPhase::Enter {
                    visual.phase = ToastVisualPhase::Settled;
                }
            }
        }
        // Contract §6: an action affordance that disappears must hand focus
        // on before its control unmounts. Compare against the last
        // reconcile's action set so a remnant never re-triggers the move.
        let mut removed_actions = Vec::new();
        for id in &self.action_ids {
            match toasts.iter().find(|toast| &toast.id == id) {
                Some(toast) if toast.action_label.is_none() => {
                    removed_actions.push(RemovedToastAction {
                        id: id.clone(),
                        row_survives: true,
                    });
                }
                Some(_) => {}
                None => removed_actions.push(RemovedToastAction {
                    id: id.clone(),
                    row_survives: false,
                }),
            }
        }
        self.action_ids = toasts
            .iter()
            .filter(|toast| toast.action_label.is_some())
            .map(|toast| toast.id.clone())
            .collect();
        // Live copy replaces the retained copy; a remnant keeps the copy its
        // row was last rendered with, exactly like the web's retained map.
        for toast in toasts {
            match self
                .retained
                .iter_mut()
                .find(|retained| retained.id == toast.id)
            {
                Some(retained) => *retained = toast.clone(),
                None => self.retained.push(toast.clone()),
            }
        }
        self.retained
            .retain(|retained| visuals.iter().any(|visual| visual.id == retained.id));
        let visuals_changed = visuals != previous;
        self.visuals = visuals;
        ToastPresenceChange {
            visuals_changed,
            removed_actions,
        }
    }

    /// Settle an entering row once its enter treatment finishes. Returns
    /// whether the phase moved.
    pub fn settle(&mut self, id: &str) -> bool {
        match self.visuals.iter_mut().find(|visual| visual.id == id) {
            Some(visual) if visual.phase == ToastVisualPhase::Enter => {
                visual.phase = ToastVisualPhase::Settled;
                true
            }
            _ => false,
        }
    }

    /// Drop an exit remnant (after its leave treatment) or a stale row.
    /// Returns whether anything was removed.
    pub fn drop_visual(&mut self, id: &str) -> bool {
        let before = self.visuals.len();
        self.visuals.retain(|visual| visual.id != id);
        self.retained.retain(|toast| toast.id != id);
        self.visuals.len() != before
    }
}

fn scoped(instance_id: Option<&str>, part: &str) -> Option<String> {
    instance_id.map(|scope| format!("toast-host:{scope}:{part}"))
}

fn position_role(position: ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopRight => "top-right",
        ToastPosition::TopLeft => "top-left",
        ToastPosition::BottomRight => "bottom-right",
        ToastPosition::BottomLeft => "bottom-left",
    }
}

/// Per-size title font-size in rem (contract §8 size table).
fn title_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.71875,
        ControlSize::Sm | ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Per-size message font-size in rem (contract §8 size table).
fn message_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Per-size dismiss square dimension in rem (contract §8 size table).
fn dismiss_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.0,
        ControlSize::Sm => 1.125,
        ControlSize::Md => 1.25,
        ControlSize::Lg => 1.5,
        ControlSize::Xl => 1.75,
    }
}

/// Per-size dismiss top/right inset in rem (contract §8 size table).
fn dismiss_inset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm | ControlSize::Md => 0.375,
        ControlSize::Lg | ControlSize::Xl => 0.5,
    }
}

/// Density toast-padding multiplier (contract §8 density table).
fn density_pad_scale(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

/// The backend element id of a toast's dismiss control: the caller-scoped
/// runtime id when set, else the semantic id. Mirrors the conversion's
/// identity so transfer requests name handles that exist.
fn dismiss_element_id(instance_id: Option<&str>, toast_id: &str) -> String {
    scoped(instance_id, &format!("toast:{toast_id}:dismiss"))
        .unwrap_or_else(|| format!("poodle-toast-dismiss-{toast_id}"))
}

/// Every element id one toast's row can own: the row, dismiss, and action
/// ids, plain and caller-scoped. The transfer tests focus ownership and the
/// entered-from boundary against exactly this set.
fn toast_element_ids(instance_id: Option<&str>, toast_id: &str) -> Vec<String> {
    let mut ids = vec![
        format!("poodle-toast-{toast_id}"),
        format!("poodle-toast-dismiss-{toast_id}"),
        format!("poodle-toast-action-{toast_id}"),
    ];
    for part in ["dismiss", "action"] {
        if let Some(scoped) = scoped(instance_id, &format!("toast:{toast_id}:{part}")) {
            ids.push(scoped);
        }
    }
    if let Some(scoped) = scoped(instance_id, &format!("toast:{toast_id}")) {
        ids.push(scoped);
    }
    ids
}

/// Whether the element id names a part of one toast's row. The transfer uses
/// it to test focus ownership and to keep the entered-from control outside
/// this stack.
fn toast_owns_element_id(instance_id: Option<&str>, toast_id: &str, element_id: &str) -> bool {
    toast_element_ids(instance_id, toast_id)
        .iter()
        .any(|id| id == element_id)
}

/// Every element id a toast's action affordance can own, plain and scoped.
fn action_element_ids(instance_id: Option<&str>, toast_id: &str) -> Vec<String> {
    let mut ids = vec![format!("poodle-toast-action-{toast_id}")];
    if let Some(scoped) = scoped(instance_id, &format!("toast:{toast_id}:action")) {
        ids.push(scoped);
    }
    ids
}

/// Whether the focused element is this row's action affordance. Contract §6:
/// only an action that still owns focus moves focus when it leaves; once
/// focus has moved on, the removal is silent.
pub fn toast_action_owns_focus(instance_id: Option<&str>, toast_id: &str) -> bool {
    let Some(focused) = poodle_node::current_focused_id() else {
        return false;
    };
    action_element_ids(instance_id, toast_id)
        .iter()
        .any(|id| id == &focused)
}

/// Contract §6 action-removal transfer target as a backend element id: the
/// same row's dismiss control while the row survives, else the surviving
/// order's next row, then previous row, then the entered-from control. Pure
/// over the rendered order so every backend shares one decision; the backend
/// drops targets with no mounted handle.
pub fn toast_action_removal_focus_target(
    order: &[String],
    instance_id: Option<&str>,
    row_id: &str,
    row_survives: bool,
    entered_from: Option<&str>,
) -> Option<String> {
    if row_survives {
        return Some(dismiss_element_id(instance_id, row_id));
    }
    toast_dismiss_focus_target(order, instance_id, row_id, entered_from)
}

/// Contract §8a transfer target for a dismissed toast, as a backend element
/// id: the dismiss control on the next surviving toast, else the previous
/// surviving toast's, else the entered-from control when it names nothing in
/// this stack's own parts. Pure over the rendered order so every backend
/// shares one decision; the backend drops targets with no mounted handle.
pub fn toast_dismiss_focus_target(
    toast_ids: &[String],
    instance_id: Option<&str>,
    removed_id: &str,
    entered_from: Option<&str>,
) -> Option<String> {
    let position = toast_ids.iter().position(|id| id == removed_id)?;
    let next = toast_ids
        .iter()
        .skip(position + 1)
        .find(|id| id.as_str() != removed_id);
    let previous = toast_ids[..position]
        .iter()
        .rev()
        .find(|id| id.as_str() != removed_id);
    if let Some(survivor) = next.or(previous) {
        return Some(dismiss_element_id(instance_id, survivor));
    }
    match entered_from {
        Some(from)
            if !toast_ids
                .iter()
                .any(|id| toast_owns_element_id(instance_id, id, from)) =>
        {
            Some(from.to_owned())
        }
        _ => None,
    }
}
fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// The bounded enter/exit treatment for one row (contract §8a): opacity plus
/// the web's 0.5rem translation in `full`, opacity only under `reduced`, and
/// nothing under `frozen` (the endpoint paints immediately). Filtering goes
/// through the shared motion policy, so the declaration names the properties
/// the backend will actually run.
fn toast_presence_animation(
    instance_id: Option<&str>,
    toast_id: &str,
    phase: ToastVisualPhase,
    policy: MotionPolicy,
) -> Option<NodeAnimation> {
    let (role, opacity, translate) = match phase {
        ToastVisualPhase::Enter => (
            MOTION_ROLE_TOAST_ENTER,
            (0.0_f32, 1.0_f32),
            (rem_to_px(0.5), 0.0),
        ),
        ToastVisualPhase::Exit => (
            MOTION_ROLE_TOAST_EXIT,
            (1.0_f32, 0.0_f32),
            (0.0, rem_to_px(0.5)),
        ),
        ToastVisualPhase::Settled => return None,
    };
    let owner = scoped(instance_id, &format!("toast:{toast_id}"))
        .unwrap_or_else(|| format!("poodle-toast-{toast_id}"));
    let animation = NodeAnimation {
        key: motion_key(&owner, role, "item"),
        keyframes: vec![
            AnimKeyframe {
                at: 0.0,
                values: vec![
                    (AnimProperty::Opacity, opacity.0),
                    (AnimProperty::TranslateY, translate.0),
                ],
            },
            AnimKeyframe {
                at: 1.0,
                values: vec![
                    (AnimProperty::Opacity, opacity.1),
                    (AnimProperty::TranslateY, translate.1),
                ],
            },
        ],
        duration_secs: MOTION_DURATION_STANDARD_MS as f32 / 1000.0,
        easing: AnimEasing::EaseOut,
        loop_mode: AnimLoop::Once,
    };
    crate::motion::animation_for_policy(policy, animation, true)
}

/// Strip a control to inert paint: no focus, no traversal stop, no cursor,
/// no hover/active restyle, no activation, and no semantic `id` (the runtime
/// scope stays for observation). An exit remnant is unreachable.
fn inert_control(node: &mut Node) {
    node.id = None;
    node.interaction.focusable = false;
    node.interaction.on_activate = None;
    node.interaction.on_activate_modified = None;
    node.interaction.on_key = None;
    node.interaction.on_key_activate = None;
    node.a11y.tab_index = Some(-1);
    node.style.focus_ring = None;
    node.style.hover = None;
    node.style.active = None;
    node.style.descriptor.cursor = CursorHint::Default;
}

/// Render a toast stack from its semantic items. This is the authored /
/// preloaded form: every row paints settled with no enter clock, and a removed
/// row unmounts immediately. A host that draws presence holds a
/// [`ToastStackPresence`] and calls [`toast_stack_with_presence`].
pub fn toast_stack(
    spec: &ToastStackSpec,
    ctx: &RenderContext<'_>,
    handlers: ToastStackHandlers,
) -> Node {
    let mut presence = ToastStackPresence::new();
    toast_stack_with_presence(spec, ctx, &mut presence, handlers)
}

/// Render a toast stack under a host-held presence ledger. The renderer owns
/// the decision: it reconciles the ledger against `spec.toasts` under the
/// context's effective motion policy, composes every row at its phase — enter,
/// settled, or the inert exit remnant — and leaves settling an enter and
/// dropping a remnant to the host's completion report.
pub fn toast_stack_with_presence(
    spec: &ToastStackSpec,
    ctx: &RenderContext<'_>,
    presence: &mut ToastStackPresence,
    handlers: ToastStackHandlers,
) -> Node {
    let change = presence.reconcile(&spec.toasts, ctx.motion_policy());
    let policy = ctx.motion_policy();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let title_px = rem_to_px(title_font_rem(effective_size));
    let message_px = rem_to_px(message_font_rem(effective_size));
    let dismiss_px = rem_to_px(dismiss_size_rem(effective_size));
    let dismiss_inset = rem_to_px(dismiss_inset_rem(effective_size));
    let instance_id = handlers.instance_id.as_deref();

    // Contract §8 toast padding = space-panel-x scaled by density.
    let base_pad = ctx.theme().resolve_space(spec.padding_token());
    let pad = base_pad * density_pad_scale(density);
    // Contract §8: comfortable widens the stack gap to space-stack-lg;
    // compact and default retain space-stack-sm.
    let stack_gap = match density {
        ControlDensity::Comfortable => ctx.theme().resolve_space("space.stack.lg"),
        ControlDensity::Compact | ControlDensity::Default => {
            ctx.theme().resolve_space(spec.gap_token())
        }
    };
    let item_gap = ctx.theme().resolve_space(spec.gap_token());
    let dismiss_reserve = rem_to_px(match density {
        ControlDensity::Compact => 1.25,
        ControlDensity::Default => 1.5,
        ControlDensity::Comfortable => 1.75,
    });

    let elevated = ctx.theme().resolve_color(spec.fill_token());
    let border_default = ctx.theme().resolve_color(spec.border_token());
    let radius_base = ctx.theme().resolve_radius(spec.radius_token());
    // Contract §8: border-radius = calc(radius-surface - 0.125rem).
    let radius = (radius_base - rem_to_px(0.125)).max(0.0);
    let title_color = ctx.theme().resolve_color(spec.title_color_token());
    let message_color = ctx.theme().resolve_color(spec.message_color_token());
    let dismiss_color = ctx.theme().resolve_color(spec.dismiss_color_token());
    let dismiss_hover_color = ctx.theme().resolve_color(spec.title_color_token());
    let dismiss_hover_fill =
        with_alpha(ctx.theme().resolve_color("color.background.surface"), 0.60);

    let mut el = Node::container();
    el.runtime_id = scoped(instance_id, "stack");
    el.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    el.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    el.roles.insert(
        "position".to_owned(),
        position_role(spec.position).to_owned(),
    );
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = stack_gap;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(22.5));
    }
    // Corner-mounted overlay: keep the stack out of flow and anchor it to
    // the nearest relative host, matching the old GPUI wrapper.
    el.position = match spec.position {
        ToastPosition::TopRight => NodePosition::Absolute {
            top: Some(pad),
            right: Some(pad),
            left: None,
            bottom: None,
        },
        ToastPosition::TopLeft => NodePosition::Absolute {
            top: Some(pad),
            left: Some(pad),
            right: None,
            bottom: None,
        },
        ToastPosition::BottomRight => NodePosition::Absolute {
            bottom: Some(pad),
            right: Some(pad),
            top: None,
            left: None,
        },
        ToastPosition::BottomLeft => NodePosition::Absolute {
            bottom: Some(pad),
            left: Some(pad),
            top: None,
            right: None,
        },
    };

    let live_order: Vec<String> = spec.toasts.iter().map(|toast| toast.id.clone()).collect();
    // Contract §6/§8a: a removed action hands focus on before its control
    // unmounts — the row's dismiss while the row survives, else the surviving
    // order's next, then previous, then the entered-from control. Only an
    // action that still owns focus moves; a removal after focus has left is
    // silent. The backend drops targets with no mounted handle.
    for removed in &change.removed_actions {
        if !toast_action_owns_focus(instance_id, &removed.id) {
            continue;
        }
        if let Some(target) = toast_action_removal_focus_target(
            &live_order,
            instance_id,
            &removed.id,
            removed.row_survives,
            poodle_node::previous_focused_id().as_deref(),
        ) {
            poodle_node::queue_focus_request(&target);
        }
    }
    let visuals = presence.visuals().to_vec();
    for visual in &visuals {
        let Some(toast) = presence.retained(&visual.id).cloned() else {
            continue;
        };
        let exiting = visual.phase == ToastVisualPhase::Exit;
        let tone_color = ctx.theme().resolve_color(spec.tone_color(&toast.tone));

        // Contract §8 tone treatments:
        //   accent bar = color-mix(tone 94%, white)
        //   border     = color-mix(tone 34%, border-default)
        //   background = color-mix(tone 12%, elevated) tint
        let accent_bar_color = mix_srgb(tone_color, WHITE, 0.94);
        let toast_border = mix_srgb(tone_color, border_default, 0.34);
        let bg_tinted = mix_srgb(tone_color, elevated, 0.12);

        // Leading tone accent bar — contract §8: 0.1875rem (3px), full height.
        let mut accent_bar = Node::container();
        accent_bar.runtime_id = scoped(instance_id, &format!("toast:{}:accent", toast.id));
        accent_bar.position = NodePosition::Absolute {
            top: Some(0.0),
            right: None,
            bottom: Some(0.0),
            left: Some(0.0),
        };
        {
            let s = &mut accent_bar.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(0.1875));
            s.descriptor.background = Some(accent_bar_color);
        }

        // Title + optional message column.
        let mut content = Node::container();
        {
            let s = &mut content.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.25);
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
        let mut title = Node::text(toast.title.as_str());
        title.style.descriptor.text_color = Some(title_color);
        title.style.text_size = Some(title_px);
        title.style.text_weight = Some(600);
        let mut content = content.child(title);

        if let Some(message) = &toast.message {
            let mut msg = Node::text(message.as_str());
            msg.style.descriptor.text_color = Some(message_color);
            msg.style.text_size = Some(message_px);
            content = content.child(msg);
        }
        // Optional action affordance — the contract-owned Button primitive.
        if let Some(action) = &toast.action_label {
            let on_click = if exiting {
                None
            } else {
                handlers.on_action.as_ref().map(|handler| {
                    let handler = Arc::clone(handler);
                    let id = toast.id.clone();
                    Arc::new(move || handler(&id)) as Arc<dyn Fn() + Send + Sync>
                })
            };
            let mut action_button = button(
                &ButtonSpec::new()
                    .with_label(action.as_str())
                    .with_variant(ButtonVariant::Secondary)
                    .with_size(effective_size)
                    .with_density(density),
                ctx,
                on_click,
            );
            action_button.id = Some(format!("poodle-toast-action-{}", toast.id));
            action_button.runtime_id = scoped(instance_id, &format!("toast:{}:action", toast.id));
            action_button
                .roles
                .insert("dependency".to_owned(), "button".to_owned());
            if exiting {
                inert_control(&mut action_button);
            }

            let mut actions = Node::container();
            actions.runtime_id = scoped(instance_id, &format!("toast:{}:actions", toast.id));
            actions
                .roles
                .insert("part".to_owned(), "actions".to_owned());
            actions.style.descriptor.layout.spacing.margin.top = rem_to_px(0.25);
            content = content.child(actions.child(action_button));
        }

        // Dismiss affordance — a native button containing the real Icon
        // primitive. It stays a focus stop without a handler, matching a web
        // button whose event has no listener while keeping activation inert.
        let dismiss_aria = format!("Dismiss {}", toast.title);
        let mut dismiss_icon = icon(&IconSpec::new("x"), ctx);
        dismiss_icon.runtime_id = scoped(instance_id, &format!("toast:{}:dismiss-icon", toast.id));
        dismiss_icon.style.descriptor.text_color = Some(dismiss_color);
        dismiss_icon
            .roles
            .insert("dependency".to_owned(), "icon".to_owned());

        let mut dismiss = Node::button("");
        dismiss.id = Some(format!("poodle-toast-dismiss-{}", toast.id));
        dismiss.runtime_id = scoped(instance_id, &format!("toast:{}:dismiss", toast.id));
        dismiss.position = NodePosition::Absolute {
            top: Some(dismiss_inset),
            right: Some(dismiss_inset),
            left: None,
            bottom: None,
        };
        dismiss.a11y.role = Some(NodeRole::Button);
        dismiss.a11y.label = Some(dismiss_aria);
        dismiss.a11y.tab_index = Some(0);
        dismiss.interaction.focusable = true;
        dismiss.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color("color.accent.focusRing"),
            width: ctx.theme().resolve_border_width("border.width.focus"),
            offset: rem_to_px(0.125),
        });
        {
            let s = &mut dismiss.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(dismiss_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(dismiss_px);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = poodle_node::MainAxisAlignment::Center;
            s.descriptor.background = Some(TRANSPARENT);
            s.descriptor.border.width = 0.0;
            s.descriptor.cursor = CursorHint::Pointer;
            s.hover = Some(StylePatch {
                background: Some(dismiss_hover_fill),
                border_color: None,
                text_color: Some(dismiss_hover_color),
                opacity: None,
            });
        }
        all_corners(&mut dismiss, ctx.theme().resolve_radius("radius.sm"));
        if exiting {
            inert_control(&mut dismiss);
        } else if let Some(handler) = &handlers.on_dismiss {
            let handler = Arc::clone(handler);
            let id = toast.id.clone();
            // Contract §8a, renderer-owned: the component moves focus, the
            // host only removes the row. The dismiss control's own activation
            // carries the transfer: next surviving row's dismiss, else the
            // previous row's, else the still-connected entered-from control
            // (the backend drops targets with no mounted handle). Only when
            // the dismissed row owns focus — activation arrives through its
            // own control, and anything else keeps its focus. Known edge:
            // chained removals without re-entry can leave a dead inside id
            // as the transit source; the backend drop then clears focus,
            // which the contract reads as no connected entry existing.
            let order = live_order.clone();
            let instance = instance_id.map(str::to_owned);
            dismiss.interaction.on_activate = Some(Arc::new(move || {
                let entered = poodle_node::previous_focused_id();
                let owned = poodle_node::current_focused_id()
                    .as_deref()
                    .is_some_and(|focused| {
                        toast_owns_element_id(instance.as_deref(), &id, focused)
                    });
                handler(&id);
                if !owned {
                    return;
                }
                if let Some(target) =
                    toast_dismiss_focus_target(&order, instance.as_deref(), &id, entered.as_deref())
                {
                    poodle_node::queue_focus_request(&target);
                }
            }));
        }
        let dismiss = dismiss.child(dismiss_icon);

        // Toast box: tinted fill + fade gradient, tone border,
        // elevation-overlay shadow, clipped. Each toast is a list item, and a
        // danger toast escalates to an alert — the native projection of the
        // contract's assertive live region (contract §6).
        let mut toast_el = Node::container();
        toast_el.a11y.role = Some(if toast.tone == ToastTone::Danger {
            NodeRole::Alert
        } else {
            NodeRole::ListItem
        });
        toast_el.position = NodePosition::Relative;
        toast_el.id = Some(format!("poodle-toast-{}", toast.id));
        toast_el.runtime_id = scoped(instance_id, &format!("toast:{}", toast.id));
        toast_el.roles.insert(
            "tone".to_owned(),
            format!("{:?}", toast.tone).to_ascii_lowercase(),
        );
        toast_el
            .roles
            .insert("phase".to_owned(), visual.phase.as_str().to_owned());
        toast_el.style.animation =
            toast_presence_animation(instance_id, &toast.id, visual.phase, policy);
        if exiting {
            // Contract §8a: the remnant leaves accessibility ownership at
            // once, stays as inert paint only, and drops its semantic id so no
            // activation path reaches it. Its runtime scope stays for
            // observation.
            toast_el.a11y.hidden = Some(true);
            toast_el.id = None;
        }
        {
            let s = &mut toast_el.style;
            s.descriptor.background = Some(bg_tinted);
            // Contract §8: linear gradient (90deg) of tone tint fading into
            // elevated.
            s.gradient = Some((
                90.0,
                vec![
                    (mix_srgb(tone_color, elevated, 0.12), 0.0),
                    (elevated, 0.18),
                ],
            ));
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = toast_border;
            s.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = poodle_node::LayoutOverflow::Hidden;
            // Token-accurate elevation.overlay (single layer, spread 0).
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
            let padc = &mut s.descriptor.layout.spacing.padding;
            padc.left = pad;
            padc.right = pad + dismiss_reserve;
            padc.top = pad;
            padc.bottom = pad;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = item_gap;
        }
        all_corners(&mut toast_el, radius);

        el = el.child(toast_el.child(dismiss).child(accent_bar).child(content));
    }

    // Contract §3: the stack name defaults to "Notifications", matching Svelte.
    let label = spec
        .aria_label
        .as_deref()
        .filter(|label| !label.is_empty())
        .unwrap_or("Notifications");
    el.a11y.label = Some(label.to_string());
    // Contract: the stack is a list of toasts.
    el.a11y.role = Some(NodeRole::List);
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_node::NodeKind;
    use poodle_specs::{Toast, ToastTone};
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn preloaded_items_do_not_enter() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = ToastStackSpec::new().with_toasts(vec![Toast::new("save", "Saved")]);
        let node = toast_stack(&spec, &ctx, ToastStackHandlers::default());
        let toast = node
            .find(&|n| n.id.as_deref() == Some("poodle-toast-save"))
            .expect("toast exists");
        assert!(
            toast.style.animation.is_none(),
            "authored items paint the endpoint; construction does not attach enter"
        );
    }

    #[test]
    fn danger_projects_as_alert_while_other_tones_stay_list_items() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = ToastStackSpec::new().with_toasts(vec![
            Toast::new("ok", "Saved").with_tone(poodle_specs::ToastTone::Success),
            Toast::new("fail", "Publishing failed").with_tone(poodle_specs::ToastTone::Danger),
        ]);
        let node = toast_stack(&spec, &ctx, ToastStackHandlers::default());
        let success = node
            .find(&|n| n.id.as_deref() == Some("poodle-toast-ok"))
            .expect("success toast");
        let danger = node
            .find(&|n| n.id.as_deref() == Some("poodle-toast-fail"))
            .expect("danger toast");
        assert_eq!(success.a11y.role, Some(NodeRole::ListItem));
        // Contract §6: danger escalates to the native alert projection.
        assert_eq!(danger.a11y.role, Some(NodeRole::Alert));
    }

    #[test]
    fn exact_tone_composition_focus_axes_and_spacing_match_contract() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let tones = [
            ("info", ToastTone::Info),
            ("success", ToastTone::Success),
            ("warning", ToastTone::Warning),
            ("danger", ToastTone::Danger),
        ];
        let spec = ToastStackSpec::new()
            .with_toasts(
                tones
                    .iter()
                    .map(|(id, tone)| {
                        Toast::new(*id, format!("{id} title"))
                            .with_message(format!("{id} message"))
                            .with_tone(*tone)
                            .with_action_label("Inspect")
                    })
                    .collect(),
            )
            .with_size(ControlSize::Lg)
            .with_density(ControlDensity::Comfortable);
        let node = toast_stack(
            &spec,
            &ctx,
            ToastStackHandlers {
                instance_id: Some("tokens".to_owned()),
                ..ToastStackHandlers::default()
            },
        );

        let elevated = theme.resolve_color(spec.fill_token());
        let border_default = theme.resolve_color(spec.border_token());
        let pad = theme.resolve_space(spec.padding_token()) * 1.25;
        assert_eq!(
            node.style.descriptor.layout.spacing.gap,
            theme.resolve_space("space.stack.lg")
        );

        for (id, tone) in tones {
            let row = node
                .find(&|node| {
                    node.runtime_id.as_deref()
                        == Some(format!("toast-host:tokens:toast:{id}").as_str())
                })
                .unwrap_or_else(|| panic!("{id} row"));
            let tone_color = theme.resolve_color(spec.tone_color(&tone));
            let fill = mix_srgb(tone_color, elevated, 0.12);
            assert_eq!(row.style.descriptor.background, Some(fill));
            assert_eq!(row.style.descriptor.border.width, 1.0);
            assert_eq!(
                row.style.descriptor.border.color,
                mix_srgb(tone_color, border_default, 0.34)
            );
            assert_eq!(
                row.style.gradient,
                Some((90.0, vec![(fill, 0.0), (elevated, 0.18)]))
            );
            assert_eq!(
                row.style.descriptor.shadow,
                Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY)
            );
            let spacing = row.style.descriptor.layout.spacing;
            assert_eq!(spacing.padding.left, pad);
            assert_eq!(spacing.padding.top, pad);
            assert_eq!(spacing.padding.bottom, pad);
            assert_eq!(spacing.padding.right, pad + rem_to_px(1.75));
            assert_eq!(spacing.gap, theme.resolve_space(spec.gap_token()));

            // Svelte puts the dismiss button first in the DOM; both it and
            // the accent bar are absolutely positioned, so order is semantic only.
            let accent = row.children.get(1).expect("accent bar");
            assert_eq!(
                accent.style.descriptor.background,
                Some(mix_srgb(tone_color, WHITE, 0.94))
            );
            assert_eq!(
                accent.style.descriptor.layout.width,
                LayoutSizing::Fixed(rem_to_px(0.1875))
            );
            assert!(matches!(
                accent.position,
                NodePosition::Absolute {
                    top: Some(0.0),
                    bottom: Some(0.0),
                    left: Some(0.0),
                    right: None,
                }
            ));

            let action = row
                .find(&|node| {
                    node.runtime_id.as_deref()
                        == Some(format!("toast-host:tokens:toast:{id}:action").as_str())
                })
                .expect("secondary action Button");
            assert!(matches!(action.kind, NodeKind::Button { .. }));
            assert_eq!(
                action.roles.get("dependency").map(String::as_str),
                Some("button")
            );
            assert_eq!(
                action.roles.get("variant").map(String::as_str),
                Some("secondary")
            );
            assert_eq!(action.roles.get("size").map(String::as_str), Some("lg"));
            assert_eq!(
                action.roles.get("density").map(String::as_str),
                Some("comfortable")
            );
            assert_eq!(
                action.style.descriptor.layout.height,
                LayoutSizing::Fixed(rem_to_px(2.75))
            );
            assert_eq!(
                action.style.descriptor.layout.spacing.padding.left,
                theme.resolve_space("space.control.x") + rem_to_px(0.125)
            );
            assert_eq!(
                action.style.descriptor.layout.spacing.padding.right,
                theme.resolve_space("space.control.x") + rem_to_px(0.125)
            );
            let actions = row
                .find(&|node| {
                    node.runtime_id.as_deref()
                        == Some(format!("toast-host:tokens:toast:{id}:actions").as_str())
                })
                .expect("actions wrapper");
            assert_eq!(
                actions.style.descriptor.layout.spacing.margin.top,
                rem_to_px(0.25)
            );

            let dismiss = row
                .find(&|node| {
                    node.runtime_id.as_deref()
                        == Some(format!("toast-host:tokens:toast:{id}:dismiss").as_str())
                })
                .expect("dismiss control");
            assert_eq!(
                dismiss.style.focus_ring,
                Some(FocusRing {
                    color: theme.resolve_color("color.accent.focusRing"),
                    width: theme.resolve_border_width("border.width.focus"),
                    offset: rem_to_px(0.125),
                })
            );
            assert_eq!(dismiss.style.descriptor.background, Some(TRANSPARENT));
        }
    }

    #[test]
    fn contract_components_callbacks_and_scope_stay_distinct() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let seen = Arc::new(Mutex::new(Vec::new()));
        let action_seen = Arc::clone(&seen);
        let dismiss_seen = Arc::clone(&seen);
        let node = toast_stack(
            &ToastStackSpec::new()
                .with_toasts(vec![Toast::new("job", "Publishing")
                    .with_tone(ToastTone::Warning)
                    .with_action_label("Retry")])
                .with_size(ControlSize::Lg)
                .with_density(ControlDensity::Comfortable),
            &ctx,
            ToastStackHandlers {
                on_action: Some(Arc::new(move |id| {
                    action_seen
                        .lock()
                        .expect("seen lock")
                        .push(format!("action:{id}"));
                })),
                on_dismiss: Some(Arc::new(move |id| {
                    dismiss_seen
                        .lock()
                        .expect("seen lock")
                        .push(format!("dismiss:{id}"));
                })),
                instance_id: Some("subject".to_owned()),
            },
        );

        assert_eq!(node.runtime_id.as_deref(), Some("toast-host:subject:stack"));
        assert_eq!(node.roles.get("size").map(String::as_str), Some("lg"));
        assert_eq!(
            node.roles.get("density").map(String::as_str),
            Some("comfortable")
        );

        let toast = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-job"))
            .expect("toast row");
        assert_eq!(
            toast.runtime_id.as_deref(),
            Some("toast-host:subject:toast:job")
        );
        assert_eq!(toast.roles.get("tone").map(String::as_str), Some("warning"));

        let action = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-action-job"))
            .expect("action button");
        assert!(matches!(action.kind, NodeKind::Button { .. }));
        assert_eq!(
            action.roles.get("dependency").map(String::as_str),
            Some("button")
        );
        assert_eq!(
            action.runtime_id.as_deref(),
            Some("toast-host:subject:toast:job:action")
        );

        let dismiss = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-dismiss-job"))
            .expect("dismiss button");
        assert!(matches!(dismiss.kind, NodeKind::Button { .. }));
        assert!(dismiss.interaction.focusable);
        assert_eq!(dismiss.a11y.tab_index, Some(0));
        assert_eq!(dismiss.a11y.label.as_deref(), Some("Dismiss Publishing"));
        assert_eq!(
            dismiss.runtime_id.as_deref(),
            Some("toast-host:subject:toast:job:dismiss")
        );
        let icon = dismiss
            .find(&|node| matches!(&node.kind, NodeKind::Icon { name, .. } if name == "x"))
            .expect("dismiss icon");
        assert_eq!(
            icon.roles.get("dependency").map(String::as_str),
            Some("icon")
        );

        (action
            .interaction
            .on_activate
            .as_ref()
            .expect("action handler"))();
        (dismiss
            .interaction
            .on_activate
            .as_ref()
            .expect("dismiss handler"))();
        assert_eq!(
            seen.lock().expect("seen lock").as_slice(),
            ["action:job", "dismiss:job"]
        );
    }

    #[test]
    fn unavailable_action_is_reachable_but_inert() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = toast_stack(
            &ToastStackSpec::new().with_toasts(vec![
                Toast::new("job", "Publishing").with_action_label("Unavailable")
            ]),
            &ctx,
            ToastStackHandlers::default(),
        );
        let action = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-action-job"))
            .expect("action button");
        assert!(action.interaction.focusable);
        assert_eq!(action.a11y.tab_index, Some(0));
        assert!(action.interaction.on_activate.is_none());
    }

    #[test]
    fn dismiss_transfer_prefers_next_then_previous_then_outside_entry() {
        let order = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
        assert_eq!(
            toast_dismiss_focus_target(&order, None, "a", Some("outside")),
            Some("poodle-toast-dismiss-b".to_owned())
        );
        assert_eq!(
            toast_dismiss_focus_target(&order, None, "c", Some("outside")),
            Some("poodle-toast-dismiss-b".to_owned())
        );
        assert_eq!(
            toast_dismiss_focus_target(&order, None, "b", Some("outside")),
            Some("poodle-toast-dismiss-c".to_owned()),
            "the next surviving row wins over the previous one"
        );
        assert_eq!(
            toast_dismiss_focus_target(&["only".to_owned()], None, "only", Some("outside")),
            Some("outside".to_owned())
        );
        assert_eq!(
            toast_dismiss_focus_target(
                &["only".to_owned()],
                None,
                "only",
                Some("poodle-toast-dismiss-only")
            ),
            None,
            "an entered-from control inside the stack never counts"
        );
        assert_eq!(
            toast_dismiss_focus_target(&["only".to_owned()], None, "only", None),
            None
        );
        assert_eq!(
            toast_dismiss_focus_target(&["only".to_owned()], None, "missing", Some("outside")),
            None
        );
        assert_eq!(
            toast_dismiss_focus_target(&order, Some("scope"), "b", Some("outside")),
            Some("toast-host:scope:toast:c:dismiss".to_owned()),
            "scoped stacks request the caller-scoped dismiss identity"
        );
    }

    #[test]
    fn presence_law_settles_initial_enters_new_and_remnants_removed() {
        let initial = next_toast_visuals(&[], &["a".to_owned(), "b".to_owned()], true);
        assert_eq!(
            initial,
            vec![
                ToastVisual {
                    id: "a".into(),
                    phase: ToastVisualPhase::Settled,
                },
                ToastVisual {
                    id: "b".into(),
                    phase: ToastVisualPhase::Settled,
                },
            ]
        );
        let entered = next_toast_visuals(
            &initial,
            &["a".to_owned(), "b".to_owned(), "c".to_owned()],
            false,
        );
        assert_eq!(entered[0].phase, ToastVisualPhase::Settled);
        assert_eq!(entered[2].id, "c");
        assert_eq!(entered[2].phase, ToastVisualPhase::Enter);

        let removed = next_toast_visuals(&entered, &["a".to_owned(), "c".to_owned()], false);
        assert_eq!(removed.len(), 3, "the removed row leaves a remnant");
        assert_eq!(removed[1].id, "c");
        assert_eq!(
            removed[1].phase,
            ToastVisualPhase::Enter,
            "a kept id keeps its phase"
        );
        assert_eq!(removed[2].id, "b");
        assert_eq!(
            removed[2].phase,
            ToastVisualPhase::Exit,
            "the remnant sits at the tail"
        );

        // Reusing the key before cleanup retargets the same row back to enter.
        let reused = next_toast_visuals(
            &removed,
            &["a".to_owned(), "c".to_owned(), "b".to_owned()],
            false,
        );
        assert_eq!(reused.len(), 3);
        assert_eq!(reused[2].id, "b");
        assert_eq!(reused[2].phase, ToastVisualPhase::Enter);
    }

    #[test]
    fn presence_ledger_keeps_phase_on_same_id_replacement_and_reports_completion() {
        let mut presence = ToastStackPresence::new();
        let first = vec![Toast::new("save", "Saved")];
        assert!(
            presence
                .reconcile(&first, MotionPolicy::Full)
                .visuals_changed
        );
        assert_eq!(presence.phase("save"), Some(ToastVisualPhase::Settled));

        // Same-id replacement keeps the row and its phase: no fresh enter.
        let replaced = vec![Toast::new("save", "Saved file")];
        presence.reconcile(&replaced, MotionPolicy::Full);
        assert_eq!(presence.phase("save"), Some(ToastVisualPhase::Settled));
        assert_eq!(
            presence.retained("save").map(|toast| toast.title.as_str()),
            Some("Saved file")
        );

        // A new id enters; the host settles it once the enter treatment ends.
        let grown = vec![
            Toast::new("save", "Saved file"),
            Toast::new("sync", "Syncing"),
        ];
        presence.reconcile(&grown, MotionPolicy::Full);
        assert_eq!(presence.phase("sync"), Some(ToastVisualPhase::Enter));
        assert!(presence.settle("sync"));
        assert_eq!(presence.phase("sync"), Some(ToastVisualPhase::Settled));

        // Removal keeps the last copy as an exit remnant until the host drops it.
        let shrunk = vec![Toast::new("save", "Saved file")];
        presence.reconcile(&shrunk, MotionPolicy::Full);
        assert_eq!(presence.phase("sync"), Some(ToastVisualPhase::Exit));
        assert!(presence.has_exiting());
        assert_eq!(
            presence.retained("sync").map(|toast| toast.title.as_str()),
            Some("Syncing")
        );
        assert!(presence.drop_visual("sync"));
        assert_eq!(presence.phase("sync"), None);
        assert!(presence.retained("sync").is_none());
    }

    #[test]
    fn frozen_presence_settles_new_rows_and_drops_remnants_immediately() {
        let mut presence = ToastStackPresence::new();
        presence.reconcile(&[Toast::new("a", "A")], MotionPolicy::Full);
        // A new row under frozen has no clock: the endpoint settles at once.
        presence.reconcile(
            &[Toast::new("a", "A"), Toast::new("b", "B")],
            MotionPolicy::Frozen,
        );
        assert_eq!(presence.phase("b"), Some(ToastVisualPhase::Settled));
        // A removed row under frozen leaves no remnant.
        presence.reconcile(&[Toast::new("b", "B")], MotionPolicy::Frozen);
        assert_eq!(presence.phase("a"), None);
        assert!(!presence.has_exiting());
    }

    fn presence_handlers() -> ToastStackHandlers {
        ToastStackHandlers {
            instance_id: Some("presence".to_owned()),
            ..ToastStackHandlers::default()
        }
    }

    fn presence_animation_has(animation: &NodeAnimation, property: AnimProperty) -> bool {
        animation
            .keyframes
            .iter()
            .any(|frame| frame.values.iter().any(|(value, _)| *value == property))
    }

    #[test]
    fn renderer_paints_the_presence_phase_and_marks_the_exit_remnant_inert() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut presence = ToastStackPresence::new();
        let row = |id: &str| Toast::new(id, id.to_uppercase());

        // Preloaded items settle: no clock declaration at all.
        let node = toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("a"), row("b")]),
            &ctx,
            &mut presence,
            presence_handlers(),
        );
        let a = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-a"))
            .expect("settled row a");
        assert_eq!(a.roles.get("phase").map(String::as_str), Some("settled"));
        assert!(a.style.animation.is_none(), "a settled row runs no clock");

        // A late item enters under full with the bounded opacity/translation.
        let node = toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("a"), row("b"), row("c")]),
            &ctx,
            &mut presence,
            presence_handlers(),
        );
        let c = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-c"))
            .expect("entering row c");
        assert_eq!(c.roles.get("phase").map(String::as_str), Some("enter"));
        let enter = c.style.animation.as_ref().expect("enter clock");
        assert!(presence_animation_has(enter, AnimProperty::Opacity));
        assert!(presence_animation_has(enter, AnimProperty::TranslateY));

        // The host reports completion: the settled repaint runs no clock.
        assert!(presence.settle("c"));
        let node = toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("a"), row("b"), row("c")]),
            &ctx,
            &mut presence,
            presence_handlers(),
        );
        let c = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-c"))
            .expect("settled row c");
        assert_eq!(c.roles.get("phase").map(String::as_str), Some("settled"));
        assert!(c.style.animation.is_none());

        // Removing b keeps it as an inert paint remnant: aria-hidden, no
        // semantic id, no focus stop, no activation, exit clock attached.
        let node = toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("a"), row("c")]),
            &ctx,
            &mut presence,
            presence_handlers(),
        );
        let remnant = &node;
        let b = remnant
            .find(&|node| node.runtime_id.as_deref() == Some("toast-host:presence:toast:b"))
            .expect("retained exit remnant");
        assert_eq!(
            b.a11y.hidden,
            Some(true),
            "the remnant leaves accessibility"
        );
        assert_eq!(b.id, None, "the remnant has no semantic activation id");
        assert_eq!(b.roles.get("phase").map(String::as_str), Some("exit"));
        assert!(b
            .style
            .animation
            .as_ref()
            .is_some_and(|animation| { presence_animation_has(animation, AnimProperty::Opacity) }));
        let dismiss = remnant
            .find(&|node| node.runtime_id.as_deref() == Some("toast-host:presence:toast:b:dismiss"))
            .expect("remnant dismiss paint");
        assert!(!dismiss.interaction.focusable, "the remnant is unfocusable");
        assert_eq!(dismiss.a11y.tab_index, Some(-1));
        assert_eq!(dismiss.id, None);
        assert!(dismiss.interaction.on_activate.is_none());
        assert!(dismiss.style.focus_ring.is_none());
    }

    #[test]
    fn presence_treatment_follows_the_effective_motion_policy() {
        let theme = theme();
        let full = RenderContext::new(&theme);
        let reduced = full.with_motion_policy(MotionPolicy::Reduced);
        let frozen = full.with_motion_policy(MotionPolicy::Frozen);
        let row = |id: &str| Toast::new(id, id.to_uppercase());

        // Reduced removes translation and keeps the short opacity enter.
        let mut presence = ToastStackPresence::new();
        toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("a")]),
            &full,
            &mut presence,
            presence_handlers(),
        );
        let node = toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("a"), row("b")]),
            &reduced,
            &mut presence,
            presence_handlers(),
        );
        let b = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-b"))
            .expect("reduced entering row");
        let animation = b.style.animation.as_ref().expect("reduced enter clock");
        assert!(presence_animation_has(animation, AnimProperty::Opacity));
        assert!(
            !presence_animation_has(animation, AnimProperty::TranslateY),
            "reduced drops translation"
        );

        // Frozen paints the endpoint with no clock and drops the remnant.
        let mut frozen_presence = ToastStackPresence::new();
        let node = toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("x"), row("y")]),
            &frozen,
            &mut frozen_presence,
            presence_handlers(),
        );
        let y = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-y"))
            .expect("frozen row y");
        assert_eq!(y.roles.get("phase").map(String::as_str), Some("settled"));
        assert!(y.style.animation.is_none());
        let node = toast_stack_with_presence(
            &ToastStackSpec::new().with_toasts(vec![row("y")]),
            &frozen,
            &mut frozen_presence,
            presence_handlers(),
        );
        assert!(
            node.find(&|node| node.runtime_id.as_deref() == Some("toast-host:presence:toast:x"))
                .is_none(),
            "frozen drops the removed row with no remnant"
        );
    }

    #[test]
    fn reconcile_reports_removed_actions_once_with_row_survival() {
        let mut presence = ToastStackPresence::new();
        let with_action = vec![
            Toast::new("a", "A").with_action_label("Retry"),
            Toast::new("b", "B"),
        ];
        assert!(presence
            .reconcile(&with_action, MotionPolicy::Full)
            .removed_actions
            .is_empty());

        // A surviving row whose action label is dropped reports the live case.
        let without = vec![Toast::new("a", "A"), Toast::new("b", "B")];
        let change = presence.reconcile(&without, MotionPolicy::Full);
        assert_eq!(
            change.removed_actions,
            vec![RemovedToastAction {
                id: "a".into(),
                row_survives: true,
            }]
        );
        // It is the transition, not the state: the next reconcile is quiet,
        // so an exit remnant never re-triggers the move.
        assert!(presence
            .reconcile(&without, MotionPolicy::Full)
            .removed_actions
            .is_empty());

        // A row that leaves while still carrying an action reports the fallback.
        let mut presence = ToastStackPresence::new();
        presence.reconcile(&with_action, MotionPolicy::Full);
        let only_b = vec![Toast::new("b", "B")];
        let change = presence.reconcile(&only_b, MotionPolicy::Full);
        assert_eq!(
            change.removed_actions,
            vec![RemovedToastAction {
                id: "a".into(),
                row_survives: false,
            }]
        );
        assert!(presence
            .reconcile(&only_b, MotionPolicy::Full)
            .removed_actions
            .is_empty());
    }

    #[test]
    fn action_removal_target_prefers_row_dismiss_then_surviving_order() {
        let order = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(
            toast_action_removal_focus_target(&order, Some("scope"), "a", true, Some("outside")),
            Some("toast-host:scope:toast:a:dismiss".to_owned()),
            "a surviving row hands its own dismiss control"
        );
        assert_eq!(
            toast_action_removal_focus_target(&order, None, "a", false, Some("outside")),
            Some("poodle-toast-dismiss-b".to_owned()),
            "a removed row falls through to the next surviving row"
        );
        assert_eq!(
            toast_action_removal_focus_target(&["a".to_owned()], None, "a", false, Some("outside")),
            Some("outside".to_owned()),
            "the last removed row restores the entered-from control"
        );
    }
}
