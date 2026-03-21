use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, SeparatorSpec, RuleTone, ButtonSpec, ButtonVariant};
use pug_gpui_components::{Surface, Separator, Button};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().flex().flex_col().gap(px(8.0))
        .child(
            Surface::from_spec(surface_spec, theme)
                .with_content(
                    div().flex().flex_col()
                        .child(
                            div().flex().gap(px(6.0)).p(px(6.0))
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("B"),
                                        theme,
                                    ).with_id("ed-bold")
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("I"),
                                        theme,
                                    ).with_id("ed-italic")
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("H1"),
                                        theme,
                                    ).with_id("ed-h1")
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Link"),
                                        theme,
                                    ).with_id("ed-link")
                                )
                        )
                        .child(Separator::from_spec(SeparatorSpec::new().with_tone(RuleTone::Default), theme))
                        .child(
                            div().p(px(10.0)).min_h(px(60.0))
                                .child(div().text_sm().child("Rich text editing area..."))
                        )
                )
        )
}
