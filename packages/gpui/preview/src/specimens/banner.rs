use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant, StatusIndicatorSpec, StatusTone};
use pug_gpui_components::{PugBadge, PugStatusIndicator};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("semantic.color.accent.base");
    let warning = theme.resolve_color("semantic.color.status.warning");

    let mut info_badge = BadgeSpec::new().with_variant(BadgeVariant::Accent);
    info_badge.content = Some("Info".to_string());

    let mut warn_badge = BadgeSpec::new().with_variant(BadgeVariant::Muted);
    warn_badge.content = Some("Warning".to_string());

    let mut info_status = StatusIndicatorSpec::new().with_status(StatusTone::Neutral);
    info_status.label = Some("Info banner message".to_string());

    let mut warn_status = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
    warn_status.label = Some("Warning banner message".to_string());

    div().flex().flex_col().gap(px(6.0))
        .child(
            div().p(px(8.0)).rounded(px(4.0))
                .bg(color_to_hsla(accent).opacity(0.1))
                .border_1().border_color(color_to_hsla(accent).opacity(0.3))
                .flex().items_center().gap(px(8.0))
                .child(PugBadge::new(info_badge, theme))
                .child(PugStatusIndicator::new(info_status, theme))
        )
        .child(
            div().p(px(8.0)).rounded(px(4.0))
                .bg(color_to_hsla(warning).opacity(0.1))
                .border_1().border_color(color_to_hsla(warning).opacity(0.3))
                .flex().items_center().gap(px(8.0))
                .child(PugBadge::new(warn_badge, theme))
                .child(PugStatusIndicator::new(warn_status, theme))
        )
}
