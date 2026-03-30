use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextInputSpec {
    pub id: Option<String>,
    pub value: Option<String>,
    pub default_value: String,
    pub placeholder: Option<String>,
    pub name: Option<String>,
    pub input_type: String,
    pub input_mode: Option<String>,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub validation_state: ValidationState,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
    pub error_message_id: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub max_length: Option<usize>,
    pub show_char_count: bool,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub submit_enabled: bool,
    pub cancel_enabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for TextInputSpec {
    fn default() -> Self {
        Self {
            id: None,
            value: None,
            default_value: String::new(),
            placeholder: None,
            name: None,
            input_type: String::from("text"),
            input_mode: None,
            is_disabled: false,
            is_read_only: false,
            validation_state: ValidationState::None,
            aria_label: None,
            description_id: None,
            error_message_id: None,
            prefix: None,
            suffix: None,
            max_length: None,
            show_char_count: false,
            leading_icon: None,
            trailing_icon: None,
            submit_enabled: false,
            cancel_enabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl TextInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = default_value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_input_type(mut self, input_type: impl Into<String>) -> Self {
        self.input_type = input_type.into();
        self
    }

    pub fn with_input_mode(mut self, input_mode: impl Into<String>) -> Self {
        self.input_mode = Some(input_mode.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_description_id(mut self, description_id: impl Into<String>) -> Self {
        self.description_id = Some(description_id.into());
        self
    }

    pub fn with_error_message_id(mut self, error_message_id: impl Into<String>) -> Self {
        self.error_message_id = Some(error_message_id.into());
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn with_show_char_count(mut self, show: bool) -> Self {
        self.show_char_count = show;
        self
    }

    pub fn with_leading_icon(mut self, leading_icon: impl Into<String>) -> Self {
        self.leading_icon = Some(leading_icon.into());
        self
    }

    pub fn with_trailing_icon(mut self, trailing_icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(trailing_icon.into());
        self
    }

    pub fn with_submit_enabled(mut self, submit_enabled: bool) -> Self {
        self.submit_enabled = submit_enabled;
        self
    }

    pub fn with_cancel_enabled(mut self, cancel_enabled: bool) -> Self {
        self.cancel_enabled = cancel_enabled;
        self
    }

    pub fn is_controlled(&self) -> bool {
        self.value.is_some()
    }

    pub fn current_value(&self) -> &str {
        self.value.as_deref().unwrap_or(self.default_value.as_str())
    }

    pub fn described_by(&self) -> Option<String> {
        let ids = [
            self.description_id.clone(),
            match self.validation_state {
                ValidationState::Invalid => self.error_message_id.clone(),
                ValidationState::None | ValidationState::Valid | ValidationState::Pending => None,
            },
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        if ids.is_empty() {
            None
        } else {
            Some(ids.join(" "))
        }
    }

    pub fn aria_invalid(&self) -> Option<&'static str> {
        self.validation_state.aria_invalid()
    }

    pub fn aria_busy(&self) -> Option<&'static str> {
        self.validation_state.aria_busy()
    }

    pub fn control_height_token(&self) -> &'static str {
        semantic::SIZE_CONTROL_HEIGHT
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        self.validation_state.border_token()
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn horizontal_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_X
    }

    pub fn vertical_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_Y
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn placeholder_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn inline_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn icon_color_token(&self) -> &'static str {
        semantic::COLOR_ICON_MUTED
    }

    pub fn affix_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn affix_separator_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn body_line_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_LINE_HEIGHT
    }

    pub fn char_count_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn char_count_over_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn focus_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
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
