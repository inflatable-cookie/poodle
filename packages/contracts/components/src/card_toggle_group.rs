use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardToggleOption {
    pub value: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl CardToggleOption {
    pub fn new(value: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardToggleGroupSpec {
    pub options: Vec<CardToggleOption>,
    pub values: Vec<String>,
    pub disabled: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Grid column upper bound (contract §3 `columns`, 1–4, default 2). The web target
    /// is a responsive auto-fit grid capped at this; the Rust targets render the options
    /// in rows of `column_count()` cards.
    pub columns: u32,
}

impl Default for CardToggleGroupSpec {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            values: Vec::new(),
            disabled: false,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            columns: 2,
        }
    }
}

impl CardToggleGroupSpec {
    pub fn new(options: Vec<CardToggleOption>) -> Self {
        Self {
            options,
            ..Self::default()
        }
    }

    pub fn with_values(mut self, values: Vec<String>) -> Self {
        self.values = values;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.values.iter().any(|item| item == value)
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

    pub fn with_columns(mut self, columns: u32) -> Self {
        self.columns = columns;
        self
    }

    /// Column count clamped to the contract's 1–4 range.
    pub fn column_count(&self) -> usize {
        self.columns.clamp(1, 4) as usize
    }

    /// Title font-size in rem for the given effective size.
    /// Contract §7 Size Adjustments: xs 0.6875 · sm 0.75 · md 0.875 · lg 1 · xl 1.125.
    pub fn title_font_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
    }

    /// Description font-size in rem for the given effective size.
    /// Contract §7 Size Adjustments: xs 0.625 · sm 0.6875 · md 0.75 · lg 0.875 · xl 0.9375.
    pub fn description_font_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.625,
            ControlSize::Sm => 0.6875,
            ControlSize::Md => 0.75,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }
}
