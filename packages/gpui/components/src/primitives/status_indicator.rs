//! StatusIndicator — real GPUI component backed by StatusIndicatorSpec.

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

        // Contract: gap 0.4375rem (7px)
        let mut row = div().flex().items_center().gap(px(7.0));

        // Svelte: dot 0.5625rem (9px) with full circle radius and shadow ring
        row = row.child(
            div()
                .w(px(9.0))
                .h(px(9.0))
                .rounded(px(999.0))
                .bg(status_color)
                .flex_shrink_0()
                // Svelte: 18% opacity shadow ring
                .shadow(vec![gpui::BoxShadow {
                    color: Hsla { a: status_color.a * 0.18, ..status_color },
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(4.0),
                    spread_radius: px(1.0),
                }]),
        );

        // Contract: label font 0.75rem (12px), weight 600, line-height 1.3
        if let Some(ref label) = spec.label {
            row = row.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .line_height(relative(1.3))
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        row.into_any_element()
    }
}
