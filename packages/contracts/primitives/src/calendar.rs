use crate::types::{CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
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
            value: None,
            default_value: None,
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

impl CalendarSpec {
    pub fn new() -> Self {
        Self::default()
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

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
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
