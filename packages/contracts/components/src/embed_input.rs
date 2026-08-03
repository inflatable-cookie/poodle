use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

use crate::composite_types::ParsedEmbed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbedInputSpec {
    pub value: String,
    pub parsed: Option<ParsedEmbed>,
    pub placeholder: Option<String>,
    pub parse_debounce_ms: u32,
    pub providers: Vec<String>,
    pub is_disabled: bool,
    pub error: Option<String>,
    /// Presentation axes (contract §3): size is intrinsic, density is sibling
    /// spacing, size_role resolves size from the inherited presentation.
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl EmbedInputSpec {
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

    pub fn new() -> Self {
        Self {
            value: String::new(),
            parsed: None,
            placeholder: None,
            parse_debounce_ms: 300,
            providers: Vec::new(),
            is_disabled: false,
            error: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_parsed(mut self, parsed: ParsedEmbed) -> Self {
        self.parsed = Some(parsed);
        self
    }

    pub fn with_parse_debounce(mut self, parse_debounce_ms: u32) -> Self {
        self.parse_debounce_ms = parse_debounce_ms;
        self
    }

    pub fn with_providers(mut self, providers: Vec<String>) -> Self {
        self.providers = providers;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn resolved_parse_state(&self) -> (Option<ParsedEmbed>, Option<String>) {
        if self.parsed.is_some() || self.error.is_some() {
            return (self.parsed.clone(), self.error.clone());
        }

        let parsed = ParsedEmbed::detect(&self.value);

        if let Some(ref parsed) = parsed {
            if !self.providers.is_empty()
                && !self
                    .providers
                    .iter()
                    .any(|provider| provider == &parsed.provider)
            {
                return (
                    None,
                    Some(format!("Provider \"{}\" is not allowed", parsed.provider)),
                );
            }
        }

        (parsed, None)
    }

    pub fn with_detected_parse(mut self) -> Self {
        let (parsed, error) = self.resolved_parse_state();
        self.parsed = parsed;
        self.error = error;
        self
    }

    pub fn status_text_color_token(&self) -> &'static str {
        let (parsed, error) = self.resolved_parse_state();
        if error.is_some() {
            semantic::COLOR_STATUS_DANGER
        } else if parsed.is_some() {
            semantic::COLOR_STATUS_SUCCESS
        } else {
            semantic::COLOR_TEXT_SECONDARY
        }
    }

    pub fn border_token(&self) -> &'static str {
        let (parsed, error) = self.resolved_parse_state();
        if error.is_some() {
            semantic::COLOR_STATUS_DANGER
        } else if parsed.is_some() {
            semantic::COLOR_STATUS_SUCCESS
        } else {
            semantic::COLOR_BORDER_DEFAULT
        }
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
