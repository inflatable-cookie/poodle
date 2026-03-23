use poodle_tokens::semantic;

use crate::types::Orientation;

/// Spec for the ResizeHandle component — a draggable divider for resizable panels.
///
/// Contract: `docs/contracts/foundation/resize-handle.md`
///
/// Anatomy:
///   [Resize Handle]
///     ├── [Hit Target] (invisible, larger than visual — min 8px)
///     └── [Visual Affordance] (thin line centered in hit target)
#[derive(Clone, Debug, PartialEq)]
pub struct ResizeHandleSpec {
    /// Resize axis: `Horizontal` means left/right drag (vertical line),
    /// `Vertical` means up/down drag (horizontal line).
    /// Contract default: `Horizontal`.
    pub orientation: Orientation,
    /// Suppresses all interaction when true.
    pub is_disabled: bool,
    /// Accessible label for the separator role.
    pub aria_label: Option<String>,
    /// Current resize ratio exposed to assistive technology.
    pub aria_value_now: Option<f32>,
    /// Minimum value for assistive technology (default 0).
    pub aria_value_min: f32,
    /// Maximum value for assistive technology (default 100).
    pub aria_value_max: f32,
}

impl Default for ResizeHandleSpec {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            is_disabled: false,
            aria_label: None,
            aria_value_now: None,
            aria_value_min: 0.0,
            aria_value_max: 100.0,
        }
    }
}

impl ResizeHandleSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_aria_value_now(mut self, value: f32) -> Self {
        self.aria_value_now = Some(value);
        self
    }

    pub fn with_aria_value_min(mut self, value: f32) -> Self {
        self.aria_value_min = value;
        self
    }

    pub fn with_aria_value_max(mut self, value: f32) -> Self {
        self.aria_value_max = value;
        self
    }

    // -- Token methods --

    /// Border color for the visual affordance line in idle state.
    /// Matches Svelte: `color-mix(in srgb, var(--poodle-color-border-default) 82%, transparent)`.
    pub fn border_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    /// Highlight color for hover and active/dragging states.
    /// Matches Svelte: `var(--poodle-color-accent-base)`.
    pub fn hover_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Opacity token applied when disabled.
    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    /// Focus ring color token.
    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    /// Focus ring width token.
    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }

    // -- Derived helpers --

    /// The ARIA role for this element.
    pub fn role(&self) -> &'static str {
        "separator"
    }

    /// ARIA orientation string.
    pub fn aria_orientation(&self) -> &'static str {
        match self.orientation {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }

    /// Effective aria-label, defaulting to "Resize" if none provided.
    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or("Resize")
    }

    /// Whether the handle should accept focus (not disabled).
    pub fn is_focusable(&self) -> bool {
        !self.is_disabled
    }
}
