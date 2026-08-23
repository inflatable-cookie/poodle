use crate::types::{CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole};

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
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub density: Option<ControlDensity>,
}

impl Default for DatePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            open: None,
            default_open: false,
            placeholder: String::from("Select date"),
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
