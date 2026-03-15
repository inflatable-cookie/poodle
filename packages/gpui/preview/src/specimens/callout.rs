use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::PugSurface;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::None);

    div().child(
        div()
            .border_l_2()
            .border_color(color_to_hsla(accent))
            .child(
                PugSurface::new(surface_spec, theme)
                    .with_content(
                        div().flex().flex_col().gap(px(2.0))
                            .child(div().text_sm().text_color(color_to_hsla(accent)).child("Note"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("This is a callout with important information."))
                    )
            )
    )
}
