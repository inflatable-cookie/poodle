use crate::types::{CalendarWeekStart, DateTimeValue};

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
}

impl Default for DateTimePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: DateTimeValue::new(None, None),
            open: None,
            default_open: false,
            placeholder: String::from("Select date and time"),
            week_starts_on: CalendarWeekStart::Sunday,
            locale: String::from("en-GB"),
            is_disabled: false,
            aria_label: None,
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
}
