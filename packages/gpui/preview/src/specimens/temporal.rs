use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, BadgeSpec, BadgeVariant};
use pug_gpui_components::{PugSurface, PugBadge};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut time_ago_badge = BadgeSpec::new().with_variant(BadgeVariant::Muted);
    time_ago_badge.content = Some("2 hours ago".to_string());

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Elevated)
        .with_border(SurfaceBorder::Default);

    div().flex().flex_col().gap(px(6.0))
        .child(PugBadge::new(time_ago_badge, theme))
        .child(
            PugSurface::new(surface_spec, theme)
                .with_content(
                    div().flex().items_center().gap(px(4.0))
                        .child(div().text_sm().child("01"))
                        .child(div().text_sm().text_color(color_to_hsla(text_secondary)).child(":"))
                        .child(div().text_sm().child("30"))
                        .child(div().text_sm().text_color(color_to_hsla(text_secondary)).child(":"))
                        .child(div().text_sm().child("00"))
                )
        )
}
