//! PugNumberEntry — real GPUI component backed by NumberEntrySpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::NumberEntrySpec;

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

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

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let control_padding_x = resolve_px(theme, "semantic.space.control.x");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let hover_bg = color_mix(surface_bg, elevated, 0.84);
        let display_value = format!("{}", spec.clamped_value());

        // Minus button
        let minus_btn = div()
            .px(control_padding_x)
            .text_sm()
            .text_color(text_secondary)
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg))
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
            .px(control_padding_x)
            .text_sm()
            .text_color(text_secondary)
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg))
            .child("+");

        let mut wrapper = div()
            .h(control_height)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .child(minus_btn)
            .child(value_display)
            .child(plus_btn);

        if spec.is_disabled {
            wrapper = wrapper.opacity(disabled_opacity);
        }

        wrapper.into_any_element()
    }
}
