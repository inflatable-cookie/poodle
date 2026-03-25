//! MetricTile — real GPUI component backed by MetricTileSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::MetricTileSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI metric tile component backed by `MetricTileSpec`.
///
/// Renders a compact metadata display tile with a label and value.
pub struct MetricTile {
    spec: MetricTileSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for MetricTile {
    type Target = MetricTileSpec;
    fn deref(&self) -> &MetricTileSpec { &self.spec }
}

impl MetricTile {
    pub fn new(label: impl Into<String>, value: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(MetricTileSpec::new(label, value), theme)
    }

    pub fn from_spec(spec: MetricTileSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = v.into(); self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = v.into(); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

}

impl IntoElement for MetricTile {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let label_color = resolve_color(theme, spec.label_color_token());
        let value_color = resolve_color(theme, spec.value_color_token());
        let padding = resolve_px(theme, spec.padding_token());
        let gap = resolve_px(theme, spec.gap_token());
        let heading_size = resolve_px(theme, "semantic.typography.heading.size");

        div()
            .flex()
            .flex_col()
            .gap(gap)
            .bg(fill)
            .rounded(radius)
            .px(padding)
            .py(padding)
            .child(
                div()
                    .text_size(px(12.0))
                    .line_height(relative(1.3))
                    .text_color(label_color)
                    .child(spec.label.clone()),
            )
            .child(
                div()
                    .text_size(heading_size)
                    .font_weight(FontWeight::BOLD)
                    .line_height(relative(1.2))
                    .text_color(value_color)
                    .child(spec.value.clone()),
            )
            .into_any_element()
    }
}
