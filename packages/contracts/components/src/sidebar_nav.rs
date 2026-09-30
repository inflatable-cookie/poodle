use crate::{types::MenuEntry, ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// A single navigation item in a sidebar group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SidebarNavItem {
    pub value: String,
    pub label: String,
    pub href: Option<String>,
    pub is_disabled: bool,
    /// Compact end-aligned metadata such as a count ("198" in "Videos 198").
    /// Exposed as the item's accessible description, never its name; put
    /// counts here, not in `label`. `None` renders the label as the item's
    /// direct text with no description.
    pub end_label: Option<String>,
    /// Per-item context-menu rows (same shape as ListCard's
    /// `context_menu_items`). Non-empty on a non-disabled item, secondary
    /// click or the keyboard menu gesture opens the shared ContextMenu for
    /// that item. Unset or empty leaves item behaviour unchanged.
    pub context_menu_items: Vec<MenuEntry>,
    /// Accessible name for the item's context-menu overlay. When `None`, the
    /// overlay is labelled `{label} actions`.
    pub context_menu_aria_label: Option<String>,
}

impl SidebarNavItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            href: None,
            is_disabled: false,
            end_label: None,
            context_menu_items: Vec::new(),
            context_menu_aria_label: None,
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

    pub fn with_end_label(mut self, end_label: impl Into<String>) -> Self {
        self.end_label = Some(end_label.into());
        self
    }

    pub fn with_context_menu_items(mut self, items: Vec<MenuEntry>) -> Self {
        self.context_menu_items = items;
        self
    }

    pub fn with_context_menu_aria_label(mut self, label: impl Into<String>) -> Self {
        self.context_menu_aria_label = Some(label.into());
        self
    }

    /// Whether the item hosts its built-in context menu: only a non-disabled
    /// item with at least one row does. Disabled items never open a menu,
    /// and unset or empty rows leave native item behaviour unchanged.
    pub fn has_context_menu(&self) -> bool {
        !self.is_disabled && !self.context_menu_items.is_empty()
    }

    /// The item's context-menu accessible name: the explicit override, else
    /// the generated `{label} actions` default.
    pub fn context_menu_aria_label_or_default(&self) -> String {
        self.context_menu_aria_label
            .clone()
            .unwrap_or_else(|| format!("{} actions", self.label))
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
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl SidebarNavSpec {
    pub fn new(groups: Vec<SidebarNavGroup>) -> Self {
        Self {
            groups,
            value: None,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
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

    pub fn end_label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    /// Gap between the flexible label and the end label in rem (contract §8):
    /// half the item's inline padding, so it scales with density.
    pub fn end_label_gap_rem(&self, density: ControlDensity) -> f32 {
        self.item_pad_inline_rem(density) * 0.5
    }

    /// End-label font-size in rem (contract §8): 0.85× the item font, so it
    /// scales with size.
    pub fn end_label_font_rem(&self, size: ControlSize) -> f32 {
        self.item_font_rem(size) * 0.85
    }

    /// Effective control size after resolving the semantic size role against
    /// the presentation-resolved `size`. The sidebar's own size table keys off
    /// the raw size (matching Svelte CSS `[data-size]` overrides), so this is
    /// exposed for callers that need the inherited presentation size for
    /// children, not the item geometry.
    pub fn effective_size(&self, size: ControlSize) -> ControlSize {
        resolve_semantic_size(size, self.size_role)
    }

    /// Item min-height in rem, by raw size (contract §8 Size Variants). Keyed off
    /// the raw `data-size` like the Svelte CSS, not the chrome-resolved size.
    pub fn item_height_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 1.375,
            ControlSize::Sm => 1.625,
            ControlSize::Md => 1.875,
            ControlSize::Lg => 2.125,
            ControlSize::Xl => 2.375,
        }
    }

    /// Item font-size in rem, by raw size (contract §8 Size Variants).
    pub fn item_font_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    /// Group-title font-size in rem, by raw size (contract §8 Size Variants).
    pub fn title_font_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.46875,
            ControlSize::Sm => 0.5,
            ControlSize::Md => 0.5625,
            ControlSize::Lg => 0.59375,
            ControlSize::Xl => 0.625,
        }
    }

    /// Gap between groups in rem, by density (contract §8 Density Variants).
    pub fn group_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Item horizontal padding in rem, by density (contract §8 Density Variants).
    pub fn item_pad_inline_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Item vertical padding in rem, by density (contract §8 Density Variants).
    pub fn item_pad_block_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.3125,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.4375,
        }
    }

    /// Gap between a group title and its list in rem, by density (contract §8).
    pub fn title_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.125,
            ControlDensity::Default => 0.1875,
            ControlDensity::Comfortable => 0.25,
        }
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
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
        let spec = SidebarNavSpec::new(vec![]);
        assert_eq!(spec.item_height_rem(ControlSize::Md), 1.875);
        assert_eq!(spec.item_height_rem(ControlSize::Xs), 1.375);
        assert_eq!(spec.item_height_rem(ControlSize::Xl), 2.375);
    }

    #[test]
    fn density_spacing_tracks_table() {
        let spec = SidebarNavSpec::new(vec![]);
        assert_eq!(spec.group_gap_rem(ControlDensity::Compact), 0.625);
        assert_eq!(spec.item_pad_block_rem(ControlDensity::Compact), 0.3125);
        assert_eq!(spec.item_pad_inline_rem(ControlDensity::Comfortable), 0.875);
        assert_eq!(spec.title_gap_rem(ControlDensity::Comfortable), 0.25);
    }

    #[test]
    fn chrome_role_resolves_one_stop_smaller() {
        let spec = SidebarNavSpec::new(vec![]).with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(spec.effective_size(ControlSize::Md), ControlSize::Sm);
    }

    #[test]
    fn end_label_geometry_scales_with_size_and_density() {
        let spec = SidebarNavSpec::new(vec![]);
        assert_eq!(spec.end_label_font_rem(ControlSize::Md), 0.8125 * 0.85);
        assert_eq!(spec.end_label_gap_rem(ControlDensity::Default), 0.375);
        assert_eq!(spec.end_label_gap_rem(ControlDensity::Comfortable), 0.4375);
    }

    #[test]
    fn context_menu_semantics_follow_the_contract() {
        let plain = SidebarNavItem::new("all", "All records");
        assert!(!plain.has_context_menu());
        assert_eq!(
            plain.context_menu_aria_label_or_default(),
            "All records actions"
        );

        let rows = vec![MenuEntry::new("rename", "Rename")];
        let hosted = SidebarNavItem::new("q4", "Q4 close")
            .with_context_menu_items(rows.clone())
            .with_context_menu_aria_label("Q4 close actions");
        assert!(hosted.has_context_menu());
        assert_eq!(
            hosted.context_menu_aria_label_or_default(),
            "Q4 close actions"
        );

        // A disabled item never hosts a menu, even with rows set.
        let disabled = SidebarNavItem::new("archive", "Archive")
            .with_context_menu_items(rows)
            .with_disabled(true);
        assert!(!disabled.has_context_menu());
        // And the generated default still names itself after the item.
        assert_eq!(
            disabled.context_menu_aria_label_or_default(),
            "Archive actions"
        );
    }
}
