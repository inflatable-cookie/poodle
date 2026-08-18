use poodle_tokens::semantic;

use crate::types::{ControlSize, SemanticControlSizeRole};

/// Icon size variants matching the contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum IconSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl IconSize {
    pub fn size_token(self) -> &'static str {
        match self {
            Self::Xs => semantic::SIZE_ICON_XS,
            Self::Sm => semantic::SIZE_ICON_SM,
            Self::Md => semantic::SIZE_ICON_MD,
            Self::Lg => semantic::SIZE_ICON_LG,
            Self::Xl => semantic::SIZE_ICON_XL,
        }
    }
}

impl From<ControlSize> for IconSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Xs => IconSize::Xs,
            ControlSize::Sm => IconSize::Sm,
            ControlSize::Md => IconSize::Md,
            ControlSize::Lg => IconSize::Lg,
            ControlSize::Xl => IconSize::Xl,
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
    /// Presentation axes (contract §3): size is intrinsic; size_role resolves
    /// size from the inherited presentation.
    pub size_role: SemanticControlSizeRole,
}

impl IconSpec {
    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: IconSize::default(),
            aria_label: None,
            size_role: SemanticControlSizeRole::Control,
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

            (IconSize::Xs, SemanticControlSizeRole::Chrome) => IconSize::Xs,
            (IconSize::Sm, SemanticControlSizeRole::Chrome) => IconSize::Xs,
            (IconSize::Md, SemanticControlSizeRole::Chrome) => IconSize::Sm,
            (IconSize::Lg, SemanticControlSizeRole::Chrome) => IconSize::Md,
            (IconSize::Xl, SemanticControlSizeRole::Chrome) => IconSize::Lg,

            (IconSize::Xs, SemanticControlSizeRole::Prominent) => IconSize::Sm,
            (IconSize::Sm, SemanticControlSizeRole::Prominent) => IconSize::Md,
            (IconSize::Md, SemanticControlSizeRole::Prominent) => IconSize::Lg,
            (IconSize::Lg, SemanticControlSizeRole::Prominent) => IconSize::Xl,
            (IconSize::Xl, SemanticControlSizeRole::Prominent) => IconSize::Xl,
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

    #[test]
    fn size_role_shifts_within_the_five_icon_stops() {
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
        assert_eq!(prominent.size_token(), semantic::SIZE_ICON_LG);
    }

    #[test]
    fn roles_clamp_at_both_ends() {
        let smallest = IconSpec::new("plus")
            .with_size(IconSize::Xs)
            .with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(smallest.resolved_size(), IconSize::Xs);
        assert_eq!(smallest.size_token(), semantic::SIZE_ICON_XS);

        let largest = IconSpec::new("plus")
            .with_size(IconSize::Xl)
            .with_size_role(SemanticControlSizeRole::Prominent);
        assert_eq!(largest.resolved_size(), IconSize::Xl);
        assert_eq!(largest.size_token(), semantic::SIZE_ICON_XL);
    }

    #[test]
    fn control_size_maps_one_to_one_without_endpoint_collapse() {
        assert_eq!(IconSize::from(ControlSize::Xs), IconSize::Xs);
        assert_eq!(IconSize::from(ControlSize::Xl), IconSize::Xl);
    }
}
