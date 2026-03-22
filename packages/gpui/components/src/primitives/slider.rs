//! Slider — real GPUI component backed by SliderSpec.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Orientation, SliderSpec};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

static SLIDER_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// A real GPUI slider component backed by `SliderSpec`.
pub struct Slider {
    spec: SliderSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Slider {
    type Target = SliderSpec;
    fn deref(&self) -> &SliderSpec { &self.spec }
}

impl Slider {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SliderSpec::default(), theme: theme.clone() }
    }

    pub fn from_spec(spec: SliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self { self.spec.value = v; self }
    pub fn min(mut self, v: f64) -> Self { self.spec.min = v; self }
    pub fn max(mut self, v: f64) -> Self { self.spec.max = v; self }
    pub fn step(mut self, v: f64) -> Self { self.spec.step = v; self }
    pub fn orientation(mut self, v: Orientation) -> Self { self.spec.orientation = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn value_text(mut self, v: impl Into<String>) -> Self { self.spec.value_text = Some(v.into()); self }

}

impl IntoElement for Slider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
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

        // Track with filled portion and thumb
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
            )
            // Thumb
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_f) / 2.0))
                    .left(relative(progress))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(accent)
                    .border_1()
                    .border_color(accent),
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

        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let slider_id = SharedString::from(format!(
            "pug-slider-{}",
            SLIDER_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));

        let mut wrapper = div()
            .id(slider_id)
            .w_full()
            .flex()
            .flex_col()
            .gap(stack_gap)
            .child(track)
            .child(labels);

        wrapper = wrapper.focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            wrapper = wrapper
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        wrapper.into_any_element()
    }
}
