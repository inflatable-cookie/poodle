//! DatePicker — a trigger and a calendar popover.
//!
//! Contract: `docs/contracts/components/date-picker.md`
//! Ported from: `packages/jetstream/components/src/date_picker.rs`.
//!
//! Open/close, outside-click dismissal, Escape and calendar selection are
//! host-owned; the component renders at the current spec state
//! (`current_open()` decides whether the calendar surface is composed). The
//! calendar is composed rather than reimplemented, so `on_select` /
//! `on_navigate` forward to it — a day pressed in the popover is the same
//! event `calendar` already raises.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole, StylePatch,
};
use poodle_specs::{CalendarSpec, DatePickerSpec};

use crate::calendar::{calendar, CalendarHandlers};
use crate::color::{mix_linear, mix_srgb, with_alpha};
use crate::presentation::{
    control_height_rem, control_space_x_rem, date_picker_indicator_font_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};

/// Host callbacks: `on_toggle` (trigger pressed; the spec owns open state),
/// `on_select` (ISO day) and `on_navigate` ("prev"/"next"), the latter two
/// forwarded to the composed calendar.
#[derive(Default)]
pub struct DatePickerHandlers {
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn date_picker(
    spec: &DatePickerSpec,
    theme: &dyn ThemeProvider,
    handlers: DatePickerHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");
    let border = theme.resolve_color("color.border.default");
    let radius = theme.resolve_radius("radius.control");
    let text_color = theme.resolve_color("color.text.primary");
    let muted = theme.resolve_color("color.text.secondary");

    // ── Sizing (contract §7/§8) ──
    let height = rem_to_px(control_height_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let indicator_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    // Contract trigger hover: color-mix(surface 86%, elevated) — the old
    // tier's sRGB mix with fill weighted 0.14.
    let hover_bg = mix_srgb(fill, elevated, 0.14);

    // ── Display text: current_value() (honors default_value); placeholder otherwise ──
    let display = spec
        .current_value()
        .map(|v| v.to_string())
        .unwrap_or_else(|| spec.placeholder.clone());
    let display_color = if spec.current_value().is_some() {
        text_color
    } else {
        muted
    };

    // ── Trigger (contract anatomy: value/placeholder + chevron indicator) ──
    let mut trigger = Node::container();
    {
        let s = &mut trigger.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
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
    // Disclosure chevron (contract §2; text-secondary; per-size font).
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

    // ── Root wrapper: contract §7/§8 min-width 14rem ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        // Closed: single trigger child in the old tier's default Row.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.min_width = Some(rem_to_px(14.0));
    }
    let mut root = root.child(trigger);

    // ── Calendar surface when open (contract §2 Surface + composed Calendar) ──
    if spec.current_open() {
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on.clone());
        if let Some(val) = spec.current_value() {
            cal_spec = cal_spec.with_value(val).with_visible_month(val);
        }

        let panel_bg = theme.resolve_color("color.background.panel");
        let surface_radius = theme.resolve_radius("radius.surface");
        // Surface border: color-mix(border-default 72%, transparent).
        let surface_border = with_alpha(border, border.3 * 0.72);
        // Surface background: color-mix(elevated 98%, panel) — linear lerp.
        let surface_bg = mix_linear(elevated, panel_bg, 0.98);

        let mut surface = Node::container();
        // Contract: the open picker surface is a `dialog`.
        surface.a11y.role = Some(NodeRole::Dialog);
        {
            let s = &mut surface.style;
            // Explicit Row (see switch.rs): one calendar child in old default.
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
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = rem_to_px(panel_space_y_rem(spec.density));
            pad.bottom = rem_to_px(panel_space_y_rem(spec.density));
            pad.left = rem_to_px(panel_space_x_rem(spec.density));
            pad.right = rem_to_px(panel_space_x_rem(spec.density));
        }
        let surface = surface.child(calendar(
            &cal_spec,
            theme,
            CalendarHandlers {
                on_select: handlers.on_select.clone(),
                on_navigate: handlers.on_navigate.clone(),
            },
        ));

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
