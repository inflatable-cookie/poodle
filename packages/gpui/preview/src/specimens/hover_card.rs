use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, HoverCard};
use poodle_specs::{EyebrowSpec, HoverCardSpec, OverlayPlacement};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");

    // --- Group: Default (top placement) ---
    let top_spec = HoverCardSpec::new().with_open(true);

    // --- Group: Bottom placement ---
    let bottom_spec = HoverCardSpec::new()
        .with_open(true)
        .with_placement(OverlayPlacement::Bottom);

    div().flex().flex_col().gap(px(24.0))
        // Default (top placement)
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (top placement)"), theme))
                .child(
                    div().flex().flex_col().items_start().gap(px(8.0))
                        // Trigger text
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(accent))
                                .child("@clay".to_string())
                        )
                        // Open hover card content
                        .child(
                            HoverCard::from_spec(top_spec, theme)
                                .with_content(
                                    div().flex().flex_col().gap(px(4.0))
                                        .max_w(px(256.0))
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Clay".to_string())
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(color_to_hsla(text_secondary))
                                                .child("Design systems engineer working on Poodle. Loves component architecture and accessibility.".to_string())
                                        )
                                )
                        )
                )
        )
        // Bottom placement
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Bottom placement"), theme))
                .child(
                    div().flex().flex_col().items_start().gap(px(8.0))
                        // Trigger text
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(accent))
                                .child("poodle/svelte-primitives".to_string())
                        )
                        // Open hover card content
                        .child(
                            HoverCard::from_spec(bottom_spec, theme)
                                .with_content(
                                    div().flex().flex_col().gap(px(4.0))
                                        .max_w(px(256.0))
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("svelte-primitives".to_string())
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(color_to_hsla(text_secondary))
                                                .child("Core primitive components for the Poodle design system. 64 components, 94% coverage.".to_string())
                                        )
                                )
                        )
                )
        )
}
