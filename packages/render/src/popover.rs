//! Popover — anchored floating overlay composition (the open panel at current
//! state).
//!
//! Contract: `docs/contracts/components/popover.md`
//! Ported from: `packages/jetstream/components/src/popover.rs`.
//!
//! The composition owns the whole portable overlay profile: a fixed-size
//! trigger, the conditional dialog surface, stable part identity, accessibility
//! metadata (dialog role/name, trigger expanded/controls, focusability),
//! token roles, and the layer/dismiss intent the generic backend path turns
//! into real Escape and outside-pointer dismissal. Open state stays host-owned
//! (the spec's `open`/`defaultOpen`); the host rebuilds the tree when it
//! changes.

use poodle_node::{
    DismissReason, LayoutDirection, LayoutSizing, Node, NodeRole, ShadowLayer,
};
use poodle_specs::PopoverSpec;

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::floating_overlay::floating_overlay;
use crate::presentation::rem_to_px;

/// The composition's fixed trigger box. The surface is positioned against
/// these same numbers, so the authored offset is the only variable in the
/// relative surface/trigger geometry.
pub const POPOVER_ANCHOR_WIDTH_PX: f32 = 96.0;
pub const POPOVER_ANCHOR_HEIGHT_PX: f32 = 32.0;

/// Stable semantic part ids. Backends key per-instance state on `runtime_id`;
/// the semantic ids stay readable, and accessibility relationships point at
/// them.
pub const POPOVER_TRIGGER_ID: &str = "popover-trigger";
pub const POPOVER_SURFACE_ID: &str = "popover-surface";

/// The layer id the open composition registers on the backend dismiss stack.
/// Scoped by the instance id so two mounted popovers stay distinct layers.
pub fn popover_layer_id(instance_id: Option<&str>) -> String {
    instance_id
        .map(|scope| format!("popover-layer:{scope}"))
        .unwrap_or_else(|| "popover-layer".to_owned())
}

/// Host-owned interaction intent. The backend turns these into real listeners.
#[derive(Clone, Default)]
pub struct PopoverHandlers {
    /// Trigger activation (toggle). Binds as the real click path.
    pub on_activate: Option<std::sync::Arc<dyn Fn() + Send + Sync>>,
    /// Document-level dismissal (escape / outside). The handler receives the
    /// reason; the component applies its own guards before closing.
    pub on_dismiss: Option<std::sync::Arc<dyn Fn(DismissReason) + Send + Sync>>,
    /// Stable native instance scope. Semantic ids remain readable; backends
    /// key focus, bounds, and layer state on the scoped runtime ids.
    pub instance_id: Option<String>,
}

/// The popover surface only — used by hosts that compose their own trigger
/// (message centre, Jetstream compat). The full overlay profile lives in
/// [`popover`].
pub fn popover_surface(spec: &PopoverSpec, ctx: &RenderContext<'_>, content: Option<Node>) -> Node {
    // Contract §8 surface: background = background-elevated, border =
    // border-subtle at 74%, radius = radius-surface.
    let fill = ctx.theme().resolve_color(spec.surface_fill_token());
    let border_base = ctx.theme().resolve_color(spec.surface_border_token());
    let border = with_alpha(border_base, border_base.3 * spec.surface_border_alpha());
    let border_width = ctx.theme().resolve_space("border.width.default");
    let radius = ctx.theme().resolve_radius("radius.surface");

    // Contract §8 padding = space.panel.y / space.panel.x. The padding lives
    // on an inner content wrapper so the surface node's own box stays the
    // bounds target (the backend records its rendered box for containment
    // and relative geometry).
    let pad_x = ctx.theme().resolve_space("space.panel.x");
    let pad_y = ctx.theme().resolve_space("space.panel.y");

    // Contract §7: min-width 14rem, max-width min(24rem, 90vw) — the 24rem
    // arm; both overridable via surfaceMinWidth/surfaceMaxWidth.
    let min_w = rem_to_px(spec.effective_surface_min_width_rem());
    let max_w = rem_to_px(spec.effective_surface_max_width_rem());

    let mut el = Node::container();
    el.id = Some(POPOVER_SURFACE_ID.to_owned());
    // Contract: the popover surface is a `dialog`.
    el.a11y.role = Some(NodeRole::Dialog);
    {
        let s = &mut el.style;
        // Explicit Row (see switch.rs): the old surface kept the default.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.min_width = Some(min_w);
        s.max_width = Some(max_w);
        // Contract §8 box-shadow: token-accurate elevation-overlay primary
        // drop + the inset top highlight layered alongside.
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: poodle_node::ColorValue(1.0, 1.0, 1.0, 0.08),
            inset: true,
        }];
        s.overlay = true; // Render above normal content.
    }

    let mut padded = Node::container();
    padded.style.descriptor.layout.direction = LayoutDirection::Row;
    {
        let pad = &mut padded.style.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
    }
    if let Some(content_el) = content {
        padded = padded.child(content_el);
    }
    let mut el = el.child(padded);

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}

/// The full portable Popover composition: trigger + conditional dialog
/// surface, wrapped and positioned through the shared floating-overlay path
/// with the authored offset as the gap. Carries stable part identity,
/// accessibility metadata, token roles, and the dismiss/layer intent the
/// generic backend path executes.
pub fn popover(
    spec: &PopoverSpec,
    ctx: &RenderContext<'_>,
    handlers: &PopoverHandlers,
    trigger: Option<Node>,
    content: Option<Node>,
) -> Node {
    let instance = handlers.instance_id.clone();
    let open = spec.current_open() && !spec.disabled;
    let layer_id = popover_layer_id(instance.as_deref());
    let surface_id = instance
        .as_ref()
        .map(|scope| format!("{scope}:{POPOVER_SURFACE_ID}"))
        .unwrap_or_else(|| POPOVER_SURFACE_ID.to_owned());

    // ── Trigger ────────────────────────────────────────────────────────────
    let mut trigger_node = Node::container();
    trigger_node.id = Some(POPOVER_TRIGGER_ID.to_owned());
    trigger_node.runtime_id = instance
        .as_ref()
        .map(|scope| format!("{scope}:{POPOVER_TRIGGER_ID}"));
    trigger_node.a11y.role = Some(NodeRole::Button);
    trigger_node.a11y.label = trigger_label(trigger.as_ref());
    trigger_node.a11y.expanded = Some(open);
    trigger_node.a11y.controls = open.then(|| surface_id.clone());
    trigger_node.a11y.tab_index = Some(if spec.disabled { -1 } else { 0 });
    trigger_node.interaction.focusable = true;
    trigger_node.interaction.disabled = spec.disabled;
    trigger_node.interaction.on_activate = handlers.on_activate.clone();
    trigger_node.interaction.on_dismiss = open.then(|| handlers.on_dismiss.clone()).flatten();
    trigger_node.interaction.dismiss_layer = open.then(|| layer_id.clone());
    {
        let s = &mut trigger_node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(POPOVER_ANCHOR_WIDTH_PX);
        s.descriptor.layout.height = LayoutSizing::Fixed(POPOVER_ANCHOR_HEIGHT_PX);
        // Contract §8 trigger focus-visible: the accent focus ring. The
        // backend applies this patch while the trigger holds focus, which
        // also makes the focus-visible state observable.
        s.focus = Some(poodle_node::StylePatch {
            border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
            ..poodle_node::StylePatch::default()
        });
        if spec.block {
            s.fill_width = true;
        }
    }
    if let Some(trigger_el) = trigger {
        trigger_node = trigger_node.child(trigger_el);
    }

    // ── Surface (conditional) ──────────────────────────────────────────────
    let surface = open.then(|| {
        let mut node = popover_surface(spec, ctx, content);
        node.runtime_id = instance.as_ref().map(|scope| format!("{scope}:{POPOVER_SURFACE_ID}"));
        node.interaction.focusable = spec.initial_focus == poodle_specs::PopoverInitialFocus::Content;
        node.a11y.tab_index = Some(if spec.initial_focus == poodle_specs::PopoverInitialFocus::Content { 0 } else { -1 });
        if spec.initial_focus == poodle_specs::PopoverInitialFocus::Content {
            // The surface itself is the focus target in content mode; the
            // focus patch also makes the focused state backend-observable.
            node.style.focus = Some(poodle_node::StylePatch {
                border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
                ..poodle_node::StylePatch::default()
            });
        }
        node.interaction.dismiss_layer = Some(layer_id.clone());
        if spec.surface_width.is_trigger() {
            // The trigger-width rule overrides the 14rem floor (contract §8:
            // `width: 100%` / `min-width: 100%`). An absolute surface cannot
            // resolve a percentage against its positioned ancestor, so the
            // width is pinned to the composition's fixed trigger width.
            node.style.min_width = None;
            node.style.descriptor.layout.width = LayoutSizing::Fixed(POPOVER_ANCHOR_WIDTH_PX);
        }
        node
    });

    let mut wrapper = floating_overlay(
        trigger_node,
        surface,
        spec.placement,
        POPOVER_ANCHOR_HEIGHT_PX,
        POPOVER_ANCHOR_WIDTH_PX,
        spec.offset,
    );
    // Token roles project onto the composition root (the web root carries
    // the data-* attributes). Values are kebab-cased like the web's.
    wrapper.roles.insert(
        "placement".to_owned(),
        kebab_case_debug(spec.placement),
    );
    wrapper.roles.insert(
        "surfaceWidth".to_owned(),
        kebab_case_debug(spec.surface_width),
    );
    wrapper
}

/// Debug-name → kebab-case (`BottomStart` → `bottom-start`), matching the
/// web's `data-placement` values.
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

/// The trigger's accessible label: the intrinsic text of the trigger node the
/// host composed, falling back to the contract default affordance.
fn trigger_label(trigger: Option<&Node>) -> Option<String> {
    trigger.and_then(|node| node.intrinsic_text().map(str::to_owned))
}
