//! StatusIndicator — real GPUI component backed by StatusIndicatorSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{StatusIndicatorSpec, StatusTone};

use crate::theme_ext::resolve_color;

/// A real GPUI status indicator (colored dot + optional label).
pub struct StatusIndicator {
    spec: StatusIndicatorSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for StatusIndicator {
    type Target = StatusIndicatorSpec;
    fn deref(&self) -> &StatusIndicatorSpec { &self.spec }
}

impl StatusIndicator {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: StatusIndicatorSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: StatusIndicatorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn status(mut self, v: StatusTone) -> Self { self.spec.status = v; self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

}

impl IntoElement for StatusIndicator {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let status_color = resolve_color(theme, spec.status_color_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");

        let mut row = div().flex().items_center().gap(px(7.0));

        // Status dot
        row = row.child(
            div()
                .w(px(8.0))
                .h(px(8.0))
                .rounded(px(4.0))
                .bg(status_color)
                .flex_shrink_0(),
        );

        // Label
        if let Some(ref label) = spec.label {
            row = row.child(
                div()
                    .text_sm()
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        row.into_any_element()
    }
}
