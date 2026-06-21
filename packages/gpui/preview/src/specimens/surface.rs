use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Surface};
use poodle_specs::{
    PaddingScale, SurfaceBorder, SurfaceRole, SurfaceSpec, SurfaceTone,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    let body = |s: &str| {
        div()
            .text_sm()
            .text_color(color_to_hsla(text_secondary))
            .child(s.to_string())
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Tone: Panel (default) ---
        .child(group(
            theme,
            "Panel tone (default)",
            Surface::from_spec(
                SurfaceSpec::new()
                    .with_tone(SurfaceTone::Panel)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md),
                theme,
            )
            .with_content(body(
                "Panel surface with subtle border \u{2014} the standard container.",
            )),
        ))
        // --- Tone: Canvas ---
        .child(group(
            theme,
            "Canvas tone",
            Surface::from_spec(
                SurfaceSpec::new()
                    .with_tone(SurfaceTone::Canvas)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md),
                theme,
            )
            .with_content(body(
                "Canvas surface sits behind panels as a background layer.",
            )),
        ))
        // --- Tone: Elevated ---
        .child(group(
            theme,
            "Elevated tone",
            Surface::from_spec(
                SurfaceSpec::new()
                    .with_tone(SurfaceTone::Elevated)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md)
                    .with_elevation(true),
                theme,
            )
            .with_content(body(
                "Elevated surface with shadow for overlays and cards.",
            )),
        ))
        // --- Border: subtle (default) / default / none ---
        .child(group(
            theme,
            "Border emphasis",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::Md),
                        theme,
                    )
                    .with_content(body("Subtle border (default) \u{2014} mixed border color.")),
                )
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_border(SurfaceBorder::Default)
                            .with_padding(PaddingScale::Md),
                        theme,
                    )
                    .with_content(body("Default border \u{2014} full border-default color.")),
                )
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_border(SurfaceBorder::None)
                            .with_padding(PaddingScale::Md),
                        theme,
                    )
                    .with_content(body(
                        "No border \u{2014} just padding and background fill.",
                    )),
                ),
        ))
        // --- Padding scale: none / sm / md / lg ---
        .child(group(
            theme,
            "Padding scale",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::None),
                        theme,
                    )
                    .with_content(body("padding=none")),
                )
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::Sm),
                        theme,
                    )
                    .with_content(body("padding=sm")),
                )
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::Md),
                        theme,
                    )
                    .with_content(body("padding=md (default)")),
                )
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_border(SurfaceBorder::Subtle)
                            .with_padding(PaddingScale::Lg),
                        theme,
                    )
                    .with_content(body("padding=lg")),
                ),
        ))
        // --- Semantic role (region with accessible label) ---
        .child(group(
            theme,
            "Region role (asRole=region)",
            Surface::from_spec(
                SurfaceSpec::new()
                    .with_role(SurfaceRole::Region)
                    .with_label("Account settings")
                    .with_padding(PaddingScale::Md),
                theme,
            )
            .with_content(body(
                "Surface as a semantic region with an accessible label.",
            )),
        ))
}

fn group(theme: &GpuiThemeProvider, title: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            poodle_specs::EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}
