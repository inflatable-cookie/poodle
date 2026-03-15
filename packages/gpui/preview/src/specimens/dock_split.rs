use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, SeparatorSpec, SeparatorOrientation, RuleTone};
use pug_gpui_components::{PugSurface, PugSeparator};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let dock_surface = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().flex().h(px(100.0))
        .child(
            div().w(px(100.0)).h_full().child(
                PugSurface::new(dock_surface, theme)
                    .with_content(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Dock"))
            )
        )
        .child(
            div().flex_1().flex()
                .child(
                    div().flex_1().p(px(8.0))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Split A"))
                )
                .child(PugSeparator::new(SeparatorSpec::new().with_tone(RuleTone::Default).with_orientation(SeparatorOrientation::Vertical), theme))
                .child(
                    div().flex_1().p(px(8.0))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Split B"))
                )
        )
}
