//! DateTimeZonePicker — a trigger and a calendar + time + zone popover.
//!
//! Contract: `docs/contracts/components/date-time-zone-picker.md`
//! Ported from: `packages/jetstream/components/src/date_time_zone_picker.rs`.
//!
//! Same shell as the sibling pickers; the open surface stacks the composed
//! calendar over TIME and TIME ZONE fields (composed [`crate::time_field::time_field`] +
//! [`crate::time_zone_select::time_zone_select`]). The trigger folds the committed date / time /
//! zone into one space-joined string; partial values display whichever fields
//! are present.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::{CalendarSpec, DateTimeZonePickerSpec, TimeFieldSpec, TimeZoneSelectSpec};

use crate::calendar::{calendar, CalendarHandlers};
use crate::color::{mix_linear, with_alpha};
use crate::context::RenderContext;
use crate::picker_trigger::{picker_trigger, PickerTrigger};
use crate::presentation::rem_to_px;
use crate::time_field::time_field;
use crate::time_zone_select::{time_zone_select, TimeZoneSelectHandlers};

/// Host callbacks: the shared picker trio plus zone toggle/change forwarded
/// to the composed time-zone select.
///
/// `instance_id` is the lifetime-stable scope for the nested TimeZoneSelect.
pub struct DateTimeZonePickerHandlers {
    pub instance_id: String,
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_zone_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_zone_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl DateTimeZonePickerHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.trim().is_empty(),
            "DateTimeZonePickerHandlers requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id,
            on_toggle: None,
            on_select: None,
            on_navigate: None,
            on_zone_toggle: None,
            on_zone_change: None,
        }
    }
}

pub fn date_time_zone_picker(
    spec: &DateTimeZonePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DateTimeZonePickerHandlers,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let theme = ctx.theme();
    let inline_gap = theme.resolve_space("space.inline.sm");
    let elevated = theme.resolve_color(spec.overlay_fill_token());
    let border_color = theme.resolve_color(spec.border_token());
    let muted = theme.resolve_color("color.text.secondary");

    // ── Display text (contract §4) ──
    // Contract trigger anatomy is Value + Indicator only, so the committed
    // constituent fields (date / time / zone) are folded into one formatted
    // string. Partial values display whichever fields are present.
    let value = spec.current_value();
    let has_value = !value.is_empty();
    let display = if has_value {
        let mut parts: Vec<&str> = Vec::new();
        if let Some(ref date) = value.date {
            parts.push(date.as_str());
        }
        if let Some(ref time) = value.time {
            parts.push(time.as_str());
        }
        if let Some(ref tz) = value.time_zone {
            parts.push(tz.as_str());
        }
        parts.join(" ")
    } else {
        spec.placeholder.clone()
    };
    let trigger = picker_trigger(
        ctx,
        PickerTrigger {
            display: &display,
            has_value,
            open: spec.current_open(),
            disabled: spec.is_disabled,
            size: base_size,
            size_role: spec.size_role,
            indicator: "calendar",
            indicator_size: None,
            elevated,
            border_color,
            on_toggle: handlers.on_toggle.as_ref(),
        },
    );

    // ── Root wrapper: contract §7/§8 min-width 18rem ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        // Explicit Row (see switch.rs): closed = single trigger child.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_width = true;
        s.min_width = Some(rem_to_px(18.0));
    }
    let mut root = root.child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body → Calendar +
    //    Fields → Time field + Time-zone field). ──
    if spec.current_open() {
        // Composed Calendar (single), seeded from the structured value's date.
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on);
        if let Some(ref date) = value.date {
            cal_spec = cal_spec
                .with_value(date.clone())
                .with_visible_month(date.clone());
        }
        cal_spec.is_disabled = spec.is_disabled;

        // Composed TimeInput (TimeField), seeded from the structured value's time.
        let mut time_spec = TimeFieldSpec::new();
        time_spec.value = value.time.clone();
        time_spec.is_disabled = spec.is_disabled;

        // Composed TimeZoneSelect, seeded from the structured value's time_zone.
        let mut tz_spec = TimeZoneSelectSpec::new();
        tz_spec.value = value.time_zone.clone();
        tz_spec.is_disabled = spec.is_disabled;
        tz_spec.is_open = spec.zone_open;
        if !spec.time_zone_options.is_empty() {
            tz_spec.options = spec.time_zone_options.clone();
        }

        // Field Label — contract §8: label-family, 0.6875rem, weight 600,
        // uppercase, text-secondary (the string is pre-uppercased).
        let field_label = |text: &str, color| -> Node {
            let mut l = Node::text(text);
            l.style.descriptor.text_color = Some(color);
            l.style.text_size = Some(rem_to_px(0.6875));
            l.style.text_weight = Some(600);
            l
        };
        let field_group = |label: Node, control: Node| -> Node {
            let mut g = Node::container();
            {
                let s = &mut g.style;
                s.fill_width = true;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = rem_to_px(0.375); // contract Field gap
            }
            g.child(label).child(control)
        };

        // Time field — contract Field: "TIME" label above composed TimeInput.
        let time_field_group = field_group(field_label("Time", muted), time_field(&time_spec, ctx));

        // Time zone field — "TIME ZONE" label above composed TimeZoneSelect.
        let tz_field_group = field_group(
            field_label("Time zone", muted),
            time_zone_select(
                &tz_spec,
                ctx,
                TimeZoneSelectHandlers {
                    on_toggle: handlers.on_zone_toggle.clone(),
                    on_change: handlers.on_zone_change.clone(),
                    ..TimeZoneSelectHandlers::new(handlers.instance_id.clone())
                },
            ),
        );

        // Fields — vertical stack of Time + Time zone fields; gap 0.75rem.
        let mut fields = Node::container();
        {
            let s = &mut fields.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.75);
        }
        let fields = fields.child(time_field_group).child(tz_field_group);

        // Body — vertical stack of Calendar + Fields; gap 0.875rem.
        // Contract: the open picker surface is a `dialog` (stated on the body
        // in the reference tier — matched exactly).
        let mut body = Node::container();
        body.a11y.role = Some(NodeRole::Dialog);
        {
            let s = &mut body.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = rem_to_px(0.875);
        }
        let body = body
            .child(calendar(
                &cal_spec,
                ctx,
                CalendarHandlers {
                    on_select: handlers.on_select.clone(),
                    on_range_select: None,
                    on_navigate: handlers.on_navigate.clone(),
                },
            ))
            .child(fields);

        // Surface — established sibling overlay treatment: elevated 98% over
        // panel (linear lerp), border at 72% alpha, elevation-overlay shadow.
        let panel_bg = theme.resolve_color("color.background.panel");
        let surface_radius = theme.resolve_radius("radius.surface");
        let surface_border = with_alpha(border_color, border_color.3 * 0.72);
        let surface_bg = mix_linear(elevated, panel_bg, 0.98);

        let mut surface = Node::container();
        {
            let s = &mut surface.style;
            // Explicit Row (see switch.rs): one body child.
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.corner_radii.top_left = surface_radius;
            s.descriptor.corner_radii.top_right = surface_radius;
            s.descriptor.corner_radii.bottom_right = surface_radius;
            s.descriptor.corner_radii.bottom_left = surface_radius;
            s.descriptor.background = Some(surface_bg);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = surface_border;
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = theme.resolve_space("space.panel.y");
            pad.bottom = theme.resolve_space("space.panel.y");
            pad.left = theme.resolve_space("space.panel.x");
            pad.right = theme.resolve_space("space.panel.x");
        }
        let surface = surface.child(body);

        // Trigger + anchored-below surface stack (overlay anchoring is a
        // platform delta; rendered as a flow column with the contract gap).
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = inline_gap;
        root = root.child(surface);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
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

    #[test]
    #[should_panic(
        expected = "DateTimeZonePickerHandlers requires a non-empty lifetime-stable instance_id"
    )]
    fn empty_instance_scope_is_rejected() {
        let _ = DateTimeZonePickerHandlers::new("");
    }
}
