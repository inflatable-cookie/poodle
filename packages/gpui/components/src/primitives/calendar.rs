//! Calendar — real GPUI component backed by CalendarSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CalendarSpec, CalendarWeekStart};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

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
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Calendar {
    type Target = CalendarSpec;
    fn deref(&self) -> &CalendarSpec { &self.spec }
}

impl Calendar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CalendarSpec::new(), theme: theme.clone(), id_suffix: None, on_select: None }
    }

    pub fn from_spec(spec: CalendarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn visible_month(mut self, v: impl Into<String>) -> Self { self.spec.visible_month = Some(v.into()); self }
    pub fn week_starts_on(mut self, v: CalendarWeekStart) -> Self { self.spec.week_starts_on = v; self }
    pub fn locale(mut self, v: impl Into<String>) -> Self { self.spec.locale = v.into(); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
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
}

impl IntoElement for Calendar {
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
        let border = resolve_color(theme, "semantic.color.border.default");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let selected_date = spec.current_value().map(|s| s.to_string());
        let selected_day = selected_date.as_deref().and_then(Self::parse_day);

        // Determine which month to show
        let (year, month) = spec
            .effective_visible_month()
            .and_then(Self::parse_year_month)
            .unwrap_or((2026, 1));

        let days_count = Self::days_in_month(year, month);
        let first_dow = Self::first_day_of_week(year, month);

        // Determine week-start offset (Sunday = 0, Monday = 1)
        let week_start_offset: u32 = match spec.week_starts_on {
            pug_primitives::CalendarWeekStart::Monday => 1,
            _ => 0,
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-calendar-{}", suffix)
        } else {
            "pug-calendar".to_string()
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

        // Build the calendar container
        let mut cal = div()
            .id(SharedString::from(id_str))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(px(12.0))
            .rounded(px(8.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border);

        if spec.is_disabled {
            cal = cal.opacity(disabled_opacity);
        }

        // Month header
        cal = cal.child(
            div()
                .flex()
                .justify_center()
                .py(px(4.0))
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(month_label),
        );

        // Weekday headers row
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
                    .text_xs()
                    .text_color(text_secondary)
                    .child(WEEKDAYS_SUN[idx]),
            );
        }
        cal = cal.child(header_row);

        // Day grid — build rows of 7
        let start_offset = (first_dow + 7 - week_start_offset) % 7;
        let total_cells = start_offset + days_count;
        let rows = (total_cells + 6) / 7;

        for row in 0..rows {
            let mut day_row = div().flex().gap(px(2.0));
            for col in 0..7u32 {
                let cell_idx = row * 7 + col;
                if cell_idx < start_offset || cell_idx >= start_offset + days_count {
                    // Empty cell
                    day_row = day_row.child(div().w(px(32.0)).h(px(32.0)));
                } else {
                    let day_num = cell_idx - start_offset + 1;
                    let is_selected = selected_day == Some(day_num);

                    let cell_id = SharedString::from(format!("pug-cal-day-{}", day_num));
                    let mut cell = div()
                        .id(cell_id)
                        .w(px(32.0))
                        .h(px(32.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(control_radius)
                        .text_sm();

                    if is_selected {
                        cell = cell
                            .bg(accent)
                            .text_color(text_inverse)
                            .font_weight(FontWeight::SEMIBOLD);
                    } else {
                        cell = cell
                            .text_color(text_primary)
                            .hover(|s| s.bg(accent.opacity(0.08)));
                    }

                    if !spec.is_disabled {
                        cell = cell.cursor_pointer();
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
