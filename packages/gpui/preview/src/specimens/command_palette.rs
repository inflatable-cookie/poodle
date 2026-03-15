use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{MenuSpec, MenuEntry, SurfaceSpec, SurfaceTone, SurfaceBorder, SeparatorSpec, RuleTone};
use pug_gpui_components::{PugMenu, PugSurface, PugSeparator};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state.specimens.selected("cmd-palette");

    let commands = vec![
        MenuEntry::new("0", "Open File").with_shortcut_label("⌘P"),
        MenuEntry::new("1", "New Window").with_shortcut_label("⇧⌘N"),
        MenuEntry::new("2", "Toggle Sidebar").with_shortcut_label("⌘B"),
    ];

    let spec = MenuSpec::new(commands).with_open(true);

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().w(px(320.0)).child(
        PugSurface::new(surface_spec, theme)
            .with_content(
                div().flex().flex_col()
                    .child(
                        div().px(px(12.0)).py(px(8.0))
                            .child(div().text_sm().text_color(
                                color_to_hsla(theme.resolve_color("semantic.color.text.secondary"))
                            ).child("Type a command..."))
                    )
                    .child(PugSeparator::new(SeparatorSpec::new().with_tone(RuleTone::Default), theme))
                    .child(
                        PugMenu::new(spec, theme)
                            .with_id("cmd-palette")
                            .with_selected(format!("{}", selected))
                    )
            )
    )
}
