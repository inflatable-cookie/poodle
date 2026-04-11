use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, PaddingScale, SurfaceBorder, SurfaceSpec, SurfaceTone};
use poodle_gpui_components::{Eyebrow, Surface};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Panel tone (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Panel tone (default)"), theme))
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_tone(SurfaceTone::Panel)
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::Md),
                        theme,
                    )
                    .with_content(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child("Panel surface with subtle border \u{2014} the standard container.".to_string()),
                    )
                )
        )
        // --- Canvas tone ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Canvas tone"), theme))
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_tone(SurfaceTone::Canvas)
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::Md),
                        theme,
                    )
                    .with_content(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child("Canvas surface sits behind panels as a background layer.".to_string()),
                    )
                )
        )
        // --- Elevated tone ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Elevated tone"), theme))
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_tone(SurfaceTone::Elevated)
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::Md)
                            .with_elevation(true),
                        theme,
                    )
                    .with_content(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child("Elevated surface with shadow for overlays and cards.".to_string()),
                    )
                )
        )
        // --- No border ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("No border"), theme))
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_tone(SurfaceTone::Panel)
                            .with_border(SurfaceBorder::None)
                            .with_padding(PaddingScale::Md),
                        theme,
                    )
                    .with_content(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child("Surface with no border \u{2014} just padding and background.".to_string()),
                    )
                )
        )
}
