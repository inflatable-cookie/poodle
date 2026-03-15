//! PugStatusIndicator — real GPUI component backed by StatusIndicatorSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::StatusIndicatorSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI status indicator (colored dot + optional label).
pub struct PugStatusIndicator {
    spec: StatusIndicatorSpec,
    theme: GpuiThemeProvider,
}

impl PugStatusIndicator {
    pub fn new(spec: StatusIndicatorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugStatusIndicator {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let status_color = resolve_color(theme, spec.status_color_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");

        let mut row = div().flex().items_center().gap(px(6.0));

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
