//! AppHeader — real GPUI composite backed by AppHeaderSpec.
//!
//! Contract: `docs/contracts/components/app-header.md`
//! Reference: `packages/svelte/components/src/AppHeader.svelte`
//!
//! Renders the contract §2 three-region shell:
//!   - Identity region (title group: title + optional subtitle, or a custom
//!     `identity` slot via `with_identity`)
//!   - Actions region (optional global-actions slot)
//!   - Utility region (optional trailing utility slot, right-aligned)
//!
//! GPUI has no CSS grid; the contract `grid minmax(0,1fr) auto auto` is
//! emulated with flex — identity grows (`flex_1` + `min_w_0` for truncation),
//! actions/utility hold intrinsic width (`flex_shrink_0`), utility justifies
//! to the end. All dimensions/colors resolve from tokens or contract-exact rem
//! ladders carried on `AppHeaderSpec`; zero hardcoded hsla.
//!
//! `aria_label` is stored on the spec for cross-runtime parity but GPUI's
//! fluent element API does not yet emit ARIA / header-landmark labels.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::AppHeaderSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color};

/// A real GPUI app header bar backed by `AppHeaderSpec`.
pub struct AppHeader {
    spec: AppHeaderSpec,
    theme: GpuiThemeProvider,
    /// Custom identity content (contract `identity()` snippet). Replaces the
    /// default title group when present.
    identity: Option<AnyElement>,
    /// Global-actions slot (contract `actions()` snippet).
    primary_actions: Option<AnyElement>,
    /// Trailing utility slot (contract `utility()` snippet).
    utility_items: Option<AnyElement>,
}

impl std::ops::Deref for AppHeader {
    type Target = AppHeaderSpec;
    fn deref(&self) -> &AppHeaderSpec {
        &self.spec
    }
}

impl AppHeader {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: AppHeaderSpec::new(),
            theme: theme.clone(),
            identity: None,
            primary_actions: None,
            utility_items: None,
        }
    }

    pub fn from_spec(spec: AppHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            identity: None,
            primary_actions: None,
            utility_items: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self {
        self.spec.subtitle = Some(v.into());
        self
    }
    pub fn drag_region(mut self, v: bool) -> Self {
        self.spec.is_drag_region = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn primary_action_count(mut self, v: usize) -> Self {
        self.spec.primary_action_count = v;
        self
    }
    pub fn utility_item_count(mut self, v: usize) -> Self {
        self.spec.utility_item_count = v;
        self
    }

    pub fn with_primary_actions(mut self, actions: impl IntoElement) -> Self {
        self.primary_actions = Some(actions.into_any_element());
        self
    }

    pub fn with_utility_items(mut self, items: impl IntoElement) -> Self {
        self.utility_items = Some(items.into_any_element());
        self
    }

    /// Custom identity slot (contract `identity()`). Replaces the default
    /// title group.
    pub fn with_identity(mut self, identity: impl IntoElement) -> Self {
        self.identity = Some(identity.into_any_element());
        self
    }

    /// Back-compat alias for the contract `identity` slot.
    pub fn with_leading(self, leading: impl IntoElement) -> Self {
        self.with_identity(leading)
    }
}

impl IntoElement for AppHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Token / contract-rem resolution ──────────────────────
        // Background: contract §9 color-mix(background-panel 94%, transparent).
        let panel = resolve_color(theme, spec.background_token());
        let transparent = Hsla {
            a: 0.0,
            ..panel
        };
        let bg = color_mix(panel, transparent, 0.94);
        let border = resolve_color(theme, spec.border_token());
        let title_color = resolve_color(theme, spec.title_color_token());
        let subtitle_color = resolve_color(theme, spec.subtitle_color_token());

        // Size ladder (height + title/subtitle font) and density ladder
        // (region gaps + padding) all carried on the spec.
        let min_height = px(rem_to_px(spec.min_height_rem()));
        let title_size = px(rem_to_px(spec.title_size_rem()));
        let subtitle_size = px(rem_to_px(spec.subtitle_size_rem()));
        let grid_gap = px(rem_to_px(spec.gap_rem()));
        let region_gap = px(rem_to_px(spec.region_gap_rem()));
        let pad_y = px(rem_to_px(spec.pad_y_rem()));
        let pad_x = px(rem_to_px(spec.pad_x_rem()));

        // ── Root shell ───────────────────────────────────────────
        let mut header = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(grid_gap)
            .w_full()
            .min_h(min_height)
            .px(pad_x)
            .py(pad_y)
            .bg(bg)
            .border_b_1()
            .border_color(border);

        // Native window-drag affordance (contract `dragRegion` → data-drag-region).
        if spec.is_drag_region {
            header = header.window_control_area(WindowControlArea::Drag);
        }

        // ── Identity region (grid column 1: minmax(0, 1fr)) ──────
        // Grows to fill, min-width 0 so the subtitle can truncate.
        let mut identity = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(region_gap)
            .flex_1()
            .min_w_0();

        if let Some(custom) = self.identity {
            identity = identity.child(custom);
        } else if spec.title.is_some() {
            // Default title group: title + optional subtitle, baseline-aligned.
            let mut title_group = div()
                .flex()
                .flex_row()
                .items_baseline()
                .gap(region_gap)
                .min_w_0();

            if let Some(ref title) = spec.title {
                title_group = title_group.child(
                    div()
                        .text_size(title_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(title_color)
                        .line_height(relative(1.2))
                        .whitespace_nowrap()
                        .child(title.clone()),
                );
            }

            if let Some(ref subtitle) = spec.subtitle {
                title_group = title_group.child(
                    div()
                        .text_size(subtitle_size)
                        .text_color(subtitle_color)
                        .line_height(relative(1.2))
                        .whitespace_nowrap()
                        .overflow_hidden()
                        .min_w_0()
                        .child(subtitle.clone()),
                );
            }

            identity = identity.child(title_group);
        }

        header = header.child(identity);

        // ── Actions region (grid column 2: auto) ─────────────────
        if let Some(actions) = self.primary_actions {
            header = header.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(region_gap)
                    .flex_shrink_0()
                    .child(actions),
            );
        }

        // ── Utility region (grid column 3: auto, justify-end) ────
        if let Some(utility) = self.utility_items {
            header = header.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap(region_gap)
                    .flex_shrink_0()
                    .child(utility),
            );
        }

        header.into_any_element()
    }
}
