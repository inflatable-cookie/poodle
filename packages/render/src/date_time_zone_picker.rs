//! DateTimeZonePicker — a trigger and a calendar + time + zone popover.
//!
//! Contract: `docs/contracts/components/date-time-zone-picker.md`
//! Ported from: `packages/jetstream/components/src/date_time_zone_picker.rs`.
//!
//! Same shell as the sibling pickers; the open surface stacks the composed
//! calendar over TIME and TIME ZONE fields (composed [`crate::time_input::time_input`] +
//! [`crate::time_zone_select::time_zone_select`]). The trigger folds the committed date / time /
//! zone into one space-joined string; partial values display whichever fields
//! are present.

use std::{fmt, sync::Arc};

use poodle_node::{CrossAxisAlignment, DismissReason, LayoutDirection, Node, NodeRole};
use poodle_specs::{CalendarSpec, DateTimeZonePickerSpec, TimeInputSpec, TimeZoneSelectSpec};

use crate::calendar::{calendar_with_identity, CalendarHandlers};
use crate::color::{mix_linear, with_alpha};
use crate::context::RenderContext;
use crate::picker_trigger::{
    configure_picker_surface, configure_picker_trigger, picker_trigger, PickerTrigger,
};
use crate::presentation::rem_to_px;
use crate::time_input::time_input_with_value_change;
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

#[derive(Default)]
pub struct DateTimeZonePickerCallbacks {
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_dismiss: Option<Arc<dyn Fn(DismissReason) + Send + Sync>>,
    pub on_value_change: Option<Arc<dyn Fn(&poodle_specs::ZonedDateTimeValue) + Send + Sync>>,
}

impl fmt::Debug for DateTimeZonePickerCallbacks {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DateTimeZonePickerCallbacks")
            .field("on_open_change", &self.on_open_change.is_some())
            .field("on_dismiss", &self.on_dismiss.is_some())
            .field("on_value_change", &self.on_value_change.is_some())
            .finish()
    }
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
    date_time_zone_picker_with_callbacks(
        spec,
        ctx,
        handlers,
        DateTimeZonePickerCallbacks::default(),
    )
}

pub fn date_time_zone_picker_with_callbacks(
    spec: &DateTimeZonePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DateTimeZonePickerHandlers,
    callbacks: DateTimeZonePickerCallbacks,
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
    let value = spec.current_value().clone();
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
    let open = spec.current_open();
    let toggle = {
        let on_toggle = handlers.on_toggle.clone();
        let on_open_change = callbacks.on_open_change.clone();
        Arc::new(move || {
            if let Some(on_toggle) = &on_toggle {
                on_toggle();
            }
            if let Some(on_open_change) = &on_open_change {
                on_open_change(!open);
            }
        }) as Arc<dyn Fn() + Send + Sync>
    };
    let dismiss = {
        if callbacks.on_dismiss.is_none() && callbacks.on_open_change.is_none() {
            None
        } else {
            let on_dismiss = callbacks.on_dismiss.clone();
            let on_open_change = callbacks.on_open_change.clone();
            Some(Arc::new(move |reason| {
                if let Some(on_dismiss) = &on_dismiss {
                    on_dismiss(reason);
                }
                if let Some(on_open_change) = &on_open_change {
                    on_open_change(false);
                }
            }) as Arc<dyn Fn(DismissReason) + Send + Sync>)
        }
    };
    let mut trigger = picker_trigger(
        ctx,
        PickerTrigger {
            display: &display,
            has_value,
            open,
            disabled: spec.is_disabled,
            size: base_size,
            size_role: spec.size_role,
            indicator: "calendar",
            indicator_size: None,
            elevated,
            border_color,
            on_toggle: Some(&toggle),
        },
    );
    configure_picker_trigger(
        &mut trigger,
        &handlers.instance_id,
        spec.aria_label
            .as_deref()
            .filter(|label| !label.trim().is_empty())
            .unwrap_or(&display),
        open,
        dismiss.clone(),
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
    if open {
        // Composed Calendar (single), seeded from the structured value's date.
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on);
        if let Some(ref date) = value.date {
            cal_spec = cal_spec
                .with_value(date.clone())
                .with_visible_month(date.clone());
        }
        cal_spec.is_disabled = spec.is_disabled;

        // Composed TimeInput, seeded from the structured value's time.
        let mut time_spec = TimeInputSpec::new();
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
        let time_change = callbacks.on_value_change.clone().map(|on_change| {
            let base = value.clone();
            Arc::new(move |time: Option<String>| {
                let mut next = base.clone();
                next.time = time;
                on_change(&next);
            }) as Arc<dyn Fn(Option<String>) + Send + Sync>
        });
        let time_input_group = field_group(
            field_label("Time", muted),
            time_input_with_value_change(
                &time_spec,
                ctx,
                &format!("{}:time", handlers.instance_id),
                time_change,
            ),
        );

        // Time zone field — "TIME ZONE" label above composed TimeZoneSelect.
        let tz_field_group = field_group(
            field_label("Time zone", muted),
            time_zone_select(
                &tz_spec,
                ctx,
                TimeZoneSelectHandlers {
                    on_toggle: handlers.on_zone_toggle.clone(),
                    on_change: {
                        let legacy = handlers.on_zone_change.clone();
                        let on_value_change = callbacks.on_value_change.clone();
                        let base = value.clone();
                        Some(Arc::new(move |zone: &str| {
                            if let Some(legacy) = &legacy {
                                legacy(zone);
                            }
                            if let Some(on_value_change) = &on_value_change {
                                let mut next = base.clone();
                                next.time_zone = Some(zone.to_owned());
                                on_value_change(&next);
                            }
                        })
                            as Arc<dyn Fn(&str) + Send + Sync>)
                    },
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
        let fields = fields.child(time_input_group).child(tz_field_group);

        // Body — vertical stack of Calendar + Fields; gap 0.875rem.
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = rem_to_px(0.875);
        }
        let date_change = {
            let on_select = handlers.on_select.clone();
            let on_value_change = callbacks.on_value_change.clone();
            let base = value.clone();
            Arc::new(move |date: &str| {
                if let Some(on_select) = &on_select {
                    on_select(date);
                }
                if let Some(on_value_change) = &on_value_change {
                    let mut next = base.clone();
                    next.date = Some(date.to_owned());
                    on_value_change(&next);
                }
            }) as Arc<dyn Fn(&str) + Send + Sync>
        };
        let body = body
            .child(calendar_with_identity(
                &cal_spec,
                ctx,
                CalendarHandlers {
                    on_select: Some(date_change),
                    on_range_select: None,
                    on_navigate: handlers.on_navigate.clone(),
                },
                Some(format!("{}:calendar", handlers.instance_id)),
            ))
            .child(fields);

        // Surface — established sibling overlay treatment: elevated 98% over
        // panel (linear lerp), border at 72% alpha, elevation-overlay shadow.
        let panel_bg = theme.resolve_color("color.background.panel");
        let surface_radius = theme.resolve_radius("radius.surface");
        let surface_border = with_alpha(border_color, border_color.3 * 0.72);
        let surface_bg = mix_linear(elevated, panel_bg, 0.98);

        let mut surface = Node::container();
        surface.a11y.role = Some(NodeRole::Dialog);
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
        configure_picker_surface(&mut surface, &handlers.instance_id, open, dismiss);
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
