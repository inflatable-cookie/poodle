use crate::types::{CalendarWeekStart, DateRangeValue};

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
}

impl Default for DateRangePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: DateRangeValue::new(None, None),
            open: None,
            default_open: false,
            placeholder: String::from("Select date range"),
            week_starts_on: CalendarWeekStart::Sunday,
            locale: String::from("en-GB"),
            is_disabled: false,
            aria_label: None,
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
}
