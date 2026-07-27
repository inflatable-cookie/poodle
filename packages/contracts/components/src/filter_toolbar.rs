use crate::types::{Dimension};
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// A layout container for filter controls. Unlike the old summary-display
/// form, this version is a real container: the consumer supplies filter
/// controls (TextInput, Select, etc.) as children which the component
/// arranges in a responsive grid. Matches the Svelte FilterToolbar composite.
#[derive(Clone, Debug, PartialEq)]
pub struct FilterToolbarSpec {
    /// Accessible label applied to the toolbar region (role="toolbar").
    pub aria_label: String,
    /// Free-form summary text rendered in the header. Typical callers
    /// compose strings like "Showing 24 of 156 items" or "3 filters active"
    /// — the contract no longer tracks query / filter counts directly.
    pub summary_text: Option<String>,
    /// When true the header row shows a collapse toggle and the filter
    /// grid can be hidden. Matches Svelte default (`true`).
    pub collapsible: bool,
    /// Current collapsed state. When `collapsible` is true and this is
    /// true the grid of filter children is hidden. Defaults to collapsed
    /// to match the Svelte default.
    pub collapsed: bool,
    /// Target number of grid columns when there is sufficient width.
    /// The actual column count is clamped by `min_item_width_rem`.
    pub columns: u32,
    /// Minimum width (in rem) of a single filter control before the
    /// grid starts dropping columns. Matches Svelte `minItemWidth` prop
    /// (which takes a CSS length — we store the rem value only).
    pub min_item_width_rem: f32,
    /// When true the toolbar renders an elevation shadow and is expected
    /// to stick to the top of its scroll container (specimen-side CSS).
    pub sticky: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Minimum width of a filter control before the row wraps.
    pub min_item_width: Option<Dimension>,
}

impl Default for FilterToolbarSpec {
    fn default() -> Self {
        Self {
            aria_label: String::from("Filters"),
            summary_text: None,
            collapsible: true,
            collapsed: true,
            columns: 4,
            min_item_width_rem: 10.0,
            sticky: false,
            size: ControlSize::Md,
            // Svelte FilterToolbar defaults to "chrome" role.
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
            min_item_width: None,
        }
    }
}

impl FilterToolbarSpec {
    pub fn with_min_item_width(mut self, width: impl Into<Dimension>) -> Self {
        self.min_item_width = Some(width.into());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_summary_text(mut self, summary_text: impl Into<String>) -> Self {
        self.summary_text = Some(summary_text.into());
        self
    }

    pub fn with_collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn with_collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn with_columns(mut self, columns: u32) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_min_item_width_rem(mut self, rem: f32) -> Self {
        self.min_item_width_rem = rem;
        self
    }

    pub fn with_sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }

    /// Whether the filter grid should be visible for the current state.
    pub fn is_grid_visible(&self) -> bool {
        !self.collapsible || !self.collapsed
    }

    // ── Token methods ────────────────────────────────────────────

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn background_opacity(&self) -> f32 {
        0.92
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    /// Base root gap token. Density `compact`/`comfortable` override this via
    /// `density_gap_rem` / `density_controls_gap` below; `default` density
    /// uses `space.inline.sm` (contract §8 density table), the un-scoped base
    /// rule uses `space.stack.sm`.
    pub fn gap_token(&self) -> &'static str {
        match self.density {
            ControlDensity::Default => semantic::SPACE_INLINE_SM,
            // Compact/comfortable resolve a literal rem via `density_gap_rem`;
            // this token is the fallback only.
            _ => semantic::SPACE_STACK_SM,
        }
    }

    pub fn controls_gap_token(&self) -> &'static str {
        match self.density {
            ControlDensity::Comfortable => semantic::SPACE_INLINE_MD,
            // Compact resolves a literal rem via `density_controls_gap_rem`.
            _ => semantic::SPACE_INLINE_SM,
        }
    }

    /// Actions slot gap token (`space.inline.xs`, 0.25rem). Contract §8.
    pub fn actions_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_XS
    }

    /// Sticky elevation shadow token (`elevation.surface`). Contract §8
    /// `.filter-toolbar[data-sticky="true"]`.
    pub fn sticky_shadow_token(&self) -> &'static str {
        semantic::ELEVATION_SURFACE
    }

    /// Collapse-toggle hit-area size token (`size.icon.md`). Drives the
    /// chevron toggle's square tap target.
    pub fn toggle_size_token(&self) -> &'static str {
        semantic::SIZE_ICON_MD
    }

    /// Collapse-toggle corner radius token (`radius.control`).
    pub fn toggle_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Focus-ring color token for the header button (`accent.focusRing`).
    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    /// Focus-ring width token for the header button (`border.width.focus`).
    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }

    pub fn summary_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    // ── Density rem scales (contract §8 density table) ────────────
    //
    // Returned as rem; the renderer converts via `rem_to_px`. Density
    // altering vertical padding here is the documented compositional
    // exception (panel-internal padding).

    /// Root gap in rem for the current density. Compact 0.25, comfortable
    /// resolves `space.inline.md`; default resolves `gap_token`.
    pub fn density_gap_rem(&self) -> Option<f32> {
        match self.density {
            ControlDensity::Compact => Some(0.25),
            _ => None,
        }
    }

    /// Controls-grid gap in rem for the current density. Compact 0.25;
    /// otherwise resolves `controls_gap_token`.
    pub fn density_controls_gap_rem(&self) -> Option<f32> {
        match self.density {
            ControlDensity::Compact => Some(0.25),
            _ => None,
        }
    }

    /// Root padding-block in rem. Contract §8: compact 0.5, default 0.75,
    /// comfortable 1.0.
    pub fn padding_block_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }

    /// Root padding-inline in rem. Contract §8: compact 0.75, default 1.0,
    /// comfortable 1.25.
    pub fn padding_inline_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.75,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.25,
        }
    }

    /// Summary font-size in rem for the current size. Contract §8 size table:
    /// xs 0.6875, sm 0.71875, md (default label-size 0.8125), lg 0.8125,
    /// xl 0.875.
    pub fn summary_font_size_rem(&self) -> f32 {
        match self.size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.71875,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.8125,
            ControlSize::Xl => 0.875,
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
