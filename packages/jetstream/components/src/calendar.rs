//! Calendar — Jetstream calendar grid backed by CalendarSpec.
//!
//! Contract: `docs/contracts/components/calendar.md`
//! Reference: `packages/gpui/components/src/primitives/calendar.rs`
//!
//! ALL dimensions resolve from tokens or contract-exact rem (`rem_to_px`).
//! ZERO hardcoded pixel literals.
//!
//! Interaction model (mirrors the GPUI build):
//! - Keyboard / roving-tabindex / actual month-change selection are bound by
//!   the preview event loop, not the component — the component renders at the
//!   current spec state and exposes interaction ids; the preview wires clicks.
//! - ARIA is N/A: the Jetstream runtime has no accessibility channel
//!   (no grid/row/gridcell roles, no aria-selected/aria-live).

use jetstream_runtime::game_ui::{Color, color_mix};
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CalendarMode, CalendarSpec, CalendarWeekStart};

use crate::presentation::{
    calendar_cell_size_rem, calendar_day_font_rem, calendar_nav_size_rem, rem_to_px,
    resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Weekday header labels, Sunday-first (rotated at render time based on spec).
const WEEKDAYS_SUN: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Month names (1-indexed; index 0 unused).
const MONTH_NAMES: [&str; 13] = [
    "",
    "January", "February", "March", "April",
    "May", "June", "July", "August",
    "September", "October", "November", "December",
];

// ── Date math ────────────────────────────────────────────────────────────────

/// Number of days in a given month (handles leap years).
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Day-of-week for the 1st of a given month (0 = Sunday, …, 6 = Saturday).
/// Uses Tomohiko Sakamoto's algorithm.
fn first_day_of_week(year: i32, month: u32) -> u32 {
    let t: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    ((y + y / 4 - y / 100 + y / 400 + t[(month as usize) - 1] + 1) % 7) as u32
}

/// Convert days since Unix epoch to (year, month, day).
/// Algorithm from <https://howardhinnant.github.io/date_algorithms.html>.
fn days_to_ymd(days: i64) -> (i32, u32, u32) {
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

/// Parse "YYYY-MM" or "YYYY-MM-DD" and return (year, month).
fn parse_year_month(s: &str) -> Option<(i32, u32)> {
    let mut parts = s.splitn(3, '-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;
    Some((year, month))
}

/// Parse "YYYY-MM-DD" and return the day number.
fn parse_day(s: &str) -> Option<u32> {
    let parts: Vec<&str> = s.splitn(3, '-').collect();
    if parts.len() == 3 {
        parts[2].parse::<u32>().ok()
    } else {
        None
    }
}

// ── Component ─────────────────────────────────────────────────────────────────

/// Build a Jetstream calendar element from a `CalendarSpec`.
///
/// Anatomy (from contract §2):
/// ```text
/// [Root]  — container, flex-col
///   ├── [Nav Header]  — prev button / Month-Trigger + Year-Trigger / next button
///   ├── [Weekday Row] — 7 abbreviated day headers
///   └── [Day Grid]    — exact week count × 7 columns
///         └── [Day Cell] — square, per-state treatment
/// ```
pub fn js_calendar(spec: &CalendarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Per-size calendar metrics — all from the contract §8 size table.
    // cell-size drives grid column + day min-height; nav button, day font,
    // and month-label font are independent calendar-specific scales.
    let cell_size_px = rem_to_px(calendar_cell_size_rem(effective_size));
    let nav_btn_size_px = rem_to_px(calendar_nav_size_rem(effective_size));
    let day_font_px = rem_to_px(calendar_day_font_rem(effective_size));
    // Month-label font scales per size (xs 0.6875 … xl 0.9375rem); matches
    // `size_font_rem`, distinct from the day-cell font.
    let month_label_font_px = rem_to_px(size_font_rem(effective_size));
    // Weekday caption font — contract §8 weekday label `0.6875rem` (== the
    // `typography.caption.size` token, now resolving to 11px in the adapter).
    // Kept as the contract-exact rem so the value is self-documenting.
    let weekday_font_px = rem_to_px(0.6875);
    // Weekday header row height — no token exists; contract-adjacent rem.
    let weekday_row_height_px = rem_to_px(1.5);

    let pad_px = rem_to_px(0.75);
    let gap_sm_px = rem_to_px(0.125);
    // Root gap (contract §8 Root gap 0.75rem).
    let root_gap_px = rem_to_px(0.75);
    // Header inner gap between the month + year triggers.
    let trigger_gap_px = rem_to_px(0.375);

    // Tokens
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let text_secondary: Color = resolve_color(theme, "color.text.secondary").into();
    let text_inverse: Color = resolve_color(theme, "color.text.inverse").into();
    let surface_bg: Color = resolve_color(theme, "color.background.surface").into();
    let elevated_bg: Color = resolve_color(theme, "color.background.elevated").into();
    let border: Color = resolve_color(theme, "color.border.default").into();
    let icon_muted: Color = resolve_color(theme, "color.icon.muted").into();
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
    // Outside-month days dim to the muted opacity token (= 0.72), not a raw
    // literal. Svelte: `.poodle-calendar__day[data-current-month=false]
    // { opacity: 0.72 }`. (Was a hardcoded 0.4 — a value bug.)
    let outside_opacity = resolve_opacity(theme, "state.opacity.muted");
    let control_radius = resolve_radius(theme, "radius.control");
    let surface_radius = resolve_radius(theme, "radius.surface");

    // Derived colors (mirror Svelte / GPUI reference color-mix formulas).
    // hover bg = color-mix(accent 14%, transparent)
    let hover_bg: Color = accent.with_alpha(accent.a * 0.14);
    // hover border = color-mix(accent 46%, border-default)
    let hover_border: Color = color_mix(accent, border, 0.46);
    // today border = color-mix(accent 44%, border-default)
    let today_border: Color = color_mix(accent, border, 0.44);
    // in-range bg = color-mix(accent 16%, transparent)
    let in_range_bg: Color = accent.with_alpha(accent.a * 0.16);
    // selected / range-endpoint hover = color-mix(accent 88%, white 8%)
    let selected_hover_bg: Color = color_mix(accent, Color::WHITE, 0.88);
    // nav hover bg = color-mix(surface 82%, elevated)
    let nav_btn_hover_bg: Color = color_mix(surface_bg, elevated_bg, 0.82);
    // trigger underline = color-mix(text-secondary 72%, transparent)
    let trigger_underline: Color = text_secondary.with_alpha(text_secondary.a * 0.72);
    // trigger underline hover = color-mix(accent 72%, transparent)
    let trigger_underline_hover: Color = accent.with_alpha(accent.a * 0.72);

    // ── Determine visible month ───────────────────────────────────────────────

    let (year, month) = spec
        .effective_visible_month()
        .and_then(parse_year_month)
        .unwrap_or_else(|| {
            // Fall back to current month derived from system clock
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let (y, m, _) = days_to_ymd((now / 86400) as i64);
            (y, m)
        });

    // ── Today ─────────────────────────────────────────────────────────────────

    let today_day: Option<u32> = {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let (ty, tm, td) = days_to_ymd((now / 86400) as i64);
        if ty == year && tm == month { Some(td) } else { None }
    };

    // ── Selection state ───────────────────────────────────────────────────────

    let is_range_mode = spec.mode == CalendarMode::Range;

    let selected_day: Option<u32> = if !is_range_mode {
        spec.current_value()
            .filter(|s| parse_year_month(s) == Some((year, month)))
            .and_then(parse_day)
    } else {
        None
    };

    let (range_start_iso, range_end_iso) = if is_range_mode {
        let rv = spec.current_range_value();
        (rv.start.clone(), rv.end.clone())
    } else {
        (None, None)
    };

    // ── Month grid parameters ─────────────────────────────────────────────────

    let days_count = days_in_month(year, month);
    let first_dow = first_day_of_week(year, month);

    // Offset into the first row (0 = first cell is day 1)
    let week_start_offset: u32 = match spec.week_starts_on {
        CalendarWeekStart::Monday => 1,
        CalendarWeekStart::Sunday => 0,
    };
    let start_offset = (first_dow + 7 - week_start_offset) % 7;

    // Exact week count for this month (Svelte builds exact weeks; the old
    // Jetstream code always rendered 6 rows, trailing an all-outside row).
    let total_cells = start_offset + days_count;
    let rows = total_cells.div_ceil(7);

    // Previous month (for leading outside-month cells)
    let (prev_year, prev_month) = if month == 1 { (year - 1, 12) } else { (year, month - 1) };
    let prev_month_days = days_in_month(prev_year, prev_month);

    let month_name = MONTH_NAMES.get(month as usize).copied().unwrap_or("");
    let year_label = format!("{year}");

    // ── Root container ────────────────────────────────────────────────────────

    let mut root = ui_element::div()
        .id("poodle-calendar")
        .flex_col()
        .gap(root_gap_px)
        .p(pad_px)
        .rounded(surface_radius)
        .bg(surface_bg)
        .border_1()
        .border_color(border)
        .focusable();

    if spec.is_disabled {
        root = root.opacity(disabled_opacity).disabled(true);
    }

    // ── Nav header ────────────────────────────────────────────────────────────

    let prev_btn = ui_element::button("")
        .id("poodle-cal-prev")
        .w(nav_btn_size_px)
        .h(nav_btn_size_px)
        .items_center()
        .justify_center()
        .rounded(control_radius)
        .border_1()
        .border_color(border)
        .bg(surface_bg)
        .cursor_pointer()
        .hover(move |s| s.bg(nav_btn_hover_bg))
        .child(
            ui_element::icon("chevron-left")
                .size(day_font_px)
                .text_color(icon_muted),
        );

    let next_btn = ui_element::button("")
        .id("poodle-cal-next")
        .w(nav_btn_size_px)
        .h(nav_btn_size_px)
        .items_center()
        .justify_center()
        .rounded(control_radius)
        .border_1()
        .border_color(border)
        .bg(surface_bg)
        .cursor_pointer()
        .hover(move |s| s.bg(nav_btn_hover_bg))
        .child(
            ui_element::icon("chevron-right")
                .size(day_font_px)
                .text_color(icon_muted),
        );

    // Month Label = composed Month Trigger + Year Trigger (contract §2). Each
    // is a control with a dashed-underline edit affordance (Svelte
    // `.poodle-calendar__month-button` / `.poodle-calendar__year-button`),
    // rendered at the current month/year. Double-click-to-edit and the inline
    // Month Select / Year Input editors are preview-loop interaction.
    let make_trigger = |id: &'static str, label: &str, disabled: bool| -> JsEl {
        // Edit affordance: bottom-only border (`border_b_1` = 1px = 0.0625rem,
        // contract Month/Year Trigger underline). Color is uniform — only the
        // bottom side has width, so only the underline shows; the hover
        // override (which carries uniform border color) recolors it.
        let mut t = ui_element::button(label)
            .id(id)
            .text_size(month_label_font_px)
            .text_weight(600)
            .text_color(text_primary)
            .border_b_1()
            .border_color(trigger_underline);
        if disabled {
            t = t.disabled(true);
        } else {
            t = t
                .cursor_pointer()
                .hover(move |s| s.border_color(trigger_underline_hover));
        }
        t
    };

    let month_label_control = ui_element::div()
        .flex_1()
        .flex_row()
        .items_center()
        .justify_center()
        .gap(trigger_gap_px)
        .child(make_trigger("poodle-cal-month-trigger", month_name, spec.is_disabled))
        .child(make_trigger("poodle-cal-year-trigger", &year_label, spec.is_disabled));

    let nav_header = ui_element::div()
        .flex_row()
        .items_center()
        .justify_between()
        .child(prev_btn)
        .child(month_label_control)
        .child(next_btn);

    root = root.child(nav_header);

    // ── Weekday header row ────────────────────────────────────────────────────

    let mut header_row = ui_element::div()
        .flex_row()
        .gap(gap_sm_px);

    for i in 0..7u32 {
        let idx = ((i + week_start_offset) % 7) as usize;
        header_row = header_row.child(
            ui_element::label(WEEKDAYS_SUN[idx])
                .w(cell_size_px)
                .h(weekday_row_height_px)
                .items_center()
                .justify_center()
                .text_size(weekday_font_px)
                .text_weight(600)
                .text_color(text_secondary)
                .text_align_center(),
        );
    }
    root = root.child(header_row);

    // ── Day grid (exact week count) ───────────────────────────────────────────

    for row in 0..rows {
        let mut day_row = ui_element::div()
            .flex_row()
            .gap(gap_sm_px);

        for col in 0..7u32 {
            let cell_idx = row * 7 + col;

            let cell = if cell_idx < start_offset {
                // Leading outside-month cell (previous month)
                let outside_day = prev_month_days.saturating_sub(start_offset - cell_idx - 1);
                outside_cell(
                    cell_size_px,
                    control_radius,
                    day_font_px,
                    text_secondary,
                    outside_opacity,
                    outside_day,
                )
            } else if cell_idx >= start_offset + days_count {
                // Trailing outside-month cell (next month)
                let outside_day = cell_idx - start_offset - days_count + 1;
                outside_cell(
                    cell_size_px,
                    control_radius,
                    day_font_px,
                    text_secondary,
                    outside_opacity,
                    outside_day,
                )
            } else {
                // Current-month day cell
                let day_num = cell_idx - start_offset + 1;
                let date_iso = format!("{year:04}-{month:02}-{day_num:02}");

                let is_today = today_day == Some(day_num);
                let is_selected = !is_range_mode && selected_day == Some(day_num);

                let is_range_start =
                    is_range_mode && range_start_iso.as_deref() == Some(date_iso.as_str());
                let is_range_end =
                    is_range_mode && range_end_iso.as_deref() == Some(date_iso.as_str());
                let is_in_range = is_range_mode
                    && match (range_start_iso.as_deref(), range_end_iso.as_deref()) {
                        (Some(s), Some(e)) => date_iso.as_str() > s && date_iso.as_str() < e,
                        _ => false,
                    };
                let is_range_edge = is_range_start || is_range_end;

                let mut cell = ui_element::div()
                    .id(format!("poodle-cal-day-{day_num}"))
                    .w(cell_size_px)
                    .h(cell_size_px)
                    .items_center()
                    .justify_center()
                    .rounded(control_radius)
                    .text_size(day_font_px)
                    .text_align_center();

                if is_selected || is_range_edge {
                    // Selected / range endpoint: filled accent, inverse text.
                    cell = cell
                        .bg(accent)
                        .text_color(text_inverse)
                        .text_weight(600);
                    // Selected / range-endpoint hover lightens the accent.
                    if !spec.is_disabled {
                        cell = cell.hover(move |s| s.bg(selected_hover_bg));
                    }
                } else if is_in_range {
                    // In-range: tinted accent fill.
                    cell = cell.bg(in_range_bg).text_color(text_primary);
                    if is_today {
                        cell = cell
                            .border_1()
                            .border_color(today_border)
                            .text_weight(600);
                    }
                } else {
                    cell = cell.text_color(text_primary);
                    if is_today {
                        cell = cell
                            .border_1()
                            .border_color(today_border)
                            .text_weight(600);
                    }
                    if !spec.is_disabled {
                        cell = cell.hover(move |s| {
                            s.bg(hover_bg).border_color(hover_border)
                        });
                    }
                }

                if !spec.is_disabled {
                    cell = cell.cursor_pointer();
                }

                cell = cell.child(ui_element::label(day_num.to_string()));
                cell
            };

            day_row = day_row.child(cell);
        }

        root = root.child(day_row);
    }

    root
}

/// Build an outside-month (adjacent-month) day cell. Contract §8 outside-month:
/// `color: text-secondary`, `opacity: 0.72` (the muted opacity token).
fn outside_cell(
    cell_size_px: f32,
    control_radius: f32,
    day_font_px: f32,
    text_secondary: Color,
    outside_opacity: f32,
    day: u32,
) -> JsEl {
    ui_element::div()
        .w(cell_size_px)
        .h(cell_size_px)
        .items_center()
        .justify_center()
        .rounded(control_radius)
        .text_size(day_font_px)
        .text_color(text_secondary)
        .text_align_center()
        .opacity(outside_opacity)
        .child(ui_element::label(day.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ControlSize, DateRangeValue};

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

    fn spec_with_month(m: &str) -> CalendarSpec {
        CalendarSpec::new().with_visible_month(m)
    }

    #[test]
    fn renders_month_and_year_header_controls() {
        // March 2026: month name and year both rendered as header triggers.
        let el = js_calendar(&spec_with_month("2026-03"), &theme());
        let tree = probe(&el, 320.0, 360.0);
        assert!(
            tree.has_text("March"),
            "month-name trigger missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("2026"),
            "year trigger missing: {:?}",
            tree.texts()
        );
        // Both triggers exist as distinct ids in the JsEl tree.
        assert!(
            find(&el, &|e| e.id.as_deref() == Some("poodle-cal-month-trigger")).is_some(),
            "month-trigger id missing"
        );
        assert!(
            find(&el, &|e| e.id.as_deref() == Some("poodle-cal-year-trigger")).is_some(),
            "year-trigger id missing"
        );
    }

    #[test]
    fn month_trigger_has_dashed_underline_affordance() {
        // Edit affordance = bottom border (border_b_1) on the month trigger.
        let el = js_calendar(&spec_with_month("2026-03"), &theme());
        let month_trigger = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-month-trigger"))
            .expect("month-trigger present");
        assert!(
            month_trigger.style.border_widths[2] > 0.0,
            "month trigger should carry a bottom-border underline affordance"
        );
    }

    #[test]
    fn outside_month_day_uses_muted_opacity_not_point_four() {
        // The fixed bug: outside-month days must use the muted opacity token
        // (0.72), NOT the old hardcoded 0.4.
        let th = theme();
        let muted = resolve_opacity(&th, "state.opacity.muted");
        let secondary: Color = resolve_color(&th, "color.text.secondary").into();
        let el = js_calendar(&spec_with_month("2026-03"), &th);

        // March 2026 (Monday-start) leads with several outside cells; find a
        // dimmed (opacity < 1.0) leaf carrying the secondary text color.
        let outside = find(&el, &|e| {
            e.style.opacity < 1.0
                && e.style.opacity > 0.0
                && e.style.text_color == Some(secondary)
        })
        .expect("an outside-month day cell with reduced opacity");

        assert!(
            (outside.style.opacity - muted).abs() < 0.001,
            "outside opacity {} != muted token {muted}",
            outside.style.opacity
        );
        assert!(
            (muted - 0.72).abs() < 0.001,
            "muted token should be 0.72, got {muted}"
        );
        assert!(
            (outside.style.opacity - 0.4).abs() > 0.001,
            "outside opacity must NOT be the old 0.4 value"
        );
    }

    #[test]
    fn selected_day_gets_selected_treatment() {
        // A preselected single date paints the accent fill + inverse text.
        let th = theme();
        let accent: Color = resolve_color(&th, "color.accent.base").into();
        let inverse: Color = resolve_color(&th, "color.text.inverse").into();
        let spec = CalendarSpec::new()
            .with_visible_month("2026-03")
            .with_value("2026-03-14");
        let el = js_calendar(&spec, &th);

        let selected = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-14"))
            .expect("day-14 cell present");
        assert_eq!(
            selected.style.background,
            Some(accent),
            "selected day should fill with accent"
        );
        assert_eq!(
            selected.style.text_color,
            Some(inverse),
            "selected day should use inverse text"
        );
        // And the accent fill is probe-visible in the laid-out tree.
        let tree = probe(&el, 320.0, 360.0);
        assert!(
            tree.has_background(
                crate::render_probe::ProbeColor {
                    r: accent.r,
                    g: accent.g,
                    b: accent.b,
                    a: accent.a,
                },
                0.01
            ),
            "selected accent fill missing from rendered tree"
        );
    }

    #[test]
    fn range_endpoints_get_accent_fill() {
        let th = theme();
        let accent: Color = resolve_color(&th, "color.accent.base").into();
        let spec = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_visible_month("2026-03")
            .with_default_range_value(DateRangeValue::new(
                Some("2026-03-10".into()),
                Some("2026-03-20".into()),
            ));
        let el = js_calendar(&spec, &th);

        let start = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-10"))
            .expect("range start present");
        let end = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-20"))
            .expect("range end present");
        assert_eq!(start.style.background, Some(accent), "range start = accent");
        assert_eq!(end.style.background, Some(accent), "range end = accent");

        // An interior day gets the tinted in-range fill (lower alpha), not the
        // solid accent.
        let mid = find(&el, &|e| e.id.as_deref() == Some("poodle-cal-day-15"))
            .expect("in-range day present");
        let mid_bg = mid.style.background.expect("in-range fill set");
        assert!(mid_bg.a < accent.a, "in-range alpha should be tinted");
    }

    #[test]
    fn per_size_scales_day_font() {
        // Day-cell font follows the calendar day-font scale, distinct per size.
        let th = theme();
        let xs = js_calendar(
            &CalendarSpec::new().with_visible_month("2026-03").with_size(ControlSize::Xs),
            &th,
        );
        let xl = js_calendar(
            &CalendarSpec::new().with_visible_month("2026-03").with_size(ControlSize::Xl),
            &th,
        );
        let xs_day = find(&xs, &|e| e.id.as_deref() == Some("poodle-cal-day-1")).unwrap();
        let xl_day = find(&xl, &|e| e.id.as_deref() == Some("poodle-cal-day-1")).unwrap();
        assert!(
            xl_day.style.text_size.unwrap() > xs_day.style.text_size.unwrap(),
            "xl day font should exceed xs day font"
        );
        assert_eq!(xs_day.style.text_size, Some(rem_to_px(0.6875)));
        assert_eq!(xl_day.style.text_size, Some(rem_to_px(0.875)));
    }

    #[test]
    fn exact_week_count_no_trailing_blank_row() {
        // Feb 2026 (Monday-start) fits in fewer than 6 weeks; the grid must
        // not render a 6th all-outside row.
        let el = js_calendar(&spec_with_month("2026-02"), &theme());
        // Day rows = children after [nav header, weekday row]. Each day row has
        // exactly 7 children.
        let day_rows: Vec<&JsEl> = el
            .children
            .iter()
            .filter(|c| c.children.len() == 7 && c.children.iter().all(|d| d.children.len() <= 1))
            .collect();
        // Feb 2026: 28 days, starts on a Sunday → Monday-start offset 6 →
        // (6 + 28) = 34 cells → 5 rows. Weekday header also has 7 children, so
        // expect 5 day rows + 1 weekday row = 6 such rows. Assert <= 6 (i.e.
        // not the old fixed 6 day rows = 7 total).
        assert!(
            day_rows.len() <= 6,
            "should not render a fixed 6 day rows for short months, got {} seven-wide rows",
            day_rows.len()
        );
    }
}
