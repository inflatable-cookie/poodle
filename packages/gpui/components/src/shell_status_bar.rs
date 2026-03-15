//! PugShellStatusBar — real GPUI component backed by ShellStatusBarSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::ShellStatusBarSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI status bar backed by `ShellStatusBarSpec`.
///
/// Renders a bottom status bar with leading items, summary text,
/// and trailing items arranged in a horizontal strip.
pub struct PugShellStatusBar {
    spec: ShellStatusBarSpec,
    theme: GpuiThemeProvider,
    /// Slot for leading status items (left side).
    leading_items: Option<AnyElement>,
    /// Slot for trailing status items (right side).
    trailing_items: Option<AnyElement>,
}

impl PugShellStatusBar {
    pub fn new(spec: ShellStatusBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            leading_items: None,
            trailing_items: None,
        }
    }

    pub fn with_leading_items(mut self, items: impl IntoElement) -> Self {
        self.leading_items = Some(items.into_any_element());
        self
    }

    pub fn with_trailing_items(mut self, items: impl IntoElement) -> Self {
        self.trailing_items = Some(items.into_any_element());
        self
    }
}

impl IntoElement for PugShellStatusBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let bg = resolve_color(theme, spec.background_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let mut bar = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(24.0))
            .px(px(8.0))
            .bg(bg)
            .border_t_1()
            .border_color(border);

        // Left section: leading items + summary
        let mut left = div().flex().items_center().gap(px(6.0)).flex_1().min_w_0();

        if let Some(leading_items) = self.leading_items {
            left = left.child(leading_items);
        }

        if let Some(ref summary) = spec.summary {
            left = left.child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .overflow_x_hidden()
                    .whitespace_nowrap()
                    .child(summary.clone()),
            );
        }

        bar = bar.child(left);

        // Right section: trailing items
        if let Some(trailing_items) = self.trailing_items {
            bar = bar.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .flex_shrink_0()
                    .child(trailing_items),
            );
        }

        bar.into_any_element()
    }
}
