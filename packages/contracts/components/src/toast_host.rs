use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

use crate::ToastTone;

/// Placement of the toast host on screen.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastHostPlacement {
    #[default]
    BottomEnd,
    BottomStart,
    TopEnd,
    TopStart,
}

/// ToastHost -- a fixed-position container that manages toast lifecycle and auto-dismiss.
#[derive(Clone, Debug, PartialEq)]
pub struct ToastHostSpec {
    pub auto_dismiss_ms: u32,
    pub sticky_tones: Vec<ToastTone>,
    pub placement: ToastHostPlacement,
    pub aria_label: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl ToastHostSpec {
    pub fn new() -> Self {
        Self {
            auto_dismiss_ms: 6000,
            sticky_tones: vec![ToastTone::Danger],
            placement: ToastHostPlacement::default(),
            aria_label: String::from("Notifications"),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }

    pub fn with_auto_dismiss_ms(mut self, ms: u32) -> Self {
        self.auto_dismiss_ms = ms;
        self
    }

    pub fn with_sticky_tones(mut self, tones: Vec<ToastTone>) -> Self {
        self.sticky_tones = tones;
        self
    }

    pub fn with_placement(mut self, placement: ToastHostPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    /// Whether a toast with the given tone should be treated as sticky (not auto-dismissed).
    pub fn is_sticky_tone(&self, tone: &ToastTone) -> bool {
        self.sticky_tones.contains(tone)
    }

    /// Whether auto-dismiss is enabled (positive duration).
    pub fn auto_dismiss_enabled(&self) -> bool {
        self.auto_dismiss_ms > 0
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
