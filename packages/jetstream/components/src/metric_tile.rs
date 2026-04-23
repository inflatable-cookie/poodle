//! MetricTile — Jetstream metric tile backed by MetricTileSpec.
//!
//! Contract: `docs/contracts/components/metric-tile.md`
//! Reference: `packages/svelte/components/src/MetricTile.svelte`
//!
//! Anatomy: Root → Label | Body[Value + Sparkline] | Trend

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{MetricTileSpec, MetricTrend};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_metric_tile(spec: &MetricTileSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, spec.radius_token());
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());
    let pad_x = resolve_px(theme, spec.padding_x_token());
    let pad_y = resolve_px(theme, spec.padding_y_token());
    let gap = resolve_px(theme, spec.gap_token());

    // Contract §8: label 0.75rem, value 1rem.
    let label_font = rem_to_px(spec.label_font_size_rem());
    let value_font = rem_to_px(spec.value_font_size_rem());

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_col().gap(gap);

    // Label
    el = el.child(
        ui_element::label(&spec.label)
            .text_color(label_color)
            .text_size(label_font),
    );

    // Body: value (+ sparkline placeholder)
    el = el.child(
        ui_element::label(&spec.value)
            .text_color(value_color)
            .text_size(value_font)
            .text_weight(700),
    );

    // Trend row (optional)
    if let Some(trend) = spec.trend {
        let trend_color = spec
            .trend_color_token()
            .map(|t| resolve_color(theme, t))
            .unwrap_or(value_color);

        let trend_icon = match trend {
            MetricTrend::Up => "trending-up",
            MetricTrend::Down => "trending-down",
            MetricTrend::Flat => "arrow-right",
        };

        let icon_size = rem_to_px(0.875); // contract: 0.875rem
        let mut trend_row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(rem_to_px(0.25));

        trend_row = trend_row.child(
            ui_element::icon(trend_icon)
                .w(icon_size).h(icon_size)
                .text_color(trend_color),
        );

        if let Some(ref trend_label) = spec.trend_label {
            trend_row = trend_row.child(
                ui_element::label(trend_label)
                    .text_color(trend_color)
                    .text_size(label_font),
            );
        }

        el = el.child(trend_row);
    }

    el
}
