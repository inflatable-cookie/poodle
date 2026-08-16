//! TextInput — a text field: affixes, icons, validation, char count.
//!
//! This module is the single authority for the TextInput declaration surface:
//! the struct, its defaults and builders, then the token recipes and derived
//! queries beside them. Native caret and residual compatibility fields sit on
//! the same struct — hosts depend on both. `g14.006` briefly generated the
//! first half from a TypeScript interface; `g14.008` rejected that path and
//! `g14.021` restored the hand-written declaration.
//!
//! Contract: `docs/contracts/components/text-input.md`

use poodle_tokens::semantic;

use crate::types::ValidationState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextInputSpec {
    pub value: Option<String>,
    pub default_value: String,
    pub placeholder: Option<String>,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_required: bool,
    pub validation_state: crate::types::ValidationState,
    pub shows_validation_status: bool,
    pub aria_label: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub max_length: Option<usize>,
    pub show_char_count: bool,
    pub size: Option<crate::types::ControlSize>,
    pub size_role: crate::types::SemanticControlSizeRole,
    pub density: Option<crate::types::ControlDensity>,
    pub input_type: String,
    pub rows: Option<u16>,
    pub resize: String,
    pub source: Option<String>,
    pub show_clear_button: bool,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub id: Option<String>,
    pub selection_start: usize,
    pub selection_end: usize,
    pub is_focused: bool,
    pub name: Option<String>,
    pub autocomplete: Option<String>,
    pub pattern: Option<String>,
    pub input_mode: Option<String>,
    pub debounce_ms: u32,
    pub description_id: Option<String>,
    pub error_message_id: Option<String>,
    pub submit_enabled: bool,
    pub cancel_enabled: bool,
}

impl Default for TextInputSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: "".to_owned(),
            placeholder: None,
            is_disabled: false,
            is_read_only: false,
            is_required: false,
            validation_state: crate::types::ValidationState::None,
            shows_validation_status: true,
            aria_label: None,
            prefix: None,
            suffix: None,
            max_length: None,
            show_char_count: false,
            size: None,
            size_role: crate::types::SemanticControlSizeRole::Control,
            density: None,
            input_type: "text".to_owned(),
            rows: None,
            resize: "vertical".to_owned(),
            source: None,
            show_clear_button: true,
            leading_icon: None,
            trailing_icon: None,
            id: None,
            selection_start: 0,
            selection_end: 0,
            is_focused: false,
            name: None,
            autocomplete: None,
            pattern: None,
            input_mode: None,
            debounce_ms: 0,
            description_id: None,
            error_message_id: None,
            submit_enabled: false,
            cancel_enabled: false,
        }
    }
}

impl TextInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn with_default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }
    pub fn with_placeholder(mut self, value: impl Into<String>) -> Self {
        self.placeholder = Some(value.into());
        self
    }
    pub fn with_disabled(mut self, value: bool) -> Self {
        self.is_disabled = value;
        self
    }
    pub fn with_read_only(mut self, value: bool) -> Self {
        self.is_read_only = value;
        self
    }
    pub fn with_required(mut self, value: bool) -> Self {
        self.is_required = value;
        self
    }
    pub fn with_validation_state(mut self, value: crate::types::ValidationState) -> Self {
        self.validation_state = value;
        self
    }
    pub fn with_show_validation_status(mut self, value: bool) -> Self {
        self.shows_validation_status = value;
        self
    }
    pub fn with_aria_label(mut self, value: impl Into<String>) -> Self {
        self.aria_label = Some(value.into());
        self
    }
    pub fn with_prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = Some(value.into());
        self
    }
    pub fn with_suffix(mut self, value: impl Into<String>) -> Self {
        self.suffix = Some(value.into());
        self
    }
    pub fn with_max_length(mut self, value: usize) -> Self {
        self.max_length = Some(value);
        self
    }
    pub fn with_show_char_count(mut self, value: bool) -> Self {
        self.show_char_count = value;
        self
    }
    pub fn with_size(mut self, value: crate::types::ControlSize) -> Self {
        self.size = Some(value);
        self
    }
    pub fn with_size_role(mut self, value: crate::types::SemanticControlSizeRole) -> Self {
        self.size_role = value;
        self
    }
    pub fn with_density(mut self, value: crate::types::ControlDensity) -> Self {
        self.density = Some(value);
        self
    }
    pub fn with_type(mut self, value: impl Into<String>) -> Self {
        self.input_type = value.into();
        self
    }
    pub fn with_rows(mut self, value: u16) -> Self {
        self.rows = Some(value);
        self
    }
    pub fn with_resize(mut self, value: impl Into<String>) -> Self {
        self.resize = value.into();
        self
    }
    pub fn with_source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }
    pub fn with_show_clear_button(mut self, value: bool) -> Self {
        self.show_clear_button = value;
        self
    }
    pub fn with_leading_icon(mut self, value: impl Into<String>) -> Self {
        self.leading_icon = Some(value.into());
        self
    }
    pub fn with_trailing_icon(mut self, value: impl Into<String>) -> Self {
        self.trailing_icon = Some(value.into());
        self
    }
    pub fn with_id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }
    pub fn with_selection_start(mut self, value: usize) -> Self {
        self.selection_start = value;
        self
    }
    pub fn with_selection_end(mut self, value: usize) -> Self {
        self.selection_end = value;
        self
    }
    pub fn with_is_focused(mut self, value: bool) -> Self {
        self.is_focused = value;
        self
    }
    pub fn with_name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    pub fn with_autocomplete(mut self, value: impl Into<String>) -> Self {
        self.autocomplete = Some(value.into());
        self
    }
    pub fn with_pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }
    pub fn with_input_mode(mut self, value: impl Into<String>) -> Self {
        self.input_mode = Some(value.into());
        self
    }
    pub fn with_debounce_ms(mut self, value: u32) -> Self {
        self.debounce_ms = value;
        self
    }
    pub fn with_description_id(mut self, value: impl Into<String>) -> Self {
        self.description_id = Some(value.into());
        self
    }
    pub fn with_error_message_id(mut self, value: impl Into<String>) -> Self {
        self.error_message_id = Some(value.into());
        self
    }
    pub fn with_submit_enabled(mut self, value: bool) -> Self {
        self.submit_enabled = value;
        self
    }
    pub fn with_cancel_enabled(mut self, value: bool) -> Self {
        self.cancel_enabled = value;
        self
    }
}


impl TextInputSpec {
    /// Place the caret, or select a range when the two differ.
    pub fn with_selection(mut self, start: usize, end: usize) -> Self {
        self.selection_start = start;
        self.selection_end = end;
        self
    }

    /// Alias for the `with_is_focused` builder.
    pub fn with_focused(self, is_focused: bool) -> Self {
        self.with_is_focused(is_focused)
    }

    /// Alias for the `with_type` builder (`input_type` field).
    pub fn with_input_type(self, value: impl Into<String>) -> Self {
        self.with_type(value)
    }

    /// The selection as an ordered `(start, end)` pair, clamped to the value.
    pub fn selection_range(&self) -> (usize, usize) {
        let len = self.current_value().chars().count();
        let a = self.selection_start.min(len);
        let b = self.selection_end.min(len);
        (a.min(b), a.max(b))
    }

    /// Whether this spec operates in multiline mode (rows > 1 or input_type is "multiline").
    pub fn is_multiline(&self) -> bool {
        self.input_type == "multiline" || self.rows.unwrap_or(1) > 1
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

    /// Char-count color. Contract §8 specifies `text-muted`; no `color.text.muted`
    /// semantic token is exported, so the most-muted available text role
    /// (`text-tertiary`, neutral.400) is used. TOKEN GAP: add `color.text.muted`.
    pub fn char_count_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    pub fn char_count_over_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    /// Char-count font size. Contract §8 char-count uses `typography-code-xs`
    /// (`0.6875rem`); `typography.caption.size` resolves to the same `0.6875rem`
    /// primitive (`font.size.xs`), so it is the token-resolved match for the
    /// counter size without a per-component literal.
    pub fn char_count_font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_CAPTION_SIZE
    }

    /// Root + affix-separator border width. Contract §8 root border is
    /// `0.0625rem solid`; `border.width.default` resolves to `0.0625rem`.
    pub fn border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    /// Affix separator color. Contract §8 (reconciled to Svelte) uses a solid
    /// `border-default` separator, not the muted `border-subtle`.
    pub fn affix_separator_solid_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    /// Pending validation indicator color. Contract §8 + Svelte use `accent-base`
    /// for the pending spinner (not `text-secondary`).
    pub fn pending_indicator_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Validation indicator color for the current validation state. Mirrors the
    /// §8 "Validation Indicator state colors" table: pending→accent-base,
    /// valid→status-success, invalid→status-danger, none→icon-muted.
    pub fn validation_indicator_color_token(&self) -> &'static str {
        match self.validation_state {
            ValidationState::Pending => semantic::COLOR_ACCENT_BASE,
            ValidationState::Valid => semantic::COLOR_STATUS_SUCCESS,
            ValidationState::Invalid => semantic::COLOR_STATUS_DANGER,
            ValidationState::None => semantic::COLOR_ICON_MUTED,
        }
    }

    pub fn focus_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
