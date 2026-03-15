//! PugNumberEntry — real GPUI component backed by NumberEntrySpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::NumberEntrySpec;

use crate::theme_ext::resolve_color;

/// A real GPUI numeric input component with +/- stepper buttons backed by `NumberEntrySpec`.
pub struct PugNumberEntry {
    spec: NumberEntrySpec,
    theme: GpuiThemeProvider,
}

impl PugNumberEntry {
    pub fn new(spec: NumberEntrySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugNumberEntry {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let display_value = format!("{}", spec.clamped_value());

        // Minus button
        let minus_btn = div()
            .px(px(8.0))
            .text_sm()
            .text_color(text_secondary)
            .cursor_pointer()
            .child("\u{2212}");

        // Value display
        let value_display = div()
            .flex_1()
            .text_center()
            .text_sm()
            .text_color(text_primary)
            .child(display_value);

        // Plus button
        let plus_btn = div()
            .px(px(8.0))
            .text_sm()
            .text_color(text_secondary)
            .cursor_pointer()
            .child("+");

        let mut wrapper = div()
            .h(px(36.0))
            .rounded(px(6.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .child(minus_btn)
            .child(value_display)
            .child(plus_btn);

        if spec.is_disabled {
            wrapper = wrapper.opacity(0.48);
        }

        wrapper.into_any_element()
    }
}
