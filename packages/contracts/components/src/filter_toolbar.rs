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
        }
    }
}

impl FilterToolbarSpec {
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

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn controls_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn summary_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
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
