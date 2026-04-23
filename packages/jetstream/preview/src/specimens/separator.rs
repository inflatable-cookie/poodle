//! Separator specimen.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::separator::js_separator;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, RuleTone, SeparatorOrientation, SeparatorSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("Horizontal", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(label("Subtle (default)").text_color(primary).text_size(body_font))
                .child(js_separator(&SeparatorSpec::new(), theme))
                .child(label("Default tone").text_color(primary).text_size(body_font))
                .child(js_separator(&SeparatorSpec::new().with_tone(RuleTone::Default), theme))
                .child(label("Below separator").text_color(primary).text_size(body_font))
        ))
        .child(group("Vertical", secondary,
            div().flex_row().gap(12.0).h(60.0).items_center()
                .child(label("Left").text_color(primary).text_size(body_font))
                .child(js_separator(&SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical), theme))
                .child(label("Right").text_color(primary).text_size(body_font))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
