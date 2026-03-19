use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant};
use pug_gpui_components::PugBadge;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Accent ---
        .child(section_label("ACCENT", text_secondary))
        .child(
            div().flex().gap(px(8.0))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Accent);
                    s.content = Some("3".to_string());
                    s
                }, theme))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Accent);
                    s.content = Some("12".to_string());
                    s
                }, theme))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Accent);
                    s.content = Some("99+".to_string());
                    s
                }, theme))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Accent);
                    s.content = Some("New".to_string());
                    s
                }, theme))
        )
        // --- Muted ---
        .child(section_label("MUTED", text_secondary))
        .child(
            div().flex().gap(px(8.0))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Muted);
                    s.content = Some("3".to_string());
                    s
                }, theme))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Muted);
                    s.content = Some("12".to_string());
                    s
                }, theme))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Muted);
                    s.content = Some("99+".to_string());
                    s
                }, theme))
                .child(PugBadge::new({
                    let mut s = BadgeSpec::new().with_variant(BadgeVariant::Muted);
                    s.content = Some("Draft".to_string());
                    s
                }, theme))
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
