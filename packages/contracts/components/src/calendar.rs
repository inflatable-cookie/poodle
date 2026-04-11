use crate::types::{CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue, SemanticControlSizeRole};

/// Selection mode for Calendar.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarMode {
    #[default]
    Single,
    Range,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarSpec {
    pub mode: CalendarMode,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub range_value: Option<DateRangeValue>,
    pub default_range_value: DateRangeValue,
    pub visible_month: Option<String>,
    pub week_starts_on: CalendarWeekStart,
    pub locale: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for CalendarSpec {
    fn default() -> Self {
        Self {
            mode: CalendarMode::Single,
            value: None,
            default_value: None,
            range_value: None,
            default_range_value: DateRangeValue::new(None, None),
            visible_month: None,
            week_starts_on: CalendarWeekStart::Monday,
            locale: String::from("en-US"),
            is_disabled: false,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl CalendarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_visible_month(mut self, visible_month: impl Into<String>) -> Self {
        self.visible_month = Some(visible_month.into());
        self
    }

    pub fn with_week_start(mut self, week_starts_on: CalendarWeekStart) -> Self {
        self.week_starts_on = week_starts_on;
        self
    }

    pub fn with_default_range_value(mut self, default_value: DateRangeValue) -> Self {
        self.default_range_value = default_value;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn current_range_value(&self) -> &DateRangeValue {
        self.range_value.as_ref().unwrap_or(&self.default_range_value)
    }

    pub fn effective_visible_month(&self) -> Option<&str> {
        self.visible_month.as_deref().or(self.current_value())
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
