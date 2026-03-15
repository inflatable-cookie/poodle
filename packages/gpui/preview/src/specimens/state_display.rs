use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, StatusIndicatorSpec, StatusTone};
use pug_gpui_components::{PugSurface, PugStatusIndicator};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut active_status = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    active_status.label = Some("Active".to_string());

    let mut pending_status = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
    pending_status.label = Some("Pending".to_string());

    let panel_surface = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().flex().flex_col().gap(px(8.0))
        .child(
            div().flex().gap(px(6.0))
                .child(
                    div().flex_1().child(
                        PugSurface::new(panel_surface.clone(), theme)
                            .with_content(
                                div().flex().flex_col().gap(px(2.0))
                                    .child(PugStatusIndicator::new(active_status, theme))
                                    .child(div().text_lg().child("12"))
                            )
                    )
                )
                .child(
                    div().flex_1().child(
                        PugSurface::new(panel_surface, theme)
                            .with_content(
                                div().flex().flex_col().gap(px(2.0))
                                    .child(PugStatusIndicator::new(pending_status, theme))
                                    .child(div().text_lg().child("5"))
                            )
                    )
                )
        )
        .child(
            div().p(px(16.0)).rounded(px(6.0)).border_1().border_color(
                color_to_hsla(theme.resolve_color("semantic.color.border.default"))
            ).flex().flex_col().items_center().gap(px(4.0))
                .child(div().text_sm().text_color(color_to_hsla(text_secondary)).child("No items found"))
                .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Try adjusting your filters"))
        )
}
