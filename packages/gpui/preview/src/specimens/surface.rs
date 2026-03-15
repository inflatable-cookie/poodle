use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::PugSurface;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().child(
        PugSurface::new(spec, theme)
            .with_content(
                div().text_sm().text_color(color_to_hsla(text_secondary)).child("Content surface area")
            )
    )
}
