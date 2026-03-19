use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::EyebrowSpec;
use pug_gpui_components::PugEyebrow;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    div().flex().flex_col().gap(px(24.0))
        // --- Section label ---
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(section_label("SECTION LABEL", text_secondary))
                .child(PugEyebrow::new(
                    EyebrowSpec::new().with_content("Section label"),
                    theme,
                ))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Eyebrow renders small uppercase text used for categorizing content above headings.".to_string())
                )
        )
        // --- Primitive category ---
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(section_label("PRIMITIVE", text_secondary))
                .child(PugEyebrow::new(
                    EyebrowSpec::new().with_content("Primitive"),
                    theme,
                ))
                .child(
                    div().text_xl().font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child("Button".to_string())
                )
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Primary interactive control for triggering actions.".to_string())
                )
        )
        // --- Composite category ---
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(section_label("COMPOSITE", text_secondary))
                .child(PugEyebrow::new(
                    EyebrowSpec::new().with_content("Composite"),
                    theme,
                ))
                .child(
                    div().text_xl().font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child("DataTable".to_string())
                )
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Feature-rich table with sorting, selection, and pagination.".to_string())
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
