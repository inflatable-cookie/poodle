use crate::types::{CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeCalendarSpec {
    pub value: Option<DateRangeValue>,
    pub default_value: DateRangeValue,
    pub visible_month: Option<String>,
    pub week_starts_on: CalendarWeekStart,
    pub locale: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for RangeCalendarSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: DateRangeValue::new(None, None),
            visible_month: None,
            week_starts_on: CalendarWeekStart::Sunday,
            locale: String::from("en-GB"),
            is_disabled: false,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl RangeCalendarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_value(mut self, default_value: DateRangeValue) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn current_value(&self) -> &DateRangeValue {
        self.value.as_ref().unwrap_or(&self.default_value)
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
