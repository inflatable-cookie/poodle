use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{HoverCardSpec, OverlayPlacement};
use pug_gpui_components::HoverCard;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let accent = theme.resolve_color("semantic.color.accent.base");

    // --- Group: Default (top placement) ---
    // Contract: trigger text "@clay", content with name heading and bio paragraph
    let top_spec = HoverCardSpec::new()
        .with_open(true);

    // --- Group: Bottom placement ---
    // Contract: trigger text "pug/svelte-primitives", placement bottom, content with repo name and stats
    let bottom_spec = HoverCardSpec::new()
        .with_open(true)
        .with_placement(OverlayPlacement::Bottom);

    div().flex().flex_col().gap(px(24.0))
        // Default (top placement)
        .child(section_label("DEFAULT (TOP PLACEMENT)", text_secondary))
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
                                        .child("Design systems engineer working on Pug. Loves component architecture and accessibility.".to_string())
                                )
                        )
                )
        )
        // Bottom placement
        .child(section_label("BOTTOM PLACEMENT", text_secondary))
        .child(
            div().flex().flex_col().items_start().gap(px(8.0))
                // Trigger text
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(accent))
                        .child("pug/svelte-primitives".to_string())
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
                                        .child("Core primitive components for the Pug design system. 64 components, 94% coverage.".to_string())
                                )
                        )
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
