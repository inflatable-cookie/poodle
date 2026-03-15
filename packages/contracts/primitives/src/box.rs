use crate::types::{Dimension, Inset, Overflow, PaddingScale};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoxSpec {
    pub padding: PaddingScale,
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub min_width: Option<Dimension>,
    pub min_height: Option<Dimension>,
    pub overflow: Overflow,
    pub role: Option<String>,
}

impl Default for BoxSpec {
    fn default() -> Self {
        Self {
            padding: PaddingScale::None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            overflow: Overflow::Visible,
            role: None,
        }
    }
}

impl BoxSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_padding(mut self, padding: PaddingScale) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_width(mut self, width: impl Into<Dimension>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn with_height(mut self, height: impl Into<Dimension>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn with_min_width(mut self, min_width: impl Into<Dimension>) -> Self {
        self.min_width = Some(min_width.into());
        self
    }

    pub fn with_min_height(mut self, min_height: impl Into<Dimension>) -> Self {
        self.min_height = Some(min_height.into());
        self
    }

    pub fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn resolved_padding(&self) -> Inset {
        self.padding.layout_inset()
    }
}
