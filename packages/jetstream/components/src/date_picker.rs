//! DatePicker — Jetstream date picker backed by DatePickerSpec.
//!
//! Contract: `docs/contracts/components/date-picker.md`
//! Reference: `packages/svelte/components/src/DatePicker.svelte`
//!
//! ALL dimensions resolve from tokens or contract-exact rem (`rem_to_px`).
//! ZERO hardcoded pixel/color literals.
//!
//! Interaction model (mirrors the GPUI build): open/close, outside-click
//! dismissal, Escape, and calendar selection are bound by the preview event
//! loop, not the component. The component renders at the current spec state
//! (`current_open()` decides whether the calendar surface is composed) and
//! exposes interaction ids; the preview wires clicks.
//!
//! ARIA is N/A: the Jetstream runtime has no accessibility channel
//! (no `aria-haspopup`/`aria-expanded`/`role="dialog"`).

use jetstream_ui::{Color, color_mix};
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CalendarSpec, DatePickerSpec};

use crate::calendar::js_calendar;
use crate::presentation::{
    control_height_rem, control_space_x_rem, date_picker_indicator_font_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_opacity, resolve_radius};

pub fn js_date_picker(spec: &DatePickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");

    // ── Sizing (contract §7/§8) ──
    let height = rem_to_px(control_height_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let indicator_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    // Contract trigger hover: color-mix(surface 86%, elevated).
    let fill_c: Color = fill.into();
    let elevated_c: Color = elevated.into();
    let hover_bg = fill_c.mix_srgb(elevated_c, 0.14);

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
    let mut trigger = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .h(height)
        .pl(pad_x)
        .pr(pad_x)
        .flex_row()
        .items_center()
        .justify_between()
        .gap(rem_to_px(0.75)) // contract trigger gap
        .focusable()
        .child(
            ui_element::label(&display)
                .text_color(display_color)
                .text_size(font_size)
                .grow(),
        )
        // Disclosure chevron (contract §2; text-secondary; per-size font).
        .child(
            ui_element::icon("chevron-down")
                .w(indicator_size)
                .h(indicator_size)
                .text_color(muted),
        );

    if !spec.is_disabled {
        trigger = trigger.cursor_pointer().hover(|s| s.bg(hover_bg));
    }

    // ── Root wrapper: contract §7/§8 min-width 14rem ──
    let mut root = ui_element::div().min_w(rem_to_px(14.0)).child(trigger);

    // ── Calendar surface when open (contract §2 Surface + composed Calendar) ──
    if spec.current_open() {
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on.clone());
        if let Some(val) = spec.current_value() {
            cal_spec = cal_spec.with_value(val).with_visible_month(val);
        }

        let panel_bg = resolve_color(theme, "color.background.panel");
        let surface_radius = resolve_radius(theme, "radius.surface");
        // Surface border: color-mix(border-default 72%, transparent).
        let border_c: Color = border.into();
        let surface_border = Color {
            a: border_c.a * 0.72,
            ..border_c
        };
        // Surface background: color-mix(elevated 98%, panel).
        let surface_bg = color_mix(elevated.into(), panel_bg.into(), 0.98);

        // Token-accurate `elevation-overlay` from the typed semantic token via
        // the runtime shadow builder (single layer, spread 0; matches GPUI).
        let surface = elevation_overlay(
            ui_element::div()
                // Contract: the open picker surface is a `dialog`.
                .aria_role(jetstream_ui::accesskit::Role::Dialog)
                .rounded(surface_radius)
                .bg(surface_bg)
                .border(1.0)
                .border_color(surface_border),
        )
        .py(rem_to_px(panel_space_y_rem(spec.density)))
        .px(rem_to_px(panel_space_x_rem(spec.density)))
        .child(js_calendar(&cal_spec, theme));

        // Trigger + anchored-below surface stack (overlay anchoring is a
        // platform delta; rendered as a flow column with the contract gap).
        root = root.flex_col().gap(rem_to_px(0.375)).child(surface);
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn trigger_shows_placeholder_when_empty() {
        let spec = DatePickerSpec::new(); // placeholder = "Select date"
        let tree = probe(&js_date_picker(&spec, &theme()), 320.0, 360.0);
        assert!(
            tree.has_text("Select date"),
            "placeholder missing: {:?}",
            tree.texts()
        );
        // No trailing-ellipsis legacy placeholder.
        assert!(!tree.has_text("Select date..."));
    }

    #[test]
    fn trigger_shows_default_value_via_current_value() {
        // "With value" specimen passes default_value; trigger must show it,
        // not the placeholder (regression: component read spec.value directly).
        let spec = DatePickerSpec::new().with_default_value("2026-03-21");
        let tree = probe(&js_date_picker(&spec, &theme()), 320.0, 360.0);
        assert!(tree.has_text("2026-03-21"), "value missing: {:?}", tree.texts());
        assert!(!tree.has_text("Select date"));
    }

    #[test]
    fn indicator_is_chevron_icon_not_emoji() {
        let tree = probe(&js_date_picker(&DatePickerSpec::new(), &theme()), 320.0, 360.0);
        // Icon widget carries its registry name as text.
        assert!(
            tree.has_text("chevron-down"),
            "chevron indicator missing: {:?}",
            tree.texts()
        );
        assert!(!tree.has_text("📅"), "legacy emoji indicator still present");
        assert!(tree.count_kind("Icon") >= 1);
    }

    #[test]
    fn disabled_reduces_opacity_and_marks_disabled() {
        let mut spec = DatePickerSpec::new().with_default_value("2026-03-21");
        spec.is_disabled = true;
        let el = js_date_picker(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn closed_picker_has_no_calendar_surface() {
        let tree = probe(&js_date_picker(&DatePickerSpec::new(), &theme()), 320.0, 360.0);
        // Calendar emits weekday headers / month label; none when closed.
        assert!(!tree.has_text("March"), "calendar leaked while closed");
    }

    #[test]
    fn open_picker_composes_real_calendar_surface() {
        let spec = DatePickerSpec::new()
            .with_default_value("2026-03-21")
            .with_default_open(true);
        let tree = probe(&js_date_picker(&spec, &theme()), 360.0, 480.0);
        // Real Calendar primitive renders the month label + a weekday header.
        assert!(
            tree.has_text("March 2026") || tree.has_text("March"),
            "composed calendar month label missing: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("Mo") || tree.has_text("Su"), "weekday headers missing");
        // Trigger value still present alongside the surface.
        assert!(tree.has_text("2026-03-21"));
    }

    #[test]
    fn sizes_produce_different_trigger_heights() {
        let sm = probe(
            &js_date_picker(&DatePickerSpec::new().with_size(poodle_specs::ControlSize::Sm), &theme()),
            320.0,
            120.0,
        );
        let lg = probe(
            &js_date_picker(&DatePickerSpec::new().with_size(poodle_specs::ControlSize::Lg), &theme()),
            320.0,
            120.0,
        );
        // Trigger is the first child of root; compare its height.
        let sm_trigger_h = sm.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        let lg_trigger_h = lg.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        assert!(lg_trigger_h > sm_trigger_h, "sm {sm_trigger_h} !< lg {lg_trigger_h}");
    }
}
