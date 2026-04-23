//! Calendar — Jetstream calendar grid backed by CalendarSpec.
//!
//! Contract: `docs/contracts/components/calendar.md`
//! Reference: `packages/gpui/components/src/primitives/calendar.rs`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::{Color, color_mix};
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CalendarMode, CalendarSpec, CalendarWeekStart};

use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem};
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
/// Anatomy (from contract):
/// ```text
/// [Root]  — container, flex-col
///   ├── [Nav Header]  — prev button / "Month Year" label / next button
///   ├── [Weekday Row] — 7 abbreviated day headers
///   └── [Day Grid]    — 6 rows × 7 columns (fixed 6 rows)
///         └── [Day Cell] — square, various states
/// ```
pub fn js_calendar(spec: &CalendarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let cell_size_px = rem_to_px(control_height_rem(effective_size));
    let body_font_px = rem_to_px(size_font_rem(effective_size));
    // Caption font: slightly smaller than body
    let caption_font_px = rem_to_px(size_font_rem(effective_size) - 0.125);
    let pad_px = rem_to_px(0.75);
    let gap_sm_px = rem_to_px(0.125);
    let nav_btn_size_px = rem_to_px(2.0);

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
    let control_radius = resolve_radius(theme, "radius.control");
    let surface_radius = resolve_radius(theme, "radius.surface");

    // Derived colors (mirror Svelte reference)
    let hover_bg: Color = accent.with_alpha(accent.a * 0.14);
    let hover_border: Color = color_mix(accent, border, 0.46);
    let today_border: Color = color_mix(accent, border, 0.44);
    let in_range_bg: Color = accent.with_alpha(accent.a * 0.16);
    let nav_btn_hover_bg: Color = color_mix(surface_bg, elevated_bg, 0.82);

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

    // Previous month (for leading outside-month cells)
    let (prev_year, prev_month) = if month == 1 { (year - 1, 12) } else { (year, month - 1) };
    let prev_month_days = days_in_month(prev_year, prev_month);

    // Navigation month strings
    // Navigation month strings — computed for completeness; callers can wire
    // click handlers on the nav buttons to drive visible_month changes.
    let _prev_month_str = {
        let (py, pm) = (prev_year, prev_month);
        format!("{:04}-{:02}", py, pm)
    };
    let _next_month_str = {
        let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
        format!("{:04}-{:02}", ny, nm)
    };

    let month_label = format!(
        "{} {}",
        MONTH_NAMES.get(month as usize).copied().unwrap_or(""),
        year
    );

    // ── Root container ────────────────────────────────────────────────────────

    let mut root = ui_element::div()
        .flex_col()
        .gap(gap_sm_px)
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
                .size(body_font_px)
                .text_color(icon_muted),
        );

    let next_btn = ui_element::button("")
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
                .size(body_font_px)
                .text_color(icon_muted),
        );

    let nav_header = ui_element::div()
        .flex_row()
        .items_center()
        .justify_between()
        .child(prev_btn)
        .child(
            ui_element::label(&month_label)
                .flex_1()
                .text_size(body_font_px)
                .text_weight(600)
                .text_color(text_primary)
                .text_align_center(),
        )
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
                .h(cell_size_px)
                .items_center()
                .justify_center()
                .text_size(caption_font_px)
                .text_weight(600)
                .text_color(text_secondary)
                .text_align_center(),
        );
    }
    root = root.child(header_row);

    // ── Day grid (fixed 6 rows) ───────────────────────────────────────────────

    for row in 0..6u32 {
        let mut day_row = ui_element::div()
            .flex_row()
            .gap(gap_sm_px);

        for col in 0..7u32 {
            let cell_idx = row * 7 + col;

            let cell = if cell_idx < start_offset {
                // Leading outside-month cell (previous month)
                let outside_day = prev_month_days.saturating_sub(start_offset - cell_idx - 1);
                ui_element::div()
                    .w(cell_size_px)
                    .h(cell_size_px)
                    .items_center()
                    .justify_center()
                    .rounded(control_radius)
                    .text_size(body_font_px)
                    .text_color(text_secondary)
                    .text_align_center()
                    .opacity(0.4)
                    .child(ui_element::label(&outside_day.to_string()))
            } else if cell_idx >= start_offset + days_count {
                // Trailing outside-month cell (next month)
                let outside_day = cell_idx - start_offset - days_count + 1;
                ui_element::div()
                    .w(cell_size_px)
                    .h(cell_size_px)
                    .items_center()
                    .justify_center()
                    .rounded(control_radius)
                    .text_size(body_font_px)
                    .text_color(text_secondary)
                    .text_align_center()
                    .opacity(0.4)
                    .child(ui_element::label(&outside_day.to_string()))
            } else {
                // Current-month day cell
                let day_num = cell_idx - start_offset + 1;
                let date_iso = format!("{:04}-{:02}-{:02}", year, month, day_num);

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
                    .w(cell_size_px)
                    .h(cell_size_px)
                    .items_center()
                    .justify_center()
                    .rounded(control_radius)
                    .text_size(body_font_px)
                    .text_align_center();

                if is_selected || is_range_edge {
                    // Selected / range endpoint: filled accent, inverse text
                    cell = cell
                        .bg(accent)
                        .text_color(text_inverse)
                        .text_weight(600);
                } else if is_in_range {
                    // In-range: tinted accent fill
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

                cell = cell.child(ui_element::label(&day_num.to_string()));
                cell
            };

            day_row = day_row.child(cell);
        }

        root = root.child(day_row);
    }

    root
}
