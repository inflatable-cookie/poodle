//! MetricTile — real GPUI component backed by MetricTileSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{MetricTileSpec, MetricTrend};
use poodle_components::{IconSize, IconSpec};

use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI metric tile component backed by `MetricTileSpec`.
///
/// Renders a compact metadata display tile with a label, headline value,
/// optional trend arrow + label, and an optional sparkline chart.
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
    pub fn trend(mut self, v: MetricTrend) -> Self { self.spec.trend = Some(v); self }
    pub fn trend_label(mut self, v: impl Into<String>) -> Self { self.spec.trend_label = Some(v.into()); self }
    pub fn sparkline(mut self, v: Vec<f32>) -> Self { self.spec.sparkline_data = v; self }
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
        let heading_size = resolve_px(theme, "typography.heading.size");

        let mut tile = div()
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
            );

        // Value row — includes the trend arrow/label alongside the headline value.
        let mut value_row = div()
            .flex()
            .items_baseline()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(heading_size)
                    .font_weight(FontWeight::BOLD)
                    .line_height(relative(1.2))
                    .text_color(value_color)
                    .child(spec.value.clone()),
            );

        if let Some(trend) = spec.trend {
            let trend_color = resolve_color(theme, trend.color_token());
            let trend_bg = Hsla { a: trend_color.a * 0.12, ..trend_color };

            let mut trend_chip = div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .px(px(6.0))
                .py(px(2.0))
                .rounded(px(10.0))
                .bg(trend_bg)
                .text_size(px(11.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(trend_color);

            trend_chip = trend_chip.child(
                Icon::from_spec(
                    IconSpec::new(trend.icon_name()).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(trend_color),
            );

            if let Some(ref trend_label) = spec.trend_label {
                trend_chip = trend_chip.child(trend_label.clone());
            }

            value_row = value_row.child(trend_chip);
        }

        tile = tile.child(value_row);

        // Sparkline — a simple polyline chart rendered via nested divs.
        // GPUI doesn't expose raw SVG; instead we approximate the line
        // with a fixed-width strip of bars that scale to each data
        // point's value. This gives a visual cue of direction without
        // requiring custom painting.
        if spec.has_sparkline() {
            let data = &spec.sparkline_data;
            let min_v = data.iter().copied().fold(f32::INFINITY, f32::min);
            let max_v = data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let range = (max_v - min_v).max(0.0001);
            let chart_height = px(24.0);
            let bar_width = px(3.0);
            let bar_gap = px(1.0);

            let bar_color = spec
                .trend
                .map(|t| resolve_color(theme, t.color_token()))
                .unwrap_or(value_color);

            let mut sparkline = div()
                .flex()
                .items_end()
                .gap(bar_gap)
                .h(chart_height)
                .mt(px(4.0));

            for value in data.iter() {
                let norm = ((*value - min_v) / range).clamp(0.0, 1.0);
                // Minimum bar height so flat values stay visible.
                let h = (norm * 24.0).max(2.0);
                sparkline = sparkline.child(
                    div()
                        .w(bar_width)
                        .h(px(h))
                        .bg(bar_color.opacity(0.7))
                        .rounded(px(1.0)),
                );
            }

            tile = tile.child(sparkline);
        }

        tile.into_any_element()
    }
}
