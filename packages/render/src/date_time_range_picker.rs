//! DateTimeRangePicker — a trigger and a range-calendar + paired-times popover.
//!
//! Contract: `docs/contracts/components/date-time-range-picker.md`
//! Ported from: `packages/jetstream/components/src/date_time_range_picker.rs`.
//!
//! Same shell as the sibling pickers; the open surface stacks a range-mode
//! calendar over a two-column START/END time row. Display text: each end
//! formats as "date time" / "date" / "time" / "…", ends joined by an en-dash;
//! empty falls back to the placeholder.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::{
    CalendarMode, CalendarSpec, DateRangeValue, DateTimeRangePickerSpec, TimeInputSpec,
};

use crate::calendar::{calendar_with_identity, CalendarHandlers};
use crate::color::{mix_linear, with_alpha};
use crate::context::RenderContext;
use crate::date_picker::{
    compose_handlers, picker_child_scope, picker_dismiss_handler, picker_toggle_handler,
    DatePickerCallbacks, DatePickerHandlers,
};
use crate::picker_trigger::{
    configure_picker_surface, configure_picker_trigger, picker_trigger, PickerTrigger,
};
use crate::presentation::rem_to_px;
use crate::time_input::time_input_with_value_change;

pub fn date_time_range_picker(
    spec: &DateTimeRangePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DatePickerHandlers,
) -> Node {
    date_time_range_picker_with_callbacks(spec, ctx, handlers, DatePickerCallbacks::default())
}

pub fn date_time_range_picker_with_callbacks(
    spec: &DateTimeRangePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DatePickerHandlers,
    callbacks: DatePickerCallbacks,
) -> Node {
    let handlers = compose_handlers(handlers, callbacks);
    let base_size = ctx.base_size(spec.size);
    let theme = ctx.theme();
    let inline_gap = theme.resolve_space("space.inline.sm");
    let elevated = theme.resolve_color("color.background.elevated");
    let border_color = theme.resolve_color("color.border.default");
    let muted = theme.resolve_color("color.text.secondary");

    // ── Display text (contract §4) ──
    // Complete/partial range → "start – end"; empty → placeholder.
    let val = spec.current_value().clone();
    let start_has = val.start.date.is_some() || val.start.time.is_some();
    let end_has = val.end.date.is_some() || val.end.time.is_some();
    let has_value = start_has || end_has;
    let display = if has_value {
        let fmt = |date: Option<&str>, time: Option<&str>| -> String {
            match (date, time) {
                (Some(d), Some(t)) => format!("{} {}", d, t),
                (Some(d), None) => d.to_string(),
                (None, Some(t)) => t.to_string(),
                (None, None) => "…".to_string(),
            }
        };
        let start_str = fmt(val.start.date.as_deref(), val.start.time.as_deref());
        let end_str = fmt(val.end.date.as_deref(), val.end.time.as_deref());
        format!("{} – {}", start_str, end_str)
    } else {
        spec.placeholder.clone()
    };
    let open = spec.current_open();
    let toggle = picker_toggle_handler(&handlers, open);
    let dismiss = picker_dismiss_handler(&handlers);
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
        s.min_width = Some(theme.resolve_space("size.dateTimeRangePicker.minWidth"));
    }
    let mut root = root.child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body →
    //    Calendar(range) + Times Row). ──
    if open {
        // Composed Calendar in range mode, seeded from the start/end dates.
        let mut cal_spec = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_week_start(spec.week_starts_on);
        cal_spec.range_value = Some(DateRangeValue::new(
            val.start.date.clone(),
            val.end.date.clone(),
        ));
        if let Some(ref start_date) = val.start.date {
            cal_spec = cal_spec.with_visible_month(start_date.clone());
        }
        cal_spec.is_disabled = spec.is_disabled;

        // A composed Time Section — contract Time Label + real time field.
        // Contract §8 Time Label: label-family, 0.6875rem, weight 600,
        // uppercase, text-secondary (the string is pre-uppercased).
        let time_section = |label: &str,
                            time_val: Option<String>,
                            field: &str,
                            on_change: Option<Arc<dyn Fn(Option<String>) + Send + Sync>>|
         -> Node {
            let mut time_spec = TimeInputSpec::new();
            time_spec.value = time_val;
            time_spec.is_disabled = spec.is_disabled;

            let mut section = Node::container();
            {
                let s = &mut section.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.flex_grow = Some(1.0);
                s.flex_basis = Some(0.0);
                s.descriptor.layout.spacing.gap = rem_to_px(0.375); // contract Time Section gap
            }
            let mut caption = Node::text(label);
            caption.style.descriptor.text_color = Some(muted);
            caption.style.text_size = Some(rem_to_px(0.6875));
            caption.style.text_weight = Some(600);
            section.child(caption).child(time_input_with_value_change(
                &time_spec,
                ctx,
                &picker_child_scope(&handlers.instance_id, field),
                on_change,
            ))
        };

        let start_time_change = handlers.on_date_time_range_change.clone().map(|on_change| {
            let base = val.clone();
            Arc::new(move |time: Option<String>| {
                let mut next = base.clone();
                next.start.time = time;
                on_change(&next);
            }) as Arc<dyn Fn(Option<String>) + Send + Sync>
        });
        let end_time_change = handlers.on_date_time_range_change.clone().map(|on_change| {
            let base = val.clone();
            Arc::new(move |time: Option<String>| {
                let mut next = base.clone();
                next.end.time = time;
                on_change(&next);
            }) as Arc<dyn Fn(Option<String>) + Send + Sync>
        });

        // Times Row — two equal columns for start/end; contract gap 0.75rem.
        let mut times_row = Node::container();
        {
            let s = &mut times_row.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = inline_gap;
        }
        let times_row = times_row
            .child(time_section(
                "START TIME",
                val.start.time.clone(),
                "start-time",
                start_time_change,
            ))
            .child(time_section(
                "END TIME",
                val.end.time.clone(),
                "end-time",
                end_time_change,
            ));

        // Body — vertical stack of range Calendar + Times Row; gap 0.875rem.
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = rem_to_px(0.875);
        }
        let range_change = handlers.on_date_time_range_change.clone().map(|on_change| {
            let base = val.clone();
            Arc::new(move |range: &DateRangeValue| {
                let mut next = base.clone();
                next.start.date = range.start.clone();
                next.end.date = range.end.clone();
                on_change(&next);
            }) as Arc<dyn Fn(&DateRangeValue) + Send + Sync>
        });
        let body = body
            .child(calendar_with_identity(
                &cal_spec,
                ctx,
                CalendarHandlers {
                    on_select: None,
                    on_range_select: range_change,
                    on_navigate: handlers.on_navigate.clone(),
                },
                (!handlers.instance_id.is_empty())
                    .then(|| format!("{}:calendar", handlers.instance_id)),
            ))
            .child(times_row);

        // Surface — established sibling overlay treatment: elevated 98% over
        // panel (linear lerp), border at 72% alpha, elevation-overlay shadow.
        let panel_bg = theme.resolve_color("color.background.panel");
        let surface_radius = theme.resolve_radius("radius.surface");
        let surface_border = with_alpha(border_color, border_color.3 * 0.72);
        let surface_bg = mix_linear(elevated, panel_bg, 0.98);

        let mut surface = Node::container();
        // Contract: the open picker surface is a `dialog`.
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
        root.style.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.xs");
        root = root.child(surface);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
    }

    root
}
