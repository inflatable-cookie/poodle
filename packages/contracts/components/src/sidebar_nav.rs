use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// A single navigation item in a sidebar group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SidebarNavItem {
    pub value: String,
    pub label: String,
    pub href: Option<String>,
    pub is_disabled: bool,
}

impl SidebarNavItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            href: None,
            is_disabled: false,
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

/// A labelled group of navigation items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SidebarNavGroup {
    pub id: String,
    pub label: Option<String>,
    pub items: Vec<SidebarNavItem>,
}

impl SidebarNavGroup {
    pub fn new(id: impl Into<String>, items: Vec<SidebarNavItem>) -> Self {
        Self {
            id: id.into(),
            label: None,
            items,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// SidebarNav -- a vertical navigation component with grouped, labelled items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SidebarNavSpec {
    pub groups: Vec<SidebarNavGroup>,
    pub value: Option<String>,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl SidebarNavSpec {
    pub fn new(groups: Vec<SidebarNavGroup>) -> Self {
        Self {
            groups,
            value: None,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// Groups that have at least one item.
    pub fn visible_groups(&self) -> Vec<&SidebarNavGroup> {
        self.groups.iter().filter(|g| !g.items.is_empty()).collect()
    }

    /// Total number of navigation items across all visible groups.
    pub fn total_item_count(&self) -> usize {
        self.visible_groups().iter().map(|g| g.items.len()).sum()
    }

    /// Whether the given item value is currently active.
    pub fn is_active(&self, item_value: &str) -> bool {
        self.value.as_deref() == Some(item_value)
    }

    pub fn item_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn item_active_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn group_title_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn active_indicator_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn active_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        "state.opacity.disabled"
    }

    /// Effective control size after resolving the semantic size role. The
    /// sidebar's own size table keys off the raw `data-size` (matching Svelte
    /// CSS `[data-size]` overrides), so this is exposed for callers that need the
    /// inherited presentation size for children, not the item geometry.
    pub fn effective_size(&self) -> ControlSize {
        resolve_semantic_size(self.size, self.size_role)
    }

    /// Item min-height in rem, by raw size (contract §8 Size Variants). Keyed off
    /// the raw `data-size` like the Svelte CSS, not the chrome-resolved size.
    pub fn item_height_rem(&self) -> f32 {
        match self.size {
            ControlSize::Xs => 1.375,
            ControlSize::Sm => 1.625,
            ControlSize::Md => 1.875,
            ControlSize::Lg => 2.125,
            ControlSize::Xl => 2.375,
        }
    }

    /// Item font-size in rem, by raw size (contract §8 Size Variants).
    pub fn item_font_rem(&self) -> f32 {
        match self.size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    /// Group-title font-size in rem, by raw size (contract §8 Size Variants).
    pub fn title_font_rem(&self) -> f32 {
        match self.size {
            ControlSize::Xs => 0.46875,
            ControlSize::Sm => 0.5,
            ControlSize::Md => 0.5625,
            ControlSize::Lg => 0.59375,
            ControlSize::Xl => 0.625,
        }
    }

    /// Gap between groups in rem, by density (contract §8 Density Variants).
    pub fn group_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Item horizontal padding in rem, by density (contract §8 Density Variants).
    pub fn item_pad_inline_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Item vertical padding in rem, by density (contract §8 Density Variants).
    pub fn item_pad_block_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.3125,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.4375,
        }
    }

    /// Gap between a group title and its list in rem, by density (contract §8).
    pub fn title_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.125,
            ControlDensity::Default => 0.1875,
            ControlDensity::Comfortable => 0.25,
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

/// Resolve a semantic size role against a base size (chrome → one stop smaller,
/// prominent → one stop larger). Mirrors `presentation::resolve_semantic_size`.
fn resolve_semantic_size(size: ControlSize, role: SemanticControlSizeRole) -> ControlSize {
    crate::types::resolve_semantic_control_size(size, role)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SidebarNavSpec {
        SidebarNavSpec::new(vec![
            SidebarNavGroup::new("g1", vec![SidebarNavItem::new("a", "Alpha")]).with_label("One"),
            SidebarNavGroup::new("g2", vec![]), // empty → filtered out
        ])
    }

    #[test]
    fn visible_groups_drops_empty() {
        assert_eq!(sample().visible_groups().len(), 1);
        assert_eq!(sample().total_item_count(), 1);
    }

    #[test]
    fn active_detection_matches_value() {
        let spec = sample().with_value("a");
        assert!(spec.is_active("a"));
        assert!(!spec.is_active("b"));
    }

    #[test]
    fn item_height_tracks_size_table() {
        // sidebar table, NOT control-height (md = 1.875rem, not 2.25rem)
        assert_eq!(
            SidebarNavSpec::new(vec![])
                .with_size(ControlSize::Md)
                .item_height_rem(),
            1.875
        );
        assert_eq!(
            SidebarNavSpec::new(vec![])
                .with_size(ControlSize::Xs)
                .item_height_rem(),
            1.375
        );
        assert_eq!(
            SidebarNavSpec::new(vec![])
                .with_size(ControlSize::Xl)
                .item_height_rem(),
            2.375
        );
    }

    #[test]
    fn density_spacing_tracks_table() {
        let compact = SidebarNavSpec::new(vec![]).with_density(ControlDensity::Compact);
        let comfy = SidebarNavSpec::new(vec![]).with_density(ControlDensity::Comfortable);
        assert_eq!(compact.group_gap_rem(), 0.625);
        assert_eq!(compact.item_pad_block_rem(), 0.3125);
        assert_eq!(comfy.item_pad_inline_rem(), 0.875);
        assert_eq!(comfy.title_gap_rem(), 0.25);
    }

    #[test]
    fn chrome_role_resolves_one_stop_smaller() {
        let spec = SidebarNavSpec::new(vec![])
            .with_size(ControlSize::Md)
            .with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(spec.effective_size(), ControlSize::Sm);
    }
}
