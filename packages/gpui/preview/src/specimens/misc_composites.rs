use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, TextInputSpec};
use pug_gpui_components::{PugSurface, PugTextInput};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().flex().flex_col().gap(px(8.0))
        .child(
            PugSurface::new(surface_spec, theme)
                .with_content(
                    div().flex().flex_col().gap(px(2.0))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("[12:34:01] Request received"))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("[12:34:02] Processing..."))
                        .child(div().text_xs().child("[12:34:03] Complete"))
                )
        )
        .child(
            div().flex().items_center().gap(px(4.0))
                .child(div().text_sm().child("my-project"))
                .child(div().text_sm().text_color(color_to_hsla(text_secondary)).child("/"))
                .child(PugTextInput::new(
                    TextInputSpec::new().with_value("my-project-slug").with_disabled(true),
                    theme,
                ))
        )
}
