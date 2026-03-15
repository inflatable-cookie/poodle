use crate::types::{Alignment, Inset, PaddingScale};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackSpec {
    pub gap: PaddingScale,
    pub align: Alignment,
    pub padding: PaddingScale,
    pub role: Option<String>,
}

impl Default for StackSpec {
    fn default() -> Self {
        Self {
            gap: PaddingScale::Md,
            align: Alignment::Stretch,
            padding: PaddingScale::None,
            role: None,
        }
    }
}

impl StackSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gap(mut self, gap: PaddingScale) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn with_padding(mut self, padding: PaddingScale) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn resolved_gap(&self) -> Option<&'static str> {
        self.gap.stack_gap()
    }

    pub fn resolved_padding(&self) -> Inset {
        self.padding.layout_inset()
    }
}
