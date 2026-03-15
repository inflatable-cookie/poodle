use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, ButtonSpec, ButtonVariant};
use pug_gpui_components::{PugSurface, PugButton};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Default);

    div().child(
        PugSurface::new(surface_spec, theme)
            .with_content(
                div().flex().flex_col().items_center().gap(px(6.0)).py(px(8.0))
                    .child(div().text_xl().text_color(color_to_hsla(text_secondary)).child("↑"))
                    .child(div().text_sm().text_color(color_to_hsla(text_secondary)).child("Drop files here or"))
                    .child(
                        PugButton::new(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Browse"),
                            theme,
                        )
                        .with_id("file-browse")
                    )
            )
    )
}
