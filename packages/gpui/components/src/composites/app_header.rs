//! AppHeader — real GPUI component backed by AppHeaderSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::AppHeaderSpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI app header bar backed by `AppHeaderSpec`.
///
/// Renders a horizontal header with optional title, primary actions slot,
/// and utility items slot. Supports drag-region for window dragging.
pub struct AppHeader {
    spec: AppHeaderSpec,
    theme: GpuiThemeProvider,
    /// Slot for primary action elements (e.g., buttons).
    primary_actions: Option<AnyElement>,
    /// Slot for utility items (e.g., user avatar, notifications).
    utility_items: Option<AnyElement>,
    /// Slot for navigation/leading content.
    leading: Option<AnyElement>,
}

impl std::ops::Deref for AppHeader {
    type Target = AppHeaderSpec;
    fn deref(&self) -> &AppHeaderSpec { &self.spec }
}

impl AppHeader {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: AppHeaderSpec::new(), theme: theme.clone(), primary_actions: None, utility_items: None, leading: None }
    }

    pub fn from_spec(spec: AppHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            primary_actions: None,
            utility_items: None,
            leading: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn drag_region(mut self, v: bool) -> Self { self.spec.is_drag_region = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn primary_action_count(mut self, v: usize) -> Self { self.spec.primary_action_count = v; self }
    pub fn utility_item_count(mut self, v: usize) -> Self { self.spec.utility_item_count = v; self }


    pub fn with_primary_actions(mut self, actions: impl IntoElement) -> Self {
        self.primary_actions = Some(actions.into_any_element());
        self
    }

    pub fn with_utility_items(mut self, items: impl IntoElement) -> Self {
        self.utility_items = Some(items.into_any_element());
        self
    }

    pub fn with_leading(mut self, leading: impl IntoElement) -> Self {
        self.leading = Some(leading.into_any_element());
        self
    }
}

impl IntoElement for AppHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        let bg = resolve_color(theme, spec.background_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");

        let mut header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(44.0))
            .px(inline_padding)
            .bg(bg)
            .border_b_1()
            .border_color(border);

        // Left section: leading + title
        let mut left = div().flex().items_center().gap(inline_gap).flex_shrink_0();

        if let Some(leading) = self.leading {
            left = left.child(leading);
        }

        if let Some(ref title) = spec.title {
            left = left.child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        header = header.child(left);

        // Right section: actions + utility
        let mut right = div().flex().items_center().gap(inline_gap).flex_shrink_0();

        if let Some(primary_actions) = self.primary_actions {
            right = right.child(primary_actions);
        }

        if let Some(utility_items) = self.utility_items {
            right = right.child(utility_items);
        }

        header = header.child(right);

        header.into_any_element()
    }
}
