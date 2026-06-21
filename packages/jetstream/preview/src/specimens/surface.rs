//! Surface specimen — themed containers across tone, border, and padding.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::surface::js_surface;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ControlSize, PaddingScale, SurfaceBorder, SurfaceRole, SurfaceSpec, SurfaceTone,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let primary = resolve_color(theme, "color.text.primary");

    let body = move |s: &str| label(s).text_color(primary).text_size(body_font);

    div()
        .flex_col()
        .gap(24.0)
        // --- Tone: Panel (default) ---
        .child(group(
            "Panel tone (default)",
            secondary,
            js_surface(
                &SurfaceSpec::new()
                    .with_tone(SurfaceTone::Panel)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md),
                theme,
                vec![body("Panel surface with subtle border — the standard container.")],
            ),
        ))
        // --- Tone: Canvas ---
        .child(group(
            "Canvas tone",
            secondary,
            js_surface(
                &SurfaceSpec::new()
                    .with_tone(SurfaceTone::Canvas)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md),
                theme,
                vec![body("Canvas surface sits behind panels as a background layer.")],
            ),
        ))
        // --- Tone: Elevated ---
        .child(group(
            "Elevated tone",
            secondary,
            js_surface(
                &SurfaceSpec::new()
                    .with_tone(SurfaceTone::Elevated)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md)
                    .with_elevation(true),
                theme,
                vec![body("Elevated surface with shadow for overlays and cards.")],
            ),
        ))
        // --- Border emphasis: subtle / default / none ---
        .child(group(
            "Border emphasis",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .self_stretch()
                .child(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Md),
                    theme,
                    vec![body("Subtle border (default) — mixed border color.")],
                ))
                .child(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::Default)
                        .with_padding(PaddingScale::Md),
                    theme,
                    vec![body("Default border — full border-default color.")],
                ))
                .child(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::None)
                        .with_padding(PaddingScale::Md),
                    theme,
                    vec![body("No border — just padding and background fill.")],
                )),
        ))
        // --- Padding scale: none / sm / md / lg ---
        .child(group(
            "Padding scale",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .self_stretch()
                .child(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::None),
                    theme,
                    vec![body("padding=none")],
                ))
                .child(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Sm),
                    theme,
                    vec![body("padding=sm")],
                ))
                .child(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Md),
                    theme,
                    vec![body("padding=md (default)")],
                ))
                .child(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Lg),
                    theme,
                    vec![body("padding=lg")],
                )),
        ))
        // --- Semantic role (region with accessible label) ---
        .child(group(
            "Region role (asRole=region)",
            secondary,
            js_surface(
                &SurfaceSpec::new()
                    .with_role(SurfaceRole::Region)
                    .with_label("Account settings")
                    .with_padding(PaddingScale::Md),
                theme,
                vec![body("Surface as a semantic region with an accessible label.")],
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
