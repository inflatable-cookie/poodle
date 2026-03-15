use gpui::*;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::PugSurface;
use pug_gpui::GpuiThemeProvider;
use pug_adapter::ThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("semantic.color.border.default");

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().child(
        PugSurface::new(surface_spec, theme)
            .with_content(
                div().flex().flex_col().gap(px(6.0))
                    .child(div().h(px(12.0)).w(px(200.0)).rounded(px(4.0)).bg(color_to_hsla(border).opacity(0.5)))
                    .child(div().h(px(12.0)).w(px(160.0)).rounded(px(4.0)).bg(color_to_hsla(border).opacity(0.3)))
                    .child(div().h(px(12.0)).w(px(180.0)).rounded(px(4.0)).bg(color_to_hsla(border).opacity(0.2)))
            )
    )
}
