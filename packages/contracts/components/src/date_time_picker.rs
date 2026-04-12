use crate::types::{
    CalendarWeekStart, ControlDensity, ControlSize, DateTimeValue, SemanticControlSizeRole,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimePickerSpec {
    pub value: Option<DateTimeValue>,
    pub default_value: DateTimeValue,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placeholder: String,
    pub week_starts_on: CalendarWeekStart,
    pub locale: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for DateTimePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: DateTimeValue::new(None, None),
            open: None,
            default_open: false,
            placeholder: String::from("Select date and time"),
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

impl DateTimePickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_value(mut self, default_value: DateTimeValue) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn current_value(&self) -> &DateTimeValue {
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
