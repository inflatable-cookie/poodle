use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, EyebrowSpec};
use poodle_gpui_components::{Surface, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let divider_color = theme.resolve_color("color.border.subtle");

    div().flex().flex_col().gap(px(24.0))
        // --- Default variant ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default variant"), theme))
                .child(
                    div().flex().gap(px(16.0)).flex_wrap()
                        .child(
                            div().w(px(280.0)).child(
                                Surface::from_spec(
                                    SurfaceSpec::new()
                                        .with_tone(SurfaceTone::Panel)
                                        .with_border(SurfaceBorder::Subtle),
                                    theme,
                                )
                                .with_content(
                                    div().flex().flex_col().gap(px(12.0))
                                        .child(
                                            div().text_base().font_weight(FontWeight::SEMIBOLD)
                                                .text_color(color_to_hsla(text_primary))
                                                .child("Project Alpha"),
                                        )
                                        .child(
                                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                                .child("A design system component library for building consistent interfaces."),
                                        )
                                        .child(
                                            div()
                                                .pt(px(8.0))
                                                .border_color(color_to_hsla(divider_color).opacity(0.52))
                                                .border_t_1()
                                                .child(
                                                    div().text_xs().text_color(color_to_hsla(text_secondary))
                                                        .child("Updated 2 days ago"),
                                                ),
                                        ),
                                ),
                            ),
                        )
                        .child(
                            div().w(px(280.0)).child(
                                Surface::from_spec(
                                    SurfaceSpec::new()
                                        .with_tone(SurfaceTone::Panel)
                                        .with_border(SurfaceBorder::Subtle),
                                    theme,
                                )
                                .with_content(
                                    div().flex().flex_col().gap(px(12.0))
                                        .child(
                                            div().text_base().font_weight(FontWeight::SEMIBOLD)
                                                .text_color(color_to_hsla(text_primary))
                                                .child("Monthly report"),
                                        )
                                        .child(
                                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                                .child("48 components shipped across 3 packages this month."),
                                        ),
                                ),
                            ),
                        ),
                )
        )
        // --- Outlined variant ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Outlined variant"), theme))
                .child(
                    div().w(px(280.0)).child(
                        Surface::from_spec(
                            SurfaceSpec::new()
                                .with_tone(SurfaceTone::Panel)
                                .with_border(SurfaceBorder::Default),
                            theme,
                        )
                        .with_content(
                            div().flex().flex_col().gap(px(12.0))
                                .child(
                                    div().text_base().font_weight(FontWeight::SEMIBOLD)
                                        .text_color(color_to_hsla(text_primary))
                                        .child("Outlined card"),
                                )
                                .child(
                                    div().text_sm().text_color(color_to_hsla(text_secondary))
                                        .child("This card uses a subtle border instead of elevation."),
                                ),
                        ),
                    ),
                )
        )
        // --- Elevated variant ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Elevated variant"), theme))
                .child(
                    div().w(px(280.0)).child(
                        Surface::from_spec(
                            SurfaceSpec::new()
                                .with_tone(SurfaceTone::Elevated)
                                .with_border(SurfaceBorder::Subtle)
                                .with_elevation(true),
                            theme,
                        )
                        .with_content(
                            div().flex().flex_col().gap(px(12.0))
                                .child(
                                    div().text_base().font_weight(FontWeight::SEMIBOLD)
                                        .text_color(color_to_hsla(text_primary))
                                        .child("Elevated card"),
                                )
                                .child(
                                    div().text_sm().text_color(color_to_hsla(text_secondary))
                                        .child("This card uses a drop shadow for visual prominence."),
                                ),
                        ),
                    ),
                )
        )
        // --- Interactive ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Interactive"), theme))
                .child(
                    div().w(px(280.0))
                        .id("interactive-card")
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.9))
                        .child(
                            Surface::from_spec(
                                SurfaceSpec::new()
                                    .with_tone(SurfaceTone::Panel)
                                    .with_border(SurfaceBorder::Subtle),
                                theme,
                            )
                            .with_content(
                                div().flex().flex_col().gap(px(12.0))
                                    .child(
                                        div().text_base().font_weight(FontWeight::SEMIBOLD)
                                            .text_color(color_to_hsla(text_primary))
                                            .child("Interactive card"),
                                    )
                                    .child(
                                        div().text_sm().text_color(color_to_hsla(text_secondary))
                                            .child("Hover to see the interactive state. Cursor changes to pointer."),
                                    ),
                            ),
                        ),
                )
        )
}
