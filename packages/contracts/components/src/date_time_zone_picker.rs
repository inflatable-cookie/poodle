use crate::types::{
    CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole, TimeZoneOption,
    ZonedDateTimeValue,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeZonePickerSpec {
    pub value: Option<ZonedDateTimeValue>,
    pub default_value: ZonedDateTimeValue,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placeholder: String,
    pub week_starts_on: CalendarWeekStart,
    pub locale: String,
    pub time_zone_options: Vec<TimeZoneOption>,
    pub is_disabled: bool,
    /// Whether the composed time-zone Select's option list is open.
    ///
    /// Native-only: on the web the Select owns its popup, but the native
    /// hosts hold every piece of state, including this nested one.
    pub zone_open: bool,
    pub aria_label: Option<String>,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub density: Option<ControlDensity>,
}

impl Default for DateTimeZonePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: ZonedDateTimeValue::default(),
            open: None,
            default_open: false,
            placeholder: String::from("Select date, time, and zone"),
            week_starts_on: CalendarWeekStart::Monday,
            locale: String::from("en-US"),
            time_zone_options: Vec::new(),
            is_disabled: false,
            zone_open: false,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl DateTimeZonePickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    // ── Value ─────────────────────────────────────────────────
    pub fn with_value(mut self, value: ZonedDateTimeValue) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_default_value(mut self, default_value: ZonedDateTimeValue) -> Self {
        self.default_value = default_value;
        self
    }

    /// Effective value: controlled `value` if set, otherwise `default_value`.
    /// Partial values (date set but time/zone empty, etc.) are preserved.
    pub fn current_value(&self) -> &ZonedDateTimeValue {
        self.value.as_ref().unwrap_or(&self.default_value)
    }

    /// True when the effective value has no committed constituent field.
    pub fn is_placeholder(&self) -> bool {
        self.current_value().is_empty()
    }

    // ── Open state ────────────────────────────────────────────
    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Effective open state: controlled `open` if set, otherwise `default_open`.
    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    // ── Remaining props ───────────────────────────────────────
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_week_starts_on(mut self, week_starts_on: CalendarWeekStart) -> Self {
        self.week_starts_on = week_starts_on;
        self
    }

    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }

    pub fn with_time_zone_options(mut self, options: Vec<TimeZoneOption>) -> Self {
        self.time_zone_options = options;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_zone_open(mut self, zone_open: bool) -> Self {
        self.zone_open = zone_open;
        self
    }

    // ── Token targets ─────────────────────────────────────────
    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}
