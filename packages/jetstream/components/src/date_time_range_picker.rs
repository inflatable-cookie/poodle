//! DateTimeRangePicker — Jetstream date+time range picker backed by DateTimeRangePickerSpec.
//!
//! Contract: `docs/contracts/components/date-time-range-picker.md`
//! Reference: `packages/svelte/components/src/DateTimeRangePicker.svelte`
//!            `packages/gpui/components/src/primitives/date_time_range_picker.rs`
//!
//! Renders the trigger button (start–end value/placeholder + disclosure chevron)
//! and, when open, the overlay surface composing the REAL Calendar primitive in
//! range mode plus paired START/END Time Sections (label + composed TimeInput) —
//! no mockup (per CLAUDE.md "No Mockups").
//!
//! ALL dimensions resolve from tokens or contract-exact rem (`rem_to_px`).
//! ZERO hardcoded pixel/color literals.
//!
//! Interaction model (mirrors the GPUI / DateTimePicker builds): open/close,
//! outside-click dismissal, Escape, and calendar/time selection are bound by the
//! preview event loop, not the component. The component renders at the current
//! spec state (`current_open()` decides whether the surface is composed) and
//! exposes interaction ids; the preview wires clicks.
//!
//! ARIA is N/A: the Jetstream runtime has no accessibility channel
//! (no `aria-haspopup`/`aria-expanded`/`role="dialog"`).

use jetstream_ui::{color_mix, Color};
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CalendarMode, CalendarSpec, DateRangeValue, DateTimeRangePickerSpec, TimeFieldSpec};

use crate::calendar::js_calendar;
use crate::presentation::{
    control_height_rem, control_space_x_rem, date_picker_indicator_font_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_opacity, resolve_radius};
use crate::time_field::js_time_field;

/// DateTimeRangePicker — a range popover with time fields.
///
/// Mirrors the GPUI target's names: `on_toggle` and `on_select`. The popover's
/// calendar is the composed `Calendar`, so day and month events forward to it
/// rather than being re-derived.
///
/// The time halves are typed, and this runtime raises no key events, so they
/// carry no handler.
pub struct DateTimeRangePicker {
    spec: DateTimeRangePickerSpec,
    theme: JetstreamThemeProvider,
    on_toggle: Option<crate::element::ActionHandler>,
    on_select: Option<crate::element::Handler>,
    on_navigate: Option<crate::element::Handler>,
}

impl DateTimeRangePicker {
    pub fn from_spec(spec: DateTimeRangePickerSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
            on_select: None,
            on_navigate: None,
        }
    }

    /// Fires when the trigger is pressed.
    pub fn on_toggle(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_toggle = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires with the pressed day as an ISO date.
    pub fn on_select(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_select = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires with `"prev"` or `"next"` when a month arrow is pressed.
    pub fn on_navigate(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_navigate = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for DateTimeRangePicker {
    fn into_js_el(self) -> JsEl {
        build(
            &self.spec,
            &self.theme,
            self.on_toggle,
            self.on_select,
            self.on_navigate,
        )
    }
}

pub fn js_date_time_range_picker(
    spec: &DateTimeRangePickerSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    build(spec, theme, None, None, None)
}

fn build(
    spec: &DateTimeRangePickerSpec,
    theme: &JetstreamThemeProvider,
    on_toggle: Option<crate::element::ActionHandler>,
    on_select: Option<crate::element::Handler>,
    on_navigate: Option<crate::element::Handler>,
) -> JsEl {
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

        if let Some(handler) = &on_toggle {
            let handler = std::sync::Arc::clone(handler);
            trigger = trigger.on_click(move |_event| handler());
        }
    }

    // ── Root wrapper: contract §7/§8 min-width 18rem ──
    let mut root = ui_element::div().min_w(rem_to_px(18.0)).child(trigger);

    // ── Overlay surface when open (contract §2 Surface → Body → Calendar(range)
    //    + Times Row). Composes the real Calendar + paired TimeInput primitives. ──
    if spec.current_open() {
        // Composed Calendar in range mode, seeded from the start/end dates.
        let mut cal_spec = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_week_start(spec.week_starts_on.clone());
        cal_spec.range_value = Some(DateRangeValue::new(
            val.start.date.clone(),
            val.end.date.clone(),
        ));
        if let Some(ref start_date) = val.start.date {
            cal_spec = cal_spec.with_visible_month(start_date.clone());
        }
        cal_spec.is_disabled = spec.is_disabled;

        // A composed Time Section — contract Time Label + real TimeInput.
        // Contract §8 Time Label: label-family, 0.6875rem, weight 600, uppercase,
        // text-secondary. (Letter-spacing / text-transform are CSS-only; the
        // string is pre-uppercased and tracking is a JsEl gap — noted.)
        let time_section = |label: &str, time_val: Option<String>| -> JsEl {
            let mut time_spec = TimeFieldSpec::new();
            time_spec.value = time_val;
            time_spec.is_disabled = spec.is_disabled;

            ui_element::div()
                .flex_col()
                .grow()
                .gap(rem_to_px(0.375)) // contract Time Section gap
                .child(
                    ui_element::label(label)
                        .text_color(muted)
                        .text_size(rem_to_px(0.6875))
                        .text_weight(600),
                )
                .child(js_time_field(&time_spec, theme))
        };

        // Times Row — two equal columns for start/end; contract gap 0.75rem.
        let times_row = ui_element::div()
            .flex_row()
            .items_start()
            .gap(rem_to_px(0.75))
            .child(time_section("START TIME", val.start.time.clone()))
            .child(time_section("END TIME", val.end.time.clone()));

        // Body — vertical stack of range Calendar + Times Row; contract gap 0.875rem.
        let body = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.875))
            .child({
            let mut calendar = crate::calendar::Calendar::from_spec(cal_spec.clone(), theme);
            if let Some(handler) = &on_select {
                let handler = std::sync::Arc::clone(handler);
                calendar = calendar.on_select(move |iso| handler(iso));
            }
            if let Some(handler) = &on_navigate {
                let handler = std::sync::Arc::clone(handler);
                calendar = calendar.on_navigate(move |dir| handler(dir));
            }
            crate::element::IntoJsEl::into_js_el(calendar)
        })
            .child(times_row);

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
                // Contract: the open picker surface is a `dialog`.
                .aria_role(jetstream_ui::accesskit::Role::Dialog)
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

    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ControlSize, DateTimeRangeValue, DateTimeValue};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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

    fn ranged(
        start_date: Option<&str>,
        start_time: Option<&str>,
        end_date: Option<&str>,
        end_time: Option<&str>,
    ) -> DateTimeRangePickerSpec {
        DateTimeRangePickerSpec {
            default_value: DateTimeRangeValue::new(
                DateTimeValue::new(start_date.map(Into::into), start_time.map(Into::into)),
                DateTimeValue::new(end_date.map(Into::into), end_time.map(Into::into)),
            ),
            ..DateTimeRangePickerSpec::new()
        }
    }

    #[test]
    fn trigger_shows_placeholder_when_empty() {
        let spec = DateTimeRangePickerSpec::new(); // placeholder = "Select date and time range"
        let tree = probe(&js_date_time_range_picker(&spec, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("Select date and time range"),
            "placeholder missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn trigger_shows_formatted_range() {
        let spec = ranged(
            Some("2026-03-10"),
            Some("09:00"),
            Some("2026-03-14"),
            Some("17:00"),
        );
        let tree = probe(&js_date_time_range_picker(&spec, &theme()), 360.0, 120.0);
        assert!(
            tree.has_text("2026-03-10 09:00 – 2026-03-14 17:00"),
            "formatted range missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn indicator_is_chevron() {
        let tree = probe(
            &js_date_time_range_picker(&DateTimeRangePickerSpec::new(), &theme()),
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
            &js_date_time_range_picker(&DateTimeRangePickerSpec::new(), &theme()),
            360.0,
            480.0,
        );
        assert!(!tree.has_text("START TIME"), "start time section leaked while closed");
        assert!(!tree.has_text("END TIME"), "end time section leaked while closed");
    }

    #[test]
    fn open_picker_composes_range_calendar_and_paired_times() {
        let mut spec = ranged(
            Some("2026-03-10"),
            Some("09:00"),
            Some("2026-03-14"),
            Some("17:00"),
        );
        spec.default_open = true;
        let el = js_date_time_range_picker(&spec, &theme());
        let tree = probe(&el, 420.0, 640.0);

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
        // Paired Time Section labels + both composed TimeInput values.
        assert!(tree.has_text("START TIME"), "start label missing: {:?}", tree.texts());
        assert!(tree.has_text("END TIME"), "end label missing: {:?}", tree.texts());
        assert!(tree.has_text("09:00"), "start time value missing: {:?}", tree.texts());
        assert!(tree.has_text("17:00"), "end time value missing: {:?}", tree.texts());
    }

    #[test]
    fn time_labels_use_contract_typography() {
        let mut spec = DateTimeRangePickerSpec::new();
        spec.default_open = true;
        let th = theme();
        let el = js_date_time_range_picker(&spec, &th);
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
        let mut spec = DateTimeRangePickerSpec::new();
        spec.is_disabled = true;
        let el = js_date_time_range_picker(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn sizes_produce_different_trigger_heights() {
        let sm = probe(
            &js_date_time_range_picker(
                &DateTimeRangePickerSpec::new().with_size(ControlSize::Sm),
                &theme(),
            ),
            360.0,
            120.0,
        );
        let lg = probe(
            &js_date_time_range_picker(
                &DateTimeRangePickerSpec::new().with_size(ControlSize::Lg),
                &theme(),
            ),
            360.0,
            120.0,
        );
        let sm_trigger_h = sm.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        let lg_trigger_h = lg.nodes.get(1).map(|n| n.h).unwrap_or(0.0);
        assert!(lg_trigger_h > sm_trigger_h, "sm {sm_trigger_h} !< lg {lg_trigger_h}");
    }

    #[test]
    fn the_trigger_reports_a_toggle() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = DateTimeRangePicker::from_spec(DateTimeRangePickerSpec::new(), &theme())
            .on_toggle(move || { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 420.0, 80.0, "Select date and time range");

        assert_eq!(hits.load(Ordering::SeqCst), 1, "on_toggle fired exactly once");
    }


    #[test]
    fn a_day_in_the_popover_reports_its_iso_date() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let days = Arc::clone(&seen);

        let el = DateTimeRangePicker::from_spec(DateTimeRangePickerSpec {
                open: Some(true),
                ..DateTimeRangePickerSpec::new().with_default_value(DateTimeRangeValue::new(
                    DateTimeValue::new(Some("2026-03-01".to_string()), Some("09:00".to_string())),
                    DateTimeValue::new(Some("2026-03-05".to_string()), Some("17:00".to_string())),
                ))
            }, &theme())
            .on_select(move |iso| days.lock().unwrap().push(iso.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 520.0, 760.0, "17");

        assert_eq!(seen.lock().unwrap().as_slice(), ["2026-03-17"]);
    }

    #[test]
    fn the_month_arrows_forward() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let moves = Arc::clone(&seen);

        let el = DateTimeRangePicker::from_spec(DateTimeRangePickerSpec {
                open: Some(true),
                ..DateTimeRangePickerSpec::new().with_default_value(DateTimeRangeValue::new(
                    DateTimeValue::new(Some("2026-03-01".to_string()), Some("09:00".to_string())),
                    DateTimeValue::new(Some("2026-03-05".to_string()), Some("17:00".to_string())),
                ))
            }, &theme())
            .on_navigate(move |dir| moves.lock().unwrap().push(dir.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 520.0, 760.0, "chevron-right");

        assert_eq!(seen.lock().unwrap().as_slice(), ["next"]);
    }

}
