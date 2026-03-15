use crate::types::{CalendarWeekStart, DateTimeRangeValue, DateTimeValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeRangePickerSpec {
    pub value: Option<DateTimeRangeValue>,
    pub default_value: DateTimeRangeValue,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placeholder: String,
    pub week_starts_on: CalendarWeekStart,
    pub locale: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl Default for DateTimeRangePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: DateTimeRangeValue::new(
                DateTimeValue::new(None, None),
                DateTimeValue::new(None, None),
            ),
            open: None,
            default_open: false,
            placeholder: String::from("Select date and time range"),
            week_starts_on: CalendarWeekStart::Sunday,
            locale: String::from("en-GB"),
            is_disabled: false,
            aria_label: None,
        }
    }
}

impl DateTimeRangePickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_value(mut self, default_value: DateTimeRangeValue) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn current_value(&self) -> &DateTimeRangeValue {
        self.value.as_ref().unwrap_or(&self.default_value)
    }
}
