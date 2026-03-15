use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant};
use pug_gpui_components::PugBadge;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut tag_a = BadgeSpec::new().with_variant(BadgeVariant::Accent);
    tag_a.content = Some("Tag A".to_string());

    let mut tag_b = BadgeSpec::new().with_variant(BadgeVariant::Accent);
    tag_b.content = Some("Tag B".to_string());

    div().flex().flex_col().gap(px(8.0))
        .child(
            div().flex().gap(px(6.0))
                .child(PugBadge::new(tag_a, theme))
                .child(PugBadge::new(tag_b, theme))
        )
        .child(
            div().text_xs().text_color(color_to_hsla(text_secondary)).child("EYEBROW LABEL")
        )
}
