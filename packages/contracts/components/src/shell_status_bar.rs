//! `ShellStatusBarSpec` — the Poodle `StatusBar` primitive
//! (`docs/contracts/components/status-bar.md`). A lightweight shell
//! utility/status row for workspace summary, connection state, and context
//! metadata, rendered as a `<footer>` landmark.
//!
//! The Rust type name (`ShellStatusBar`) predates the doc rename
//! `ShellStatusBar` → `StatusBar`; both refer to the same contract.
//!
//! Per the contract prop set, this models `summary`, `aria_label`, `chrome`,
//! `size`, `size_role`, and `density`. The legacy `leading_item_count` /
//! `trailing_item_count` fields are retained for back-compat with existing
//! shell call-sites but no longer drive any visual behaviour — slot content
//! (leading / trailing element lists) is supplied by the renderer.

use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct ShellStatusBarSpec {
    /// Summary text shown in the leading region when no leading slot content
    /// is provided; also the fallback `aria-label`.
    pub summary: Option<String>,
    /// Accessible label for the `<footer>`; falls back to `summary`, then
    /// `"Status"`.
    pub aria_label: Option<String>,
    /// When true, renders an explicit border-top + 94% panel background.
    /// When false, the bar blends into its container (transparent).
    pub chrome: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Legacy shell counters — retained for back-compat, not used visually.
    pub leading_item_count: usize,
    /// Legacy shell counters — retained for back-compat, not used visually.
    pub trailing_item_count: usize,
}

impl Default for ShellStatusBarSpec {
    fn default() -> Self {
        Self {
            summary: None,
            aria_label: None,
            chrome: false,
            size: None,
            // Contract default: status bar resolves at the "chrome" role.
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
            leading_item_count: 0,
            trailing_item_count: 0,
        }
    }
}

impl ShellStatusBarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_chrome(mut self, chrome: bool) -> Self {
        self.chrome = chrome;
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

    pub fn with_leading_item_count(mut self, leading_item_count: usize) -> Self {
        self.leading_item_count = leading_item_count;
        self
    }

    pub fn with_trailing_item_count(mut self, trailing_item_count: usize) -> Self {
        self.trailing_item_count = trailing_item_count;
        self
    }

    /// Resolved `aria-label`: `aria_label` > `summary` > `"Status"`.
    pub fn resolved_aria_label(&self) -> &str {
        self.aria_label
            .as_deref()
            .or(self.summary.as_deref())
            .unwrap_or("Status")
    }

    // ── Token methods ────────────────────────────────────────────

    /// Chrome background token (94% panel via `color-mix`). Only painted when
    /// `chrome` is true.
    pub fn chrome_background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Opacity applied to the chrome background (`color-mix … 94%`).
    pub fn chrome_background_opacity(&self) -> f32 {
        0.94
    }

    /// Border-subtle token for the chrome border-top. Only painted when
    /// `chrome` is true.
    pub fn chrome_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Border-width token for the chrome border-top (`0.0625rem`).
    pub fn chrome_border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    /// Foreground / text color token.
    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Root gap (between leading and trailing regions). Contract base
    /// `space.inline.md`; density `compact`/`comfortable` override below.
    pub fn root_gap_token(&self, density: ControlDensity) -> &'static str {
        match density {
            ControlDensity::Compact | ControlDensity::Comfortable => semantic::SPACE_INLINE_MD,
            ControlDensity::Default => semantic::SPACE_INLINE_MD,
        }
    }

    /// Inner gap (within each leading / trailing region) — `space.inline.sm`.
    pub fn inner_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    // ── Rem scales (contract §8) ──────────────────────────────────
    //
    // Status-bar-specific size / density scales that have no shared token.
    // Returned as rem; the renderer converts via `rem_to_px`.

    /// Root font-size in rem for the resolved size. Contract §8 size table:
    /// xs 0.6875, sm 0.75, md 0.8125 (default), lg 0.875, xl 0.9375.
    pub fn font_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    /// Root padding-block in rem for the resolved size. Contract §8: default
    /// `0.375rem`; xs 0.25, sm 0.3125, lg 0.4375, xl 0.5.
    pub fn padding_block_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.25,
            ControlSize::Sm => 0.3125,
            ControlSize::Md => 0.375,
            ControlSize::Lg => 0.4375,
            ControlSize::Xl => 0.5,
        }
    }

    /// Root padding-inline in rem for the resolved density. Contract §8:
    /// default `0.75rem`; compact 0.5, comfortable 1.125.
    pub fn padding_inline_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.125,
        }
    }

    /// Root gap override in rem for the resolved density. Contract §8: compact
    /// `0.375rem`, comfortable `1rem`. Default returns `None` (use
    /// `root_gap_token`).
    pub fn density_gap_rem(&self, density: ControlDensity) -> Option<f32> {
        match density {
            ControlDensity::Compact => Some(0.375),
            ControlDensity::Comfortable => Some(1.0),
            ControlDensity::Default => None,
        }
    }
}
