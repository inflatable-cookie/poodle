//! DurationInput — real GPUI component backed by DurationInputSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{DurationInputSpec, ValidationState};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI duration input (HH:MM:SS) component backed by `DurationInputSpec`.
pub struct DurationInput {
    spec: DurationInputSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for DurationInput {
    type Target = DurationInputSpec;
    fn deref(&self) -> &DurationInputSpec { &self.spec }
}

impl DurationInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DurationInputSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: DurationInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }
    pub fn show_seconds(mut self, v: bool) -> Self { self.spec.show_seconds = v; self }

}

impl IntoElement for DurationInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let segment_radius = resolve_radius(theme, "semantic.radius.surface");
        let control_padding_y = resolve_px(theme, "semantic.space.control.y");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let hover_bg = color_mix(surface_bg, elevated, 0.84);

        let display = spec.value.as_deref().unwrap_or(if spec.show_seconds {
            "00:00:00"
        } else {
            "00:00"
        });

        let mut segments = div()
            .flex()
            .items_center()
            .gap(px(2.0));

        // Split display into segments and render with separators
        let parts: Vec<&str> = display.split(':').collect();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                segments = segments.child(
                    div().text_sm().text_color(text_secondary).child(":"),
                );
            }
            segments = segments.child(
                div()
                    .px(control_padding_y)
                    .py(px(2.0))
                    .rounded(segment_radius)
                    .text_sm()
                    .text_color(text_primary)
                    .child(part.to_string()),
            );
        }

        let mut wrapper = div()
            .h(control_height)
            .px(control_padding_x)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .child(segments);

        if spec.is_disabled {
            wrapper = wrapper.opacity(disabled_opacity);
        } else {
            wrapper = wrapper
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg));
        }

        wrapper.into_any_element()
    }
}
