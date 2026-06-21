use poodle_tokens::semantic;

use crate::composite_types::{RemediationAction, SelectionSummaryItem};
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSummarySpec {
    pub items: Vec<SelectionSummaryItem>,
    pub clear_action: Option<RemediationAction>,
    /// Optional cap on the number of item chips rendered inline. When
    /// the list exceeds this limit, the overflow is surfaced as a
    /// "+N more" chip instead of a long wrapping row. Matches the
    /// Svelte `maxVisibleItems` prop.
    pub max_visible_items: Option<usize>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SelectionSummarySpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            clear_action: None,
            max_visible_items: Some(4),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl SelectionSummarySpec {
    pub fn new(items: Vec<SelectionSummaryItem>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_clear_action(mut self, clear_action: RemediationAction) -> Self {
        self.clear_action = Some(clear_action);
        self
    }

    pub fn with_max_visible_items(mut self, max: usize) -> Self {
        self.max_visible_items = Some(max);
        self
    }

    pub fn selected_count(&self) -> usize {
        self.items.len()
    }

    pub fn has_clear_action(&self) -> bool {
        self.clear_action.is_some()
    }

    /// Number of items actually rendered inline, taking `max_visible_items`
    /// into account.
    pub fn visible_item_count(&self) -> usize {
        self.max_visible_items
            .map(|max| self.items.len().min(max))
            .unwrap_or(self.items.len())
    }

    /// Number of items hidden behind the "+N more" overflow chip.
    pub fn overflow_count(&self) -> usize {
        self.items.len().saturating_sub(self.visible_item_count())
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Chip / overflow corner radius token. Svelte/contract use
    /// `var(--poodle-radius-control)`.
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Chip / overflow border width token (`0.0625rem`).
    pub fn border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    // ── Size ladder (contract §8 size variants + Svelte custom props) ──
    //
    // Contract-exact rem values keyed by effective size; both Rust targets
    // resolve the effective size via the shared presentation helper then read
    // these, instead of hardcoding per-size literals inline.

    /// Chip font-size in rem. Contract §8: xs 0.6875, sm 0.71875, md 0.75,
    /// lg 0.8125, xl 0.875.
    pub fn chip_font_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.71875,
            ControlSize::Md => 0.75,
            ControlSize::Lg => 0.8125,
            ControlSize::Xl => 0.875,
        }
    }

    /// Chip min-height in rem. Contract §8: xs 1, sm 1.125, md 1.5, lg 1.75,
    /// xl 2.
    pub fn chip_min_height_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 1.0,
            ControlSize::Sm => 1.125,
            ControlSize::Md => 1.5,
            ControlSize::Lg => 1.75,
            ControlSize::Xl => 2.0,
        }
    }

    /// Overflow-badge font-size in rem. Svelte
    /// `--poodle-selection-summary-overflow-font-size`: xs 0.6875, sm 0.75,
    /// md 0.8125, lg 0.875, xl 0.9375.
    pub fn overflow_font_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    /// Overflow-badge line-height in rem. Svelte
    /// `--poodle-selection-summary-overflow-line-height`: xs 1.5, sm 1.625,
    /// md 2, lg 2.125, xl 2.25.
    pub fn overflow_line_height_rem(size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 1.5,
            ControlSize::Sm => 1.625,
            ControlSize::Md => 2.0,
            ControlSize::Lg => 2.125,
            ControlSize::Xl => 2.25,
        }
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
