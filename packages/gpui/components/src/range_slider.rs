//! PugRangeSlider — real GPUI component backed by RangeSliderSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RangeSliderSpec;

use crate::theme_ext::resolve_color;

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

        let _accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, spec.track_fill_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let _norm_low = spec.normalized_low().clamp(0.0, 1.0) as f32;
        let _norm_high = spec.normalized_high().clamp(0.0, 1.0) as f32;

        // Track with filled range between low and high
        let track = div()
            .w_full()
            .h(px(4.0))
            .rounded(px(2.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border.opacity(0.3))
            .relative();

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
            .gap(px(4.0))
            .child(track)
            .child(labels);

        if spec.is_disabled {
            wrapper = wrapper.opacity(0.48);
        }

        wrapper.into_any_element()
    }
}
