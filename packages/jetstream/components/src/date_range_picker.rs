//! DateRangePicker — Jetstream date-range picker backed by DateRangePickerSpec.
//!
//! Contract: `docs/contracts/components/date-range-picker.md`
//! Reference: `packages/svelte/components/src/DateRangePicker.svelte`
//!
//! ALL dimensions resolve from tokens or contract-exact rem (`rem_to_px`).
//! ZERO hardcoded pixel/color literals.
//!
//! Renders the trigger (range/placeholder value + disclosure chevron) and,
//! when open (`current_open()`), the real composed `Calendar` in range mode
//! (per CLAUDE.md "No Mockups" — the surface is the actual Calendar primitive,
//! never a faked grid).
//!
//! Interaction model (mirrors the GPUI + DatePicker build): open/close,
//! outside-click dismissal, Escape, and range selection are bound by the
//! preview event loop, not the component. The component renders at the current
//! spec state and exposes interaction ids; the preview wires clicks.
//!
//! ARIA is N/A: the Jetstream runtime has no accessibility channel
//! (no `aria-haspopup`/`aria-expanded`/`role="dialog"`).

use jetstream_ui::{Color, color_mix};
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CalendarMode, CalendarSpec, DateRangePickerSpec};

use crate::calendar::js_calendar;
use crate::presentation::{
    control_height_rem, control_space_x_rem, date_picker_indicator_font_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_opacity, resolve_radius};

pub fn js_date_range_picker(spec: &DateRangePickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Disclosure chevron font-size — per-size indicator scale (contract §8),
    // shared with DatePicker. Distinct from the trigger value font.
    let indicator_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    let fill = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border_color = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");

    // Contract trigger hover: color-mix(surface 86%, elevated).
    let fill_c: Color = fill.into();
    let elevated_c: Color = elevated.into();
    let hover_bg = fill_c.mix_srgb(elevated_c, 0.14);

    // ── Display text ──
    // Mirror Svelte `valueLabel`: show range text only when a start exists;
    // partial range (start chosen, end pending) renders `"<start> – End date"`
    // (en-dash + literal "End date"); a missing start falls back to placeholder.
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

    let mut trigger = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
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
        // Disclosure chevron (contract §2 Indicator; text-secondary; per-size font).
        .child(
            ui_element::icon("chevron-down")
                .w(indicator_size)
                .h(indicator_size)
                .text_color(muted),
        );

    if !spec.is_disabled {
        trigger = trigger.cursor_pointer().hover(|s| s.bg(hover_bg));
    }

    // ── Root wrapper: contract §7/§8 min-width 16rem ──
    let mut root = ui_element::div().min_w(rem_to_px(16.0)).child(trigger);

    // ── Range-calendar surface when open (contract §2 Surface + composed
    //    Calendar mode="range"). The surface is the REAL Calendar primitive. ──
    if spec.current_open() {
        let mut cal_spec = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_week_start(spec.week_starts_on.clone())
            .with_default_range_value(range.clone());
        // Anchor the visible month to the range start when present.
        if let Some(ref start) = range.start {
            cal_spec = cal_spec.with_visible_month(start);
        }

        let panel_bg = resolve_color(theme, "color.background.panel");
        let surface_radius = resolve_radius(theme, "radius.surface");
        // Surface border: color-mix(border-default 72%, transparent).
        let border_c: Color = border_color.into();
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

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ControlSize, DateRangeValue};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn trigger_shows_placeholder_when_empty() {
        let spec = DateRangePickerSpec::new(); // placeholder = "Select date range"
        let tree = probe(&js_date_range_picker(&spec, &theme()), 360.0, 80.0);
        assert!(
            tree.has_text("Select date range"),
            "placeholder missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn trigger_shows_complete_range_with_en_dash() {
        let spec = DateRangePickerSpec::new().with_default_value(DateRangeValue::new(
            Some("2026-03-01".into()),
            Some("2026-03-14".into()),
        ));
        let tree = probe(&js_date_range_picker(&spec, &theme()), 360.0, 80.0);
        assert!(
            tree.has_text("2026-03-01 – 2026-03-14"),
            "complete range string missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn partial_range_shows_end_date_literal_not_ellipsis() {
        // Start chosen, end pending → "<start> – End date" (Svelte parity).
        // Regression: the old build joined with " - ..." (ASCII hyphen + dots).
        let spec = DateRangePickerSpec::new()
            .with_default_value(DateRangeValue::new(Some("2026-03-01".into()), None));
        let tree = probe(&js_date_range_picker(&spec, &theme()), 360.0, 80.0);
        assert!(
            tree.has_text("2026-03-01 – End date"),
            "partial-range 'End date' string missing: {:?}",
            tree.texts()
        );
        assert!(
            !tree.has_text("2026-03-01 - ..."),
            "legacy ellipsis partial-range string still present"
        );
    }

    #[test]
    fn indicator_is_chevron_icon() {
        let tree = probe(&js_date_range_picker(&DateRangePickerSpec::new(), &theme()), 360.0, 80.0);
        // Icon widget carries its registry name as text.
        assert!(
            tree.has_text("chevron-down"),
            "chevron indicator missing: {:?}",
            tree.texts()
        );
        assert!(tree.count_kind("Icon") >= 1);
    }

    #[test]
    fn closed_picker_has_no_calendar_surface() {
        let tree = probe(&js_date_range_picker(&DateRangePickerSpec::new(), &theme()), 360.0, 80.0);
        // Calendar emits weekday headers / month label; none when closed.
        assert!(!tree.has_text("March"), "calendar leaked while closed");
        assert!(!tree.has_text("Mo"), "weekday header leaked while closed");
    }

    #[test]
    fn open_picker_composes_real_range_calendar() {
        // Per "No Mockups": the open surface must be the REAL Calendar (range
        // mode), not a faked grid. With a range seeded, the calendar paints its
        // month label + weekday headers + accent range endpoints.
        let mut spec = DateRangePickerSpec::new().with_default_value(DateRangeValue::new(
            Some("2026-03-01".into()),
            Some("2026-03-14".into()),
        ));
        spec.default_open = true;
        let el = js_date_range_picker(&spec, &theme());
        let tree = probe(&el, 380.0, 520.0);
        assert!(
            tree.has_text("March"),
            "composed range-calendar month label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Mo") || tree.has_text("Su"),
            "composed calendar weekday headers missing"
        );

        // Endpoints carry the accent fill — proves real range-mode Calendar,
        // not a placeholder. Day-1 and day-14 cells exist with accent bg.
        let accent: Color = resolve_color(&theme(), "color.accent.base").into();
        fn find<'a>(el: &'a JsEl, pred: &dyn Fn(&JsEl) -> bool) -> Option<&'a JsEl> {
            if pred(el) {
                return Some(el);
            }
            el.children.iter().find_map(|c| find(c, pred))
        }
        let start = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-1"))
            .expect("range start cell present in composed calendar");
        assert_eq!(start.style.background, Some(accent), "range start = accent fill");
    }

    #[test]
    fn disabled_reduces_opacity_and_marks_disabled() {
        let mut spec = DateRangePickerSpec::new();
        spec.is_disabled = true;
        let el = js_date_range_picker(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn sizes_produce_different_trigger_heights() {
        let sm = probe(
            &js_date_range_picker(&DateRangePickerSpec::new().with_size(ControlSize::Sm), &theme()),
            360.0,
            120.0,
        );
        let lg = probe(
            &js_date_range_picker(&DateRangePickerSpec::new().with_size(ControlSize::Lg), &theme()),
            360.0,
            120.0,
        );
        // Trigger is the first child of root; compare its height.
        let sm_trigger_h = sm.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        let lg_trigger_h = lg.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        assert!(lg_trigger_h > sm_trigger_h, "sm {sm_trigger_h} !< lg {lg_trigger_h}");
    }
}
