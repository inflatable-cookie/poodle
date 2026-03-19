//! PugMetricTile — real GPUI component backed by MetricTileSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::MetricTileSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI metric tile component backed by `MetricTileSpec`.
///
/// Renders a compact metadata display tile with a label and value.
pub struct PugMetricTile {
    spec: MetricTileSpec,
    theme: GpuiThemeProvider,
}

impl PugMetricTile {
    pub fn new(spec: MetricTileSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugMetricTile {
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
                    .text_xs()
                    .text_color(label_color)
                    .child(spec.label.clone()),
            )
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::BOLD)
                    .text_color(value_color)
                    .child(spec.value.clone()),
            )
            .into_any_element()
    }
}
