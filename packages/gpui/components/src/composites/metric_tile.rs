//! MetricTile — real GPUI component backed by MetricTileSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::ControlDensity;
use poodle_specs::{IconSize, IconSpec};
use poodle_specs::{MetricTileSpec, MetricTrend};

use crate::presentation::rem_to_px;
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI metric tile component backed by `MetricTileSpec`.
///
/// Renders a compact metadata display tile with a label, headline value,
/// optional trend arrow + label (on its own row below the body), and an
/// optional sparkline chart inline beside the value.
pub struct MetricTile {
    spec: MetricTileSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for MetricTile {
    type Target = MetricTileSpec;
    fn deref(&self) -> &MetricTileSpec {
        &self.spec
    }
}

impl MetricTile {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self::from_spec(MetricTileSpec::new(label, value), theme)
    }

    pub fn from_spec(spec: MetricTileSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = v.into();
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = v.into();
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn trend(mut self, v: MetricTrend) -> Self {
        self.spec.trend = Some(v);
        self
    }
    pub fn trend_label(mut self, v: impl Into<String>) -> Self {
        self.spec.trend_label = Some(v.into());
        self
    }
    pub fn sparkline(mut self, v: Vec<f32>) -> Self {
        self.spec.sparkline_data = v;
        self
    }
    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
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

        // Density-resolved spacing (contract §8 density table).
        let root_gap = px(rem_to_px(spec.root_gap_rem()));
        let pad_x = px(rem_to_px(spec.padding_x_rem()));
        let pad_y = px(rem_to_px(spec.padding_y_rem()));
        let body_gap = px(rem_to_px(spec.body_gap_rem()));

        // Typography (contract §8) — all from rem, never heading size.
        let label_size = px(rem_to_px(spec.label_font_size_rem()));
        let value_size = px(rem_to_px(spec.value_font_size_rem()));
        let trend_size = px(rem_to_px(spec.trend_font_size_rem()));

        // Contract §8: 0.0625rem transparent border (keeps box geometry stable
        // across the light-theme override without a visible edge).
        let border_w = px(rem_to_px(spec.border_width_rem()));

        let mut tile = div()
            .flex()
            .flex_col()
            .gap(root_gap)
            .bg(fill)
            .rounded(radius)
            .border(border_w)
            .border_color(gpui::transparent_black())
            .px(pad_x)
            .py(pad_y)
            .child(
                div()
                    .text_size(label_size)
                    .text_color(label_color)
                    .child(spec.label.clone()),
            );

        // Body row — value + optional inline sparkline.
        let mut body = div()
            .flex()
            .items_center()
            .gap(body_gap)
            .child(
                div()
                    .text_size(value_size)
                    .font_weight(FontWeight::BOLD)
                    .text_color(value_color)
                    .child(spec.value.clone()),
            );

        // Sparkline — fixed 4rem × 1.5rem, text-tertiary color. GPUI exposes no
        // raw SVG/polyline, so the line is approximated by a fixed-width strip of
        // value-scaled bars (Tier-3 rendering substitution); dimensions and color
        // match the contract. (note)
        if spec.has_sparkline() {
            let data = &spec.sparkline_data;
            let min_v = data.iter().copied().fold(f32::INFINITY, f32::min);
            let max_v = data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let range = (max_v - min_v).max(0.0001);

            let chart_w = px(rem_to_px(spec.sparkline_width_rem()));
            let chart_h = px(rem_to_px(spec.sparkline_height_rem()));
            let spark_color = resolve_color(theme, spec.sparkline_color_token());
            let bar_gap = px(rem_to_px(0.0625));

            let mut sparkline = div()
                .flex()
                .flex_none()
                .items_end()
                .gap(bar_gap)
                .w(chart_w)
                .h(chart_h)
                .overflow_hidden();

            let h_rem = spec.sparkline_height_rem();
            for value in data.iter() {
                let norm = ((*value - min_v) / range).clamp(0.0, 1.0);
                let bar_h = px(rem_to_px((norm * h_rem).max(0.125)));
                sparkline = sparkline.child(
                    div()
                        .flex_grow()
                        .h(bar_h)
                        .bg(spark_color)
                        .rounded(px(rem_to_px(0.0625))),
                );
            }

            body = body.child(sparkline);
        }

        tile = tile.child(body);

        // Trend — its own row below the body (contract §2 + Svelte place
        // `.state-tile__trend` after the body, not beside the value). Plain
        // inline icon+label row in the trend tone color; no chip background.
        if let Some(trend) = spec.trend {
            let trend_color = resolve_color(theme, trend.color_token());
            let trend_gap = px(rem_to_px(spec.trend_gap_rem()));

            let mut trend_row = div()
                .flex()
                .items_center()
                .gap(trend_gap)
                .text_size(trend_size)
                .text_color(trend_color)
                .child(
                    Icon::from_spec(
                        IconSpec::new(trend.icon_name()).with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(trend_color),
                );

            if let Some(ref trend_label) = spec.trend_label {
                trend_row = trend_row.child(trend_label.clone());
            }

            tile = tile.child(trend_row);
        }

        tile.into_any_element()
    }
}
