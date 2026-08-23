//! MetricTile — metric tile (Label | Value + Sparkline | Trend).
//!
//! Contract: `docs/contracts/components/metric-tile.md`
//! Ported from: `packages/jetstream/components/src/metric_tile.rs`.
//!
//! The sparkline is a fixed-width strip of value-scaled bars (the Tier-3
//! rendering substitution — no raw polyline primitive); dimensions and colour
//! match the contract.

use poodle_node::{
    ColorValue, CrossAxisAlignment, FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing, Node,
};
use poodle_specs::{MetricTileSpec, MetricTrend};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn metric_tile(spec: &MetricTileSpec, ctx: &RenderContext<'_>) -> Node {
    let fill = ctx.theme().resolve_color(spec.fill_token());
    // Contract §8: border is `0.0625rem solid transparent` (invisible — keeps
    // box geometry stable).
    let border_w = rem_to_px(spec.border_width_rem());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let label_color = ctx.theme().resolve_color(spec.label_color_token());
    let value_color = ctx.theme().resolve_color(spec.value_color_token());

    // Density-resolved spacing (contract §8 density table).
    let density = ctx.resolve_density(spec.density);
    let pad_x = rem_to_px(spec.padding_x_rem(density));
    let pad_y = rem_to_px(spec.padding_y_rem(density));
    let root_gap = rem_to_px(spec.root_gap_rem(density));
    let body_gap = rem_to_px(spec.body_gap_rem(density));

    // Typography (contract §8): label 0.75rem, value 1rem, trend 0.75rem.
    let label_font = rem_to_px(spec.label_font_size_rem());
    let value_font = rem_to_px(spec.value_font_size_rem());
    let trend_font = rem_to_px(spec.trend_font_size_rem());

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_w;
        s.descriptor.border.color = ColorValue(0.0, 0.0, 0.0, 0.0);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
    }

    // Label (code-style metadata key) — code-family (contract §8).
    let mut label = Node::text(&spec.label);
    label.style.descriptor.text_color = Some(label_color);
    label.style.text_size = Some(label_font);
    label.style.font_family = Some(FontFamily::Mono);
    let el = el.child(label);

    // Body: value + optional inline sparkline.
    let mut body = Node::container();
    {
        let s = &mut body.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = body_gap;
    }
    let mut value = Node::text(&spec.value);
    value.style.descriptor.text_color = Some(value_color);
    value.style.text_size = Some(value_font);
    value.style.text_weight = Some(700);
    let mut body = body.child(value);

    // Sparkline — fixed 4rem × 1.5rem, colour text.tertiary (contract §7/§8).
    if spec.has_sparkline() {
        let data = &spec.sparkline_data;
        let min_v = data.iter().copied().fold(f32::INFINITY, f32::min);
        let max_v = data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let range = (max_v - min_v).max(0.0001);

        let chart_w = rem_to_px(spec.sparkline_width_rem());
        let chart_h = rem_to_px(spec.sparkline_height_rem());
        let spark_color = ctx.theme().resolve_color(spec.sparkline_color_token());
        let bar_gap = rem_to_px(0.0625);

        let mut sparkline = Node::container();
        {
            let s = &mut sparkline.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_none = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::End;
            s.descriptor.layout.spacing.gap = bar_gap;
            s.descriptor.layout.width = LayoutSizing::Fixed(chart_w);
            s.descriptor.layout.height = LayoutSizing::Fixed(chart_h);
            s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        }

        let h_rem = spec.sparkline_height_rem();
        for value in data.iter() {
            let norm = ((*value - min_v) / range).clamp(0.0, 1.0);
            let bar_h = rem_to_px((norm * h_rem).max(0.125));
            let mut bar = Node::container();
            {
                let s = &mut bar.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.flex_fill = true;
                s.descriptor.layout.height = LayoutSizing::Fixed(bar_h);
                s.descriptor.background = Some(spark_color);
                let r = rem_to_px(0.0625);
                let c = &mut s.descriptor.corner_radii;
                c.top_left = r;
                c.top_right = r;
                c.bottom_right = r;
                c.bottom_left = r;
            }
            sparkline = sparkline.child(bar);
        }

        body = body.child(sparkline);
    }

    let mut el = el.child(body);

    // Trend row (optional) — its own row below the body (contract §2).
    if let Some(trend) = spec.trend {
        let trend_color = spec
            .trend_color_token()
            .map(|t| ctx.theme().resolve_color(t))
            .unwrap_or(value_color);

        let trend_icon = match trend {
            MetricTrend::Up => "trending-up",
            MetricTrend::Down => "trending-down",
            MetricTrend::Flat => "arrow-right",
        };

        // Contract §8: trend-arrow font 0.875rem, trend gap 0.25rem.
        let icon_size = rem_to_px(spec.trend_arrow_font_size_rem());
        let mut trend_row = Node::container();
        {
            let s = &mut trend_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = rem_to_px(spec.trend_gap_rem());
        }
        let mut arrow = Node::icon(trend_icon, icon_size);
        arrow.style.descriptor.text_color = Some(trend_color);
        let mut trend_row = trend_row.child(arrow);

        if let Some(ref trend_label) = spec.trend_label {
            // Trend text — code-family (contract §8).
            let mut t = Node::text(trend_label);
            t.style.descriptor.text_color = Some(trend_color);
            t.style.text_size = Some(trend_font);
            t.style.font_family = Some(FontFamily::Mono);
            trend_row = trend_row.child(t);
        }

        el = el.child(trend_row);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
