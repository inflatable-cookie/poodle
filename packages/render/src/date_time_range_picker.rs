//! DateTimeRangePicker — a trigger and a range-calendar + paired-times popover.
//!
//! Contract: `docs/contracts/components/date-time-range-picker.md`
//! Ported from: `packages/jetstream/components/src/date_time_range_picker.rs`.
//!
//! Same shell as the sibling pickers; the open surface stacks a range-mode
//! calendar over a two-column START/END time row. Display text: each end
//! formats as "date time" / "date" / "time" / "…", ends joined by an en-dash;
//! empty falls back to the placeholder.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::{
    CalendarMode, CalendarSpec, DateRangeValue, DateTimeRangePickerSpec, TimeFieldSpec,
};

use crate::calendar::{calendar, CalendarHandlers};
use crate::color::{mix_linear, with_alpha};
use crate::date_picker::DatePickerHandlers;
use crate::picker_trigger::{picker_trigger, PickerTrigger};
use crate::presentation::rem_to_px;
use crate::time_field::time_field;

pub fn date_time_range_picker(
    spec: &DateTimeRangePickerSpec,
    theme: &dyn ThemeProvider,
    handlers: DatePickerHandlers,
) -> Node {
    let inline_gap = theme.resolve_space("space.inline.sm");
    let elevated = theme.resolve_color("color.background.elevated");
    let border_color = theme.resolve_color("color.border.default");
    let muted = theme.resolve_color("color.text.secondary");

    // ── Display text (contract §4) ──
    // Complete/partial range → "start – end"; empty → placeholder.
    let val = spec.current_value();
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
    let trigger = picker_trigger(
        theme,
        PickerTrigger {
            display: &display,
            has_value,
            open: spec.current_open(),
            disabled: spec.is_disabled,
            size: spec.size,
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
        s.min_width = Some(theme.resolve_space("size.dateTimeRangePicker.minWidth"));
    }
    let mut root = root.child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body →
    //    Calendar(range) + Times Row). ──
    if spec.current_open() {
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
        let time_section = |label: &str, time_val: Option<String>| -> Node {
            let mut time_spec = TimeFieldSpec::new();
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
            section.child(caption).child(time_field(&time_spec, theme))
        };

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
            .child(time_section("START TIME", val.start.time.clone()))
            .child(time_section("END TIME", val.end.time.clone()));

        // Body — vertical stack of range Calendar + Times Row; gap 0.875rem.
        let mut body = Node::container();
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
                theme,
                CalendarHandlers {
                    on_select: handlers.on_select.clone(),
                    on_range_select: None,
                    on_navigate: handlers.on_navigate.clone(),
                },
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

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}
