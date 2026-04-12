/// FormLayout — responsive form grid with error/success messaging.
///
/// Matches docs/contracts/components/form-layout.md.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormLayoutSpec {
    /// Base grid columns. 6 enables mixed 2-col and 3-col layouts
    /// via the Field span prop.
    pub columns: u32,
    /// Form-level error message (displayed via Callout tone="danger").
    pub error: Option<String>,
    /// Form-level success message (displayed via Callout tone="success").
    pub success: Option<String>,
    /// Introductory text above the form grid.
    pub description: Option<String>,
}

impl Default for FormLayoutSpec {
    fn default() -> Self {
        Self {
            columns: 6,
            error: None,
            success: None,
            description: None,
        }
    }
}

impl FormLayoutSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, columns: u32) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_success(mut self, success: impl Into<String>) -> Self {
        self.success = Some(success.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn has_status_message(&self) -> bool {
        self.error.is_some() || self.success.is_some()
    }
}
