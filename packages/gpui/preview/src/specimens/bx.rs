use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{BoxSpec, EyebrowSpec, Overflow, PaddingScale};
use poodle_gpui_components::{Box, Eyebrow};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border = theme.resolve_color("color.border.default");

    let demo_outline = |child: AnyElement| {
        div()
            .border_1()
            .border_color(color_to_hsla(border))
            .rounded(px(4.0))
            .child(child)
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Default (no padding) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (no padding)"), theme))
                .child(demo_outline(
                    Box::from_spec(BoxSpec::new(), theme)
                        .with_child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Content inside a Box with no padding.".to_string())
                        )
                        .into_any_element(),
                ))
        )
        // --- With padding ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With padding"), theme))
                .child(demo_outline(
                    Box::from_spec(BoxSpec::new().with_padding(PaddingScale::Lg), theme)
                        .with_child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Content inside a Box with large padding.".to_string())
                        )
                        .into_any_element(),
                ))
        )
        // --- Fixed dimensions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Fixed dimensions"), theme))
                .child(demo_outline(
                    div().w(px(192.0)).h(px(96.0)).child(
                        Box::from_spec(BoxSpec::new().with_padding(PaddingScale::Md), theme)
                            .with_child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child("Fixed 12\u{00d7}6rem box.".to_string())
                            )
                    ).into_any_element(),
                ))
        )
        // --- Overflow hidden ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Overflow hidden"), theme))
                .child(demo_outline(
                    div().w(px(160.0)).h(px(48.0)).overflow_hidden().child(
                        Box::from_spec(
                            BoxSpec::new()
                                .with_padding(PaddingScale::Sm)
                                .with_overflow(Overflow::Hidden),
                            theme,
                        )
                        .with_child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("This text is too long and will be clipped by the overflow hidden setting on the box container.".to_string())
                        )
                    ).into_any_element(),
                ))
        )
}
