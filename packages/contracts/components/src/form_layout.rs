/// FormLayout — responsive form grid with error/success messaging.
///
/// Matches docs/contracts/components/form-layout.md.
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormLayoutSpec {
    /// Base grid columns. 6 enables mixed 2-col and 3-col layouts
    /// via the Field span prop.
    pub columns: u32,
    /// Form-level error message (displayed via Callout tone="danger").
    pub error: Option<String>,
    /// Form-level success message (displayed via Callout tone="success").
    pub success: Option<String>,
    /// Introductory text above the form grid.
    pub description: Option<String>,
    /// Per-field error map: ordered `(field-name, message)` pairs. Renders the
    /// accessible FieldErrors summary (contract §2 / §6, `role="alert"`).
    /// Ordered Vec (not a map) so render order is deterministic across targets.
    pub field_errors: Vec<(String, String)>,
}

impl Default for FormLayoutSpec {
    fn default() -> Self {
        Self {
            columns: 6,
            error: None,
            success: None,
            description: None,
            field_errors: Vec::new(),
        }
    }
}

impl FormLayoutSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, columns: u32) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_success(mut self, success: impl Into<String>) -> Self {
        self.success = Some(success.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add a single field-error entry. Order is preserved.
    pub fn with_field_error(
        mut self,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        self.field_errors.push((field.into(), message.into()));
        self
    }

    /// Replace the full field-error list.
    pub fn with_field_errors(mut self, errors: Vec<(String, String)>) -> Self {
        self.field_errors = errors;
        self
    }

    pub fn has_status_message(&self) -> bool {
        self.error.is_some() || self.success.is_some()
    }

    /// Whether the accessible field-errors summary should render.
    pub fn has_field_errors(&self) -> bool {
        !self.field_errors.is_empty()
    }

    // ── FieldErrors summary token methods (contract §8) ──────────────────

    /// FieldErrors heading text (contract §6).
    pub fn field_errors_heading(&self) -> &'static str {
        "Please fix the following errors:"
    }

    /// FieldErrors background = `color-mix(status-danger 8%, transparent)`.
    /// Mix source token; the 8% mix toward transparent is applied at render.
    pub fn field_errors_tone_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn field_errors_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn field_errors_border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    /// Inline padding (contract §8 panel-x, `1rem`).
    pub fn field_errors_padding_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    /// Block padding (contract §8 panel-y, `0.75rem`).
    pub fn field_errors_padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }

    /// FieldErrors body font (contract §8 `typography.label.size`, `0.75rem`).
    pub fn field_errors_font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    pub fn field_errors_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    // ── Layout token methods ─────────────────────────────────────────────

    /// Root flex-column gap (contract §7 `space.stack.lg`).
    pub fn section_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_LG
    }

    /// Grid column gap (contract §8 `space.inline.md`).
    pub fn column_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    /// Per-column min-width for the multi-column flex approximation. The contract
    /// grid (CSS) has no explicit column min-width; the Rust flex-wrap layout
    /// needs one to keep columns usable. `size.select.minWidth` (8rem) is the
    /// nearest field-control min-width semantic. TOKEN GAP: a dedicated
    /// `size.field.minWidth` token would be more precise.
    pub fn column_min_width_token(&self) -> &'static str {
        semantic::SIZE_SELECT_MIN_WIDTH
    }

    /// FieldErrors heading/item stack gap. Contract §8 uses `0.25rem`/`0.125rem`
    /// margins; `space.stack.sm` is the nearest small stack token.
    pub fn field_errors_stack_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    /// Description / body text color (contract §8).
    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Description / body font size (contract §8 `typography.body.size`).
    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }
}
