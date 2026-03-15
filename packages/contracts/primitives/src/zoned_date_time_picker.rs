use pug_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct ZonedDateTimePickerSpec {
    pub value: Option<String>,
    pub time_zone: Option<String>,
    pub is_open: bool,
    pub is_disabled: bool,
}

impl Default for ZonedDateTimePickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            time_zone: None,
            is_open: false,
            is_disabled: false,
        }
    }
}

impl ZonedDateTimePickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
}
