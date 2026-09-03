use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, DialogKind, DialogWidth, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DialogSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    /// ARIA role for the dialog element. Use `DialogKind::AlertDialog`
    /// for alert dialogs that require an immediate response.
    /// Matches Svelte `role` prop. The legacy field name `kind` is deprecated.
    pub role: DialogKind,
    pub dismiss_on_escape: bool,
    pub dismiss_on_backdrop: bool,
    /// Layer-level outside dismissal. Defaults off: a modal that vanishes on
    /// an outside click loses work, and the backdrop click (guarded by
    /// `dismiss_on_backdrop`) is the modal's own dismissal path. When true, a
    /// document-level outside mousedown dismisses through the layer's escape
    /// path, still guarded by `dismiss_on_escape`.
    pub dismiss_on_outside_interact: bool,
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
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub density: Option<ControlDensity>,
}

impl Default for DialogSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            title: None,
            description: None,
            role: DialogKind::Dialog,
            dismiss_on_escape: true,
            dismiss_on_backdrop: true,
            dismiss_on_outside_interact: false,
            aria_label: None,
            width: DialogWidth::Md,
            bare: false,
            show_close_button: false,
            close_label: "Close dialog".to_string(),
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
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

    pub fn with_role(mut self, role: DialogKind) -> Self {
        self.role = role;
        self
    }

    /// Deprecated — use `with_role` instead.
    #[deprecated(note = "Use with_role instead")]
    pub fn with_kind(mut self, kind: DialogKind) -> Self {
        self.role = kind;
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

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
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
        self.role == DialogKind::AlertDialog
    }

    /// Whether a backdrop click should dismiss.
    ///
    /// The role does **not** enter into it. This used to read
    /// `dismiss_on_backdrop && !is_alert_dialog()`, which contradicted the
    /// Svelte reference: `AlertDialog.svelte` passes
    /// `dismissOnBackdrop={!working}`, so an alert dialog dismisses on backdrop
    /// exactly like any other dialog and stops only while its confirm is in
    /// flight. The carve-out made every native alert dialog undismissable —
    /// found by a Jetstream click test, not by inspection.
    pub fn effective_dismiss_on_backdrop(&self) -> bool {
        self.dismiss_on_backdrop
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_spec_and_builder_methods() {
        let default_spec = DialogSpec::default();
        assert_eq!(default_spec.open, None);
        assert!(!default_spec.default_open);
        assert_eq!(default_spec.title, None);
        assert_eq!(default_spec.description, None);
        assert_eq!(default_spec.role, DialogKind::Dialog);
        assert!(default_spec.dismiss_on_escape);
        assert!(default_spec.dismiss_on_backdrop);
        assert!(!default_spec.dismiss_on_outside_interact);
        assert_eq!(default_spec.aria_label, None);
        assert_eq!(default_spec.width, DialogWidth::Md);
        assert!(!default_spec.bare);
        assert!(!default_spec.show_close_button);
        assert_eq!(default_spec.close_label, "Close dialog");
        assert_eq!(default_spec.size, None);
        assert_eq!(default_spec.size_role, SemanticControlSizeRole::Control);
        assert_eq!(default_spec.density, None);

        assert_eq!(default_spec.surface_width_rem(), 34.0);
        assert!(!default_spec.is_full_width());
        assert!(!default_spec.current_open());
        assert!(!default_spec.is_alert_dialog());
        assert!(default_spec.effective_dismiss_on_backdrop());
        assert!(default_spec.requires_accessible_name());

        let built = DialogSpec::new()
            .with_open(true)
            .with_default_open(true)
            .with_title("Delete repository?")
            .with_description("This cannot be undone.")
            .with_role(DialogKind::AlertDialog)
            .with_dismiss_on_escape(false)
            .with_dismiss_on_backdrop(false)
            .with_dismiss_on_outside_interact(true)
            .with_aria_label("Delete confirmation")
            .with_width(DialogWidth::Lg)
            .with_bare(true)
            .with_show_close_button(true)
            .with_close_label("Dismiss modal")
            .with_size(ControlSize::Lg)
            .with_size_role(SemanticControlSizeRole::Prominent)
            .with_density(ControlDensity::Comfortable);

        assert_eq!(built.open, Some(true));
        assert!(built.default_open);
        assert_eq!(built.title.as_deref(), Some("Delete repository?"));
        assert_eq!(built.description.as_deref(), Some("This cannot be undone."));
        assert_eq!(built.role, DialogKind::AlertDialog);
        assert!(!built.dismiss_on_escape);
        assert!(!built.dismiss_on_backdrop);
        assert!(built.dismiss_on_outside_interact);
        assert_eq!(built.aria_label.as_deref(), Some("Delete confirmation"));
        assert_eq!(built.width, DialogWidth::Lg);
        assert!(built.bare);
        assert!(built.show_close_button);
        assert_eq!(built.close_label, "Dismiss modal");
        assert_eq!(built.size, Some(ControlSize::Lg));
        assert_eq!(built.size_role, SemanticControlSizeRole::Prominent);
        assert_eq!(built.density, Some(ControlDensity::Comfortable));

        assert_eq!(built.surface_width_rem(), 48.0);
        assert!(!built.is_full_width());
        assert!(built.current_open());
        assert!(built.is_alert_dialog());
        assert!(!built.effective_dismiss_on_backdrop());
        assert!(!built.requires_accessible_name());
    }

    #[test]
    fn width_presets_match_contract_dimensions() {
        assert_eq!(DialogSpec::new().with_width(DialogWidth::Sm).surface_width_rem(), 24.0);
        assert_eq!(DialogSpec::new().with_width(DialogWidth::Md).surface_width_rem(), 34.0);
        assert_eq!(DialogSpec::new().with_width(DialogWidth::Lg).surface_width_rem(), 48.0);
        assert_eq!(DialogSpec::new().with_width(DialogWidth::Xl).surface_width_rem(), 64.0);
        assert!(DialogSpec::new().with_width(DialogWidth::Full).is_full_width());
    }

    #[test]
    fn semantic_token_resolvers_match_contract() {
        let spec = DialogSpec::default();
        assert_eq!(spec.surface_fill_token(), semantic::COLOR_BACKGROUND_ELEVATED);
        assert_eq!(spec.backdrop_fill_token(), semantic::COLOR_BACKGROUND_OVERLAY);
        assert_eq!(spec.shadow_token(), semantic::ELEVATION_DIALOG);
    }
}
