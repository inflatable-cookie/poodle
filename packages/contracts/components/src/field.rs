use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldRelationships {
    pub described_by: Option<String>,
    pub description_id: Option<String>,
    pub error_id: Option<String>,
    pub message_id: Option<String>,
    pub validation_state: ValidationState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldSpec {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub hint: Option<String>,
    pub error: Option<String>,
    pub pending_message: Option<String>,
    pub validation_state: ValidationState,
    pub is_required: bool,
    pub optional_label: Option<String>,
    pub span: Option<String>,
    pub grid_area: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl FieldSpec {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            hint: None,
            error: None,
            pending_message: None,
            validation_state: ValidationState::None,
            is_required: false,
            optional_label: None,
            span: None,
            grid_area: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_pending_message(mut self, pending_message: impl Into<String>) -> Self {
        self.pending_message = Some(pending_message.into());
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_optional_label(mut self, optional_label: impl Into<String>) -> Self {
        self.optional_label = Some(optional_label.into());
        self
    }

    pub fn with_span(mut self, span: impl Into<String>) -> Self {
        self.span = Some(span.into());
        self
    }

    pub fn with_grid_area(mut self, grid_area: impl Into<String>) -> Self {
        self.grid_area = Some(grid_area.into());
        self
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

    pub fn info_text(&self) -> Option<&str> {
        self.description.as_deref().or(self.hint.as_deref())
    }

    pub fn description_id(&self) -> Option<String> {
        self.description
            .as_ref()
            .map(|_| format!("{}-description", self.id))
    }

    pub fn error_id(&self) -> Option<String> {
        self.error.as_ref().map(|_| format!("{}-error", self.id))
    }

    pub fn pending_id(&self) -> Option<String> {
        self.pending_message
            .as_ref()
            .map(|_| format!("{}-pending", self.id))
    }

    pub fn message_id(&self) -> Option<String> {
        match self.validation_state {
            ValidationState::Invalid => self.error_id(),
            ValidationState::Pending => self.pending_id(),
            ValidationState::None | ValidationState::Valid => None,
        }
    }

    pub fn described_by(&self) -> Option<String> {
        let ids = [self.description_id(), self.message_id()]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if ids.is_empty() {
            None
        } else {
            Some(ids.join(" "))
        }
    }

    pub fn relationships(&self) -> FieldRelationships {
        FieldRelationships {
            described_by: self.described_by(),
            description_id: self.description_id(),
            error_id: self.error_id(),
            message_id: self.message_id(),
            validation_state: self.validation_state,
        }
    }

    pub fn shows_optional_label(&self) -> bool {
        !self.is_required
            && self
                .optional_label
                .as_ref()
                .map(|label| !label.trim().is_empty())
                .unwrap_or(false)
    }

    pub fn label_typography_token(&self, size: ControlSize) -> &'static str {
        match size {
            ControlSize::Xs => semantic::TYPOGRAPHY_CAPTION_SIZE,
            ControlSize::Sm => semantic::TYPOGRAPHY_COUNTER_SIZE,
            ControlSize::Md => semantic::TYPOGRAPHY_LABEL_SIZE,
            ControlSize::Lg => semantic::TYPOGRAPHY_BODY_SIZE,
            ControlSize::Xl => semantic::TYPOGRAPHY_BODY_SIZE,
        }
    }

    /// Helper/optional/error/pending copy at the `md` size stop (contract §7: `0.75rem`).
    pub fn supporting_text_typography_token(&self, size: ControlSize) -> &'static str {
        match size {
            ControlSize::Xs => semantic::TYPOGRAPHY_CAPTION_SIZE,
            ControlSize::Sm => semantic::TYPOGRAPHY_CAPTION_SIZE,
            ControlSize::Md => semantic::TYPOGRAPHY_COUNTER_SIZE,
            ControlSize::Lg => semantic::TYPOGRAPHY_LABEL_SIZE,
            ControlSize::Xl => semantic::TYPOGRAPHY_BODY_SIZE,
        }
    }

    pub fn error_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Label color is `color-mix(in srgb, text-primary 45%, text-secondary)`
    /// (contract §8). These two methods expose the source tokens and
    /// [`Self::LABEL_COLOR_PRIMARY_RATIO`] the mix ratio so each target resolves
    /// an identical color instead of an opacity shortcut.
    pub fn label_color_primary_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn label_color_secondary_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Proportion of `text-primary` kept in the label-color mix (contract §8).
    pub const LABEL_COLOR_PRIMARY_RATIO: f32 = 0.45;

    /// Info-icon pill background is `color-mix(text-secondary 14%, transparent)`
    /// (contract §8). The source token is `text-secondary`; targets apply the
    /// 0.14 alpha factor at [`Self::INFO_ICON_BG_ALPHA`].
    pub fn info_icon_bg_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Info-icon pill color (contract §8: `text-secondary`).
    pub fn info_icon_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Alpha factor applied to `info_icon_bg_token` for the pill background
    /// (contract §8: `text-secondary 14%`).
    pub const INFO_ICON_BG_ALPHA: f32 = 0.14;

    pub fn info_icon_radius_token(&self) -> &'static str {
        semantic::RADIUS_PILL
    }

    /// Info-icon wrapper edge in `em` of the label font (contract §7: `1.25em`).
    pub const INFO_ICON_EM: f32 = 1.25;

    /// Info-icon SVG edge in `em` of the label font (contract §7: `0.75em`).
    pub const INFO_ICON_SVG_EM: f32 = 0.75;

    pub fn row_gap_token(&self, density: ControlDensity) -> &'static str {
        match density {
            ControlDensity::Compact => semantic::SPACE_BUTTON_GAP,
            ControlDensity::Default => semantic::SPACE_STACK_SM,
            ControlDensity::Comfortable => semantic::SPACE_STACK_MD,
        }
    }

    pub fn header_gap_token(&self, density: ControlDensity) -> &'static str {
        match density {
            ControlDensity::Compact => semantic::SPACE_INLINE_SM,
            ControlDensity::Default => semantic::SPACE_INLINE_MD,
            ControlDensity::Comfortable => semantic::SPACE_INLINE_LG,
        }
    }

    /// Inline gap inside the label row (contract §7: `0.375rem`).
    pub fn label_row_gap_token(&self, density: ControlDensity) -> &'static str {
        match density {
            ControlDensity::Compact => semantic::SPACE_INLINE_XS,
            ControlDensity::Default => semantic::SPACE_BUTTON_GAP,
            ControlDensity::Comfortable => semantic::SPACE_INLINE_SM,
        }
    }
}
