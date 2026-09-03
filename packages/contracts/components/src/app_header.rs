use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppHeaderSpec {
    pub title: Option<String>,
    /// Secondary text rendered beside the title in baseline alignment.
    /// Matches the `subtitle` prop on the contract doc.
    pub subtitle: Option<String>,
    pub is_drag_region: bool,
    pub aria_label: Option<String>,
    /// Explicit semantic size override for header height + title/subtitle text
    /// (contract §3 `size`). `None` resolves the role-default (`Md`).
    pub size: Option<ControlSize>,
    /// Semantic role used to resolve the inherited size scale (contract §3
    /// `sizeRole`). Defaults to `Control`.
    pub size_role: SemanticControlSizeRole,
    /// Explicit density override for shell spacing (contract §3 `density`).
    /// `None` resolves to `Default`.
    pub density: Option<ControlDensity>,
    /// Presence of the optional centre region (contract §3/§4 `center`).
    /// Presence is the signal: it switches the grid to the symmetric
    /// side-column layout and groups actions + utility into the trailing
    /// column. The rendered content arrives as the renderer's `center` node.
    pub center: bool,
    pub primary_action_count: usize,
    pub utility_item_count: usize,
}

impl Default for AppHeaderSpec {
    fn default() -> Self {
        Self {
            title: None,
            subtitle: None,
            is_drag_region: false,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            center: false,
            primary_action_count: 0,
            utility_item_count: 0,
        }
    }
}

impl AppHeaderSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_drag_region(mut self, is_drag_region: bool) -> Self {
        self.is_drag_region = is_drag_region;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, role: SemanticControlSizeRole) -> Self {
        self.size_role = role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    /// Declare the optional centre region present (contract `center`). The
    /// renderer still keys the layout switch off the actual `center` node
    /// being passed, mirroring the web snippet's presence semantics.
    pub fn with_center(mut self, center: bool) -> Self {
        self.center = center;
        self
    }

    pub fn with_primary_action_count(mut self, primary_action_count: usize) -> Self {
        self.primary_action_count = primary_action_count;
        self
    }

    pub fn with_utility_item_count(mut self, utility_item_count: usize) -> Self {
        self.utility_item_count = utility_item_count;
        self
    }

    pub fn is_utility_heavy(&self) -> bool {
        self.utility_item_count > 0
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Border token for the shell bottom edge (contract §8/§9: `border-subtle`).
    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Title text token (contract §9: `text-primary`).
    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Subtitle text token (contract §9: `text-secondary`).
    pub fn subtitle_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    // ── Size / density resolution ────────────────────────────────
    //
    // Mirrors the Svelte `presentation.ts` `resolveSemanticControlSize` and the
    // `.poodle-app-header[data-size]` / `[data-density]` custom-property ladders
    // in `AppHeader.svelte` verbatim. The arithmetic is duplicated here (rather
    // than referencing a per-target presentation module) so both Rust targets
    // resolve identical values from a single spec source of truth.

    /// Effective control size after applying `size_role` to a resolved base
    /// size. Omission is resolved by the render context, which supplies the
    /// base; an explicit `md` under a non-default scope stays `md`-based.
    pub fn effective_size(&self, base: ControlSize) -> ControlSize {
        resolve_semantic_size(base, self.size_role)
    }

    /// Shell min-height in rem (Svelte `--poodle-app-header-min-height` ladder).
    /// Base (md) is `size.panel.header`; the ladder overrides per size.
    pub fn min_height_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 2.25,
            ControlSize::Sm => 2.5,
            ControlSize::Md => 2.75,
            ControlSize::Lg => 3.0,
            ControlSize::Xl => 3.25,
        }
    }

    /// Title font-size in rem (Svelte `--poodle-app-header-title-size` ladder).
    pub fn title_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.8125,
            ControlSize::Sm => 0.875,
            ControlSize::Md => 0.9375,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.0625,
        }
    }

    /// Subtitle font-size in rem (Svelte `--poodle-app-header-subtitle-size` ladder).
    pub fn subtitle_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.71875,
            ControlSize::Md => 0.75,
            ControlSize::Lg => 0.8125,
            ControlSize::Xl => 0.875,
        }
    }

    /// Inter-region grid gap in rem (Svelte `--poodle-app-header-gap`).
    /// Base is `space.inline.md` (1rem); density overrides per Svelte.
    pub fn gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.0,
        }
    }

    /// Intra-region gap in rem (Svelte `--poodle-app-header-region-gap`).
    /// Base is `space.inline.sm` (0.5rem); density overrides per Svelte.
    pub fn region_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.625,
        }
    }

    /// Vertical padding in rem (Svelte `--poodle-app-header-padding-block`).
    /// Base is `space.control.y` (0.375rem); density overrides per Svelte.
    pub fn pad_y_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }
    }

    /// Horizontal padding in rem (Svelte `--poodle-app-header-padding-inline`).
    /// Base is `space.panel.x` (1rem); density insets/expands by 0.125rem per Svelte.
    pub fn pad_x_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.875,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.125,
        }
    }
}

/// Resolve a semantic size role against a base size (mirrors Svelte
/// `resolveSemanticControlSize` / the per-target presentation modules).
fn resolve_semantic_size(size: ControlSize, role: SemanticControlSizeRole) -> ControlSize {
    crate::types::resolve_semantic_control_size(size, role)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_header_spec_defaults_and_builder_methods() {
        let spec = AppHeaderSpec::new();
        assert_eq!(spec.title, None);
        assert_eq!(spec.subtitle, None);
        assert_eq!(spec.is_drag_region, false);
        assert_eq!(spec.aria_label, None);
        assert_eq!(spec.size, None);
        assert_eq!(spec.size_role, SemanticControlSizeRole::Control);
        assert_eq!(spec.density, None);
        assert_eq!(spec.center, false);
        assert_eq!(spec.primary_action_count, 0);
        assert_eq!(spec.utility_item_count, 0);
        assert!(!spec.is_utility_heavy());

        let configured = AppHeaderSpec::new()
            .with_title("Poodle Studio")
            .with_subtitle("v1.0")
            .with_drag_region(true)
            .with_aria_label("Main Header")
            .with_size(ControlSize::Lg)
            .with_size_role(SemanticControlSizeRole::Chrome)
            .with_density(ControlDensity::Comfortable)
            .with_center(true)
            .with_primary_action_count(2)
            .with_utility_item_count(3);

        assert_eq!(configured.title.as_deref(), Some("Poodle Studio"));
        assert_eq!(configured.subtitle.as_deref(), Some("v1.0"));
        assert!(configured.is_drag_region);
        assert_eq!(configured.aria_label.as_deref(), Some("Main Header"));
        assert_eq!(configured.size, Some(ControlSize::Lg));
        assert_eq!(configured.size_role, SemanticControlSizeRole::Chrome);
        assert_eq!(configured.density, Some(ControlDensity::Comfortable));
        assert!(configured.center);
        assert_eq!(configured.primary_action_count, 2);
        assert_eq!(configured.utility_item_count, 3);
        assert!(configured.is_utility_heavy());
    }

    #[test]
    fn app_header_semantic_tokens() {
        let spec = AppHeaderSpec::new();
        assert_eq!(spec.background_token(), semantic::COLOR_BACKGROUND_PANEL);
        assert_eq!(spec.border_token(), semantic::COLOR_BORDER_SUBTLE);
        assert_eq!(spec.title_color_token(), semantic::COLOR_TEXT_PRIMARY);
        assert_eq!(spec.subtitle_color_token(), semantic::COLOR_TEXT_SECONDARY);
    }

    #[test]
    fn app_header_size_and_typography_ladders() {
        let spec = AppHeaderSpec::new();
        // Min-height ladder in rem
        assert_eq!(spec.min_height_rem(ControlSize::Xs), 2.25);
        assert_eq!(spec.min_height_rem(ControlSize::Sm), 2.5);
        assert_eq!(spec.min_height_rem(ControlSize::Md), 2.75);
        assert_eq!(spec.min_height_rem(ControlSize::Lg), 3.0);
        assert_eq!(spec.min_height_rem(ControlSize::Xl), 3.25);

        // Title size ladder in rem
        assert_eq!(spec.title_size_rem(ControlSize::Xs), 0.8125);
        assert_eq!(spec.title_size_rem(ControlSize::Sm), 0.875);
        assert_eq!(spec.title_size_rem(ControlSize::Md), 0.9375);
        assert_eq!(spec.title_size_rem(ControlSize::Lg), 1.0);
        assert_eq!(spec.title_size_rem(ControlSize::Xl), 1.0625);

        // Subtitle size ladder in rem
        assert_eq!(spec.subtitle_size_rem(ControlSize::Xs), 0.6875);
        assert_eq!(spec.subtitle_size_rem(ControlSize::Sm), 0.71875);
        assert_eq!(spec.subtitle_size_rem(ControlSize::Md), 0.75);
        assert_eq!(spec.subtitle_size_rem(ControlSize::Lg), 0.8125);
        assert_eq!(spec.subtitle_size_rem(ControlSize::Xl), 0.875);
    }

    #[test]
    fn app_header_density_ladder() {
        let spec = AppHeaderSpec::new();
        // Inter-region gap
        assert_eq!(spec.gap_rem(ControlDensity::Compact), 0.625);
        assert_eq!(spec.gap_rem(ControlDensity::Default), 1.0);
        assert_eq!(spec.gap_rem(ControlDensity::Comfortable), 1.0);

        // Intra-region gap
        assert_eq!(spec.region_gap_rem(ControlDensity::Compact), 0.375);
        assert_eq!(spec.region_gap_rem(ControlDensity::Default), 0.5);
        assert_eq!(spec.region_gap_rem(ControlDensity::Comfortable), 0.625);

        // Vertical padding
        assert_eq!(spec.pad_y_rem(ControlDensity::Compact), 0.25);
        assert_eq!(spec.pad_y_rem(ControlDensity::Default), 0.375);
        assert_eq!(spec.pad_y_rem(ControlDensity::Comfortable), 0.5);

        // Horizontal padding
        assert_eq!(spec.pad_x_rem(ControlDensity::Compact), 0.875);
        assert_eq!(spec.pad_x_rem(ControlDensity::Default), 1.0);
        assert_eq!(spec.pad_x_rem(ControlDensity::Comfortable), 1.125);
    }

    #[test]
    fn app_header_effective_size_resolution() {
        let spec_control = AppHeaderSpec::new().with_size_role(SemanticControlSizeRole::Control);
        assert_eq!(spec_control.effective_size(ControlSize::Md), ControlSize::Md);

        let spec_chrome = AppHeaderSpec::new().with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(spec_chrome.effective_size(ControlSize::Md), ControlSize::Sm);

        let spec_prominent = AppHeaderSpec::new().with_size_role(SemanticControlSizeRole::Prominent);
        assert_eq!(spec_prominent.effective_size(ControlSize::Md), ControlSize::Lg);
    }
}
