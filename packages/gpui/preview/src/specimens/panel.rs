use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{TabStripSpec, TabStripItem, SurfaceSpec, SurfaceTone, SurfaceBorder, SeparatorSpec, RuleTone};
use pug_gpui_components::{PugTabStrip, PugSurface, PugSeparator};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let selected = state.specimens.selected("panel-tab");

    let tabs = vec![
        TabStripItem::new("explorer", "Explorer"),
        TabStripItem::new("search", "Search"),
    ];
    let contents = ["Panel content — Explorer view", "Panel content — Search results"];

    let tab_spec = TabStripSpec::new(tabs)
        .with_value(if selected == 0 { "explorer" } else { "search" });

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().child(
        PugSurface::new(surface_spec, theme)
            .with_content(
                div().flex().flex_col()
                    .child(
                        div().flex().items_center().justify_between().px(px(10.0)).h(px(32.0))
                            .child(PugTabStrip::new(tab_spec, theme).with_id("panel-tab"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("×"))
                    )
                    .child(PugSeparator::new(SeparatorSpec::new().with_tone(RuleTone::Default), theme))
                    .child(
                        div().p(px(10.0)).min_h(px(60.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child(contents[selected].to_string()))
                    )
            )
    )
}
