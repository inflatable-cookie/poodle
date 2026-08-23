//! ModelConnectionCard — a controlled disclosure card for one configured
//! model connection.
//!
//! Contract: `docs/contracts/components/model-connection-card.md`
//!
//! Disclosure and enabled state stay independent: they are separate hit
//! regions, separate tab stops, and separate callbacks, and neither derives
//! the other. Readiness is display posture the host supplied — this file maps
//! it to a tone and a label and nothing else. Closing the details region asks
//! the backend to return focus to the disclosure control.
//!
//! The provider mark, badges, the closed accessory, actions, and the details
//! body are host content keyed by this card's opaque id.

use std::sync::Arc;

use poodle_headless::disclosure::{
    disclosure_transition, DisclosureContext, DisclosureEffect, DisclosureEvent,
};
use poodle_headless::model_connection::{
    model_connection_card_status_label, model_connection_readiness_tone, ModelConnectionReadiness,
    ModelConnectionStatusTone,
};
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeRole,
    StylePatch,
};
use poodle_specs::{
    ButtonVariant, ControlSize, IconButtonSpec, ModelConnectionCardSpec, SemanticControlSizeRole,
    StatusIndicatorSpec, StatusTone, SwitchSpec,
};

use crate::context::RenderContext;
use crate::icon_button::icon_button;
use crate::presentation::rem_to_px;
use crate::status_indicator::status_indicator;
use crate::switch::switch;

/// Contract §8: the label weight the card title and family share.
const LABEL_WEIGHT: u16 = 500;

/// Host callbacks. The two are never combined and never derive each other.
#[derive(Default)]
pub struct ModelConnectionCardHandlers {
    /// The disclosure control was activated, with the next open state.
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// The enable Switch was activated, with the next preference. No readiness
    /// or authorisation side effect belongs here.
    pub on_enabled_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Closing asks the backend to restore focus to the disclosure control.
    pub on_focus_request: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Stable native instance scope. `id` already scopes one card within a
    /// list; this scopes two surfaces that show the same connection, which
    /// would otherwise share one backend focus handle.
    pub instance_id: Option<String>,
}

/// The backend-state id of one card part: the instance scope when the host
/// supplied one, else the spec's own instance-scoped semantic id.
fn scoped(instance_id: Option<&str>, semantic: String) -> Option<String> {
    instance_id.map(|scope| format!("model-connection-card:{scope}:{semantic}"))
}

/// Host-composed content, keyed by this card's opaque id at the call site.
#[derive(Default)]
pub struct ModelConnectionCardSlots {
    /// The provider mark, inline immediately before the name.
    pub leading: Option<Node>,
    /// Route maturity or host labels, beside the name.
    pub badges: Option<Node>,
    /// Closed-only accessory — intended for `UpdateCenter`.
    pub closed_accessory: Option<Node>,
    /// Optional host actions or menu.
    pub actions: Option<Node>,
    /// The open details body: forms, access actions, diagnostics, the model
    /// catalogue editor. Poodle retains none of its state.
    pub details: Option<Node>,
}

pub fn model_connection_card(
    spec: &ModelConnectionCardSpec,
    ctx: &RenderContext<'_>,
    handlers: ModelConnectionCardHandlers,
) -> Node {
    model_connection_card_with_slots(spec, ctx, ModelConnectionCardSlots::default(), handlers)
}

pub fn model_connection_card_with_slots(
    spec: &ModelConnectionCardSpec,
    ctx: &RenderContext<'_>,
    slots: ModelConnectionCardSlots,
    handlers: ModelConnectionCardHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let border = ctx.theme().resolve_color("color.border.subtle");
    let label_size = ctx.theme().resolve_space("typography.label.size");
    let disabled_opacity = ctx.theme().resolve_opacity("state.opacity.disabled");

    // ── Identity ──
    let mut title_row = Node::container();
    {
        let s = &mut title_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.xs");
        s.flex_wrap = true;
        s.min_width = Some(0.0);
    }
    // The provider mark sits inline before the name and never indents the
    // summary lines below it.
    let mut mark = slots
        .leading
        .unwrap_or_else(|| Node::icon("package", rem_to_px(1.0)));
    if mark.style.descriptor.text_color.is_none() {
        mark.style.descriptor.text_color = Some(text_secondary);
    }
    mark.style.flex_none = true;
    let mut title = Node::text(&spec.title);
    title.style.text_size = Some(ctx.theme().resolve_space("typography.body.size"));
    title.style.text_weight = Some(LABEL_WEIGHT);
    title.style.descriptor.text_color = Some(text_primary);
    title.style.text_ellipsis = true;
    let mut title_row = title_row.child(mark).child(title);
    if let Some(badges) = slots.badges {
        title_row = title_row.child(badges);
    }

    let mut identity = Node::container();
    {
        let s = &mut identity.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
        if !spec.is_enabled {
            s.descriptor.opacity = disabled_opacity;
        }
    }
    // The provider family is part of the card's accessible identity, but the
    // visible summary already carries title and route — so it is stated on the
    // identity group rather than as a third visible line.
    identity.a11y.role = Some(NodeRole::Group);
    identity.a11y.label = Some(spec.provider_label.clone());
    let mut identity = identity.child(title_row);
    let meta = spec.meta_line();
    if !meta.is_empty() {
        let mut meta_node = Node::text(&meta);
        meta_node.style.text_size = Some(label_size);
        meta_node.style.descriptor.text_color = Some(text_secondary);
        meta_node.style.text_ellipsis = true;
        identity = identity.child(meta_node);
    }

    // ── Controls: status, accessory, actions, disclosure, switch ──
    let status = status_indicator(
        &StatusIndicatorSpec::new()
            .with_status(status_tone(model_connection_readiness_tone(spec.readiness)))
            .with_label(model_connection_card_status_label(
                spec.readiness,
                spec.access_summary.as_deref(),
                &spec.readiness_label,
            ))
            .with_size(effective_size)
            .with_density(density),
        ctx,
    );

    let mut controls = Node::container();
    {
        let s = &mut controls.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.xs");
        s.flex_wrap = true;
        s.flex_none = true;
    }
    // Everything but the Switch dims with the card; the Switch stays clear so
    // an off connection can always be turned back on.
    let mut dimmable = Node::container();
    {
        let s = &mut dimmable.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.xs");
        if !spec.is_enabled {
            s.descriptor.opacity = disabled_opacity;
        }
    }
    let mut dimmable = dimmable.child(status);
    if !spec.is_open {
        if let Some(accessory) = slots.closed_accessory {
            dimmable = dimmable.child(accessory);
        }
    }
    if let Some(actions) = slots.actions {
        dimmable = dimmable.child(actions);
    }
    dimmable = dimmable.child(disclosure_button(spec, ctx, effective_size, &handlers));

    let switch_node = switch(
        &SwitchSpec::new()
            .with_checked(spec.is_enabled)
            .with_disabled(spec.is_disabled || spec.is_enable_disabled)
            .with_aria_label(spec.enable_label())
            .with_size(effective_size)
            .with_density(density),
        ctx,
        (!(spec.is_disabled || spec.is_enable_disabled))
            .then(|| {
                handlers.on_enabled_change.clone().map(|handler| {
                    Arc::new(move |next: bool| handler(next)) as Arc<dyn Fn(bool) + Send + Sync>
                })
            })
            .flatten(),
    );
    let controls = controls.child(dimmable).child(switch_node);

    // ── Summary row ──
    let mut summary = Node::container();
    {
        let s = &mut summary.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = ctx.theme().resolve_space("space.stack.sm");
        pad.bottom = ctx.theme().resolve_space("space.stack.sm");
        pad.left = ctx.theme().resolve_space("space.inline.md");
        pad.right = ctx.theme().resolve_space("space.inline.md");
        s.flex_wrap = true;
        s.fill_width = true;
        s.min_width = Some(0.0);
    }
    let summary = summary.child(identity).child(controls);

    // ── Root ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.border.width = rem_to_px(0.0625);
        s.descriptor.border.color = border;
        s.descriptor.background = Some(if spec.is_enabled {
            ctx.theme().resolve_color("color.background.panel")
        } else {
            ctx.theme().resolve_color("color.background.surface")
        });
        s.fill_width = true;
        s.min_width = Some(0.0);
        let radius = ctx.theme().resolve_radius("radius.surface");
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
    }
    root.a11y.role = Some(NodeRole::Region);
    root.a11y.label = Some(spec.effective_aria_label().to_string());
    root.roles
        .insert("open".to_string(), spec.is_open.to_string());
    root.roles
        .insert("enabled".to_string(), spec.is_enabled.to_string());
    root.roles
        .insert("readiness".to_string(), readiness_role(spec.readiness).to_string());
    let mut root = root.child(summary);

    if spec.is_open {
        let mut details = Node::container();
        {
            let s = &mut details.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.md");
            let inset = ctx.theme().resolve_space("space.stack.md");
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = inset;
            pad.bottom = inset;
            pad.left = inset;
            pad.right = inset;
            s.border_top_width = Some(rem_to_px(0.0625));
            s.border_color_top = Some(border);
            s.fill_width = true;
            s.min_width = Some(0.0);
        }
        details.id = Some(spec.details_id());
        details.runtime_id = scoped(handlers.instance_id.as_deref(), spec.details_id());
        details.a11y.role = Some(NodeRole::Region);
        details.a11y.label = Some(spec.details_label());
        if let Some(content) = slots.details {
            details = details.child(content);
        }
        root = root.child(details);
    }

    root
}

fn status_tone(tone: ModelConnectionStatusTone) -> StatusTone {
    match tone {
        ModelConnectionStatusTone::Neutral => StatusTone::Neutral,
        ModelConnectionStatusTone::Info => StatusTone::Info,
        ModelConnectionStatusTone::Success => StatusTone::Success,
        ModelConnectionStatusTone::Warning => StatusTone::Warning,
        ModelConnectionStatusTone::Danger => StatusTone::Danger,
    }
}

fn readiness_role(readiness: ModelConnectionReadiness) -> &'static str {
    match readiness {
        ModelConnectionReadiness::Ready => "ready",
        ModelConnectionReadiness::Checking => "checking",
        ModelConnectionReadiness::Attention => "attention",
        ModelConnectionReadiness::Unavailable => "unavailable",
        ModelConnectionReadiness::Unknown => "unknown",
        ModelConnectionReadiness::Error => "error",
    }
}

fn disclosure_button(
    spec: &ModelConnectionCardSpec,
    ctx: &RenderContext<'_>,
    effective_size: ControlSize,
    handlers: &ModelConnectionCardHandlers,
) -> Node {
    let on_open = handlers.on_open_change.clone();
    let density = ctx.resolve_density(spec.density);
    let on_focus = handlers.on_focus_request.clone();
    let is_open = spec.is_open;
    let is_disabled = spec.is_disabled;
    let disclosure_id = spec.disclosure_id();
    // The focus destination is what the backend keys focus handles by: the
    // scoped runtime id when the host supplied a scope, else the semantic id.
    let focus_target = scoped(handlers.instance_id.as_deref(), disclosure_id.clone())
        .unwrap_or_else(|| disclosure_id.clone());

    let activate: Option<Arc<dyn Fn() + Send + Sync>> = (!is_disabled).then(|| {
        Arc::new(move || {
            // The shared disclosure transition owns the guard; the card only
            // decides what a close means for focus.
            let (_, effects) = disclosure_transition(
                DisclosureContext {
                    open: is_open,
                    disabled: is_disabled,
                },
                DisclosureEvent::Toggle,
            );
            for effect in effects {
                let DisclosureEffect::EmitOpenChange { open } = effect;
                if let Some(handler) = &on_open {
                    handler(open);
                }
                if !open {
                    if let Some(handler) = &on_focus {
                        handler(&focus_target);
                    }
                }
            }
        }) as Arc<dyn Fn() + Send + Sync>
    });

    let mut node = icon_button(
        &IconButtonSpec::new()
            .with_icon(if is_open { "chevron-up" } else { "chevron-down" })
            .with_variant(ButtonVariant::Ghost)
            .with_size_role(SemanticControlSizeRole::Chrome)
            .with_size(effective_size)
            .with_density(density)
            .with_aria_label(spec.disclosure_label())
            .with_expanded(is_open)
            .with_controls(spec.details_id())
            .with_disabled(is_disabled),
        ctx,
        activate,
    );
    node.runtime_id = scoped(handlers.instance_id.as_deref(), disclosure_id.clone());
    node.id = Some(disclosure_id);
    // `icon_button` renders no focus patch, and the GPUI backend only creates
    // a focus handle for a focusable node that carries one — so without this
    // the disclosure is unreachable by keyboard and focus cannot be restored
    // to it (PAPERCUTS: icon-button focus patch). Same workaround
    // `poodle-render::history_center` already carries.
    node.style.focus = Some(StylePatch {
        border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
        ..StylePatch::default()
    });
    // `IconButtonSpec`'s expanded/controls flags do not reach `Node.a11y`
    // through `icon_button` (PAPERCUTS: icon-button-expanded-controls), so the
    // disclosure states its own relationship, as HistoryCenter and
    // ChangedFiles already do.
    node.a11y.expanded = Some(is_open);
    node.a11y.controls = Some(spec.details_id());
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> ModelConnectionCardSpec {
        ModelConnectionCardSpec::new("conn-openai-work", "OpenAI · Work", "OpenAI")
            .with_route_label("Responses API")
            .with_version("2026-08")
            .with_access_summary("API key on file")
            .with_readiness(ModelConnectionReadiness::Ready, "Ready")
    }

    fn disclosure<'a>(node: &'a Node, spec: &ModelConnectionCardSpec) -> &'a Node {
        node.find(&|n| n.id.as_deref() == Some(spec.disclosure_id().as_str()))
            .expect("disclosure control")
    }

    fn switch_node<'a>(node: &'a Node, spec: &ModelConnectionCardSpec) -> &'a Node {
        node.find(&|n| n.a11y.label.as_deref() == Some(spec.enable_label().as_str()))
            .expect("enable switch")
    }

    #[test]
    fn open_and_enabled_stay_independent() {
        let opens = Arc::new(Mutex::new(Vec::new()));
        let enables = Arc::new(Mutex::new(Vec::new()));
        let open_sink = Arc::clone(&opens);
        let enable_sink = Arc::clone(&enables);
        let spec = spec();
        let node = model_connection_card(
            &spec,
            &RenderContext::new(&theme()),
            ModelConnectionCardHandlers {
                on_open_change: Some(Arc::new(move |open| open_sink.lock().unwrap().push(open))),
                on_enabled_change: Some(Arc::new(move |enabled| {
                    enable_sink.lock().unwrap().push(enabled)
                })),
                ..ModelConnectionCardHandlers::default()
            },
        );

        (disclosure(&node, &spec)
            .interaction
            .on_activate
            .as_ref()
            .expect("disclosure activation"))();
        assert_eq!(opens.lock().unwrap().as_slice(), [true]);
        assert!(
            enables.lock().unwrap().is_empty(),
            "disclosing never touches the preference"
        );

        let switch = switch_node(&node, &spec);
        (switch
            .interaction
            .on_activate
            .as_ref()
            .expect("switch activation"))();
        assert_eq!(enables.lock().unwrap().as_slice(), [false]);
        assert_eq!(
            opens.lock().unwrap().as_slice(),
            [true],
            "the preference never touches disclosure"
        );
    }

    #[test]
    fn ready_shows_the_access_summary_and_other_postures_show_readiness() {
        let ready = model_connection_card(&spec(), &RenderContext::new(&theme()), ModelConnectionCardHandlers::default());
        assert!(ready
            .texts().contains(&"API key on file"));
        assert!(!ready.texts().contains(&"Ready"));

        let checking = model_connection_card(
            &spec()
                .with_readiness(ModelConnectionReadiness::Checking, "Checking install")
                .with_access_summary("Signed in"),
            &RenderContext::new(&theme()),
            ModelConnectionCardHandlers::default(),
        );
        assert!(checking
            .texts().contains(&"Checking install"));
        assert!(!checking.texts().contains(&"Signed in"));
        assert_eq!(
            checking.roles.get("readiness").map(String::as_str),
            Some("checking")
        );
    }

    #[test]
    fn the_closed_accessory_is_mounted_only_while_closed() {
        let closed = model_connection_card_with_slots(
            &spec(),
            &RenderContext::new(&theme()),
            ModelConnectionCardSlots {
                closed_accessory: Some(Node::text("UPDATE CENTER")),
                actions: Some(Node::text("HOST ACTIONS")),
                details: Some(Node::text("HOST DETAILS")),
                ..ModelConnectionCardSlots::default()
            },
            ModelConnectionCardHandlers::default(),
        );
        assert!(closed.texts().contains(&"UPDATE CENTER"));
        assert!(closed.texts().contains(&"HOST ACTIONS"));
        assert!(!closed.texts().contains(&"HOST DETAILS"));

        let open = model_connection_card_with_slots(
            &spec().with_open(true),
            &RenderContext::new(&theme()),
            ModelConnectionCardSlots {
                closed_accessory: Some(Node::text("UPDATE CENTER")),
                actions: Some(Node::text("HOST ACTIONS")),
                details: Some(Node::text("HOST DETAILS")),
                ..ModelConnectionCardSlots::default()
            },
            ModelConnectionCardHandlers::default(),
        );
        assert!(!open.texts().contains(&"UPDATE CENTER"));
        assert!(open.texts().contains(&"HOST ACTIONS"));
        assert!(open.texts().contains(&"HOST DETAILS"));
    }

    #[test]
    fn the_details_region_is_labelled_and_named_by_the_disclosure() {
        let spec = spec().with_open(true);
        let node = model_connection_card_with_slots(
            &spec,
            &RenderContext::new(&theme()),
            ModelConnectionCardSlots {
                details: Some(Node::text("HOST DETAILS")),
                ..ModelConnectionCardSlots::default()
            },
            ModelConnectionCardHandlers::default(),
        );

        let details = node
            .find(&|n| n.id.as_deref() == Some(spec.details_id().as_str()))
            .expect("details region");
        assert_eq!(details.a11y.role, Some(NodeRole::Region));
        assert_eq!(
            details.a11y.label.as_deref(),
            Some("OpenAI · Work details")
        );

        let control = disclosure(&node, &spec);
        assert_eq!(control.a11y.expanded, Some(true));
        assert_eq!(control.a11y.controls.as_deref(), Some(spec.details_id().as_str()));
        assert_eq!(
            control.a11y.label.as_deref(),
            Some("Collapse OpenAI · Work")
        );
    }

    #[test]
    fn closing_restores_focus_to_the_disclosure_control() {
        let focus = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&focus);
        let spec = spec().with_open(true);
        let node = model_connection_card(
            &spec,
            &RenderContext::new(&theme()),
            ModelConnectionCardHandlers {
                on_open_change: Some(Arc::new(|_| {})),
                on_focus_request: Some(Arc::new(move |id: &str| {
                    sink.lock().unwrap().push(id.to_string())
                })),
                ..ModelConnectionCardHandlers::default()
            },
        );
        (disclosure(&node, &spec)
            .interaction
            .on_activate
            .as_ref()
            .expect("activation"))();
        assert_eq!(focus.lock().unwrap().as_slice(), [spec.disclosure_id()]);

        // Opening does not move focus.
        let focus = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&focus);
        let closed_spec = self::spec();
        let node = model_connection_card(
            &closed_spec,
            &RenderContext::new(&theme()),
            ModelConnectionCardHandlers {
                on_open_change: Some(Arc::new(|_| {})),
                on_focus_request: Some(Arc::new(move |id: &str| {
                    sink.lock().unwrap().push(id.to_string())
                })),
                ..ModelConnectionCardHandlers::default()
            },
        );
        (disclosure(&node, &closed_spec)
            .interaction
            .on_activate
            .as_ref()
            .expect("activation"))();
        assert!(focus.lock().unwrap().is_empty());
    }

    #[test]
    fn an_off_card_dims_its_copy_but_never_its_switch() {
        let spec = spec().with_enabled(false);
        let node = model_connection_card(&spec, &RenderContext::new(&theme()), ModelConnectionCardHandlers::default());
        assert_eq!(node.roles.get("enabled").map(String::as_str), Some("false"));

        let switch = switch_node(&node, &spec);
        assert_eq!(
            switch.style.descriptor.opacity, 1.0,
            "the enable switch stays clear on an off card"
        );

        // The identity group carries the dimmed treatment.
        let identity = node
            .find(&|n| n.a11y.label.as_deref() == Some("OpenAI"))
            .expect("identity group");
        assert!(identity.style.descriptor.opacity < 1.0);
    }

    #[test]
    fn a_disabled_card_is_inert_but_readable() {
        let spec = spec().with_disabled(true);
        let node = model_connection_card(
            &spec,
            &RenderContext::new(&theme()),
            ModelConnectionCardHandlers {
                on_open_change: Some(Arc::new(|_| unreachable!("a disabled card discloses"))),
                on_enabled_change: Some(Arc::new(|_| unreachable!("a disabled card toggles"))),
                ..ModelConnectionCardHandlers::default()
            },
        );
        assert!(disclosure(&node, &spec).interaction.on_activate.is_none());
        assert!(switch_node(&node, &spec).interaction.on_activate.is_none());
        assert!(node.texts().contains(&"OpenAI · Work"));
        assert!(node
            .texts().contains(&"Responses API · 2026-08"));
    }

    #[test]
    fn the_enable_switch_can_be_disabled_on_its_own() {
        let spec = spec().with_enable_disabled(true);
        let node = model_connection_card(
            &spec,
            &RenderContext::new(&theme()),
            ModelConnectionCardHandlers {
                on_open_change: Some(Arc::new(|_| {})),
                on_enabled_change: Some(Arc::new(|_| unreachable!("the switch is disabled"))),
                ..ModelConnectionCardHandlers::default()
            },
        );
        assert!(switch_node(&node, &spec).interaction.on_activate.is_none());
        assert!(
            disclosure(&node, &spec).interaction.on_activate.is_some(),
            "only the switch is disabled"
        );
    }

    #[test]
    fn repeated_records_get_instance_scoped_details_ids() {
        let first = ModelConnectionCardSpec::new("conn-a", "Same", "OpenAI").with_open(true);
        let second = ModelConnectionCardSpec::new("conn-b", "Same", "OpenAI").with_open(true);
        assert_ne!(first.details_id(), second.details_id());
        assert_ne!(first.disclosure_id(), second.disclosure_id());

        let node = model_connection_card(&first, &RenderContext::new(&theme()), ModelConnectionCardHandlers::default());
        assert!(node
            .find(&|n| n.id.as_deref() == Some(second.details_id().as_str()))
            .is_none());
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let scoped = |scope: &str| ModelConnectionCardHandlers {
            instance_id: Some(scope.to_string()),
            ..ModelConnectionCardHandlers::default()
        };
        let spec = spec().with_open(true);
        let first = model_connection_card(&spec, &RenderContext::new(&theme()), scoped("first"));
        let second = model_connection_card(&spec, &RenderContext::new(&theme()), scoped("second"));

        for (node, scope) in [(&first, "first"), (&second, "second")] {
            let expected = format!("model-connection-card:{scope}:{}", spec.disclosure_id());
            assert!(node
                .find(&|n| n.runtime_id.as_deref() == Some(expected.as_str()))
                .is_some());
        }
        let other = format!("model-connection-card:second:{}", spec.disclosure_id());
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(other.as_str()))
            .is_none());
        assert!(first
            .find(&|n| n.id.as_deref() == Some(spec.disclosure_id().as_str()))
            .is_some());
    }

    #[test]
    fn a_scoped_card_restores_focus_to_its_own_disclosure() {
        let focus = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&focus);
        let spec = spec().with_open(true);
        let node = model_connection_card(
            &spec,
            &RenderContext::new(&theme()),
            ModelConnectionCardHandlers {
                on_open_change: Some(Arc::new(|_| {})),
                on_focus_request: Some(Arc::new(move |id: &str| {
                    sink.lock().unwrap().push(id.to_string())
                })),
                instance_id: Some("second".to_string()),
                ..ModelConnectionCardHandlers::default()
            },
        );
        (disclosure(&node, &spec)
            .interaction
            .on_activate
            .as_ref()
            .expect("activation"))();
        assert_eq!(
            focus.lock().unwrap().as_slice(),
            [format!("model-connection-card:second:{}", spec.disclosure_id())]
        );
    }

    #[test]
    fn the_provider_mark_precedes_the_name_without_indenting_the_summary() {
        let node = model_connection_card_with_slots(
            &spec(),
            &RenderContext::new(&theme()),
            ModelConnectionCardSlots {
                leading: Some(Node::text("MARK")),
                ..ModelConnectionCardSlots::default()
            },
            ModelConnectionCardHandlers::default(),
        );
        let title_row = node
            .find(&|n| {
                n.children
                    .iter()
                    .any(|c| matches!(&c.kind, poodle_node::NodeKind::Text { content } if content == "MARK"))
            })
            .expect("title row");
        let texts: Vec<&str> = title_row.texts();
        assert_eq!(
            texts.first().copied(),
            Some("MARK"),
            "the mark is the first thing on the title line"
        );
        assert!(texts.contains(&"OpenAI · Work"));
        // The meta line is a sibling of the title row, not a child of it, so
        // it shares the identity's left edge.
        assert!(!texts.contains(&"Responses API · 2026-08"));
    }
}
