use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::PugSurface;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Elevated)
        .with_border(SurfaceBorder::Subtle);

    div().child(
        PugSurface::new(surface_spec, theme)
            .with_content(
                div().flex().flex_col().gap(px(2.0))
                    .child(div().text_xs().text_color(color_to_hsla(accent)).child("fn main() {"))
                    .child(div().text_xs().pl(px(16.0)).child("println!(\"Hello, world!\");"))
                    .child(div().text_xs().text_color(color_to_hsla(accent)).child("}"))
            )
    )
}
