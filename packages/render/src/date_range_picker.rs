//! DateRangePicker — a trigger and a range-calendar popover.
//!
//! Contract: `docs/contracts/components/date-range-picker.md`
//! Ported from: `packages/jetstream/components/src/date_range_picker.rs`.
//!
//! Same shell as [`crate::date_picker::date_picker`]; the composed calendar runs in range
//! mode and the trigger's display string mirrors Svelte `valueLabel`: a
//! complete range joins with an en-dash, a partial range renders
//! `"<start> – End date"`, and a missing start falls back to the placeholder.

use std::sync::Arc;

use poodle_node::{LayoutDirection, Node, NodeRole};
use poodle_specs::{CalendarMode, CalendarSpec, DateRangePickerSpec, DateRangeValue};

use crate::calendar::{calendar_with_identity, CalendarHandlers};
use crate::color::{mix_linear, with_alpha};
use crate::context::RenderContext;
use crate::date_picker::{
    compose_handlers, picker_dismiss_handler, picker_toggle_handler, DatePickerCallbacks,
    DatePickerHandlers,
};
use crate::picker_trigger::{
    configure_picker_surface, configure_picker_trigger, picker_trigger, PickerTrigger,
};
use crate::presentation::{date_picker_indicator_font_rem, rem_to_px};

pub fn date_range_picker(
    spec: &DateRangePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DatePickerHandlers,
) -> Node {
    date_range_picker_with_callbacks(spec, ctx, handlers, DatePickerCallbacks::default())
}

pub fn date_range_picker_with_callbacks(
    spec: &DateRangePickerSpec,
    ctx: &RenderContext<'_>,
    handlers: DatePickerHandlers,
    callbacks: DatePickerCallbacks,
) -> Node {
    let handlers = compose_handlers(handlers, callbacks);
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let base_size = ctx.base_size(spec.size);
    let theme = ctx.theme();
    // Disclosure chevron font-size — per-size indicator scale (contract §8),
    // shared with DatePicker. Distinct from the trigger value font.
    let indicator_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    let elevated = theme.resolve_color("color.background.elevated");
    let border_color = theme.resolve_color("color.border.default");
    let radius = theme.resolve_radius("radius.control");

    // ── Display text ──
    // Show range text only when a start exists; partial range renders
    // `"<start> – End date"`; a missing start falls back to placeholder.
    let range = spec.current_value();
    let display = match &range.start {
        Some(start) => match &range.end {
            Some(end) => format!("{start} – {end}"),
            None => format!("{start} – End date"),
        },
        None => spec.placeholder.clone(),
    };
    let has_start = range.start.is_some();
    let open = spec.current_open();
    let toggle = picker_toggle_handler(&handlers, open);
    let dismiss = picker_dismiss_handler(&handlers);
    let mut trigger = picker_trigger(
        ctx,
        PickerTrigger {
            display: &display,
            has_value: has_start,
            open,
            disabled: spec.is_disabled,
            size: base_size,
            size_role: spec.size_role,
            indicator: "chevron-down",
            indicator_size: Some(indicator_size),
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

    // ── Range-calendar surface when open (contract §2 Surface + composed
    //    Calendar mode="range"). The surface is the REAL calendar primitive. ──
    if open {
        let mut cal_spec = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_week_start(spec.week_starts_on)
            .with_default_range_value(range.clone());
        // Anchor the visible month to the range start when present.
        if let Some(ref start) = range.start {
            cal_spec = cal_spec.with_visible_month(start);
        }

        let panel_bg = theme.resolve_color("color.background.panel");
        let surface_radius = radius;
        // Surface border: color-mix(border-default 72%, transparent).
        let surface_border = with_alpha(border_color, border_color.3 * 0.72);
        // Surface background: color-mix(elevated 98%, panel) — linear lerp.
        let surface_bg = mix_linear(elevated, panel_bg, 0.98);

        let mut surface = Node::container();
        // Contract: the open picker surface is a `dialog`.
        surface.a11y.role = Some(NodeRole::Dialog);
        {
            let s = &mut surface.style;
            // Explicit Row (see switch.rs): one calendar child.
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.corner_radii.top_left = surface_radius;
            s.descriptor.corner_radii.top_right = surface_radius;
            s.descriptor.corner_radii.bottom_right = surface_radius;
            s.descriptor.corner_radii.bottom_left = surface_radius;
            s.descriptor.background = Some(surface_bg);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = surface_border;
            // Token-accurate elevation.overlay.
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        }
        configure_picker_surface(&mut surface, &handlers.instance_id, open, dismiss);
        let on_range_select = {
            let on_range_change = handlers.on_range_change.clone();
            let on_open_change = handlers.on_open_change.clone();
            Arc::new(move |range: &DateRangeValue| {
                if let Some(on_range_change) = &on_range_change {
                    on_range_change(range);
                }
                if range.end.is_some() {
                    if let Some(on_open_change) = &on_open_change {
                        on_open_change(false);
                    }
                }
            }) as Arc<dyn Fn(&DateRangeValue) + Send + Sync>
        };
        let surface = surface.child(calendar_with_identity(
            &cal_spec,
            ctx,
            CalendarHandlers {
                on_select: None,
                on_range_select: Some(on_range_select),
                on_navigate: handlers.on_navigate.clone(),
            },
            (!handlers.instance_id.is_empty())
                .then(|| format!("{}:calendar", handlers.instance_id)),
        ));

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
