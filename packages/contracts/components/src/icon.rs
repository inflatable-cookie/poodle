use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Icon size variants matching the contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum IconSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl IconSize {
    pub fn size_token(self) -> &'static str {
        match self {
            Self::Sm => semantic::SIZE_ICON_SM,
            Self::Md => semantic::SIZE_ICON_MD,
            Self::Lg => semantic::SIZE_ICON_LG,
        }
    }
}

impl From<ControlSize> for IconSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Xs => IconSize::Sm,
            ControlSize::Sm => IconSize::Sm,
            ControlSize::Md => IconSize::Md,
            ControlSize::Lg => IconSize::Lg,
            ControlSize::Xl => IconSize::Lg,
        }
    }
}

/// Spec for the Icon component per the contract.
///
/// A sized, accessible inline SVG icon element that resolves names
/// from an icon registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconSpec {
    /// Icon registry identifier (e.g., "plus", "chevron-down").
    pub name: String,
    /// Icon dimensions.
    pub size: IconSize,
    /// Accessible name; absence triggers decorative mode.
    pub aria_label: Option<String>,
    /// Presentation axes (contract §3): size is intrinsic, density is sibling
    /// spacing, size_role resolves size from the inherited presentation.
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl IconSpec {
    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: IconSize::default(),
            aria_label: None,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Whether this icon is decorative (no accessible name).
    pub fn is_decorative(&self) -> bool {
        self.aria_label.is_none()
    }

    /// Size after the semantic role is applied.
    ///
    /// `size` is the inherited scale; `size_role` shifts it one stop, clamped
    /// at each end. Mirrors the Svelte
    /// `size ?? resolveSemanticControlSize(scale, sizeRole)`.
    pub fn resolved_size(&self) -> IconSize {
        match (self.size, self.size_role) {
            (size, SemanticControlSizeRole::Control) => size,

            (IconSize::Sm, SemanticControlSizeRole::Chrome) => IconSize::Sm,
            (IconSize::Md, SemanticControlSizeRole::Chrome) => IconSize::Sm,
            (IconSize::Lg, SemanticControlSizeRole::Chrome) => IconSize::Md,

            (IconSize::Sm, SemanticControlSizeRole::Prominent) => IconSize::Md,
            (IconSize::Md, SemanticControlSizeRole::Prominent) => IconSize::Lg,
            (IconSize::Lg, SemanticControlSizeRole::Prominent) => IconSize::Lg,
        }
    }

    /// The size token for resolving dimensions.
    pub fn size_token(&self) -> &'static str {
        self.resolved_size().size_token()
    }
}

#[cfg(test)]
mod size_role_tests {
    use super::*;

    /// Icon has three stops rather than five, so the role clamps sooner — but
    /// it still shifts, which is what keeps an icon in chrome from out-sizing
    /// the control it sits in.
    #[test]
    fn size_role_shifts_within_the_three_icon_stops() {
        let base = IconSpec::new("plus").with_size(IconSize::Md);
        assert_eq!(base.resolved_size(), IconSize::Md);
        assert_eq!(base.size_token(), semantic::SIZE_ICON_MD);

        let chrome = IconSpec::new("plus")
            .with_size(IconSize::Md)
            .with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(chrome.resolved_size(), IconSize::Sm);
        assert_eq!(chrome.size_token(), semantic::SIZE_ICON_SM);

        let prominent = IconSpec::new("plus")
            .with_size(IconSize::Md)
            .with_size_role(SemanticControlSizeRole::Prominent);
        assert_eq!(prominent.resolved_size(), IconSize::Lg);
    }

    #[test]
    fn roles_clamp_at_both_ends() {
        let smallest = IconSpec::new("plus")
            .with_size(IconSize::Sm)
            .with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(smallest.resolved_size(), IconSize::Sm);

        let largest = IconSpec::new("plus")
            .with_size(IconSize::Lg)
            .with_size_role(SemanticControlSizeRole::Prominent);
        assert_eq!(largest.resolved_size(), IconSize::Lg);
    }
}
