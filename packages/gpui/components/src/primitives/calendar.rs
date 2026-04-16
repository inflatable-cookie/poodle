//! Calendar — real GPUI component backed by CalendarSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CalendarMode, CalendarSpec, CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue,
    IconSize, IconSpec, SemanticControlSizeRole,
};

use super::icon::Icon;
use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Weekday header labels (Sunday-first; rotated at render time based on spec).
const WEEKDAYS_SUN: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// A real GPUI calendar component backed by `CalendarSpec`.
///
/// Renders a month grid with weekday headers and day cells.
/// The selected date is highlighted with the accent colour.
pub struct Calendar {
    spec: CalendarSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_select: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Called when prev/next month is clicked, with the new "YYYY-MM" string.
    on_navigate: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Range-mode selection callback. Called with the updated DateRangeValue
    /// after a click: first click sets `start` (end = None), second click
    /// sets `end`, third click resets back to start-only. The caller is
    /// responsible for storing the new range and feeding it back via spec.
    on_range_select: Option<std::rc::Rc<dyn Fn(&DateRangeValue, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Calendar {
    type Target = CalendarSpec;
    fn deref(&self) -> &CalendarSpec {
        &self.spec
    }
}

impl Calendar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CalendarSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_select: None,
            on_navigate: None,
            on_range_select: None,
        }
    }

    pub fn from_spec(spec: CalendarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_select: None,
            on_navigate: None,
            on_range_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn visible_month(mut self, v: impl Into<String>) -> Self {
        self.spec.visible_month = Some(v.into());
        self
    }
    pub fn week_starts_on(mut self, v: CalendarWeekStart) -> Self {
        self.spec.week_starts_on = v;
        self
    }
    pub fn locale(mut self, v: impl Into<String>) -> Self {
        self.spec.locale = v.into();
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(std::rc::Rc::new(handler));
        self
    }

    /// Called when prev/next month navigation is clicked.
    pub fn on_navigate(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_navigate = Some(std::rc::Rc::new(handler));
        self
    }

    /// Fluent shortcut for entering range mode.
    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.spec.mode = mode;
        self
    }

    /// Seed the initial range value for range mode.
    pub fn default_range(mut self, range: DateRangeValue) -> Self {
        self.spec.default_range_value = range;
        self
    }

    /// Controlled range value (wins over `default_range_value`).
    pub fn range_value(mut self, range: DateRangeValue) -> Self {
        self.spec.range_value = Some(range);
        self
    }

    /// Called when a day is clicked in range mode. The handler receives
    /// the new DateRangeValue computed from the click (first click →
    /// start only; second click → start + end, swapped if clicked
    /// before start; third click → reset to start only).
    pub fn on_range_select(
        mut self,
        handler: impl Fn(&DateRangeValue, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_range_select = Some(std::rc::Rc::new(handler));
        self
    }

    /// Parse a "YYYY-MM" or "YYYY-MM-DD" string and return (year, month).
    fn parse_year_month(s: &str) -> Option<(i32, u32)> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() >= 2 {
            let year = parts[0].parse::<i32>().ok()?;
            let month = parts[1].parse::<u32>().ok()?;
            Some((year, month))
        } else {
            None
        }
    }

    /// Parse "YYYY-MM-DD" and return the day number.
    fn parse_day(s: &str) -> Option<u32> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() == 3 {
            parts[2].parse::<u32>().ok()
        } else {
            None
        }
    }

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

    /// Day-of-week for the 1st of a given month (0 = Sunday).
    fn first_day_of_week(year: i32, month: u32) -> u32 {
        // Tomohiko Sakamoto's algorithm
        let t = [0i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let y = if month < 3 { year - 1 } else { year };
        let m = month as usize;
        ((y + y / 4 - y / 100 + y / 400 + t[m - 1] + 1) % 7) as u32
    }

    /// Convert days since Unix epoch to (year, month, day).
    fn days_to_ymd(days: i64) -> (i32, u32, u32) {
        // Algorithm from https://howardhinnant.github.io/date_algorithms.html
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
}

impl IntoElement for Calendar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let _cell_size = px(rem_to_px(control_height_rem(effective_size)));
        let cal_font = px(rem_to_px(size_font_rem(effective_size)));

        let control_radius = resolve_radius(theme, "radius.control");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let gap_sm = resolve_px(theme, "space.inline.sm");

        let accent = resolve_color(theme, "color.accent.base");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_inverse = resolve_color(theme, "color.text.inverse");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border = resolve_color(theme, "color.border.default");
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let body_size = cal_font;

        // Contract: hover = color-mix(accent 8%, surface)
        let hover_bg = color_mix(accent, surface_bg, 0.08);
        // Contract: today cell border = border-default
        let today_border = border;

        let selected_date = spec.current_value().map(|s| s.to_string());
        let selected_day = selected_date.as_deref().and_then(Self::parse_day);

        // Range-mode state: resolve start / end ISO strings once. Cells
        // compare their own YYYY-MM-DD against these to decide whether
        // they are the range start, end, or an interior day.
        let is_range_mode = spec.mode == CalendarMode::Range;
        let (range_start_iso, range_end_iso) = if is_range_mode {
            let range = spec.current_range_value();
            (range.start.clone(), range.end.clone())
        } else {
            (None, None)
        };

        // Determine which month to show
        let (year, month) = spec
            .effective_visible_month()
            .and_then(Self::parse_year_month)
            .unwrap_or((2026, 1));

        // Compute "today" for highlighting (YYYY-MM-DD)
        // Simple approach: check if today is in the visible month
        let today_day: Option<u32> = {
            // Use a static today value for consistency within a single render
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let days_since_epoch = now / 86400;
            // Approximate date calculation
            let (ty, tm, td) = Self::days_to_ymd(days_since_epoch as i64);
            if ty == year && tm == month {
                Some(td)
            } else {
                None
            }
        };

        let days_count = Self::days_in_month(year, month);
        let first_dow = Self::first_day_of_week(year, month);

        // Determine week-start offset (Sunday = 0, Monday = 1)
        let week_start_offset: u32 = match spec.week_starts_on {
            poodle_specs::CalendarWeekStart::Monday => 1,
            _ => 0,
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-calendar-{}", suffix)
        } else {
            "poodle-calendar".to_string()
        };

        let month_names = [
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
        let month_label = format!(
            "{} {}",
            month_names.get(month as usize - 1).unwrap_or(&""),
            year
        );

        let surface_radius = resolve_radius(theme, "radius.surface");

        // Build the calendar container
        // Layout: 7 cells × 2.25rem + 6 gaps × 0.125rem + 2 × 0.75rem padding = 18rem
        let calendar_width = px(rem_to_px(18.0));
        let cell_size = px(rem_to_px(2.25)); // Svelte default: 2.25rem
        let nav_btn_size = px(rem_to_px(2.0)); // Svelte default: 2rem

        let mut cal = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .flex_col()
            .gap(gap_sm)
            .p(px(rem_to_px(0.75)))
            .w(calendar_width)
            .rounded(surface_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border);

        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        cal = cal.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if spec.is_disabled {
            cal = cal
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // Compute prev/next month strings for navigation
        let prev_month_str = {
            let (py, pm) = if month == 1 {
                (year - 1, 12)
            } else {
                (year, month - 1)
            };
            format!("{:04}-{:02}", py, pm)
        };
        let next_month_str = {
            let (ny, nm) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
            format!("{:04}-{:02}", ny, nm)
        };

        // Contract: nav header with prev/next month buttons and centered month label
        let nav_btn_hover = color_mix(elevated_bg, surface_bg, 0.84);

        let mut prev_btn = div()
            .id("poodle-cal-prev")
            .w(nav_btn_size)
            .h(nav_btn_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded(control_radius)
            .cursor_pointer()
            .hover(move |s| s.bg(nav_btn_hover))
            .child(
                Icon::from_spec(IconSpec::new("chevron-left").with_size(IconSize::Sm), theme)
                    .with_color(icon_muted),
            );

        if let Some(ref handler) = self.on_navigate {
            let handler = handler.clone();
            let prev = prev_month_str.clone();
            prev_btn = prev_btn.on_click(move |_event, window, cx| {
                handler(&prev, window, cx);
            });
        }

        let mut next_btn = div()
            .id("poodle-cal-next")
            .w(nav_btn_size)
            .h(nav_btn_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded(control_radius)
            .cursor_pointer()
            .child(
                Icon::from_spec(
                    IconSpec::new("chevron-right").with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_muted),
            );

        if let Some(ref handler) = self.on_navigate {
            let handler = handler.clone();
            let next = next_month_str.clone();
            next_btn = next_btn.on_click(move |_event, window, cx| {
                handler(&next, window, cx);
            });
        }

        let nav_header = div()
            .flex()
            .items_center()
            .justify_between()
            .py(gap_sm)
            .child(prev_btn)
            .child(
                div()
                    .flex_1()
                    .text_center()
                    .text_size(body_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(month_label),
            )
            .child(next_btn);

        cal = cal.child(nav_header);

        // Keyboard navigation: arrow keys move day selection, PageUp/PageDown change month
        if !spec.is_disabled {
            let key_select = self.on_select.clone();
            let key_navigate = self.on_navigate.clone();
            let key_year = year;
            let key_month = month;
            let key_days = days_count;
            let key_selected = selected_date.clone();
            let key_prev = prev_month_str.clone();
            let key_next = next_month_str.clone();

            cal = cal.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                match key {
                    "left" | "right" | "up" | "down" => {
                        let current_day = key_selected
                            .as_deref()
                            .and_then(|s| {
                                let parts: Vec<&str> = s.split('-').collect();
                                if parts.len() == 3 {
                                    let sy = parts[0].parse::<i32>().ok()?;
                                    let sm = parts[1].parse::<u32>().ok()?;
                                    let sd = parts[2].parse::<u32>().ok()?;
                                    if sy == key_year && sm == key_month {
                                        Some(sd)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(1);

                        let delta: i32 = match key {
                            "left" => -1,
                            "right" => 1,
                            "up" => -7,
                            "down" => 7,
                            _ => 0,
                        };

                        let new_day = current_day as i32 + delta;
                        if new_day >= 1 && new_day <= key_days as i32 {
                            if let Some(ref handler) = key_select {
                                let date_str =
                                    format!("{:04}-{:02}-{:02}", key_year, key_month, new_day);
                                handler(&date_str, window, cx);
                            }
                        } else if new_day < 1 {
                            if let Some(ref handler) = key_navigate {
                                handler(&key_prev, window, cx);
                            }
                        } else {
                            if let Some(ref handler) = key_navigate {
                                handler(&key_next, window, cx);
                            }
                        }
                    }
                    "pageup" => {
                        if let Some(ref handler) = key_navigate {
                            handler(&key_prev, window, cx);
                        }
                    }
                    "pagedown" => {
                        if let Some(ref handler) = key_navigate {
                            handler(&key_next, window, cx);
                        }
                    }
                    _ => {}
                }
            });
        }

        // Weekday headers row
        // Contract: weekday font 0.6875rem (11px), weight 600, uppercase
        let mut header_row = div().flex().gap(px(rem_to_px(0.125)));
        for i in 0..7u32 {
            let idx = ((i + week_start_offset) % 7) as usize;
            header_row = header_row.child(
                div()
                    .w(cell_size)
                    .h(px(rem_to_px(1.5)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(caption_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_secondary)
                    .child(WEEKDAYS_SUN[idx].to_uppercase()),
            );
        }
        cal = cal.child(header_row);

        // Day grid — build rows of 7
        let start_offset = (first_dow + 7 - week_start_offset) % 7;
        let total_cells = start_offset + days_count;
        let rows = (total_cells + 6) / 7;

        // Compute previous month's days for leading outside-month cells
        let (prev_year, prev_month) = if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        };
        let prev_month_days = Self::days_in_month(prev_year, prev_month);

        for row in 0..rows {
            let mut day_row = div().flex().gap(px(rem_to_px(0.125)));
            for col in 0..7u32 {
                let cell_idx = row * 7 + col;
                if cell_idx < start_offset {
                    // Outside-month cell (previous month)
                    let outside_day = prev_month_days - (start_offset - cell_idx - 1);
                    day_row = day_row.child(
                        div()
                            .w(cell_size)
                            .h(cell_size)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(control_radius)
                            .text_size(body_size)
                            .text_color(text_secondary.opacity(0.4))
                            .child(format!("{}", outside_day)),
                    );
                } else if cell_idx >= start_offset + days_count {
                    // Outside-month cell (next month)
                    let outside_day = cell_idx - start_offset - days_count + 1;
                    day_row = day_row.child(
                        div()
                            .w(cell_size)
                            .h(cell_size)
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(control_radius)
                            .text_size(body_size)
                            .text_color(text_secondary.opacity(0.4))
                            .child(format!("{}", outside_day)),
                    );
                } else {
                    let day_num = cell_idx - start_offset + 1;
                    let date_iso = format!("{:04}-{:02}-{:02}", year, month, day_num);
                    let is_selected = !is_range_mode && selected_day == Some(day_num);
                    let is_today = today_day == Some(day_num);

                    // Range-mode state for this cell
                    let is_range_start =
                        is_range_mode && range_start_iso.as_deref() == Some(&date_iso);
                    let is_range_end = is_range_mode && range_end_iso.as_deref() == Some(&date_iso);
                    let is_in_range = is_range_mode
                        && match (range_start_iso.as_deref(), range_end_iso.as_deref()) {
                            (Some(s), Some(e)) => date_iso.as_str() > s && date_iso.as_str() < e,
                            _ => false,
                        };
                    let is_range_edge = is_range_start || is_range_end;

                    let cell_id = SharedString::from(format!("poodle-cal-day-{}", day_num));
                    let mut cell = div()
                        .id(cell_id)
                        .focusable()
                        .w(cell_size)
                        .h(cell_size)
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(control_radius)
                        .text_size(body_size);

                    if is_selected || is_range_edge {
                        // Solid accent pill for selected / range edge days.
                        cell = cell
                            .bg(accent)
                            .text_color(text_inverse)
                            .font_weight(FontWeight::SEMIBOLD);
                    } else if is_in_range {
                        // Interior range days: accent 20% mix with surface.
                        let in_range_bg = color_mix(accent, surface_bg, 0.20);
                        cell = cell.bg(in_range_bg).text_color(text_primary);
                        if is_today {
                            cell = cell
                                .border_1()
                                .border_color(today_border)
                                .font_weight(FontWeight::SEMIBOLD);
                        }
                    } else {
                        cell = cell.text_color(text_primary);
                        // Contract: today = border ring around cell
                        if is_today {
                            cell = cell
                                .border_1()
                                .border_color(today_border)
                                .font_weight(FontWeight::SEMIBOLD);
                        }
                        // Contract: hover = color-mix(accent 8%, surface)
                        cell = cell.hover(move |s| s.bg(hover_bg));
                    }

                    if spec.is_disabled {
                        cell = cell.cursor(CursorStyle::OperationNotAllowed);
                    } else {
                        cell = cell.cursor_pointer();
                    }

                    // Wire click handler — range mode computes a new
                    // DateRangeValue and fires on_range_select; single
                    // mode fires on_select with the ISO date.
                    if !spec.is_disabled {
                        if is_range_mode {
                            if let Some(ref handler) = self.on_range_select {
                                let handler = handler.clone();
                                let current_start = range_start_iso.clone();
                                let current_end = range_end_iso.clone();
                                let date_clicked = date_iso.clone();
                                cell = cell.on_click(move |_event, window, cx| {
                                    let next = compute_next_range(
                                        current_start.as_deref(),
                                        current_end.as_deref(),
                                        &date_clicked,
                                    );
                                    handler(&next, window, cx);
                                });
                            }
                        } else if let Some(ref handler) = self.on_select {
                            let handler = handler.clone();
                            let date_str = date_iso.clone();
                            cell = cell.on_click(move |_event, window, cx| {
                                handler(&date_str, window, cx);
                            });
                        }
                    }

                    cell = cell.child(format!("{}", day_num));
                    day_row = day_row.child(cell);
                }
            }
            cal = cal.child(day_row);
        }

        cal.into_any_element()
    }
}

/// Compute the next range value after a click in range mode. ISO date
/// strings are lexicographically comparable so raw `<`/`>` works.
///
/// - First click (start=None): sets start = clicked, end = None.
/// - Second click (start=Some, end=None):
///   * if clicked >= start → end = clicked.
///   * if clicked < start → swap so start = clicked, end = (old start).
/// - Third click (start=Some, end=Some): reset to start = clicked, end = None.
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
