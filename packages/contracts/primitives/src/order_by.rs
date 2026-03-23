use flint_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}

#[derive(Clone, Debug)]
pub struct SortField {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
}

impl SortField {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
        }
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ActiveSort {
    pub field: String,
    pub direction: SortDirection,
}

impl ActiveSort {
    pub fn new(field: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            field: field.into(),
            direction,
        }
    }
}

#[derive(Clone)]
pub struct OrderBySpec {
    pub fields: Vec<SortField>,
    pub active_sort: Option<ActiveSort>,
    pub aria_label: String,
    pub is_disabled: bool,
}

impl OrderBySpec {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            active_sort: None,
            aria_label: "Sort by".to_string(),
            is_disabled: false,
        }
    }

    // Builder methods

    pub fn with_fields(mut self, fields: Vec<SortField>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_active_sort(mut self, active_sort: ActiveSort) -> Self {
        self.active_sort = Some(active_sort);
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = label.into();
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    // Helper methods

    pub fn is_field_active(&self, value: &str) -> bool {
        self.active_sort
            .as_ref()
            .map(|s| s.field == value)
            .unwrap_or(false)
    }

    pub fn active_direction(&self) -> Option<&SortDirection> {
        self.active_sort.as_ref().map(|s| &s.direction)
    }

    // Token methods

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn field_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn field_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn field_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn field_hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn active_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn active_border_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn active_text_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn reset_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
}
