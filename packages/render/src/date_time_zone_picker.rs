//! DateTimeZonePicker — a trigger and a calendar + time + zone popover.
//!
//! Contract: `docs/contracts/components/date-time-zone-picker.md`
//! Ported from: `packages/jetstream/components/src/date_time_zone_picker.rs`.
//!
//! Same shell as the sibling pickers; the open surface stacks the composed
//! calendar over TIME and TIME ZONE fields (composed [`crate::time_field`] +
//! [`crate::time_zone_select`]). The trigger folds the committed date / time /
//! zone into one space-joined string; partial values display whichever fields
//! are present.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, StylePatch,
};
use poodle_specs::{CalendarSpec, DateTimeZonePickerSpec, TimeFieldSpec, TimeZoneSelectSpec};

use crate::calendar::{calendar, CalendarHandlers};
use crate::color::{mix_linear, mix_srgb, with_alpha};
use crate::presentation::{
    control_height_rem, control_space_x_rem, date_picker_indicator_font_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::time_field::time_field;
use crate::time_zone_select::{time_zone_select, TimeZoneSelectHandlers};

/// Host callbacks: the shared picker trio plus zone toggle/change forwarded
/// to the composed time-zone select.
#[derive(Default)]
pub struct DateTimeZonePickerHandlers {
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_zone_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_zone_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn date_time_zone_picker(
    spec: &DateTimeZonePickerSpec,
    theme: &dyn ThemeProvider,
    handlers: DateTimeZonePickerHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract §8 indicator font-size per size — shared ladder with the
    // sibling date/time pickers.
    let icon_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    let fill = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color(spec.overlay_fill_token());
    let border_color = theme.resolve_color(spec.border_token());
    let radius = theme.resolve_radius("radius.control");
    let text_color = theme.resolve_color("color.text.primary");
    let muted = theme.resolve_color("color.text.secondary");

    // Hover: color-mix(surface 86%, elevated).
    let hover_bg = mix_srgb(fill, elevated, 0.14);

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
    let display_color = if has_value { text_color } else { muted };

    let mut trigger = Node::container();
    {
        let s = &mut trigger.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_color;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = rem_to_px(0.75); // contract trigger gap
    }
    trigger.interaction.focusable = true;

    let mut value_label = Node::text(&display);
    value_label.style.descriptor.text_color = Some(display_color);
    value_label.style.text_size = Some(font_size);
    value_label.style.descriptor.layout.width = LayoutSizing::Grow;
    // Disclosure chevron (contract §2 Indicator; text-secondary, per-size).
    let mut chevron = Node::icon("chevron-down", icon_size);
    chevron.style.descriptor.text_color = Some(muted);
    let mut trigger = trigger.child(value_label).child(chevron);

    if !spec.is_disabled {
        trigger.style.descriptor.cursor = CursorHint::Pointer;
        trigger.style.hover = Some(StylePatch {
            background: Some(hover_bg),
            border_color: None,
            text_color: None,
            opacity: None,
        });

        if let Some(handler) = &handlers.on_toggle {
            let handler = Arc::clone(handler);
            trigger.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    // ── Root wrapper: contract §7/§8 min-width 18rem ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        // Explicit Row (see switch.rs): closed = single trigger child.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.min_width = Some(rem_to_px(18.0));
    }
    let mut root = root.child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body → Calendar +
    //    Fields → Time field + Time-zone field). ──
    if spec.current_open() {
        // Composed Calendar (single), seeded from the structured value's date.
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on.clone());
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
        let field_label = |text: &str, color: ColorValue| -> Node {
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
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = rem_to_px(0.375); // contract Field gap
            }
            g.child(label).child(control)
        };

        // Time field — contract Field: "TIME" label above composed TimeInput.
        let time_field_group =
            field_group(field_label("TIME", muted), time_field(&time_spec, theme));

        // Time zone field — "TIME ZONE" label above composed TimeZoneSelect.
        let tz_field_group = field_group(
            field_label("TIME ZONE", muted),
            time_zone_select(
                &tz_spec,
                theme,
                TimeZoneSelectHandlers {
                    on_toggle: handlers.on_zone_toggle.clone(),
                    on_change: handlers.on_zone_change.clone(),
                },
            ),
        );

        // Fields — vertical stack of Time + Time zone fields; gap 0.75rem.
        let mut fields = Node::container();
        {
            let s = &mut fields.style;
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
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.875);
        }
        let body = body
            .child(calendar(
                &cal_spec,
                theme,
                CalendarHandlers {
                    on_select: handlers.on_select.clone(),
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
            pad.top = rem_to_px(panel_space_y_rem(spec.density));
            pad.bottom = rem_to_px(panel_space_y_rem(spec.density));
            pad.left = rem_to_px(panel_space_x_rem(spec.density));
            pad.right = rem_to_px(panel_space_x_rem(spec.density));
        }
        let surface = surface.child(body);

        // Trigger + anchored-below surface stack (overlay anchoring is a
        // platform delta; rendered as a flow column with the contract gap).
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = rem_to_px(0.375);
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
