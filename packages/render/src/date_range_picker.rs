//! DateRangePicker — a trigger and a range-calendar popover.
//!
//! Contract: `docs/contracts/components/date-range-picker.md`
//! Ported from: `packages/jetstream/components/src/date_range_picker.rs`.
//!
//! Same shell as [`crate::date_picker`]; the composed calendar runs in range
//! mode and the trigger's display string mirrors Svelte `valueLabel`: a
//! complete range joins with an en-dash, a partial range renders
//! `"<start> – End date"`, and a missing start falls back to the placeholder.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole, StylePatch,
};
use poodle_specs::{CalendarMode, CalendarSpec, DateRangePickerSpec};

use crate::calendar::{calendar, CalendarHandlers};
use crate::color::{mix_linear, mix_srgb, with_alpha};
use crate::date_picker::DatePickerHandlers;
use crate::presentation::{
    date_picker_indicator_font_rem, rem_to_px, resolve_semantic_size, size_font_rem,
    size_height_offset_rem, size_padding_x_offset_rem,
};

pub fn date_range_picker(
    spec: &DateRangePickerSpec,
    theme: &dyn ThemeProvider,
    handlers: DatePickerHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = theme.resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = theme.resolve_space("space.inline.md")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let inline_gap = theme.resolve_space("space.inline.sm");
    // Disclosure chevron font-size — per-size indicator scale (contract §8),
    // shared with DatePicker. Distinct from the trigger value font.
    let indicator_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    let fill = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");
    let border_color = theme.resolve_color("color.border.default");
    let accent = theme.resolve_color("color.accent.base");
    let radius = theme.resolve_radius("radius.control");
    let text_color = theme.resolve_color("color.text.primary");
    let muted = theme.resolve_color("color.text.secondary");

    // Contract trigger hover: color-mix(surface 86%, elevated).
    let hover_bg = mix_srgb(fill, elevated, 0.14);

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
    let display_color = if has_start { text_color } else { muted };

    let mut trigger = Node::container();
    {
        let s = &mut trigger.style;
        s.fill_width = true;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = if spec.current_open() {
            accent
        } else {
            border_color
        };
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
        s.descriptor.layout.spacing.gap = inline_gap;
    }
    trigger.interaction.focusable = true;

    let mut value_label = Node::text(&display);
    value_label.style.descriptor.text_color = Some(display_color);
    value_label.style.text_size = Some(font_size);
    value_label.style.descriptor.layout.width = LayoutSizing::Grow;
    // Disclosure chevron (contract §2 Indicator; text-secondary; per-size font).
    let mut chevron = Node::icon("chevron-down", indicator_size);
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
    if spec.current_open() {
        let mut cal_spec = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_week_start(spec.week_starts_on.clone())
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
        let surface = surface.child(calendar(
            &cal_spec,
            theme,
            CalendarHandlers {
                on_select: handlers.on_select.clone(),
                on_range_select: None,
                on_navigate: handlers.on_navigate.clone(),
            },
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

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}
