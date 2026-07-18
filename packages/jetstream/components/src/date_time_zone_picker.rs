//! DateTimeZonePicker — Jetstream date+time+zone picker backed by DateTimeZonePickerSpec.
//!
//! Contract: `docs/contracts/components/date-time-zone-picker.md`
//! Reference: `packages/svelte/components/src/DateTimeZonePicker.svelte`
//!            `packages/gpui/components/src/primitives/date_time_zone_picker.rs`
//!
//! Renders the trigger button (date-time + zone value/placeholder + disclosure
//! chevron) and, when open, the overlay surface composing the REAL Calendar +
//! TimeInput + TimeZoneSelect primitives, each wrapped in a labelled Field — no
//! mockup (per CLAUDE.md "No Mockups").
//!
//! ALL dimensions resolve from tokens or contract-exact rem (`rem_to_px`).
//! ZERO hardcoded pixel/color literals.
//!
//! Interaction model (mirrors the GPUI / DateTimePicker builds): open/close,
//! outside-click dismissal, Escape, and calendar/time/zone selection are bound by
//! the preview event loop, not the component. The component renders at the
//! current spec state (`current_open()` decides whether the surface is composed)
//! and exposes interaction ids; the preview wires clicks.
//!
//! ARIA is N/A: the Jetstream runtime has no accessibility channel
//! (no `aria-haspopup`/`aria-expanded`/`role="dialog"`).

use jetstream_ui::{color_mix, Color};
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CalendarSpec, DateTimeZonePickerSpec, TimeFieldSpec, TimeZoneSelectSpec};

use crate::calendar::js_calendar;
use crate::presentation::{
    control_height_rem, control_space_x_rem, date_picker_indicator_font_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_opacity, resolve_radius};
use crate::time_field::js_time_field;
use crate::time_zone_select::js_time_zone_select;

pub fn js_date_time_zone_picker(
    spec: &DateTimeZonePickerSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract §8 indicator font-size per size (xs 0.625 … xl 0.875) — shared
    // ladder with the sibling date/time pickers.
    let icon_size = rem_to_px(date_picker_indicator_font_rem(effective_size));

    let fill = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, spec.overlay_fill_token());
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");

    // Hover: color-mix(surface 86%, elevated).
    let fill_c: Color = fill.into();
    let elevated_c: Color = elevated.into();
    let hover_bg = fill_c.mix_srgb(elevated_c, 0.14);

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

    // ── Root wrapper: contract §7/§8 min-width 18rem ──
    let mut root = ui_element::div().min_w(rem_to_px(18.0)).child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body → Calendar +
    //    Fields → Time field + Time-zone field). Composes the real Calendar +
    //    TimeInput + TimeZoneSelect primitives. ──
    if spec.current_open() {
        // Composed Calendar (single), seeded from the structured value's date.
        let mut cal_spec = CalendarSpec::new().with_week_start(spec.week_starts_on.clone());
        if let Some(ref date) = value.date {
            cal_spec = cal_spec.with_value(date.clone()).with_visible_month(date.clone());
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
        if !spec.time_zone_options.is_empty() {
            tz_spec.options = spec.time_zone_options.clone();
        }

        // Field Label — contract §8: label-family, 0.6875rem, weight 600,
        // uppercase, text-secondary. (Letter-spacing / text-transform are
        // CSS-only; the string is pre-uppercased and tracking is a JsEl gap.)
        let field_label = |text: &str| -> JsEl {
            ui_element::label(text)
                .text_color(muted)
                .text_size(rem_to_px(0.6875))
                .text_weight(600)
        };

        // Time field — contract Field: "TIME" label above composed TimeInput;
        // contract Field gap 0.375rem.
        let time_field_group = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.375))
            .child(field_label("TIME"))
            .child(js_time_field(&time_spec, theme));

        // Time zone field — contract Field: "TIME ZONE" label above composed
        // TimeZoneSelect; contract Field gap 0.375rem.
        let tz_field_group = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.375))
            .child(field_label("TIME ZONE"))
            .child(js_time_zone_select(&tz_spec, theme));

        // Fields — vertical stack of Time + Time zone fields; contract gap 0.75rem.
        let fields = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.75))
            .child(time_field_group)
            .child(tz_field_group);

        // Body — vertical stack of Calendar + Fields; contract gap 0.875rem.
        let body = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.875))
            .child(js_calendar(&cal_spec, theme))
            .child(fields);

        // Surface — established sibling overlay treatment (date_time_picker.rs):
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

        // Trigger + anchored-below surface stack (overlay anchoring is a platform
        // delta; rendered as a flow column with the contract surface gap).
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
    use poodle_specs::{ControlSize, ZonedDateTimeValue};

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
        let spec = DateTimeZonePickerSpec::new(); // "Select date, time, and zone"
        let tree = probe(&js_date_time_zone_picker(&spec, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("Select date, time, and zone"),
            "placeholder missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn trigger_shows_formatted_zoned_value() {
        let spec = DateTimeZonePickerSpec::new().with_default_value(ZonedDateTimeValue::new(
            Some("2026-03-14".into()),
            Some("10:00".into()),
            Some("America/Los_Angeles".into()),
        ));
        let tree = probe(&js_date_time_zone_picker(&spec, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("2026-03-14 10:00 America/Los_Angeles"),
            "formatted zoned value missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn indicator_is_chevron() {
        let tree = probe(
            &js_date_time_zone_picker(&DateTimeZonePickerSpec::new(), &theme()),
            360.0,
            120.0,
        );
        assert!(
            tree.has_text("chevron-down"),
            "chevron indicator missing: {:?}",
            tree.texts()
        );
        assert!(tree.count_kind("Icon") >= 1);
    }

    #[test]
    fn closed_picker_has_no_overlay() {
        let tree = probe(
            &js_date_time_zone_picker(&DateTimeZonePickerSpec::new(), &theme()),
            360.0,
            480.0,
        );
        assert!(!tree.has_text("TIME"), "time field leaked while closed");
        assert!(!tree.has_text("TIME ZONE"), "tz field leaked while closed");
    }

    #[test]
    fn open_picker_composes_calendar_time_and_zone() {
        let mut spec = DateTimeZonePickerSpec::new().with_default_value(ZonedDateTimeValue::new(
            Some("2026-03-14".into()),
            Some("10:00".into()),
            Some("America/Los_Angeles".into()),
        ));
        spec.default_open = true;
        let el = js_date_time_zone_picker(&spec, &theme());
        let tree = probe(&el, 420.0, 720.0);

        // Real Calendar primitive: month label + weekday headers.
        assert!(
            tree.has_text("March 2026") || tree.has_text("March"),
            "composed calendar month label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Mo") || tree.has_text("Su"),
            "calendar weekday headers missing: {:?}",
            tree.texts()
        );
        // Field labels (contract §2 anatomy).
        assert!(tree.has_text("TIME"), "time field label missing: {:?}", tree.texts());
        assert!(tree.has_text("TIME ZONE"), "tz field label missing: {:?}", tree.texts());
        // Composed TimeInput value.
        assert!(tree.has_text("10:00"), "composed time value missing: {:?}", tree.texts());
        // Composed TimeZoneSelect value label (`_`→space formatted, per Svelte).
        assert!(
            tree.has_text("America/Los Angeles"),
            "composed timezone value missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn field_labels_use_contract_typography() {
        let mut spec = DateTimeZonePickerSpec::new();
        spec.default_open = true;
        let th = theme();
        let el = js_date_time_zone_picker(&spec, &th);
        let secondary = resolve_color(&th, "color.text.secondary");
        // Contract §8: 0.6875rem, weight 600, text-secondary.
        let label = find(&el, &|e| e.style.text_size == Some(rem_to_px(0.6875)))
            .expect("field label present");
        assert_eq!(label.style.text_weight, Some(600), "field label weight not 600");
        assert_eq!(
            label.style.text_color,
            Some(secondary.into()),
            "field label color not text-secondary"
        );
    }

    #[test]
    fn disabled_reduces_opacity_and_marks_disabled() {
        let mut spec = DateTimeZonePickerSpec::new();
        spec.is_disabled = true;
        let el = js_date_time_zone_picker(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn sizes_produce_different_trigger_heights() {
        let sm = probe(
            &js_date_time_zone_picker(
                &DateTimeZonePickerSpec::new().with_size(ControlSize::Sm),
                &theme(),
            ),
            360.0,
            120.0,
        );
        let lg = probe(
            &js_date_time_zone_picker(
                &DateTimeZonePickerSpec::new().with_size(ControlSize::Lg),
                &theme(),
            ),
            360.0,
            120.0,
        );
        let sm_trigger_h = sm.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        let lg_trigger_h = lg.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        assert!(lg_trigger_h > sm_trigger_h, "sm {sm_trigger_h} !< lg {lg_trigger_h}");
    }
}
