//! PugColorPicker — real GPUI component backed by ColorPickerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ColorPickerSpec;

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Parse a hex color string (e.g. "#ff0000") into an Hsla color.
fn parse_hex_color(hex: &str) -> Hsla {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
    let rgba = gpui::Rgba { r, g, b, a: 1.0 };
    rgba.into()
}

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

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let swatch_radius = resolve_radius(theme, "semantic.radius.surface");
        let swatch_size = resolve_px(theme, "semantic.size.icon.lg");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let hover_bg = color_mix(surface_bg, elevated, 0.84);

        let current = spec
            .current_value()
            .unwrap_or("#000000")
            .to_string();

        // Parse current color for the preview swatch
        let current_color = parse_hex_color(&current);

        // Color swatch trigger
        let mut trigger = div()
            .h(control_height)
            .px(control_padding_x)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap(inline_gap)
            .text_sm()
            .child(
                div()
                    .w(swatch_size)
                    .h(swatch_size)
                    .rounded(swatch_radius)
                    .border_1()
                    .border_color(border)
                    .bg(current_color),
            )
            .child(
                div()
                    .text_color(text_primary)
                    .child(current.clone()),
            );

        if spec.is_disabled {
            trigger = trigger.opacity(disabled_opacity);
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg));
        }

        let mut wrapper = div().flex().flex_col().gap(stack_gap).child(trigger);

        if spec.is_open {
            // Swatch grid overlay
            let mut grid = div()
                .flex()
                .flex_wrap()
                .gap(stack_gap);

            let preset_colors = [
                "#000000", "#ffffff", "#ff0000", "#00ff00",
                "#0000ff", "#ffff00", "#ff00ff", "#00ffff",
            ];

            for color_hex in &preset_colors {
                let swatch_color = parse_hex_color(color_hex);
                grid = grid.child(
                    div()
                        .w(swatch_size)
                        .h(swatch_size)
                        .rounded(swatch_radius)
                        .border_1()
                        .border_color(border)
                        .bg(swatch_color)
                        .cursor_pointer(),
                );
            }

            let overlay = div()
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow_md()
                .p(control_padding_x)
                .flex()
                .flex_col()
                .gap(inline_gap)
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
