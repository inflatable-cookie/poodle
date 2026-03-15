use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::PugSurface;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let detail_surface = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    let list_surface = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().flex().gap(px(12.0))
        .child(
            div().flex_1().child(
                PugSurface::new(detail_surface, theme)
                    .with_content(
                        div().flex().flex_col().gap(px(6.0))
                            .child(div().text_sm().child("Detail View"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Key: Value"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Status: Active"))
                    )
            )
        )
        .child(
            div().flex_1().child(
                PugSurface::new(list_surface, theme)
                    .with_content(
                        div().flex().flex_col().gap(px(4.0))
                            .child(div().text_sm().child("List Shell"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("• Item one"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("• Item two"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("• Item three"))
                    )
            )
        )
}
