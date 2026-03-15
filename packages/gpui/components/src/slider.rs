//! PugSlider — real GPUI component backed by SliderSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SliderSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI slider component backed by `SliderSpec`.
pub struct PugSlider {
    spec: SliderSpec,
    theme: GpuiThemeProvider,
}

impl PugSlider {
    pub fn new(spec: SliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugSlider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");

        let progress = spec.normalized_progress().clamp(0.0, 1.0) as f32;

        // Track
        let track = div()
            .w_full()
            .h(px(4.0))
            .rounded(px(2.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border.opacity(0.3))
            .relative()
            // Filled portion
            .child(
                div()
                    .h_full()
                    .rounded(px(2.0))
                    .bg(accent)
                    .w(relative(progress)),
            );

        // Thumb (positioned at progress point)
        let thumb = div()
            .w(px(16.0))
            .h(px(16.0))
            .rounded(px(8.0))
            .bg(accent)
            .border_1()
            .border_color(accent)
            .shadow_sm();

        let mut wrapper = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(track)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_xs()
                    .text_color(resolve_color(theme, "semantic.color.text.secondary"))
                    .child(format!("{:.0}", spec.min))
                    .child(format!("{:.0}", spec.clamped_value()))
                    .child(format!("{:.0}", spec.max)),
            );

        if spec.is_disabled {
            wrapper = wrapper.opacity(0.48);
        }

        wrapper.into_any_element()
    }
}
