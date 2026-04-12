use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// How the user enters edit mode. Matches `activationMode` prop.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EditableLabelActivation {
    #[default]
    DoubleClick,
    EnterOrSpace,
    Programmatic,
}

/// Visual treatment for the label container.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EditableLabelVariant {
    /// Default: has padding and border in edit mode.
    #[default]
    Default,
    /// Flush: no padding or border, text renders inline.
    Flush,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditableLabelSpec {
    pub value: String,
    pub placeholder: Option<String>,
    pub is_editing: bool,
    pub is_disabled: bool,
    /// How the user enters edit mode.
    pub activation_mode: EditableLabelActivation,
    /// Select all text when editing begins.
    pub select_on_focus: bool,
    /// Visual variant — default has border/padding, flush is inline.
    pub variant: EditableLabelVariant,
    /// Italic text shown when value is empty (display mode only).
    pub empty_text: Option<String>,
    /// Maximum character count for the input.
    pub max_length: Option<usize>,
    /// Show pencil icon on hover/focus to signal editability.
    pub show_edit_icon: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for EditableLabelSpec {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            is_editing: false,
            is_disabled: false,
            activation_mode: EditableLabelActivation::DoubleClick,
            select_on_focus: true,
            variant: EditableLabelVariant::Default,
            empty_text: None,
            max_length: None,
            show_edit_icon: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl EditableLabelSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_editing(mut self, is_editing: bool) -> Self {
        self.is_editing = is_editing;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_activation_mode(mut self, mode: EditableLabelActivation) -> Self {
        self.activation_mode = mode;
        self
    }

    pub fn with_select_on_focus(mut self, v: bool) -> Self {
        self.select_on_focus = v;
        self
    }

    pub fn with_variant(mut self, variant: EditableLabelVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = Some(text.into());
        self
    }

    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn with_show_edit_icon(mut self, v: bool) -> Self {
        self.show_edit_icon = v;
        self
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn edit_border_token(&self) -> &'static str {
        if self.is_editing {
            semantic::COLOR_ACCENT_FOCUS_RING
        } else {
            semantic::COLOR_BORDER_SUBTLE
        }
    }

    pub fn placeholder_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn fill_token(&self) -> &'static str {
        if self.is_editing {
            semantic::COLOR_BACKGROUND_SURFACE
        } else {
            // transparent in display mode
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn body_line_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_LINE_HEIGHT
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
