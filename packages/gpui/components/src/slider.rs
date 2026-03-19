//! PugSlider — real GPUI component backed by SliderSpec.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SliderSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

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

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");

        // Contract: track height = 0.25rem (4px), thumb = 1rem (16px)
        let track_height = resolve_px(theme, "semantic.space.stack.sm");
        let track_f = theme.resolve_space("semantic.space.stack.sm");
        let track_radius = px(track_f / 2.0);
        let thumb_f = theme.resolve_space("semantic.size.icon.md");
        let thumb_size = px(thumb_f);
        let thumb_radius = px(thumb_f / 2.0);

        let progress = spec.normalized_progress().clamp(0.0, 1.0) as f32;

        // Track with filled portion
        let track = div()
            .w_full()
            .h(track_height)
            .rounded(track_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border.opacity(0.3))
            .relative()
            .child(
                div()
                    .h_full()
                    .rounded(track_radius)
                    .bg(accent)
                    .w(relative(progress)),
            );

        // Value labels
        let labels = div()
            .flex()
            .items_center()
            .justify_between()
            .text_xs()
            .text_color(text_secondary)
            .child(format!("{:.0}", spec.min))
            .child(format!("{:.0}", spec.clamped_value()))
            .child(format!("{:.0}", spec.max));

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
