use poodle_tokens::semantic;

use crate::types::{ControlDensity, FormActionAlign, FormActionDangerItem};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormActionsSpec {
    pub align: FormActionAlign,
    pub show_top_separation: bool,
    pub show_top_border: bool,
    /// Spacing density. Controls inline gap, top separation, and border
    /// gap per contract §8 (Density Variants + Divider Offset Variants).
    pub density: Option<ControlDensity>,
    /// Overflow danger-action descriptors (contract `dangerItems`, §3). When
    /// non-empty (and inline danger content is present), these are the
    /// narrow-container overflow treatment shown behind the danger-menu
    /// trigger. Empty by default.
    pub danger_items: Vec<FormActionDangerItem>,
}

impl Default for FormActionsSpec {
    fn default() -> Self {
        Self {
            align: FormActionAlign::End,
            show_top_separation: true,
            show_top_border: false,
            density: None,
            danger_items: Vec::new(),
        }
    }
}

impl FormActionsSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_align(mut self, align: FormActionAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_top_separation(mut self, show_top_separation: bool) -> Self {
        self.show_top_separation = show_top_separation;
        self
    }

    pub fn with_top_border(mut self, show_top_border: bool) -> Self {
        self.show_top_border = show_top_border;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    /// Replace the overflow danger-item descriptors (contract `dangerItems`).
    pub fn with_danger_items(mut self, items: Vec<FormActionDangerItem>) -> Self {
        self.danger_items = items;
        self
    }

    /// Append a single overflow danger-item descriptor.
    pub fn with_danger_item(mut self, item: FormActionDangerItem) -> Self {
        self.danger_items.push(item);
        self
    }

    pub fn action_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn stack_separation_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    /// Inline gap between actions, in rem, when density overrides the
    /// token default. `None` means use [`Self::action_gap_token`]. Contract §8
    /// Density Variants: compact `0.5rem`, comfortable `0.875rem`;
    /// default inherits `--poodle-space-inline-md`.
    pub fn gap_rem(&self, density: ControlDensity) -> Option<f32> {
        match density {
            ControlDensity::Compact => Some(0.5),
            ControlDensity::Default => None,
            ControlDensity::Comfortable => Some(0.875),
        }
    }

    /// Top separation (padding-top), in rem, when density overrides the
    /// token default. `None` means use [`Self::stack_separation_token`].
    /// Contract §8: compact `0.375rem`, comfortable `0.75rem`; default
    /// inherits `--poodle-space-stack-sm`.
    pub fn top_separation_rem(&self, density: ControlDensity) -> Option<f32> {
        match density {
            ControlDensity::Compact => Some(0.375),
            ControlDensity::Default => None,
            ControlDensity::Comfortable => Some(0.75),
        }
    }

    /// Top margin applied when `show_top_border` is true, in rem.
    /// Contract §8 Divider Offset Variants: compact `0.25rem`, default
    /// `0.5rem`, comfortable `0.625rem` (raw literals, matching Svelte
    /// `--poodle-form-actions-border-gap`).
    pub fn border_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.625,
        }
    }

    pub fn shows_top_separation(&self) -> bool {
        self.show_top_separation
    }

    pub fn shows_top_border(&self) -> bool {
        self.show_top_border
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Whether an overflow danger menu is configured (contract §8 Responsive
    /// Swap requires `dangerItems` to be present). The actual collapse also
    /// requires inline `danger` content + a narrow container, both of which
    /// are host/runtime concerns.
    pub fn has_danger_menu(&self) -> bool {
        !self.danger_items.is_empty()
    }

    /// Inline danger group gap (contract §8 Danger Inline: `gap =
    /// var(--poodle-form-actions-gap)`), in rem, when density overrides the
    /// token default. Mirrors [`Self::gap_rem`] so the danger row tracks the root.
    pub fn danger_inline_gap_rem(&self, density: ControlDensity) -> Option<f32> {
        self.gap_rem(density)
    }

    /// Token backing the inline danger group gap when no density override
    /// applies (the root [`Self::action_gap_token`]).
    pub fn danger_inline_gap_token(&self) -> &'static str {
        self.action_gap_token()
    }

    pub fn wraps_on_narrow_widths(&self) -> bool {
        true
    }
}
