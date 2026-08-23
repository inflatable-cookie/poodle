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

/// Host callbacks: `on_toggle` (trigger) and `on_change` (chosen zone id),
/// forwarded to the composed select.
#[derive(Default)]
pub struct TimeZoneSelectHandlers {
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
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
    let mut root = select(
        &select_spec,
        ctx,
        &SelectHandlers {
            toggle: handlers.on_toggle,
            change: handlers.on_change,
            clear: None,
        },
    );

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
