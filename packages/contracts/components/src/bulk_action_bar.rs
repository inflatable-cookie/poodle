use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BulkActionTone {
    Default,
    Warning,
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
    /// Named icon for the action's ghost IconButton. The label is used
    /// only as the accessible name / tooltip (Svelte parity). When None,
    /// `resolved_icon()` derives the contract fallback (`trash-2` for
    /// danger, `circle` otherwise).
    pub icon: Option<String>,
    /// When true, the action renders in a disabled state and does not
    /// fire on click. Useful for gating destructive operations until
    /// the selection meets some precondition.
    pub is_disabled: bool,
}

impl BulkAction {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            tone: BulkActionTone::Default,
            icon: None,
            is_disabled: false,
        }
    }

    pub fn with_tone(mut self, tone: BulkActionTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    /// Icon name to render, falling back to the contract default when no
    /// explicit icon is set: `trash-2` for the danger tone, `circle`
    /// otherwise (Svelte `BulkActionBar.svelte` line 88).
    pub fn resolved_icon(&self) -> &str {
        match &self.icon {
            Some(name) => name.as_str(),
            None => match self.tone {
                BulkActionTone::Danger => "trash-2",
                _ => "circle",
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct BulkActionBarSpec {
    pub selection_count: usize,
    pub total_count: Option<usize>,
    pub actions: Vec<BulkAction>,
    /// When true, render a select-all affordance in the summary region.
    /// Label text shifts between "Select all" and "Deselect all" based
    /// on `all_selected`.
    pub show_select_all: bool,
    /// Current all-selected state. Read by the bar to toggle the
    /// select-all label / visual indicator.
    pub all_selected: bool,
    /// When true, the bar renders in a loading state — action buttons
    /// are dimmed and interactive handlers are suppressed.
    pub loading: bool,
    /// When true, the whole bar is disabled — action and clear/select-all
    /// controls are dimmed and non-interactive. Mirrors Svelte `disabled`.
    pub disabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Copy for the select-all control.
    pub select_all_label: String,
}

impl Default for BulkActionBarSpec {
    fn default() -> Self {
        Self {
            selection_count: 0,
            total_count: None,
            actions: Vec::new(),
            show_select_all: false,
            all_selected: false,
            loading: false,
            disabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            select_all_label: "Select all".to_string(),
        }
    }
}

impl BulkActionBarSpec {
    pub fn with_select_all_label(mut self, value: impl Into<String>) -> Self {
        self.select_all_label = value.into();
        self
    }

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

    pub fn with_show_select_all(mut self, show: bool) -> Self {
        self.show_select_all = show;
        self
    }

    pub fn with_all_selected(mut self, all_selected: bool) -> Self {
        self.all_selected = all_selected;
        self
    }

    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The bar is unavailable when loading or disabled (Svelte
    /// `isUnavailable`). Clear / select-all controls gate on this.
    pub fn is_unavailable(&self) -> bool {
        self.loading || self.disabled
    }

    /// Per-action availability (Svelte `actionsDisabled`): unavailable, or
    /// nothing selected.
    pub fn actions_disabled(&self) -> bool {
        self.is_unavailable() || self.selection_count == 0
    }

    pub fn select_all_label(&self) -> &'static str {
        if self.all_selected {
            "Deselect all"
        } else {
            "Select all"
        }
    }

    pub fn warning_border_token(&self) -> &'static str {
        semantic::COLOR_STATUS_WARNING
    }

    pub fn warning_text_token(&self) -> &'static str {
        semantic::COLOR_STATUS_WARNING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
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
