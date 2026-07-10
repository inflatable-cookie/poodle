//! Calendar — real GPUI component backed by CalendarSpec.

use gpui::*;
use poodle_headless::date as headless_date;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CalendarMode, CalendarSpec, CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue, SemanticControlSizeRole,
};


/// Weekday header labels (Sunday-first; rotated at render time based on spec).
pub(super) const WEEKDAYS_SUN: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

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

    /// Parse "YYYY-MM-DD" and return the day number (poodle-headless,
    /// conformance-tested against the TS core).
    fn parse_day(s: &str) -> Option<u32> {
        headless_date::parse_iso_date(s).map(|date| date.day)
    }

    /// Number of days in a given month (handles leap years).
    fn days_in_month(year: i32, month: u32) -> u32 {
        headless_date::days_in_month(year, month)
    }

    /// Day-of-week for the 1st of a given month (0 = Sunday).
    fn first_day_of_week(year: i32, month: u32) -> u32 {
        headless_date::weekday(headless_date::IsoDate { year, month, day: 1 })
    }

    /// Convert days since Unix epoch to (year, month, day).
    fn days_to_ymd(days: i64) -> (i32, u32, u32) {
        let date = headless_date::from_epoch_days(days);
        (date.year, date.month, date.day)
    }
}


impl IntoElement for Calendar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.render()
    }
}

mod render;
