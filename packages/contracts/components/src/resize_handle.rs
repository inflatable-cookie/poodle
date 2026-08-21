use poodle_tokens::semantic;

use crate::types::Orientation;

/// Spec for the ResizeHandle component — a draggable divider for resizable panels.
///
/// Contract: `docs/contracts/components/resize-handle.md`
///
/// Anatomy:
///   [Resize Handle]  (layout footprint == the line's thickness)
///     ├── [Hit Target] (invisible overlay, centred on the line and wider than
///     │                 it — costs no layout space, so the divider reads as a
///     │                 single hairline between two bordered regions)
///     └── [Visual Affordance] (the line; fills the root)
#[derive(Clone, Debug, PartialEq)]
pub struct ResizeHandleSpec {
    /// Stable native instance scope. Shared render is stateless and two
    /// handles can legitimately share an axis, a name, and a range, so the
    /// caller states which handle this is; nothing derived from semantics or
    /// render order can tell them apart, and the backend keys focus and
    /// gesture state on it.
    pub instance_id: String,
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

impl ResizeHandleSpec {
    /// The instance scope has no default: an invented one is indistinguishable
    /// from a stated one, and the first duplicate silently shares a focus
    /// handle. There is no `Default` for the same reason.
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            orientation: Orientation::Horizontal,
            is_disabled: false,
            aria_label: None,
            aria_value_now: None,
            aria_value_min: 0.0,
            aria_value_max: 100.0,
        }
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
    /// Matches Svelte: `var(--poodle-color-border-subtle)`.
    pub fn border_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
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

    // -- Geometry --

    /// Thickness of the visual line — and of the handle's whole layout
    /// footprint, in rem. The grab area is an overlay (see
    /// [`Self::hit_size_rem`]), so the handle costs no layout space beyond the
    /// line itself. Contract §8 value.
    pub fn thickness_rem(&self) -> f32 {
        0.125
    }

    /// Grab-area extent across the resize axis, in rem. Centred on the line and
    /// overlapping the adjacent regions, so it never widens the divider.
    pub fn hit_size_rem(&self) -> f32 {
        0.5
    }

    /// Offset of the grab overlay from the line's leading edge, in rem. Negative:
    /// the overlay starts before the line and ends after it.
    pub fn hit_offset_rem(&self) -> f32 {
        -(self.hit_size_rem() - self.thickness_rem()) / 2.0
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
