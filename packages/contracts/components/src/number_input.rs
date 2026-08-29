//! NumberInput — one finite numeric value with optional raw draft.
//!
//! Contract: `docs/contracts/components/number-input.md`
//! Committed value is `Option<f64>` (`None` = empty). Bounds and step are
//! optional; omitted step means effective `1`. Infinite sentinels and silent
//! clamp helpers are removed.

use poodle_headless::number_input::{
    format_number_committed, number_input_invalid, step_number_value, NumberInputContext,
};
use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

#[derive(Clone, Debug, PartialEq)]
pub struct NumberInputSpec {
    /// Controlled committed value. `None` is committed empty.
    pub value: Option<f64>,
    /// Initial committed value for uncontrolled host use.
    pub default_value: Option<f64>,
    /// Optional host-owned raw draft. `None` means no draft override.
    pub draft_value: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// Authored step; `None` means omitted (effective step is `1`).
    pub step: Option<f64>,
    pub is_disabled: bool,
    /// Read-only display — value can't be edited but is not visually
    /// dimmed the way disabled is.
    pub is_read_only: bool,
    /// When true, the field is required for form submission.
    pub is_required: bool,
    /// Number of decimal places for committed display / draft acceptance.
    /// `None` means shortest canonical form.
    pub precision: Option<u8>,
    /// Placeholder text shown when committed empty and no draft.
    pub placeholder: Option<String>,
    /// Optional prefix label rendered inside the left edge of the
    /// field (e.g. "$").
    pub prefix: Option<String>,
    /// Optional suffix label rendered inside the right edge of the
    /// field (e.g. "kg").
    pub suffix: Option<String>,
    pub validation_state: ValidationState,
    /// When true, the +/- stepper buttons are rendered. Matches Svelte
    /// `showSteppers` (default `false`).
    pub show_steppers: bool,
    pub aria_label: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    pub id: Option<String>,
    /// Host-owned caret/selection for the editable value node.
    pub selection_start: usize,
    pub selection_end: usize,
    pub is_focused: bool,
}

impl Default for NumberInputSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            draft_value: None,
            min: None,
            max: None,
            step: None,
            is_disabled: false,
            is_read_only: false,
            is_required: false,
            precision: None,
            placeholder: None,
            prefix: None,
            suffix: None,
            validation_state: ValidationState::None,
            show_steppers: false,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            id: None,
            selection_start: 0,
            selection_end: 0,
            is_focused: false,
        }
    }
}

impl NumberInputSpec {
    pub fn new(value: Option<f64>) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: Option<f64>) -> Self {
        self.value = value;
        self
    }

    pub fn with_default_value(mut self, default_value: Option<f64>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn with_draft_value(mut self, draft_value: impl Into<Option<String>>) -> Self {
        self.draft_value = draft_value.into();
        self
    }

    pub fn with_min(mut self, min: Option<f64>) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: Option<f64>) -> Self {
        self.max = max;
        self
    }

    pub fn with_step(mut self, step: Option<f64>) -> Self {
        self.step = step;
        self
    }

    pub fn with_steppers(mut self, show_steppers: bool) -> Self {
        self.show_steppers = show_steppers;
        self
    }

    pub fn with_read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_precision(mut self, precision: u8) -> Self {
        self.precision = Some(precision);
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
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

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
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

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_selection(mut self, start: usize, end: usize) -> Self {
        self.selection_start = start;
        self.selection_end = end;
        self
    }

    pub fn with_focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
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

    pub fn precision_f64(&self) -> Option<f64> {
        self.precision.map(|p| f64::from(p))
    }

    /// Committed display text from shared headless formatting. Empty when
    /// committed `None`.
    pub fn formatted_value(&self) -> String {
        format_number_committed(self.value, self.precision_f64())
    }

    /// Visible editor text: raw draft when present, otherwise committed
    /// formatting (or empty for committed empty).
    pub fn display_text(&self) -> String {
        if let Some(draft) = &self.draft_value {
            return draft.clone();
        }
        self.formatted_value()
    }

    pub fn selection_range(&self) -> (usize, usize) {
        (self.selection_start, self.selection_end)
    }

    pub fn to_context(&self) -> NumberInputContext {
        NumberInputContext {
            committed: self.value,
            default_value: self.default_value,
            draft: self.draft_value.clone(),
            min: self.min,
            max: self.max,
            step: self.step,
            precision: self.precision_f64(),
            disabled: self.is_disabled,
            read_only: self.is_read_only,
        }
    }

    /// Whether an increment/decrement from the active draft or committed
    /// value would produce a valid next result. Used to disable steppers
    /// without inventing a clamped value.
    ///
    /// Matches the transition machine: no draft → committed; empty draft →
    /// null baseline; valid draft → parsed value; invalid draft → inert.
    pub fn can_step(&self, direction: i32) -> bool {
        let context = self.to_context();
        if context.disabled || context.read_only {
            return false;
        }

        let baseline = match self.draft_value.as_deref() {
            None => self.value,
            Some("") => None,
            Some(text) => {
                if poodle_headless::number_input::number_draft_constraint_valid(
                    text,
                    self.min,
                    self.max,
                    self.step,
                    self.precision_f64(),
                ) {
                    poodle_headless::number_input::parse_number_decimal(text)
                        .map(poodle_headless::number_input::number_decimal_to_number)
                } else {
                    return false;
                }
            }
        };

        step_number_value(
            baseline,
            direction,
            self.min,
            self.max,
            self.step,
            self.precision_f64(),
        )
        .is_some()
    }

    pub fn is_invalid_draft(&self) -> bool {
        number_input_invalid(&self.to_context())
    }

    /// `aria-valuenow`: constraint-valid draft when available, otherwise
    /// committed; absent when committed empty and draft unresolved.
    pub fn accessible_value_now(&self) -> Option<f64> {
        if let Some(draft) = &self.draft_value {
            if poodle_headless::number_input::number_draft_constraint_valid(
                draft,
                self.min,
                self.max,
                self.step,
                self.precision_f64(),
            ) {
                return poodle_headless::number_input::parse_number_decimal(draft)
                    .map(poodle_headless::number_input::number_decimal_to_number);
            }
            return None;
        }
        self.value
    }

    pub fn border_token(&self) -> &'static str {
        self.validation_state.border_token()
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn placeholder_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn stepper_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// Affix (prefix/suffix) box fill — Svelte `.poodle-number-input__prefix`
    /// uses `background-surface`.
    pub fn affix_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// Affix box border — Svelte affix uses `border-default`.
    pub fn affix_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    /// Affix text color. Svelte uses `--poodle-color-text-muted`; the Rust
    /// semantic set has no `text.muted` token, so `text.secondary` is the
    /// closest available stand-in (token gap, noted in parity doc).
    pub fn affix_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn stepper_icon_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn body_line_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_LINE_HEIGHT
    }

    pub fn control_height_token(&self) -> &'static str {
        semantic::SIZE_CONTROL_HEIGHT
    }

    pub fn horizontal_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_X
    }

    /// Inner gap between a stepper button and the field edge / value.
    /// Svelte stepper column has no extra inline gap; the Jetstream layout
    /// uses the smallest inline space token here instead of a bare rem.
    pub fn stepper_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_XS
    }

    /// Hairline width token for affix dividers / field border (1px).
    pub fn border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_committed_formats_blank() {
        let spec = NumberInputSpec::new(None);
        assert_eq!(spec.formatted_value(), "");
        assert_eq!(spec.display_text(), "");
        assert_eq!(spec.accessible_value_now(), None);
    }

    #[test]
    fn draft_overrides_display_without_inventing_committed() {
        let spec = NumberInputSpec::new(Some(12.0)).with_draft_value(Some("-".into()));
        assert_eq!(spec.formatted_value(), "12");
        assert_eq!(spec.display_text(), "-");
        assert_eq!(spec.accessible_value_now(), None);
    }

    #[test]
    fn precision_uses_headless_fixed_scale() {
        let spec = NumberInputSpec::new(Some(1.5)).with_precision(2);
        assert_eq!(spec.formatted_value(), "1.50");
    }

    #[test]
    fn steppers_disable_at_authored_bounds() {
        let at_max = NumberInputSpec::new(Some(10.0))
            .with_min(Some(0.0))
            .with_max(Some(10.0))
            .with_step(Some(1.0));
        assert!(!at_max.can_step(1));
        assert!(at_max.can_step(-1));
    }

    #[test]
    fn steppers_match_transition_baselines_for_empty_and_invalid_drafts() {
        let empty_draft = NumberInputSpec::new(Some(5.0))
            .with_min(Some(0.0))
            .with_max(Some(10.0))
            .with_step(Some(1.0))
            .with_draft_value(Some(String::new()));
        // Empty draft uses the null baseline (min), not committed 5.
        assert!(empty_draft.can_step(1));
        assert_eq!(
            step_number_value(None, 1, Some(0.0), Some(10.0), Some(1.0), None),
            Some(0.0)
        );

        let invalid_draft = NumberInputSpec::new(Some(5.0))
            .with_min(Some(0.0))
            .with_max(Some(10.0))
            .with_step(Some(1.0))
            .with_draft_value(Some("1e2".into()));
        assert!(!invalid_draft.can_step(1));
        assert!(!invalid_draft.can_step(-1));
        assert_eq!(invalid_draft.accessible_value_now(), None);
    }

    #[test]
    fn omitted_bounds_do_not_use_infinity_sentinels() {
        let spec = NumberInputSpec::new(Some(0.0));
        assert_eq!(spec.min, None);
        assert_eq!(spec.max, None);
        assert_eq!(spec.step, None);
        assert!(spec.can_step(1));
        assert!(spec.can_step(-1));
    }
}
