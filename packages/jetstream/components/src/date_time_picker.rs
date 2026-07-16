//! DateTimePicker — Jetstream date+time picker backed by DateTimePickerSpec.
//!
//! Contract: `docs/contracts/components/date-time-picker.md`
//! Reference: `packages/svelte/components/src/DateTimePicker.svelte`
//!            `packages/gpui/components/src/primitives/date_time_picker.rs`
//!
//! Renders the trigger button (value/placeholder + disclosure chevron) and,
//! when open, the overlay surface composing the REAL Calendar primitive plus a
//! Time Section (label + composed TimeInput) — no mockup (per CLAUDE.md).
//!
//! ALL dimensions resolve from tokens or contract-exact rem (`rem_to_px`).
//! ZERO hardcoded pixel/color literals.
//!
//! Interaction model (mirrors the GPUI / DatePicker builds): open/close,
//! outside-click dismissal, Escape, and calendar/time selection are bound by
//! the preview event loop, not the component. The component renders at the
//! current spec state (`current_open()` decides whether the surface is
//! composed) and exposes interaction ids; the preview wires clicks.
//!
//! ARIA is N/A: the Jetstream runtime has no accessibility channel
//! (no `aria-haspopup`/`aria-expanded`/`role="dialog"`).

use jetstream_runtime::game_ui::{Color, color_mix};
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CalendarSpec, DateTimePickerSpec, TimeFieldSpec};

use crate::calendar::js_calendar;
use crate::presentation::{
    control_height_rem, control_space_x_rem, date_picker_indicator_font_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_opacity, resolve_radius};
use crate::time_field::js_time_field;

pub fn js_date_time_picker(spec: &DateTimePickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract §8 indicator font-size per size (xs 0.625 … xl 0.875) — shared
    // ladder with the sibling date/time pickers.
    let icon_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    let fill = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border_color = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");

    // Hover: color-mix(surface 86%, elevated).
    let fill_c: Color = fill.into();
    let elevated_c: Color = elevated.into();
    let hover_bg = fill_c.mix_srgb(elevated_c, 0.14);

    // ── Display text (contract §4) ──
    // Complete value → "date time"; partial → the prompt for the missing part
    // (Svelte: "Select time" when date set, "Select date" when time set);
    // empty → placeholder.
    let val = spec.current_value();
    let has_value = val.date.is_some() || val.time.is_some();
    let display = match (val.date.as_deref(), val.time.as_deref()) {
        (Some(d), Some(t)) => format!("{} {}", d, t),
        (Some(d), None) => format!("{} Select time", d),
        (None, Some(t)) => format!("Select date {}", t),
        (None, None) => spec.placeholder.clone(),
    };
    let display_color = if has_value { text_color } else { muted };

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
        // Disclosure chevron (contract §2 Indicator; text-secondary, per-size).
        .child(
            ui_element::icon("chevron-down")
                .w(icon_size)
                .h(icon_size)
                .text_color(muted),
        );

    if !spec.is_disabled {
        trigger = trigger.cursor_pointer().hover(|s| s.bg(hover_bg));
    }

    // ── Root wrapper: contract §7/§8 min-width 16rem ──
    let mut root = ui_element::div().min_w(rem_to_px(16.0)).child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body → Calendar +
    //    Time Section). Composes the real Calendar + TimeInput primitives. ──
    if spec.current_open() {
        // Composed Calendar (single), seeded from the picker's date.
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on.clone());
        if let Some(ref date) = val.date {
            cal_spec = cal_spec.with_value(date.clone()).with_visible_month(date.clone());
        }
        cal_spec.is_disabled = spec.is_disabled;

        // Composed TimeInput (TimeField), seeded from the picker's time.
        let mut time_spec = TimeFieldSpec::new();
        time_spec.value = val.time.clone();
        time_spec.is_disabled = spec.is_disabled;

        // Contract §8 Time Label: label-family, 0.6875rem, weight 600, uppercase,
        // text-secondary. (Letter-spacing / text-transform are CSS-only; the
        // string is pre-uppercased and tracking is a JsEl gap — noted.)
        let time_label = ui_element::label("TIME")
            .text_color(muted)
            .text_size(rem_to_px(0.6875))
            .text_weight(600);

        // Time Section — label above the composed TimeInput; contract gap 0.375rem.
        let time_section = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.375))
            .child(time_label)
            .child(js_time_field(&time_spec, theme));

        // Body — vertical stack of Calendar + Time Section; contract gap 0.875rem.
        let body = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.875))
            .child(js_calendar(&cal_spec, theme))
            .child(time_section);

        // Surface — established sibling overlay treatment (date_picker.rs):
        // elevated 98% over panel, border at 72% alpha, overlay shadow preset.
        let panel_bg = resolve_color(theme, "color.background.panel");
        let surface_radius = resolve_radius(theme, "radius.surface");
        let border_c: Color = border_color.into();
        let surface_border = Color {
            a: border_c.a * 0.72,
            ..border_c
        };
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
        .child(body);

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
    use poodle_specs::{ControlSize, DateTimeValue};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    /// Recursively find the first descendant (incl. self) matching `pred`.
    fn find<'a>(el: &'a JsEl, pred: &dyn Fn(&JsEl) -> bool) -> Option<&'a JsEl> {
        if pred(el) {
            return Some(el);
        }
        for c in &el.children {
            if let Some(found) = find(c, pred) {
                return Some(found);
            }
        }
        None
    }

    #[test]
    fn trigger_shows_placeholder_when_empty() {
        let spec = DateTimePickerSpec::new(); // placeholder = "Select date and time"
        let tree = probe(&js_date_time_picker(&spec, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("Select date and time"),
            "placeholder missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn trigger_shows_complete_value() {
        let spec = DateTimePickerSpec::new()
            .with_default_value(DateTimeValue::new(Some("2026-03-14".into()), Some("14:30".into())));
        let tree = probe(&js_date_time_picker(&spec, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("2026-03-14 14:30"),
            "complete value missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn partial_value_shows_contract_prompts() {
        // Date set, no time → "<date> Select time".
        let date_only = DateTimePickerSpec::new()
            .with_default_value(DateTimeValue::new(Some("2026-03-14".into()), None));
        let tree = probe(&js_date_time_picker(&date_only, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("2026-03-14 Select time"),
            "date-only prompt missing: {:?}",
            tree.texts()
        );

        // Time set, no date → "Select date <time>".
        let time_only = DateTimePickerSpec::new()
            .with_default_value(DateTimeValue::new(None, Some("14:30".into())));
        let tree = probe(&js_date_time_picker(&time_only, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("Select date 14:30"),
            "time-only prompt missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn indicator_is_chevron_not_calendar_glyph() {
        let tree = probe(&js_date_time_picker(&DateTimePickerSpec::new(), &theme()), 360.0, 120.0);
        // Icon widget carries its registry name as text.
        assert!(
            tree.has_text("chevron-down"),
            "chevron indicator missing: {:?}",
            tree.texts()
        );
        assert!(!tree.has_text("📅"), "legacy emoji indicator present");
        assert!(tree.count_kind("Icon") >= 1);
    }

    #[test]
    fn closed_picker_has_no_overlay() {
        let tree = probe(&js_date_time_picker(&DateTimePickerSpec::new(), &theme()), 360.0, 480.0);
        // Calendar emits a month label; the Time Section emits a "TIME" label.
        assert!(!tree.has_text("TIME"), "time section leaked while closed");
        assert!(!tree.has_text("March"), "calendar leaked while closed");
    }

    #[test]
    fn open_picker_composes_calendar_and_time_section() {
        let mut spec = DateTimePickerSpec::new()
            .with_default_value(DateTimeValue::new(Some("2026-03-14".into()), Some("14:30".into())));
        spec.default_open = true;
        let el = js_date_time_picker(&spec, &theme());
        let tree = probe(&el, 380.0, 560.0);

        // Real Calendar primitive: month label + a weekday header.
        assert!(
            tree.has_text("March 2026") || tree.has_text("March"),
            "composed calendar month label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Mo") || tree.has_text("Su"),
            "calendar weekday headers missing"
        );
        // Time Section label + composed TimeInput value.
        assert!(tree.has_text("TIME"), "time label missing: {:?}", tree.texts());
        assert!(tree.has_text("14:30"), "composed time value missing: {:?}", tree.texts());
    }

    #[test]
    fn time_label_uses_contract_typography() {
        let mut spec = DateTimePickerSpec::new();
        spec.default_open = true;
        let th = theme();
        let el = js_date_time_picker(&spec, &th);
        let secondary = resolve_color(&th, "color.text.secondary");
        // Contract §8: 0.6875rem, weight 600, text-secondary.
        let label = find(&el, &|e| e.style.text_size == Some(rem_to_px(0.6875)))
            .expect("time label present");
        assert_eq!(label.style.text_weight, Some(600), "time label weight not 600");
        assert_eq!(
            label.style.text_color,
            Some(secondary.into()),
            "time label color not text-secondary"
        );
    }

    #[test]
    fn disabled_reduces_opacity_and_marks_disabled() {
        let mut spec = DateTimePickerSpec::new();
        spec.is_disabled = true;
        let el = js_date_time_picker(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn sizes_produce_different_trigger_heights() {
        let sm = probe(
            &js_date_time_picker(&DateTimePickerSpec::new().with_size(ControlSize::Sm), &theme()),
            360.0,
            120.0,
        );
        let lg = probe(
            &js_date_time_picker(&DateTimePickerSpec::new().with_size(ControlSize::Lg), &theme()),
            360.0,
            120.0,
        );
        // Trigger is the first child of root; compare its height.
        let sm_trigger_h = sm.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        let lg_trigger_h = lg.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        assert!(lg_trigger_h > sm_trigger_h, "sm {sm_trigger_h} !< lg {lg_trigger_h}");
    }
}
