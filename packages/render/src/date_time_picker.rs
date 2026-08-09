//! DateTimePicker — a trigger and a calendar + time popover.
//!
//! Contract: `docs/contracts/components/date-time-picker.md`
//! Ported from: `packages/jetstream/components/src/date_time_picker.rs`.
//!
//! Same shell as [`crate::date_picker`]; the open surface stacks the composed
//! calendar over a labelled time section (composed [`crate::time_field`]).
//! Display text (contract §4): complete value → "date time"; partial → the
//! prompt for the missing part; empty → placeholder.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::{CalendarSpec, DateTimePickerSpec, TimeFieldSpec};

use crate::calendar::{calendar, CalendarHandlers};
use crate::color::{mix_linear, with_alpha};
use crate::date_picker::DatePickerHandlers;
use crate::picker_trigger::{picker_trigger, PickerTrigger};
use crate::presentation::rem_to_px;
use crate::time_field::time_field;

pub fn date_time_picker(
    spec: &DateTimePickerSpec,
    theme: &dyn ThemeProvider,
    handlers: DatePickerHandlers,
) -> Node {
    let inline_gap = theme.resolve_space("space.inline.sm");
    let elevated = theme.resolve_color("color.background.elevated");
    let border_color = theme.resolve_color("color.border.default");
    let muted = theme.resolve_color("color.text.secondary");

    // ── Display text (contract §4) ──
    let val = spec.current_value();
    let has_value = val.date.is_some() || val.time.is_some();
    let display = match (val.date.as_deref(), val.time.as_deref()) {
        (Some(d), Some(t)) => format!("{} {}", d, t),
        (Some(d), None) => format!("{} Select time", d),
        (None, Some(t)) => format!("Select date {}", t),
        (None, None) => spec.placeholder.clone(),
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
            indicator: "chevron-down",
            indicator_size: None,
            elevated,
            border_color,
            on_toggle: handlers.on_toggle.as_ref(),
        },
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
    //    Time Section). Composes the real calendar + time_field primitives. ──
    if spec.current_open() {
        // Composed Calendar (single), seeded from the picker's date.
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on);
        if let Some(ref date) = val.date {
            cal_spec = cal_spec
                .with_value(date.clone())
                .with_visible_month(date.clone());
        }
        cal_spec.is_disabled = spec.is_disabled;

        // Composed TimeInput (TimeField), seeded from the picker's time.
        let mut time_spec = TimeFieldSpec::new();
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
        let time_section = time_section
            .child(time_label)
            .child(time_field(&time_spec, theme));

        // Body — vertical stack of Calendar + Time Section; gap 0.875rem.
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
