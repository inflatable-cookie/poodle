use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, DialogKind, DialogWidth, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialogSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub kind: DialogKind,
    pub dismiss_on_escape: bool,
    pub dismiss_on_backdrop: bool,
    pub aria_label: Option<String>,
    /// Width preset for the dialog surface. Defaults to Md (34rem).
    pub width: DialogWidth,
    /// When true, strips the default chrome (title/description/padding)
    /// so the consumer can render fully custom content end-to-end.
    pub bare: bool,
    /// When true, renders a close affordance (×) in the header area.
    pub show_close_button: bool,
    /// Accessible label applied to the close button.
    pub close_label: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for DialogSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            title: None,
            description: None,
            kind: DialogKind::Dialog,
            dismiss_on_escape: true,
            dismiss_on_backdrop: true,
            aria_label: None,
            width: DialogWidth::Md,
            bare: false,
            show_close_button: false,
            close_label: "Close dialog".to_string(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl DialogSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_kind(mut self, kind: DialogKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_dismiss_on_escape(mut self, dismiss_on_escape: bool) -> Self {
        self.dismiss_on_escape = dismiss_on_escape;
        self
    }

    pub fn with_dismiss_on_backdrop(mut self, dismiss_on_backdrop: bool) -> Self {
        self.dismiss_on_backdrop = dismiss_on_backdrop;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_width(mut self, width: DialogWidth) -> Self {
        self.width = width;
        self
    }

    pub fn with_bare(mut self, bare: bool) -> Self {
        self.bare = bare;
        self
    }

    pub fn with_show_close_button(mut self, show_close_button: bool) -> Self {
        self.show_close_button = show_close_button;
        self
    }

    pub fn with_close_label(mut self, close_label: impl Into<String>) -> Self {
        self.close_label = close_label.into();
        self
    }

    /// Surface width in rem for the configured width preset. Matches the
    /// Svelte CSS values exactly (`min(<rem>, 100%)` — the caller clamps
    /// against the viewport).
    pub fn surface_width_rem(&self) -> f32 {
        match self.width {
            DialogWidth::Sm => 24.0,
            DialogWidth::Md => 34.0,
            DialogWidth::Lg => 48.0,
            DialogWidth::Xl => 64.0,
            DialogWidth::Full => f32::INFINITY,
        }
    }

    pub fn is_full_width(&self) -> bool {
        matches!(self.width, DialogWidth::Full)
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn is_alert_dialog(&self) -> bool {
        self.kind == DialogKind::AlertDialog
    }

    pub fn effective_dismiss_on_backdrop(&self) -> bool {
        self.dismiss_on_backdrop && !self.is_alert_dialog()
    }

    pub fn requires_accessible_name(&self) -> bool {
        self.title
            .as_ref()
            .map(|title| title.trim().is_empty())
            .unwrap_or(true)
            && self
                .aria_label
                .as_ref()
                .map(|label| label.trim().is_empty())
                .unwrap_or(true)
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn backdrop_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_DIALOG
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
