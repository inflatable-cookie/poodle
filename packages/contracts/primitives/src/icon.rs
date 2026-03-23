use flint_tokens::semantic;

use crate::types::ControlSize;

/// Icon size variants matching the contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum IconSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl IconSize {
    pub fn size_token(self) -> &'static str {
        match self {
            Self::Sm => semantic::SIZE_ICON_SM,
            Self::Md => semantic::SIZE_ICON_MD,
            Self::Lg => semantic::SIZE_ICON_LG,
        }
    }
}

impl From<ControlSize> for IconSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm => IconSize::Sm,
            ControlSize::Md => IconSize::Md,
            ControlSize::Lg => IconSize::Lg,
        }
    }
}

/// Spec for the Icon component per the contract.
///
/// A sized, accessible inline SVG icon element that resolves names
/// from an icon registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconSpec {
    /// Icon registry identifier (e.g., "plus", "chevron-down").
    pub name: String,
    /// Icon dimensions.
    pub size: IconSize,
    /// Accessible name; absence triggers decorative mode.
    pub aria_label: Option<String>,
}

impl IconSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: IconSize::default(),
            aria_label: None,
        }
    }

    pub fn with_size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Whether this icon is decorative (no accessible name).
    pub fn is_decorative(&self) -> bool {
        self.aria_label.is_none()
    }

    /// The size token for resolving dimensions.
    pub fn size_token(&self) -> &'static str {
        self.size.size_token()
    }
}
