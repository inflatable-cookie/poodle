use crate::types::CalendarWeekStart;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatePickerSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placeholder: String,
    pub week_starts_on: CalendarWeekStart,
    pub locale: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl Default for DatePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            open: None,
            default_open: false,
            placeholder: String::from("Select date"),
            week_starts_on: CalendarWeekStart::Sunday,
            locale: String::from("en-GB"),
            is_disabled: false,
            aria_label: None,
        }
    }
}

impl DatePickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }
}
