use crate::types::{Dimension, Inset, PaddingScale};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridSpec {
    pub columns: Dimension,
    pub rows: Option<Dimension>,
    pub gap: PaddingScale,
    pub padding: PaddingScale,
    pub role: Option<String>,
}

impl Default for GridSpec {
    fn default() -> Self {
        Self {
            columns: Dimension::from("1fr"),
            rows: None,
            gap: PaddingScale::Md,
            padding: PaddingScale::None,
            role: None,
        }
    }
}

impl GridSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, columns: impl Into<Dimension>) -> Self {
        self.columns = columns.into();
        self
    }

    pub fn with_rows(mut self, rows: impl Into<Dimension>) -> Self {
        self.rows = Some(rows.into());
        self
    }

    pub fn with_gap(mut self, gap: PaddingScale) -> Self {
        self.gap = gap;
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

    pub fn resolved_column_gap(&self) -> Option<&'static str> {
        self.gap.inline_gap()
    }

    pub fn resolved_row_gap(&self) -> Option<&'static str> {
        self.gap.stack_gap()
    }

    pub fn resolved_padding(&self) -> Inset {
        self.padding.layout_inset()
    }
}
