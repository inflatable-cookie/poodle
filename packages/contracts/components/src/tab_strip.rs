use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, Orientation, TabStripItem};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabStripSpec {
    pub items: Vec<TabStripItem>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub orientation: Orientation,
    pub is_reorderable: bool,
    pub aria_label: Option<String>,
    /// Physical size of each tab item (height, font-size, padding). Defaults to Md.
    pub size: ControlSize,
    /// Horizontal density — controls padding-x and inline gap. Defaults to Default.
    pub density: ControlDensity,
}

impl Default for TabStripSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            orientation: Orientation::Horizontal,
            is_reorderable: false,
            aria_label: None,
            size: ControlSize::Md,
            density: ControlDensity::Default,
        }
    }
}

impl TabStripSpec {
    pub fn new(items: Vec<TabStripItem>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_reorderable(mut self, is_reorderable: bool) -> Self {
        self.is_reorderable = is_reorderable;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
            .or_else(|| {
                self.items
                    .iter()
                    .find(|item| !item.is_disabled)
                    .map(|item| item.value.as_str())
            })
    }

    pub fn current_item(&self) -> Option<&TabStripItem> {
        let current = self.current_value()?;
        self.items.iter().find(|item| item.value == current)
    }

    pub fn closable_item_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_closable).count()
    }

    pub fn item_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    /// Per-item close-button square size, in rem (contract §2 CloseButton:
    /// `1.25rem`). Centralizes the literal so impls feed it through `rem_to_px`
    /// instead of carrying a bare float.
    pub fn close_button_size_rem(&self) -> f32 {
        1.25
    }

    /// Base radius token for the close button; impls subtract
    /// `close_button_radius_inset_rem()` from the resolved value
    /// (contract: `radius-control − 0.125rem`).
    pub fn close_button_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Radius inset (in rem) applied to the close button vs `radius-control`
    /// (contract §2: `radius-control − 0.125rem`).
    pub fn close_button_radius_inset_rem(&self) -> f32 {
        0.125
    }

    /// Inter-affordance gap token used between the label and the close button
    /// inside a tab (contract §6 follows the inline spacing scale).
    pub fn close_button_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_XS
    }

    /// Opacity multiplier for the vertical-orientation active tab fill tint
    /// (accent over surface). Names the previously-magic `0.08` multiplier so
    /// both targets share one source of truth.
    pub fn vertical_active_fill_opacity(&self) -> f32 {
        0.08
    }
}
