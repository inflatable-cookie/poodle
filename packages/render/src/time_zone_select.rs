//! TimeZoneSelect — a searchable list of zones.
//!
//! Contract: `docs/contracts/components/time-zone-select.md`
//! Ported from: `packages/jetstream/components/src/time_zone_select.rs`.
//!
//! Thin wrapper over `select`: the timezone options map into a `SelectSpec`
//! (via `spec.to_select_spec()`), so the trigger, search input, option list,
//! grouping, selected indicator, empty state and size/density all come from
//! the shared select implementation for free.

use std::sync::Arc;

use poodle_node::{LayoutSizing, Node, NodeKind};
use poodle_specs::TimeZoneSelectSpec;

use crate::context::RenderContext;
use crate::presentation::{rem_to_px, size_height_offset_rem, size_padding_x_offset_rem};
use crate::select::{select, SelectHandlers};

/// Host-owned native interaction for one TimeZoneSelect instance.
///
/// `instance_id` is the lifetime-stable scope. It is not a web public prop, and
/// the renderer never invents one from render order or selected value.
pub struct TimeZoneSelectHandlers {
    pub instance_id: String,
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl TimeZoneSelectHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.trim().is_empty(),
            "TimeZoneSelectHandlers requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id,
            on_toggle: None,
            on_change: None,
        }
    }
}

pub fn time_zone_select(
    spec: &TimeZoneSelectSpec,
    ctx: &RenderContext<'_>,
    handlers: TimeZoneSelectHandlers,
) -> Node {
    // Build the searchable `SelectSpec` exactly as the Svelte wrapper does
    // (searchable always on, timezone empty message, mapped option list,
    // placeholder + value + size/density forwarded) and delegate.
    let select_spec = spec.to_select_spec();
    let toggle = handlers.on_toggle;
    let change = handlers.on_change;
    let mut select_handlers = SelectHandlers::new(&handlers.instance_id);
    if toggle.is_some() || change.is_some() {
        select_handlers = select_handlers.on_transition(Arc::new(move |result| {
            for effect in &result.effects {
                match effect {
                    crate::SelectEffect::OpenChanged { .. } => {
                        if let Some(handler) = &toggle {
                            handler();
                        }
                    }
                    crate::SelectEffect::ValueChanged { value } => {
                        if let Some(handler) = &change {
                            handler(value);
                        }
                    }
                    crate::SelectEffect::QueryChanged { .. } => {}
                }
            }
        }));
    }
    let mut root = select(&select_spec, ctx, &select_handlers);

    // The standalone GPUI tier predates the generic Select's translucent
    // trigger recipe. Preserve its public TimeZoneSelect treatment while
    // retaining Select's option filtering and handler wiring.
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let height = ctx.theme().resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let pad_x = ctx.theme().resolve_space("space.inline.md")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let inline_gap = ctx.theme().resolve_space("space.inline.sm");
    let surface = ctx.theme().resolve_color("color.background.surface");
    let elevated = ctx.theme().resolve_color(spec.overlay_fill_token());
    let border = ctx.theme().resolve_color(spec.border_token());
    let accent = ctx.theme().resolve_color("color.accent.base");
    let icon_muted = ctx.theme().resolve_color("color.icon.muted");
    let radius = ctx.theme().resolve_radius("radius.control");
    let icon_size = ctx.theme().resolve_space("size.icon.sm");

    let tune_trigger = |trigger: &mut Node| {
        let s = &mut trigger.style;
        s.fill_width = true;
        s.descriptor.background = Some(surface);
        s.descriptor.border.color = if spec.is_open { accent } else { border };
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.gap = inline_gap;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        if let Some(icon) = trigger
            .children
            .iter_mut()
            .rev()
            .find(|child| matches!(child.kind, NodeKind::Icon { .. }))
        {
            icon.kind = NodeKind::Icon {
                name: if spec.is_open {
                    "chevron-up".to_string()
                } else {
                    "chevron-down".to_string()
                },
                size: icon_size,
            };
            icon.style.descriptor.text_color = Some(icon_muted);
        }
    };

    if spec.is_open {
        if let Some(trigger) = root.children.first_mut() {
            tune_trigger(trigger);
        }
        if let Some(panel) = root.children.get_mut(1) {
            let s = &mut panel.style;
            s.fill_width = true;
            s.descriptor.background = Some(elevated);
            s.descriptor.border.color = border;
            s.descriptor.corner_radii.top_left = radius;
            s.descriptor.corner_radii.top_right = radius;
            s.descriptor.corner_radii.bottom_right = radius;
            s.descriptor.corner_radii.bottom_left = radius;
        }
        root.style.fill_width = true;
    } else {
        tune_trigger(&mut root);
    }
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn two_time_zone_selects_do_not_share_runtime_ids() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TimeZoneSelectSpec::new();
        let left = time_zone_select(&spec, &ctx, TimeZoneSelectHandlers::new("zone-a"));
        let right = time_zone_select(&spec, &ctx, TimeZoneSelectHandlers::new("zone-b"));
        let mut tree = Node::container();
        tree = tree.child(left).child(right);
        assert!(tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:zone-a:trigger"))
            .is_some());
        assert!(tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:zone-b:trigger"))
            .is_some());
    }

    #[test]
    #[should_panic(
        expected = "TimeZoneSelectHandlers requires a non-empty lifetime-stable instance_id"
    )]
    fn empty_instance_scope_is_rejected() {
        let _ = TimeZoneSelectHandlers::new("");
    }
}
