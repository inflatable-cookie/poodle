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
    /// Caret / selection as char offsets into the value, and whether the field
    /// holds focus.
    ///
    /// The web target gets these from the DOM's own selection. The Rust targets
    /// have no native editor, so the host owns them the same way it owns
    /// `TreeSpec::focused_value` — the component reports changes, the host
    /// stores them, and the next render draws the caret where they say.
    /// `selection_start == selection_end` is a plain caret.
    pub selection_start: usize,
    pub selection_end: usize,
    pub is_focused: bool,
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
    pub rows: u16,
    pub resize: String,
    /// Source string for slug mode auto-generation. When `input_type` is
    /// `"slug"`, the slug value is auto-derived from this string.
    /// Matches Svelte `source`.
    pub source: Option<String>,
    /// When true, a clear (×) button appears in search mode when the
    /// input has a value. Matches Svelte `showClearButton`.
    pub show_clear_button: bool,
    pub submit_enabled: bool,
    pub cancel_enabled: bool,
    /// When true, the input is required for form submission. Renders
    /// a subtle affordance (asterisk on the associated Field label)
    /// and flips to the Invalid validation state when left empty on
    /// submit attempt. Matches Svelte `required`.
    pub is_required: bool,
    /// Optional validation regex pattern. Matches HTML `pattern`
    /// attribute / Svelte `pattern` prop. GPUI doesn't run the
    /// validation itself — the caller enforces it — but the field is
    /// carried so the rendered surface can advertise the constraint.
    pub pattern: Option<String>,
    /// Optional autocomplete hint (e.g. "email", "current-password",
    /// "off"). Carried for consumer wiring.
    pub autocomplete: Option<String>,
    /// Optional debounce in milliseconds for the on-change stream.
    /// 0 means fire on every keystroke. Matches Svelte `debounce`.
    pub debounce_ms: u32,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Whether the inline validation status icon renders.
    pub shows_validation_status: bool,
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
            selection_start: 0,
            selection_end: 0,
            is_focused: false,
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
            rows: 1,
            resize: String::from("vertical"),
            source: None,
            show_clear_button: true,
            submit_enabled: false,
            cancel_enabled: false,
            is_required: false,
            pattern: None,
            autocomplete: None,
            debounce_ms: 0,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            shows_validation_status: true,
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

    /// Place the caret, or select a range when the two differ.
    pub fn with_selection(mut self, start: usize, end: usize) -> Self {
        self.selection_start = start;
        self.selection_end = end;
        self
    }

    pub fn with_focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }

    /// The selection as an ordered `(start, end)` pair, clamped to the value.
    pub fn selection_range(&self) -> (usize, usize) {
        let len = self.current_value().chars().count();
        let a = self.selection_start.min(len);
        let b = self.selection_end.min(len);
        (a.min(b), a.max(b))
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

    pub fn with_rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }

    pub fn with_resize(mut self, resize: impl Into<String>) -> Self {
        self.resize = resize.into();
        self
    }

    /// Whether this spec operates in multiline mode (rows > 1 or input_type is "multiline").
    pub fn is_multiline(&self) -> bool {
        self.input_type == "multiline" || self.rows > 1
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_show_clear_button(mut self, show: bool) -> Self {
        self.show_clear_button = show;
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

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    pub fn with_autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.autocomplete = Some(autocomplete.into());
        self
    }

    pub fn with_debounce_ms(mut self, debounce_ms: u32) -> Self {
        self.debounce_ms = debounce_ms;
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
