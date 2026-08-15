//! TextInput — a text field: affixes, icons, validation, char count.
//!
//! The portable declaration surface — struct, defaults, builders — is
//! generated from the conformance interface module
//! (`packages/core/src/conformance/text-input.ts`) into
//! [`crate::generated::text_input`] (regenerate with `effigy conformance:build`,
//! gated by `effigy conformance:check`). This module is the hand-written
//! extension beside the generated surface: token recipes and derived
//! queries. Native caret/compat fields stay on the generated spec so existing
//! hosts compile; they are not in `PortablePropsOf`.
//!
//! Contract: `docs/contracts/components/text-input.md`

use poodle_tokens::semantic;

use crate::types::ValidationState;

pub use crate::generated::text_input::TextInputSpec;

impl TextInputSpec {
    /// Place the caret, or select a range when the two differ.
    pub fn with_selection(mut self, start: usize, end: usize) -> Self {
        self.selection_start = start;
        self.selection_end = end;
        self
    }

    /// Alias for the generated `with_is_focused` builder.
    pub fn with_focused(self, is_focused: bool) -> Self {
        self.with_is_focused(is_focused)
    }

    /// Alias for the generated `with_type` builder (`input_type` field).
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
