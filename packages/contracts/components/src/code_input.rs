use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};
use poodle_tokens::semantic;

/// CodeInput -- a segmented code entry field with visual digit slots.
///
/// Consolidates the former PinInput and TotpInput into a single component.
/// Use `mask: true` for PIN-style masked entry, `mask: false` (default) for
/// visible code entry (OTP, verification codes, etc.).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeInputSpec {
    pub length: usize,
    pub value: Option<String>,
    pub default_value: String,
    pub name: String,
    pub label: String,
    pub hint: Option<String>,
    pub error: Option<String>,
    pub mask: bool,
    /// Caret/selection across the slots, as character indices into the value.
    ///
    /// **Rust targets only.** The web target hides a real `<input>` behind the
    /// slots and lets the browser own the caret; with no such input the host
    /// owns it, the same way it owns `TextInputSpec::selection_start`.
    /// `selection_start == selection_end` is a plain caret; a one-wide range is
    /// a selected slot, which is what clicking a filled slot produces.
    pub selection_start: usize,
    pub selection_end: usize,
    /// When true (default), the value sanitizes to digits only. Set false to
    /// allow arbitrary alphanumeric characters. Mirrors the Svelte
    /// `numbersOnly` prop.
    pub numbers_only: bool,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub autocomplete: String,
    pub validation_state: ValidationState,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for CodeInputSpec {
    fn default() -> Self {
        Self {
            length: 6,
            value: None,
            default_value: String::new(),
            name: String::from("code"),
            label: String::from("Authenticator code"),
            hint: None,
            error: None,
            mask: false,
            selection_start: 0,
            selection_end: 0,
            numbers_only: true,
            is_disabled: false,
            aria_label: None,
            autocomplete: String::from("one-time-code"),
            validation_state: ValidationState::None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl CodeInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
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

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
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

    pub fn with_mask(mut self, mask: bool) -> Self {
        self.mask = mask;
        self
    }

    pub fn with_numbers_only(mut self, numbers_only: bool) -> Self {
        self.numbers_only = numbers_only;
        self
    }

    /// The current value sanitized per `numbers_only` and clamped to `length`.
    /// When `numbers_only` is true, only ASCII digits are kept; otherwise all
    /// characters are retained. This is the slot-distribution source of truth.
    pub fn sanitized_chars(&self) -> Vec<char> {
        self.current_value()
            .chars()
            .filter(|c| !self.numbers_only || c.is_ascii_digit())
            .take(self.length)
            .collect()
    }

    /// Where the caret sits, clamped to the sanitized value.
    ///
    /// Defaults to the first empty slot, which is where typing lands in a code
    /// nobody has clicked into — the behaviour before a caret existed.
    pub fn selection_range(&self) -> (usize, usize) {
        let len = self.sanitized_chars().len();
        let default = len.min(self.length.saturating_sub(1));
        if self.selection_start == 0 && self.selection_end == 0 && len > 0 {
            return (default, default);
        }
        let start = self.selection_start.min(len);
        let end = self.selection_end.min(len).max(start);
        (start, end)
    }

    pub fn with_selection(mut self, start: usize, end: usize) -> Self {
        self.selection_start = start;
        self.selection_end = end;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.autocomplete = autocomplete.into();
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    /// Returns the effective validation state, promoting to Invalid when an error message is set.
    pub fn effective_validation_state(&self) -> ValidationState {
        if self.error.is_some() {
            ValidationState::Invalid
        } else {
            self.validation_state
        }
    }

    /// Whether the input is controlled (value explicitly provided).
    pub fn is_controlled(&self) -> bool {
        self.value.is_some()
    }

    /// The current effective value of the input.
    pub fn current_value(&self) -> &str {
        match &self.value {
            Some(v) => v.as_str(),
            None => self.default_value.as_str(),
        }
    }

    /// Whether the code has been fully entered (all slots filled).
    pub fn is_complete(&self) -> bool {
        self.sanitized_chars().len() >= self.length
    }

    /// The effective accessible label for the input group.
    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(self.label.as_str())
    }

    /// Count of filled slot positions.
    pub fn filled_count(&self) -> usize {
        self.sanitized_chars().len()
    }

    // ── Token methods ────────────────────────────────────────

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn code_font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
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
