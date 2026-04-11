//! Surface specimen — themed containers at different tones and borders.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::surface::js_surface;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{SurfaceBorder, SurfaceSpec, SurfaceTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("Panel tone", secondary,
            js_surface(&SurfaceSpec::new().with_tone(SurfaceTone::Panel), theme, vec![
                label("Panel surface").text_color(text_primary).text_size(13.0),
            ])
        ))
        .child(group("Elevated tone", secondary,
            js_surface(&SurfaceSpec::new().with_tone(SurfaceTone::Elevated), theme, vec![
                label("Elevated surface").text_color(text_primary).text_size(13.0),
            ])
        ))
        .child(group("With border", secondary,
            js_surface(&SurfaceSpec::new().with_border(SurfaceBorder::Default), theme, vec![
                label("Bordered surface").text_color(text_primary).text_size(13.0),
            ])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
