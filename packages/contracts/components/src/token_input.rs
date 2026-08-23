use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenInputSpec {
    pub id: String,
    pub values: Vec<String>,
    pub name: Option<String>,
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub aria_label: Option<String>,
    pub described_by: Option<String>,
    /// `None` inherits from the presentation context; an explicit value wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value wins.
    pub density: Option<ControlDensity>,
    pub separators: Vec<String>,
    pub dedupe: bool,
    pub commit_on_blur: bool,
    pub max_length: Option<usize>,
}

impl Default for TokenInputSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            values: Vec::new(),
            name: None,
            placeholder: None,
            disabled: false,
            read_only: false,
            required: false,
            aria_label: None,
            described_by: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            separators: vec![String::from(",")],
            dedupe: true,
            commit_on_blur: true,
            max_length: None,
        }
    }
}

impl TokenInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_values(mut self, values: Vec<String>) -> Self {
        self.values = values;
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    pub fn can_edit(&self) -> bool {
        !self.disabled && !self.read_only
    }
}
