use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BulkActionTone {
    Default,
    Danger,
}

impl Default for BulkActionTone {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BulkAction {
    pub id: String,
    pub label: String,
    pub tone: BulkActionTone,
}

impl BulkAction {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tone: BulkActionTone::Default,
        }
    }

    pub fn with_tone(mut self, tone: BulkActionTone) -> Self {
        self.tone = tone;
        self
    }
}

#[derive(Clone, Debug)]
pub struct BulkActionBarSpec {
    pub selection_count: usize,
    pub total_count: Option<usize>,
    pub actions: Vec<BulkAction>,
}

impl Default for BulkActionBarSpec {
    fn default() -> Self {
        Self {
            selection_count: 0,
            total_count: None,
            actions: Vec::new(),
        }
    }
}

impl BulkActionBarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_selection_count(mut self, count: usize) -> Self {
        self.selection_count = count;
        self
    }

    pub fn with_total_count(mut self, total: usize) -> Self {
        self.total_count = Some(total);
        self
    }

    pub fn with_actions(mut self, actions: Vec<BulkAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn add_action(mut self, action: BulkAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn summary_text(&self) -> String {
        match self.total_count {
            Some(total) => format!("{} of {} selected", self.selection_count, total),
            None => format!("{} selected", self.selection_count),
        }
    }

    // ── Token methods ──────────────────────────────────────────

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn total_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn button_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn button_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn button_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn danger_border_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn danger_text_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn padding_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
