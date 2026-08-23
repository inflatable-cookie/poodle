use crate::types::{
    CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue, SemanticControlSizeRole,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateRangePickerSpec {
    pub value: Option<DateRangeValue>,
    pub default_value: DateRangeValue,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placeholder: String,
    pub week_starts_on: CalendarWeekStart,
    pub locale: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub density: Option<ControlDensity>,
}

impl Default for DateRangePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: DateRangeValue::new(None, None),
            open: None,
            default_open: false,
            placeholder: String::from("Select date range"),
            week_starts_on: CalendarWeekStart::Monday,
            locale: String::from("en-US"),
            is_disabled: false,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl DateRangePickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_value(mut self, default_value: DateRangeValue) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn current_value(&self) -> &DateRangeValue {
        self.value.as_ref().unwrap_or(&self.default_value)
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
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
