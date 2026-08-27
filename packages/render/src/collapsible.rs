//! Collapsible — a single disclosure region with heading, chevron, content.
//!
//! Contract: `docs/contracts/components/collapsible.md`
//! Ported from: `packages/jetstream/components/src/collapsible.rs`.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, Node, NodeRole, ShadowLayer,
};
use poodle_specs::{CollapsibleSpec, ControlDensity, ControlSize};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, size_font_rem};

pub const COLLAPSIBLE_TRIGGER_SEMANTIC_ID: &str = "poodle-collapsible-trigger";
pub const COLLAPSIBLE_CONTENT_SEMANTIC_ID: &str = "poodle-collapsible-content";

/// Host callbacks. Open-state reporting travels through
/// [`collapsible_with_handlers`].
#[derive(Default)]
pub struct CollapsibleHandlers {
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Stable native instance scope. Two same-titled instances must not share
    /// one backend focus handle. Identity never includes open state.
    pub instance_id: Option<String>,
}

fn scoped(instance_id: Option<&str>, part: &str) -> Option<String> {
    instance_id.map(|scope| format!("collapsible:{scope}:{part}"))
}

/// The backend-state id of the disclosure trigger.
pub fn collapsible_trigger_focus_id(instance_id: Option<&str>) -> String {
    scoped(instance_id, "trigger").unwrap_or_else(|| COLLAPSIBLE_TRIGGER_SEMANTIC_ID.to_string())
}

/// The backend-state id of the open content region.
pub fn collapsible_content_focus_id(instance_id: Option<&str>) -> String {
    scoped(instance_id, "content").unwrap_or_else(|| COLLAPSIBLE_CONTENT_SEMANTIC_ID.to_string())
}

pub fn collapsible(
    spec: &CollapsibleSpec,
    ctx: &RenderContext<'_>,
    content: Option<Node>,
    on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    collapsible_with_handlers(
        spec,
        ctx,
        content,
        CollapsibleHandlers {
            on_open_change,
            instance_id: None,
        },
    )
}

/// Build a collapsible node with open-change and optional instance scope.
pub fn collapsible_with_handlers(
    spec: &CollapsibleSpec,
    ctx: &RenderContext<'_>,
    content: Option<Node>,
    handlers: CollapsibleHandlers,
) -> Node {
    let theme = ctx.theme();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let is_open = spec.current_open();
    let trigger_id = collapsible_trigger_focus_id(handlers.instance_id.as_deref());
    let content_id = collapsible_content_focus_id(handlers.instance_id.as_deref());

    let open_gap = theme.resolve_space("space.stack.md");
    let root_gap = if is_open { open_gap } else { 0.0 };
    let trigger_gap = theme.resolve_space("space.inline.md");

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let title_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    });
    let desc_font = rem_to_px(size_font_rem(effective_size));
    let icon_size = rem_to_px(0.75);

    let elevated = theme.resolve_color("color.background.elevated");
    let panel = theme.resolve_color("color.background.panel");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let accent_base = theme.resolve_color(spec.highlight_accent_token());
    let radius = theme.resolve_radius("radius.surface");

    let root_bg = mix_srgb(elevated, panel, 0.40);
    let root_border = with_alpha(border_subtle, border_subtle.3 * spec.border_subtle_alpha());
    let highlight_border = with_alpha(accent_base, accent_base.3 * spec.highlight_border_alpha());
    let highlight_halo = with_alpha(accent_base, accent_base.3 * spec.highlight_halo_alpha());

    let pad_y = rem_to_px(0.625);
    let pad_x = rem_to_px(match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.0,
    });

    let mut outer = Node::container();
    {
        let s = &mut outer.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        s.min_width = Some(0.0);
        s.self_stretch = true;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.descriptor.background = Some(root_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = if spec.highlighted {
            highlight_border
        } else {
            root_border
        };
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        if spec.highlighted {
            s.shadow_layers = vec![ShadowLayer {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: rem_to_px(0.125),
                color: highlight_halo,
                inset: false,
            }];
        }
    }

    // Trigger: heading block + chevron.
    let mut heading = Node::container();
    {
        let s = &mut heading.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.min_width = Some(0.0);
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
    }
    if let Some(ref title) = spec.title {
        let mut t = Node::text(title);
        t.style.descriptor.text_color = Some(text_primary);
        t.style.text_size = Some(title_font);
        t.style.text_weight = Some(700);
        t.style.line_height = Some(1.2);
        heading = heading.child(t);
    }
    if let Some(ref description) = spec.description {
        let mut d = Node::text(description);
        d.style.descriptor.text_color = Some(text_secondary);
        d.style.text_size = Some(desc_font);
        d.style.line_height = Some(1.45);
        heading = heading.child(d);
    }

    let chevron_icon = if is_open {
        "chevron-down"
    } else {
        "chevron-right"
    };
    let mut indicator = Node::icon(chevron_icon, icon_size);
    indicator.style.flex_shrink_zero = true;
    indicator.style.descriptor.text_color = Some(text_secondary);

    let mut trigger = Node::button("");
    trigger.id = Some(COLLAPSIBLE_TRIGGER_SEMANTIC_ID.to_string());
    trigger.runtime_id = scoped(handlers.instance_id.as_deref(), "trigger");
    trigger.a11y.role = Some(NodeRole::Button);
    trigger.a11y.expanded = Some(is_open);
    trigger.a11y.controls = Some(content_id.clone());
    if let Some(ref title) = spec.title {
        trigger.a11y.label = Some(title.clone());
    } else if let Some(ref label) = spec.aria_label {
        trigger.a11y.label = Some(label.clone());
    }
    {
        let s = &mut trigger.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = trigger_gap;
        s.fill_width = true;
        s.descriptor.background = Some(crate::color::TRANSPARENT);
        s.descriptor.border.width = 0.0;
    }
    trigger = trigger.child(heading).child(indicator);

    if spec.is_disabled {
        trigger.style.descriptor.cursor = CursorHint::NotAllowed;
        trigger.interaction.disabled = true;
        trigger.interaction.focusable = false;
        trigger.a11y.tab_index = None;
        trigger.interaction.on_activate = None;
    } else {
        trigger.style.descriptor.cursor = CursorHint::Pointer;
        trigger.interaction.focusable = true;
        trigger.a11y.tab_index = Some(0);
        trigger.style.focus_ring = Some(FocusRing {
            color: theme.resolve_color("color.accent.focusRing"),
            width: theme.resolve_border_width("border.width.focus"),
            offset: rem_to_px(0.125),
        });
        if let Some(handler) = handlers.on_open_change {
            let next = !is_open;
            trigger.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
    }

    outer = outer.child(trigger);

    // Content region only when open.
    if is_open {
        if let Some(content_el) = content {
            let mut wrapper = Node::container();
            {
                let s = &mut wrapper.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.min_width = Some(0.0);
                s.self_stretch = true;
                s.descriptor.layout.spacing.padding.top = rem_to_px(0.125);
            }
            wrapper.id = Some(COLLAPSIBLE_CONTENT_SEMANTIC_ID.to_string());
            wrapper.runtime_id = scoped(handlers.instance_id.as_deref(), "content");
            wrapper.a11y.role = Some(NodeRole::Region);
            wrapper.a11y.labelled_by = Some(trigger_id);
            outer = outer.child(wrapper.child(content_el));
        }
    }

    // Disabled: whole-element opacity (contract §8 Root disabled).
    if spec.is_disabled {
        outer.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
    }

    outer
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn trigger<'a>(root: &'a Node) -> &'a Node {
        root.find(&|node| node.id.as_deref() == Some(COLLAPSIBLE_TRIGGER_SEMANTIC_ID))
            .expect("trigger")
    }

    fn content<'a>(root: &'a Node) -> Option<&'a Node> {
        root.find(&|node| node.id.as_deref() == Some(COLLAPSIBLE_CONTENT_SEMANTIC_ID))
    }

    #[test]
    fn default_open_projects_to_paint_and_expanded_state() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = CollapsibleSpec::new()
            .with_title("Advanced options")
            .with_default_open(true);
        let node = collapsible(
            &spec,
            &ctx,
            Some(Node::text("inside")),
            None,
        );
        assert!(content(&node).is_some(), "default-open paints content");
        let trigger = trigger(&node);
        assert_eq!(trigger.a11y.expanded, Some(true));
        assert_eq!(outer_has_region_role(&node), false);
    }

    #[test]
    fn controlled_open_wins_over_default_open_seed() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = CollapsibleSpec::new()
            .with_default_open(true)
            .with_open(false);
        let node = collapsible(&spec, &ctx, Some(Node::text("inside")), None);
        assert!(content(&node).is_none());
        assert_eq!(trigger(&node).a11y.expanded, Some(false));
    }

    #[test]
    fn activation_reports_the_next_open_value() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let reported = Arc::new(std::sync::Mutex::new(None::<bool>));
        let sink = Arc::clone(&reported);
        let spec = CollapsibleSpec::new().with_title("Section");
        let node = collapsible(
            &spec,
            &ctx,
            None,
            Some(Arc::new(move |next| {
                *sink.lock().unwrap() = Some(next);
            })),
        );
        trigger(&node)
            .interaction
            .on_activate
            .as_ref()
            .expect("handler")
            .as_ref()();
        assert_eq!(*reported.lock().unwrap(), Some(true));
    }

    #[test]
    fn disabled_suppresses_activation_and_focus() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = CollapsibleSpec::new()
            .with_title("Locked")
            .with_disabled(true);
        let node = collapsible(
            &spec,
            &ctx,
            None,
            Some(Arc::new(|_| panic!("disabled collapsible does not fire"))),
        );
        let trigger = trigger(&node);
        assert!(trigger.interaction.disabled);
        assert!(!trigger.interaction.focusable);
        assert!(trigger.interaction.on_activate.is_none());
        assert_eq!(trigger.a11y.tab_index, None);
        assert_eq!(trigger.style.descriptor.cursor, CursorHint::NotAllowed);
        assert!(
            node.style.descriptor.opacity > 0.0 && node.style.descriptor.opacity < 1.0,
            "root keeps disabled opacity"
        );
    }

    #[test]
    fn trigger_owns_button_semantics_and_content_owns_region() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = CollapsibleSpec::new()
            .with_title("Project settings")
            .with_open(true);
        let node = collapsible(&spec, &ctx, Some(Node::text("content")), None);
        let trigger = trigger(&node);
        assert_eq!(trigger.a11y.role, Some(NodeRole::Button));
        assert_eq!(trigger.a11y.label.as_deref(), Some("Project settings"));
        assert_eq!(
            trigger.a11y.controls.as_deref(),
            Some(COLLAPSIBLE_CONTENT_SEMANTIC_ID)
        );
        assert!(trigger.style.focus_ring.is_some());

        let region = content(&node).expect("open content");
        assert_eq!(region.a11y.role, Some(NodeRole::Region));
        assert_eq!(
            region.a11y.labelled_by.as_deref(),
            Some(COLLAPSIBLE_TRIGGER_SEMANTIC_ID)
        );
        assert_eq!(outer_has_region_role(&node), false);
    }

    #[test]
    fn aria_label_names_the_trigger_when_title_is_absent() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = CollapsibleSpec::new().with_aria_label("Hidden section");
        let node = collapsible(&spec, &ctx, None, None);
        assert_eq!(
            trigger(&node).a11y.label.as_deref(),
            Some("Hidden section")
        );
    }

    #[test]
    fn instance_scope_isolates_backend_state_ids() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let first = collapsible_with_handlers(
            &CollapsibleSpec::new().with_title("Same title"),
            &ctx,
            None,
            CollapsibleHandlers {
                instance_id: Some("first".to_string()),
                ..CollapsibleHandlers::default()
            },
        );
        let second = collapsible_with_handlers(
            &CollapsibleSpec::new().with_title("Same title"),
            &ctx,
            None,
            CollapsibleHandlers {
                instance_id: Some("second".to_string()),
                ..CollapsibleHandlers::default()
            },
        );
        let first_trigger = trigger(&first);
        let second_trigger = trigger(&second);
        assert_eq!(first_trigger.id, second_trigger.id);
        assert_ne!(first_trigger.runtime_id, second_trigger.runtime_id);
        assert_eq!(
            first_trigger.runtime_id.as_deref(),
            Some("collapsible:first:trigger")
        );
        assert_eq!(
            second_trigger.runtime_id.as_deref(),
            Some("collapsible:second:trigger")
        );
    }

    #[test]
    fn trigger_identity_does_not_include_open_state() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let shut = collapsible_with_handlers(
            &CollapsibleSpec::new().with_title("Section").with_open(false),
            &ctx,
            None,
            CollapsibleHandlers {
                instance_id: Some("scope".to_string()),
                ..CollapsibleHandlers::default()
            },
        );
        let open = collapsible_with_handlers(
            &CollapsibleSpec::new().with_title("Section").with_open(true),
            &ctx,
            None,
            CollapsibleHandlers {
                instance_id: Some("scope".to_string()),
                ..CollapsibleHandlers::default()
            },
        );
        let shut_trigger = trigger(&shut);
        let open_trigger = trigger(&open);
        assert_eq!(shut_trigger.runtime_id, open_trigger.runtime_id);
        assert_eq!(shut_trigger.a11y.expanded, Some(false));
        assert_eq!(open_trigger.a11y.expanded, Some(true));
    }

    fn outer_has_region_role(node: &Node) -> bool {
        matches!(&node.kind, NodeKind::Container { .. })
            && node.a11y.role == Some(NodeRole::Region)
    }
}
