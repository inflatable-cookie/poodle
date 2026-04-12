use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// FormDialog — dialog wrapping a form with submit/cancel actions.
///
/// Composes Dialog + FormLayout. Matches
/// docs/contracts/components/form-dialog.md.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormDialogSpec {
    pub open: Option<bool>,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub submit_label: String,
    pub cancel_label: String,
    pub is_submitting: bool,
    pub is_disabled: bool,
    pub error: Option<String>,
    pub success: Option<String>,
    pub aria_label: Option<String>,
    /// Number of FormLayout columns (ignored when bare is true).
    pub columns: u32,
    /// When true, skip the FormLayout wrapper and suppress default
    /// action buttons. For custom body layouts.
    pub is_bare: bool,
    /// When true (default), render built-in Submit/Cancel row.
    pub show_default_actions: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for FormDialogSpec {
    fn default() -> Self {
        Self {
            open: None,
            title: String::new(),
            subtitle: None,
            description: None,
            submit_label: String::from("Submit"),
            cancel_label: String::from("Cancel"),
            is_submitting: false,
            is_disabled: false,
            error: None,
            success: None,
            aria_label: None,
            columns: 6,
            is_bare: false,
            show_default_actions: true,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl FormDialogSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn with_open(mut self, open: bool) -> Self { self.open = Some(open); self }
    pub fn with_subtitle(mut self, v: impl Into<String>) -> Self { self.subtitle = Some(v.into()); self }
    pub fn with_description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn with_submit_label(mut self, v: impl Into<String>) -> Self { self.submit_label = v.into(); self }
    pub fn with_cancel_label(mut self, v: impl Into<String>) -> Self { self.cancel_label = v.into(); self }
    pub fn with_submitting(mut self, v: bool) -> Self { self.is_submitting = v; self }
    pub fn with_disabled(mut self, v: bool) -> Self { self.is_disabled = v; self }
    pub fn with_error(mut self, v: impl Into<String>) -> Self { self.error = Some(v.into()); self }
    pub fn with_success(mut self, v: impl Into<String>) -> Self { self.success = Some(v.into()); self }
    pub fn with_aria_label(mut self, v: impl Into<String>) -> Self { self.aria_label = Some(v.into()); self }
    pub fn with_columns(mut self, v: u32) -> Self { self.columns = v; self }
    pub fn with_bare(mut self, v: bool) -> Self { self.is_bare = v; self }
    pub fn with_show_default_actions(mut self, v: bool) -> Self { self.show_default_actions = v; self }

    /// Effective description: subtitle takes precedence, then description.
    pub fn effective_description(&self) -> Option<&str> {
        self.subtitle.as_deref().or(self.description.as_deref())
    }

    pub fn blocks_dismiss(&self) -> bool {
        self.is_submitting
    }

    pub fn with_size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self { self.size_role = size_role; self }
    pub fn with_density(mut self, density: ControlDensity) -> Self { self.density = density; self }
}
