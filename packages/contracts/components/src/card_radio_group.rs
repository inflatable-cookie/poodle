use crate::{ChoiceOption, ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardRadioGroupSpec {
    pub options: Vec<ChoiceOption>,
    pub default_value: Option<String>,
    pub value: Option<String>,
    pub is_disabled: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Grid column count for the card layout.
    pub columns: u8,
}

impl CardRadioGroupSpec {
    pub fn with_columns(mut self, value: u8) -> Self {
        self.columns = value;
        self
    }

    pub fn new(options: Vec<ChoiceOption>) -> Self {
        Self {
            options,
            default_value: None,
            value: None,
            is_disabled: false,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            columns: 2,
        }
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn selected_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// Radio indicator outer-circle size in rem for the given effective size.
    /// Contract §7 / §8 size table: xs 0.875 · sm 1 · md 1.125 · lg 1.25 · xl 1.375.
    pub fn indicator_size_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.875,
            ControlSize::Sm => 1.0,
            ControlSize::Md => 1.125,
            ControlSize::Lg => 1.25,
            ControlSize::Xl => 1.375,
        }
    }

    /// Radio indicator inner-dot size in rem for the given effective size.
    /// Contract §8 size table: xs 0.25 · sm 0.375 · md 0.375 · lg 0.4375 · xl 0.5.
    pub fn dot_size_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.25,
            ControlSize::Sm => 0.375,
            ControlSize::Md => 0.375,
            ControlSize::Lg => 0.4375,
            ControlSize::Xl => 0.5,
        }
    }

    /// Title font-size in rem for the given effective size.
    /// Contract §8 size table: xs 0.75 · sm 0.8125 · md 0.9375 · lg 1.0625 · xl 1.125.
    pub fn title_font_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.9375,
            ControlSize::Lg => 1.0625,
            ControlSize::Xl => 1.125,
        }
    }

    /// Description font-size in rem for the given effective size.
    /// Contract §8 size table: xs 0.6875 · sm 0.75 · md 0.8125 · lg 0.875 · xl 0.9375.
    pub fn description_font_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    /// Indicator border width in rem (contract §8: `0.125rem solid`).
    pub fn indicator_border_rem(&self) -> f32 {
        0.125
    }

    pub fn unselected_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
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
