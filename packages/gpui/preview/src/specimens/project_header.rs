use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant};
use pug_gpui_components::PugBadge;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut branch_badge = BadgeSpec::new().with_variant(BadgeVariant::Muted);
    branch_badge.content = Some("main".to_string());

    div().flex().items_center().gap(px(10.0))
        .child(
            div().w(px(28.0)).h(px(28.0)).rounded(px(6.0))
                .bg(color_to_hsla(accent).opacity(0.2))
                .flex().items_center().justify_center()
                .text_sm().text_color(color_to_hsla(accent))
                .child("P")
        )
        .child(
            div().flex().flex_col().gap(px(2.0))
                .child(div().text_sm().child("Project Name"))
                .child(
                    div().flex().items_center().gap(px(6.0))
                        .child(PugBadge::new(branch_badge, theme))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("3 contributors"))
                )
        )
}
