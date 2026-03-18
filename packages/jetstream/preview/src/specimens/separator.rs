//! Separator specimen.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::separator::js_separator;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::{RuleTone, SeparatorOrientation, SeparatorSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let primary = resolve_color(theme, "semantic.color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("Horizontal", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(label("Subtle (default)").text_color(primary).text_size(13.0))
                .child(js_separator(&SeparatorSpec::new(), theme))
                .child(label("Default tone").text_color(primary).text_size(13.0))
                .child(js_separator(&SeparatorSpec::new().with_tone(RuleTone::Default), theme))
                .child(label("Below separator").text_color(primary).text_size(13.0))
        ))
        .child(group("Vertical", secondary,
            div().flex_row().gap(12.0).h(60.0).items_center()
                .child(label("Left").text_color(primary).text_size(13.0))
                .child(js_separator(&SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical), theme))
                .child(label("Right").text_color(primary).text_size(13.0))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
