//! PugRangeSlider — real GPUI component backed by RangeSliderSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RangeSliderSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// A real GPUI dual-thumb range slider component backed by `RangeSliderSpec`.
pub struct PugRangeSlider {
    spec: RangeSliderSpec,
    theme: GpuiThemeProvider,
}

impl PugRangeSlider {
    pub fn new(spec: RangeSliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugRangeSlider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, spec.track_fill_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");

        // Track dimensions
        let track_height_f = theme.resolve_space("semantic.space.stack.sm");
        let track_height = px(track_height_f);
        let track_radius = px(track_height_f / 2.0);

        // Thumb dimensions
        let thumb_f = theme.resolve_space("semantic.size.icon.md");
        let thumb_size = px(thumb_f);
        let thumb_radius = px(thumb_f / 2.0);

        let norm_low = spec.normalized_low().clamp(0.0, 1.0) as f32;
        let norm_high = spec.normalized_high().clamp(0.0, 1.0) as f32;

        // Track with filled range between low and high thumbs
        let track = div()
            .w_full()
            .h(track_height)
            .rounded(track_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border.opacity(0.3))
            .relative()
            // Filled range segment
            .child(
                div()
                    .absolute()
                    .top_0()
                    .h_full()
                    .rounded(track_radius)
                    .bg(accent)
                    .left(relative(norm_low))
                    .w(relative(norm_high - norm_low)),
            )
            // Low thumb
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_height_f) / 2.0))
                    .left(relative(norm_low))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(accent)
                    .border_1()
                    .border_color(accent),
            )
            // High thumb
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_height_f) / 2.0))
                    .left(relative(norm_high))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(accent)
                    .border_1()
                    .border_color(accent),
            );

        // Labels showing low/high values
        let labels = div()
            .flex()
            .items_center()
            .justify_between()
            .text_xs()
            .text_color(text_secondary)
            .child(format!("{:.0}", spec.clamped_low()))
            .child(format!("{:.0}", spec.clamped_high()));

        let mut wrapper = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(stack_gap)
            .child(track)
            .child(labels);

        if spec.is_disabled {
            wrapper = wrapper.opacity(disabled_opacity);
        }

        wrapper.into_any_element()
    }
}
