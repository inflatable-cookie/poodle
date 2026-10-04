//! DateTimePicker — a trigger and a calendar + time popover.
//!
//! Contract: `docs/contracts/components/date-time-picker.md`
//! Ported from: `packages/jetstream/components/src/date_time_picker.rs`.
//!
//! Same shell as [`crate::date_picker::date_picker`]; the open surface stacks the composed
//! calendar over a labelled time section (composed [`crate::time_input::time_input`]).
//! Display text (contract §4): complete value → "date time"; partial → the
//! prompt for the missing part; empty → placeholder.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::{CalendarSpec, DateTimePickerSpec, TimeInputSpec};

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

pub fn date_time_picker(
    spec: &DateTimePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DatePickerHandlers,
) -> Node {
    date_time_picker_with_callbacks(spec, ctx, handlers, DatePickerCallbacks::default())
}

pub fn date_time_picker_with_callbacks(
    spec: &DateTimePickerSpec,
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
    let val = spec.current_value().clone();
    let has_value = val.date.is_some() || val.time.is_some();
    let display = match (val.date.as_deref(), val.time.as_deref()) {
        (Some(d), Some(t)) => format!("{} {}", d, t),
        (Some(d), None) => format!("{} Select time", d),
        (None, Some(t)) => format!("Select date {}", t),
        (None, None) => spec.placeholder.clone(),
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
            indicator: "chevron-down",
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

    // ── Root wrapper: contract §7/§8 min-width 16rem ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        // Explicit Row (see switch.rs): closed = single trigger child.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_width = true;
    }
    let mut root = root.child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body → Calendar +
    //    Time Section). Composes the real calendar + time_input primitives. ──
    if open {
        // Composed Calendar (single), seeded from the picker's date.
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on);
        if let Some(ref date) = val.date {
            cal_spec = cal_spec
                .with_value(date.clone())
                .with_visible_month(date.clone());
        }
        cal_spec.is_disabled = spec.is_disabled;

        // Composed TimeInput, seeded from the picker's time.
        let mut time_spec = TimeInputSpec::new();
        time_spec.value = val.time.clone();
        time_spec.is_disabled = spec.is_disabled;

        // Contract §8 Time Label: label-family, 0.6875rem, weight 600,
        // uppercase, text-secondary (the string is pre-uppercased).
        let mut time_label = Node::text("TIME");
        time_label.style.descriptor.text_color = Some(muted);
        time_label.style.text_size = Some(rem_to_px(0.6875));
        time_label.style.text_weight = Some(600);

        // Time Section — label above the composed time field; gap 0.375rem.
        let mut time_section = Node::container();
        {
            let s = &mut time_section.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.375);
        }
        let time_value = val.clone();
        let time_change = handlers.on_date_time_change.clone().map(|on_change| {
            Arc::new(move |time: Option<String>| {
                let mut next = time_value.clone();
                next.time = time;
                on_change(&next);
            }) as Arc<dyn Fn(Option<String>) + Send + Sync>
        });
        let time_section = time_section
            .child(time_label)
            .child(time_input_with_value_change(
                &time_spec,
                ctx,
                &picker_child_scope(&handlers.instance_id, "time"),
                time_change,
            ));

        // Body — vertical stack of Calendar + Time Section; gap 0.875rem.
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.fill_width = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = rem_to_px(0.875);
        }
        let date_change = {
            let on_change = handlers.on_date_time_change.clone();
            let on_select = handlers.on_select.clone();
            let date_value = val.clone();
            Arc::new(move |date: &str| {
                if let Some(on_select) = &on_select {
                    on_select(date);
                }
                if let Some(on_change) = &on_change {
                    let mut next = date_value.clone();
                    next.date = Some(date.to_owned());
                    on_change(&next);
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
                (!handlers.instance_id.is_empty())
                    .then(|| format!("{}:calendar", handlers.instance_id)),
            ))
            .child(time_section);

        // Surface — established sibling overlay treatment (date_picker.rs):
        // elevated 98% over panel (linear lerp), border at 72% alpha,
        // elevation-overlay shadow.
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
        root.style.descriptor.layout.spacing.gap = inline_gap;
        root = root.child(surface);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
    }

    root
}
