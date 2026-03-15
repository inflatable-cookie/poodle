//! PugDurationInput — real GPUI component backed by DurationInputSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::DurationInputSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI duration input (HH:MM:SS) component backed by `DurationInputSpec`.
pub struct PugDurationInput {
    spec: DurationInputSpec,
    theme: GpuiThemeProvider,
}

impl PugDurationInput {
    pub fn new(spec: DurationInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugDurationInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

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
                    .px(px(4.0))
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .text_sm()
                    .text_color(text_primary)
                    .child(part.to_string()),
            );
        }

        let mut wrapper = div()
            .h(px(36.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .child(segments);

        if spec.is_disabled {
            wrapper = wrapper.opacity(0.48);
        }

        wrapper.into_any_element()
    }
}
