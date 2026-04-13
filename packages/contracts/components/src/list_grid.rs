//! ListGrid — responsive card/tile grid layout.
//!
//! Matches `docs/contracts/components/list-grid.md`.

use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListGridVariant {
    Default,
    Compact,
}

impl Default for ListGridVariant {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Debug)]
pub struct ListGridSpec {
    pub variant: ListGridVariant,
    /// When set, minimum column width in `em` (web contract: numeric `minItemWidth`).
    pub min_item_width_em: Option<f32>,
}

impl Default for ListGridSpec {
    fn default() -> Self {
        Self {
            variant: ListGridVariant::Default,
            min_item_width_em: None,
        }
    }
}

impl ListGridSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, v: ListGridVariant) -> Self {
        self.variant = v;
        self
    }

    pub fn with_min_item_width_em(mut self, em: f32) -> Self {
        self.min_item_width_em = Some(em);
        self
    }

    /// Content gap: default uses `space.stack.lg` (~1.25rem); compact uses `space.inline.sm` (~0.5rem).
    pub fn gap_token(&self) -> &'static str {
        match self.variant {
            ListGridVariant::Compact => semantic::SPACE_INLINE_SM,
            ListGridVariant::Default => semantic::SPACE_STACK_LG,
        }
    }

    /// Fallback minimum tile width when `min_item_width_em` is `None` (contract default 360px).
    pub fn min_item_width_token(&self) -> &'static str {
        semantic::SIZE_LIST_GRID_MIN_ITEM_WIDTH
    }

    /// Header actions row gap (contract 0.5rem).
    pub fn header_actions_gap_token() -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Space below the optional header row (contract uses grid gap).
    pub fn header_margin_bottom_token(&self) -> &'static str {
        self.gap_token()
    }
}
