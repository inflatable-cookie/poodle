//! PugColorPicker — real GPUI component backed by ColorPickerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ColorPickerSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI color picker component with swatch grid backed by `ColorPickerSpec`.
pub struct PugColorPicker {
    spec: ColorPickerSpec,
    theme: GpuiThemeProvider,
}

impl PugColorPicker {
    pub fn new(spec: ColorPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugColorPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());

        let current = spec
            .current_value()
            .unwrap_or("#000000")
            .to_string();

        // Color swatch trigger
        let mut trigger = div()
            .h(px(36.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap(px(8.0))
            .text_sm()
            .child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(border)
                    .bg(hsla(0.0, 0.0, 0.0, 1.0)), // placeholder swatch
            )
            .child(
                div()
                    .text_color(text_primary)
                    .child(current.clone()),
            );

        if spec.is_disabled {
            trigger = trigger.opacity(0.48);
        } else {
            trigger = trigger.cursor_pointer();
        }

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        if spec.is_open {
            // Swatch grid overlay
            let mut grid = div()
                .flex()
                .flex_wrap()
                .gap(px(4.0));

            let preset_colors = [
                "#000000", "#ffffff", "#ff0000", "#00ff00",
                "#0000ff", "#ffff00", "#ff00ff", "#00ffff",
            ];

            for _color in &preset_colors {
                grid = grid.child(
                    div()
                        .w(px(24.0))
                        .h(px(24.0))
                        .rounded(px(4.0))
                        .border_1()
                        .border_color(border)
                        .cursor_pointer(),
                );
            }

            let overlay = div()
                .rounded(px(6.0))
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow_md()
                .p(px(12.0))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(grid)
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary)
                        .child(format!("Selected: {}", current)),
                );

            wrapper = wrapper.child(overlay);
        }

        wrapper.into_any_element()
    }
}
