//! RangeCalendar — real GPUI component backed by RangeCalendarSpec.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{CalendarWeekStart, DateRangeValue, IconSize, IconSpec, RangeCalendarSpec};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

const WEEKDAYS_SUN: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// A real GPUI range calendar component backed by `RangeCalendarSpec`.
///
/// Similar to `Calendar` but highlights a date range instead of a single day.
pub struct RangeCalendar {
    spec: RangeCalendarSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl std::ops::Deref for RangeCalendar {
    type Target = RangeCalendarSpec;
    fn deref(&self) -> &RangeCalendarSpec { &self.spec }
}

impl RangeCalendar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: RangeCalendarSpec::new(), theme: theme.clone(), id_suffix: None }
    }

    pub fn from_spec(spec: RangeCalendarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: DateRangeValue) -> Self { self.spec.value = Some(v); self }
    pub fn default_value(mut self, v: DateRangeValue) -> Self { self.spec.default_value = v; self }
    pub fn visible_month(mut self, v: impl Into<String>) -> Self { self.spec.visible_month = Some(v.into()); self }
    pub fn week_starts_on(mut self, v: CalendarWeekStart) -> Self { self.spec.week_starts_on = v; self }
    pub fn locale(mut self, v: impl Into<String>) -> Self { self.spec.locale = v.into(); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

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

    fn parse_day(s: &str) -> Option<u32> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() == 3 {
            parts[2].parse::<u32>().ok()
        } else {
            None
        }
    }

    /// Check if a date string matches the given year/month and return the day.
    fn day_in_month(date_str: &str, year: i32, month: u32) -> Option<u32> {
        let (y, m) = Self::parse_year_month(date_str)?;
        if y == year && m == month {
            Self::parse_day(date_str)
        } else {
            None
        }
    }

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

    fn first_day_of_week(year: i32, month: u32) -> u32 {
        let t = [0i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let y = if month < 3 { year - 1 } else { year };
        let m = month as usize;
        ((y + y / 4 - y / 100 + y / 400 + t[m - 1] + 1) % 7) as u32
    }

    /// Convert days since Unix epoch to (year, month, day).
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
}

impl IntoElement for RangeCalendar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        // Contract: hover = color-mix(accent 8%, surface)
        let hover_bg = color_mix(accent, surface_bg, 0.08);
        // Contract: in-range tint = color-mix(accent 15%, surface)
        let range_tint = color_mix(accent, surface_bg, 0.15);
        let today_border = border;

        let range = spec.current_value();

        // Determine visible month — prefer visible_month, else range start
        let visible_month_str = spec
            .visible_month
            .as_deref()
            .or(range.start.as_deref());

        let (year, month) = visible_month_str
            .and_then(Self::parse_year_month)
            .unwrap_or((2026, 1));

        // Compute today for highlighting
        let today_day: Option<u32> = {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let days_since_epoch = now / 86400;
            let (ty, tm, td) = Self::days_to_ymd(days_since_epoch as i64);
            if ty == year && tm == month { Some(td) } else { None }
        };

        let start_day = range
            .start
            .as_deref()
            .and_then(|s| Self::day_in_month(s, year, month));
        let end_day = range
            .end
            .as_deref()
            .and_then(|s| Self::day_in_month(s, year, month));

        let days_count = Self::days_in_month(year, month);
        let first_dow = Self::first_day_of_week(year, month);

        let week_start_offset: u32 = match spec.week_starts_on {
            flint_primitives::CalendarWeekStart::Monday => 1,
            _ => 0,
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("flint-range-cal-{}", suffix)
        } else {
            "flint-range-cal".to_string()
        };

        let month_names = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December",
        ];
        let month_label = format!(
            "{} {}",
            month_names.get(month as usize - 1).unwrap_or(&""),
            year
        );

        let surface_radius = resolve_radius(theme, "semantic.radius.surface");

        let mut cal = div()
            .id(SharedString::from(id_str))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(px(12.0))
            .rounded(surface_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border);

        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        cal = cal.focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            cal = cal
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // Contract: nav header with prev/next month buttons and centered month label
        let nav_btn_hover = color_mix(elevated_bg, surface_bg, 0.84);
        let nav_header = div()
            .flex()
            .items_center()
            .justify_between()
            .py(px(4.0))
            .child(
                div()
                    .id("flint-rcal-prev")
                    .w(px(28.0))
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(control_radius)
                    .cursor_pointer()
                    .hover(move |s| s.bg(nav_btn_hover))
                    .child(
                        Icon::from_spec(
                            IconSpec::new("chevron-left").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(icon_muted),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .text_center()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(month_label),
            )
            .child(
                div()
                    .id("flint-rcal-next")
                    .w(px(28.0))
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(control_radius)
                    .cursor_pointer()
                    .hover(move |s| s.bg(nav_btn_hover))
                    .child(
                        Icon::from_spec(
                            IconSpec::new("chevron-right").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(icon_muted),
                    ),
            );

        cal = cal.child(nav_header);

        // Weekday headers
        // Contract: weekday font 0.6875rem (11px), weight 600, uppercase
        let mut header_row = div().flex().gap(px(2.0));
        for i in 0..7u32 {
            let idx = ((i + week_start_offset) % 7) as usize;
            header_row = header_row.child(
                div()
                    .w(px(32.0))
                    .h(px(24.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_secondary)
                    .child(WEEKDAYS_SUN[idx].to_uppercase()),
            );
        }
        cal = cal.child(header_row);

        // Day grid
        let start_offset = (first_dow + 7 - week_start_offset) % 7;
        let total_cells = start_offset + days_count;
        let rows = (total_cells + 6) / 7;

        // Compute previous month's days for leading outside-month cells
        let (prev_year, prev_month) = if month == 1 { (year - 1, 12) } else { (year, month - 1) };
        let prev_month_days = Self::days_in_month(prev_year, prev_month);

        for row in 0..rows {
            let mut day_row = div().flex().gap(px(2.0));
            for col in 0..7u32 {
                let cell_idx = row * 7 + col;
                if cell_idx < start_offset {
                    // Outside-month cell (previous month)
                    let outside_day = prev_month_days - (start_offset - cell_idx - 1);
                    day_row = day_row.child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(control_radius)
                            .text_size(px(14.0))
                            .text_color(text_secondary.opacity(0.4))
                            .child(format!("{}", outside_day)),
                    );
                } else if cell_idx >= start_offset + days_count {
                    // Outside-month cell (next month)
                    let outside_day = cell_idx - start_offset - days_count + 1;
                    day_row = day_row.child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(control_radius)
                            .text_size(px(14.0))
                            .text_color(text_secondary.opacity(0.4))
                            .child(format!("{}", outside_day)),
                    );
                } else {
                    let day_num = cell_idx - start_offset + 1;

                    let is_start = start_day == Some(day_num);
                    let is_end = end_day == Some(day_num);
                    let is_in_range = match (start_day, end_day) {
                        (Some(s), Some(e)) => day_num >= s && day_num <= e,
                        _ => false,
                    };
                    let is_endpoint = is_start || is_end;
                    let is_today = today_day == Some(day_num);

                    let mut cell = div()
                        .w(px(32.0))
                        .h(px(32.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(14.0));

                    if is_endpoint {
                        cell = cell
                            .rounded(control_radius)
                            .bg(accent)
                            .text_color(text_inverse)
                            .font_weight(FontWeight::SEMIBOLD);
                    } else if is_in_range {
                        // Contract: in-range tint = color-mix(accent 15%, surface)
                        cell = cell
                            .bg(range_tint)
                            .text_color(text_primary);
                    } else {
                        cell = cell
                            .rounded(control_radius)
                            .text_color(text_primary);
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

                    if !spec.is_disabled {
                        cell = cell.cursor_pointer();
                    } else {
                        cell = cell.cursor(CursorStyle::OperationNotAllowed);
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
