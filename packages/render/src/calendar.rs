//! Calendar — a month grid with navigation.
//!
//! Contract: `docs/contracts/components/calendar.md`
//! Ported from: `packages/jetstream/components/src/calendar/` (mod + parts).
//!
//! Anatomy (contract §2):
//! ```text
//! [Root]  — container, flex-col
//!   ├── [Nav Header]  — prev button / Month-Trigger + Year-Trigger / next button
//!   ├── [Weekday Row] — 7 abbreviated day headers
//!   └── [Day Grid]    — exact week count × 7 columns (a11y grid/row/cell)
//! ```
//!
//! Keyboard / roving-tabindex / month-change editors are host-owned; the
//! component renders at the current spec state and exposes interaction ids.
//! `on_select` fires with the pressed day as an ISO date (`2026-07-31`);
//! `on_navigate` with the resulting `"YYYY-MM"` month.
//!
//! Colour recipes here are the old tier's *linear-space* lerp
//! (`jetstream_ui::color_mix`), so they go through [`mix_linear`], not
//! `mix_srgb` — the third mix recipe after theme_ext's sRGB mix and the
//! alpha tint.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, StylePatch,
};
use poodle_specs::{CalendarMode, CalendarSpec, CalendarWeekStart, DateRangeValue};

use crate::color::{mix_linear, with_alpha, WHITE};
use crate::presentation::{
    calendar_cell_size_rem, calendar_day_font_rem, calendar_nav_size_rem, rem_to_px,
    resolve_semantic_size, size_font_rem,
};

/// Weekday header labels, Sunday-first (rotated at render time based on spec).
const WEEKDAYS_SUN: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Month names (1-indexed; index 0 unused).
const MONTH_NAMES: [&str; 13] = [
    "",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
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

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// Build an outside-month (adjacent-month) day cell. Contract §8
/// outside-month: `color: text-secondary`, `opacity: 0.72` (the muted
/// opacity token).
fn outside_cell(
    cell_size_px: f32,
    control_radius: f32,
    day_font_px: f32,
    text_secondary: ColorValue,
    outside_opacity: f32,
    day: u32,
) -> Node {
    let mut cell = Node::text(day.to_string());
    cell.a11y.role = Some(NodeRole::Cell);
    {
        let s = &mut cell.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(cell_size_px);
        s.descriptor.layout.height = LayoutSizing::Fixed(cell_size_px);
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.text_size = Some(day_font_px);
        s.descriptor.text_color = Some(text_secondary);
        s.descriptor.opacity = outside_opacity;
    }
    all_corners(&mut cell, control_radius);
    cell
}

/// Host callbacks: single-day selection, range selection, and month navigation.
#[derive(Default)]
pub struct CalendarHandlers {
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_range_select: Option<Arc<dyn Fn(&DateRangeValue) + Send + Sync>>,
    pub on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

fn compute_next_range(
    current_start: Option<&str>,
    current_end: Option<&str>,
    clicked: &str,
) -> DateRangeValue {
    match (current_start, current_end) {
        (None, _) => DateRangeValue::new(Some(clicked.to_string()), None),
        (Some(start), None) => {
            if clicked >= start {
                DateRangeValue::new(Some(start.to_string()), Some(clicked.to_string()))
            } else {
                DateRangeValue::new(Some(clicked.to_string()), Some(start.to_string()))
            }
        }
        (Some(_), Some(_)) => DateRangeValue::new(Some(clicked.to_string()), None),
    }
}

pub fn calendar(
    spec: &CalendarSpec,
    theme: &dyn ThemeProvider,
    handlers: CalendarHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Per-size calendar metrics — all from the contract §8 size table.
    let cell_size_px = rem_to_px(calendar_cell_size_rem(effective_size));
    let nav_btn_size_px = rem_to_px(calendar_nav_size_rem(effective_size));
    let day_font_px = rem_to_px(calendar_day_font_rem(effective_size));
    let month_label_font_px = rem_to_px(size_font_rem(effective_size));
    let weekday_font_px = theme.resolve_space("typography.caption.size");
    // Weekday header row height — no token exists; contract-adjacent rem.
    let weekday_row_height_px = rem_to_px(1.5);

    let pad_px = rem_to_px(0.75);
    let gap_sm_px = rem_to_px(0.125);
    let root_gap_px = theme.resolve_space("space.inline.sm");
    // Header inner gap between the month + year triggers.
    let trigger_gap_px = rem_to_px(0.375);

    // Tokens
    let accent = theme.resolve_color("color.accent.base");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_inverse = theme.resolve_color("color.text.inverse");
    let surface_bg = theme.resolve_color("color.background.surface");
    let elevated_bg = theme.resolve_color("color.background.elevated");
    let border = theme.resolve_color("color.border.default");
    let icon_muted = theme.resolve_color("color.icon.muted");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
    // Outside-month days dim to the muted opacity token (= 0.72).
    let outside_opacity = theme.resolve_opacity("state.opacity.muted");
    let control_radius = theme.resolve_radius("radius.control");
    let surface_radius = theme.resolve_radius("radius.surface");

    // Derived colors — the old tier's linear-space lerp recipes.
    // hover bg = color-mix(accent 14%, transparent)
    let hover_bg = with_alpha(accent, accent.3 * 0.14);
    // hover border = color-mix(accent 46%, border-default)
    let hover_border = mix_linear(accent, border, 0.46);
    // today border = color-mix(accent 44%, border-default)
    let today_border = mix_linear(accent, border, 0.44);
    // in-range bg = color-mix(accent 16%, transparent)
    let in_range_bg = with_alpha(accent, accent.3 * 0.16);
    // selected / range-endpoint hover = color-mix(accent 88%, white 8%)
    let selected_hover_bg = mix_linear(accent, WHITE, 0.88);
    // nav hover bg = color-mix(surface 82%, elevated)
    let nav_btn_hover_bg = mix_linear(surface_bg, elevated_bg, 0.82);
    // trigger underline = color-mix(text-secondary 72%, transparent)
    let trigger_underline = with_alpha(text_secondary, text_secondary.3 * 0.72);
    // trigger underline hover = color-mix(accent 72%, transparent)
    let trigger_underline_hover = with_alpha(accent, accent.3 * 0.72);

    // ── Determine visible month ───────────────────────────────────────────────

    let (year, month) = spec
        .effective_visible_month()
        .and_then(parse_year_month)
        .unwrap_or((2026, 1));

    // ── Today ─────────────────────────────────────────────────────────────────

    let today_day: Option<u32> = {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let (ty, tm, td) = days_to_ymd((now / 86400) as i64);
        if ty == year && tm == month {
            Some(td)
        } else {
            None
        }
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

    // Exact week count for this month.
    let total_cells = start_offset + days_count;
    let rows = total_cells.div_ceil(7);

    // Previous month (for leading outside-month cells)
    let (prev_year, prev_month) = if month == 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    };
    let prev_month_days = days_in_month(prev_year, prev_month);

    let month_name = MONTH_NAMES.get(month as usize).copied().unwrap_or("");
    let year_label = format!("{year}");

    // ── Root container ────────────────────────────────────────────────────────

    let mut root = Node::container();
    root.id = Some("poodle-calendar".to_string());
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(
            calendar_cell_size_rem(effective_size) * 7.0 + 0.125 * 6.0 + 0.75 * 2.0,
        ));
        s.descriptor.layout.spacing.gap = root_gap_px;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_px;
        pad.right = pad_px;
        pad.top = pad_px;
        pad.bottom = pad_px;
        s.descriptor.background = Some(surface_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
    }
    all_corners(&mut root, surface_radius);
    root.interaction.focusable = true;

    if spec.is_disabled {
        root.style.descriptor.opacity = disabled_opacity;
        root.interaction.disabled = true;
    }

    // ── Nav header ────────────────────────────────────────────────────────────

    let prev_month = if month == 1 {
        format!("{:04}-12", year - 1)
    } else {
        format!("{year:04}-{:02}", month - 1)
    };
    let next_month = if month == 12 {
        format!("{:04}-01", year + 1)
    } else {
        format!("{year:04}-{:02}", month + 1)
    };

    let nav_button = |icon: &str, id: &str, label: &str, target_month: String| -> Node {
        let mut btn = Node::button("");
        btn.a11y.label = Some(label.to_string());
        btn.id = Some(id.to_string());
        {
            let s = &mut btn.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(nav_btn_size_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(nav_btn_size_px);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border;
            s.descriptor.background = Some(surface_bg);
            s.descriptor.cursor = CursorHint::Pointer;
            s.hover = Some(StylePatch {
                background: Some(nav_btn_hover_bg),
                border_color: None,
                text_color: None,
                opacity: None,
            });
        }
        all_corners(&mut btn, control_radius);
        let mut chevron = Node::icon(icon, theme.resolve_space("size.icon.sm"));
        chevron.style.descriptor.text_color = Some(icon_muted);
        let mut btn = btn.child(chevron);
        if let (false, Some(handler)) = (spec.is_disabled, &handlers.on_navigate) {
            let handler = Arc::clone(handler);
            btn.interaction.on_activate = Some(Arc::new(move || handler(&target_month)));
        }
        btn
    };
    let prev_btn = nav_button(
        "chevron-left",
        "poodle-cal-prev",
        "Previous month",
        prev_month,
    );
    let next_btn = nav_button("chevron-right", "poodle-cal-next", "Next month", next_month);

    // Month Label = composed Month Trigger + Year Trigger (contract §2). Each
    // is a control with a dashed-underline edit affordance rendered at the
    // current month/year. Double-click-to-edit and the inline Month Select /
    // Year Input editors are host interaction.
    let make_trigger = |id: &'static str, label: &str, disabled: bool| -> Node {
        // Edit affordance: bottom-only border (1px, contract Month/Year
        // Trigger underline). Color is uniform — only the bottom side has
        // width, so only the underline shows; the hover override (which
        // carries uniform border color) recolors it.
        let mut t = Node::text(label);
        t.id = Some(id.to_string());
        {
            let s = &mut t.style;
            s.text_size = Some(month_label_font_px);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_primary);
            s.border_bottom_width = Some(1.0);
            s.descriptor.border.color = trigger_underline;
        }
        if disabled {
            t.interaction.disabled = true;
        } else {
            t.style.descriptor.cursor = CursorHint::Pointer;
            t.style.hover = Some(StylePatch {
                background: None,
                border_color: Some(trigger_underline_hover),
                text_color: None,
                opacity: None,
            });
        }
        t
    };

    let mut month_label_control = Node::container();
    {
        let s = &mut month_label_control.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = trigger_gap_px;
    }
    let month_label_control = month_label_control
        .child(make_trigger(
            "poodle-cal-month-trigger",
            month_name,
            spec.is_disabled,
        ))
        .child(make_trigger(
            "poodle-cal-year-trigger",
            &year_label,
            spec.is_disabled,
        ));

    let mut nav_header = Node::container();
    {
        let s = &mut nav_header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.padding.top = root_gap_px;
        s.descriptor.layout.spacing.padding.bottom = root_gap_px;
    }
    root = root.child(
        nav_header
            .child(prev_btn)
            .child(month_label_control)
            .child(next_btn),
    );

    // ── Weekday header row ────────────────────────────────────────────────────

    let mut header_row = Node::container();
    {
        let s = &mut header_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = gap_sm_px;
    }

    for i in 0..7u32 {
        let idx = ((i + week_start_offset) % 7) as usize;
        let mut day = Node::text(WEEKDAYS_SUN[idx].to_uppercase());
        {
            let s = &mut day.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(cell_size_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(weekday_row_height_px);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.text_size = Some(weekday_font_px);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_secondary);
        }
        header_row = header_row.child(day);
    }
    root = root.child(header_row);

    // ── Day grid (exact week count) ───────────────────────────────────────────

    // Contract §2: the day cells are a `grid` of `row`s of `gridcell`s —
    // grid navigation, the reason a date picker is usable by keyboard at all,
    // needs the roles to navigate.
    let mut day_grid = Node::container();
    day_grid.a11y.role = Some(NodeRole::Grid);
    {
        let s = &mut day_grid.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap_px;
    }

    for row in 0..rows {
        let mut day_row = Node::container();
        day_row.a11y.role = Some(NodeRole::Row);
        {
            let s = &mut day_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.spacing.gap = gap_sm_px;
        }

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

                let mut cell = Node::text(day_num.to_string());
                cell.id = Some(format!("poodle-cal-day-{day_num}"));
                {
                    let s = &mut cell.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.width = LayoutSizing::Fixed(cell_size_px);
                    s.descriptor.layout.height = LayoutSizing::Fixed(cell_size_px);
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                    s.text_size = Some(day_font_px);
                }
                all_corners(&mut cell, control_radius);

                if is_selected || is_range_edge {
                    // Selected / range endpoint: filled accent, inverse text.
                    let s = &mut cell.style;
                    s.descriptor.background = Some(accent);
                    s.descriptor.text_color = Some(text_inverse);
                    s.text_weight = Some(600);
                    // Selected / range-endpoint hover lightens the accent.
                    if !spec.is_disabled {
                        s.hover = Some(StylePatch {
                            background: Some(selected_hover_bg),
                            border_color: None,
                            text_color: None,
                            opacity: None,
                        });
                    }
                } else if is_in_range {
                    // In-range: tinted accent fill.
                    let s = &mut cell.style;
                    s.descriptor.background = Some(in_range_bg);
                    s.descriptor.text_color = Some(text_primary);
                    if is_today {
                        s.descriptor.border.width = 1.0;
                        s.descriptor.border.color = today_border;
                        s.text_weight = Some(600);
                    }
                } else {
                    let s = &mut cell.style;
                    s.descriptor.text_color = Some(text_primary);
                    if is_today {
                        s.descriptor.border.width = 1.0;
                        s.descriptor.border.color = today_border;
                        s.text_weight = Some(600);
                    }
                    if !spec.is_disabled {
                        s.hover = Some(StylePatch {
                            background: Some(hover_bg),
                            border_color: Some(hover_border),
                            text_color: None,
                            opacity: None,
                        });
                    }
                }

                if !spec.is_disabled {
                    cell.style.descriptor.cursor = CursorHint::Pointer;
                }

                if !spec.is_disabled {
                    if is_range_mode {
                        if let Some(handler) = &handlers.on_range_select {
                            let handler = Arc::clone(handler);
                            let start = range_start_iso.clone();
                            let end = range_end_iso.clone();
                            let iso = date_iso.clone();
                            cell.interaction.on_activate = Some(Arc::new(move || {
                                handler(&compute_next_range(
                                    start.as_deref(),
                                    end.as_deref(),
                                    &iso,
                                ));
                            }));
                        }
                    } else if let Some(handler) = &handlers.on_select {
                        let handler = Arc::clone(handler);
                        let iso = date_iso.clone();
                        cell.interaction.on_activate = Some(Arc::new(move || handler(&iso)));
                    }
                }

                cell
            };

            day_row = day_row.child(cell);
        }

        day_grid = day_grid.child(day_row);
    }
    root = root.child(day_grid);

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::compute_next_range;

    #[test]
    fn range_selection_starts_completes_swaps_and_restarts() {
        let started = compute_next_range(None, None, "2026-03-12");
        assert_eq!(started.start.as_deref(), Some("2026-03-12"));
        assert_eq!(started.end, None);

        let completed = compute_next_range(started.start.as_deref(), None, "2026-03-20");
        assert_eq!(completed.start.as_deref(), Some("2026-03-12"));
        assert_eq!(completed.end.as_deref(), Some("2026-03-20"));

        let swapped = compute_next_range(Some("2026-03-12"), None, "2026-03-05");
        assert_eq!(swapped.start.as_deref(), Some("2026-03-05"));
        assert_eq!(swapped.end.as_deref(), Some("2026-03-12"));

        let restarted = compute_next_range(Some("2026-03-05"), Some("2026-03-12"), "2026-03-25");
        assert_eq!(restarted.start.as_deref(), Some("2026-03-25"));
        assert_eq!(restarted.end, None);
    }
}
