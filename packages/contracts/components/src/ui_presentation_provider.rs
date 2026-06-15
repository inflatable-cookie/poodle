use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPresentationProviderSpec {
    pub density: ControlDensity,
    pub size_scale: ControlSize,
}

impl Default for UiPresentationProviderSpec {
    fn default() -> Self {
        Self {
            density: ControlDensity::Default,
            size_scale: ControlSize::Md,
        }
    }
}

impl UiPresentationProviderSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_size_scale(mut self, size_scale: ControlSize) -> Self {
        self.size_scale = size_scale;
        self
    }

    pub fn resolve_size(self, role: SemanticControlSizeRole) -> ControlSize {
        match (self.size_scale, role) {
            (_, SemanticControlSizeRole::Control) => self.size_scale,
            (ControlSize::Xs, SemanticControlSizeRole::Chrome) => ControlSize::Xs,
            (ControlSize::Sm, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
            (ControlSize::Md, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
            (ControlSize::Lg, SemanticControlSizeRole::Chrome) => ControlSize::Md,
            (ControlSize::Xl, SemanticControlSizeRole::Chrome) => ControlSize::Lg,
            (ControlSize::Xs, SemanticControlSizeRole::Prominent) => ControlSize::Sm,
            (ControlSize::Sm, SemanticControlSizeRole::Prominent) => ControlSize::Md,
            (ControlSize::Md, SemanticControlSizeRole::Prominent) => ControlSize::Lg,
            (ControlSize::Lg, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
            (ControlSize::Xl, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
        }
    }
}
