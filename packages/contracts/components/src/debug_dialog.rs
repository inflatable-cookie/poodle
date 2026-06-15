use crate::{ButtonVariant, ControlSize};

#[derive(Clone, Debug)]
pub struct DebugDialogSpec {
    pub value: Option<String>,
    pub title: String,
    pub trigger_label: String,
    pub max_height: String,
    pub trigger_variant: ButtonVariant,
    pub trigger_size: Option<ControlSize>,
    pub show_close_button: bool,
    pub close_label: String,
}

impl Default for DebugDialogSpec {
    fn default() -> Self {
        Self {
            value: None,
            title: String::from("Debug data"),
            trigger_label: String::from("View debug data"),
            max_height: String::from("min(60vh, 32rem)"),
            trigger_variant: ButtonVariant::Ghost,
            trigger_size: Some(ControlSize::Sm),
            show_close_button: true,
            close_label: String::from("Close debug dialog"),
        }
    }
}

impl DebugDialogSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_trigger_label(mut self, trigger_label: impl Into<String>) -> Self {
        self.trigger_label = trigger_label.into();
        self
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}
