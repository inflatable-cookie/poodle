use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant};
use pug_gpui_components::PugBadge;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let mut accent_badge = BadgeSpec::new().with_variant(BadgeVariant::Accent);
    accent_badge.content = Some("Default".to_string());

    let mut muted_badge = BadgeSpec::new().with_variant(BadgeVariant::Muted);
    muted_badge.content = Some("Muted".to_string());

    div().flex().gap(px(6.0)).flex_wrap()
        .child(PugBadge::new(accent_badge, theme))
        .child(PugBadge::new(muted_badge, theme))
}
