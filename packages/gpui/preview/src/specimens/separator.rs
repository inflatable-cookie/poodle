use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{SeparatorSpec, SeparatorOrientation, EyebrowSpec};
use poodle_gpui_components::{Separator, Eyebrow};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Horizontal (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Horizontal (default)"), theme))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Content above".to_string()),
                )
                .child(Separator::from_spec(SeparatorSpec::new(), theme))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Content below".to_string()),
                )
        )
        // --- Vertical ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Vertical"), theme))
                .child(
                    div().flex().items_center().gap(px(12.0)).h(px(32.0))
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Left".to_string()),
                        )
                        .child(Separator::from_spec(
                            SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
                            theme,
                        ))
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Center".to_string()),
                        )
                        .child(Separator::from_spec(
                            SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
                            theme,
                        ))
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Right".to_string()),
                        )
                )
        )
        // --- Decorative ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Decorative"), theme))
                .child(Separator::from_spec(
                    SeparatorSpec::new().with_decorative(true),
                    theme,
                ))
        )
}
