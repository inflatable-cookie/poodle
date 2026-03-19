use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SeparatorSpec, SeparatorOrientation, RuleTone};
use pug_gpui_components::PugSeparator;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Horizontal (default) ---
        .child(section_label("HORIZONTAL (DEFAULT)", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Content above".to_string()),
                )
                .child(PugSeparator::new(SeparatorSpec::new(), theme))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Content below".to_string()),
                ),
        )
        // --- Vertical ---
        .child(section_label("VERTICAL", text_secondary))
        .child(
            div().flex().items_center().gap(px(12.0)).h(px(32.0))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Left".to_string()),
                )
                .child(PugSeparator::new(
                    SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
                    theme,
                ))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Center".to_string()),
                )
                .child(PugSeparator::new(
                    SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
                    theme,
                ))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Right".to_string()),
                ),
        )
        // --- Decorative ---
        .child(section_label("DECORATIVE", text_secondary))
        .child(PugSeparator::new(
            SeparatorSpec::new().with_decorative(true),
            theme,
        ))
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
