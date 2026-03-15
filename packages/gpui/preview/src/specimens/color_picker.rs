use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant};
use pug_gpui_components::PugBadge;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let accent = theme.resolve_color("semantic.color.accent.base");
    let border = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let selected = state.specimens.selected("color-picker");

    let colors = ["#ef4444", "#f59e0b", "#22c55e", "#3b82f6", "#8b5cf6"];
    let mut swatches = div().flex().gap(px(4.0));

    for (i, color) in colors.iter().enumerate() {
        let cv = pug_gpui::GpuiThemeProvider::new().resolve_color_value(color);
        let is_selected = selected == i;

        let mut swatch = div()
            .id(SharedString::from(format!("color-{}", i)))
            .w(px(24.0)).h(px(24.0)).rounded(px(4.0))
            .bg(color_to_hsla(cv))
            .cursor_pointer()
            .hover(|s| s.opacity(0.8));

        swatch = if is_selected {
            swatch.border_2().border_color(color_to_hsla(accent))
        } else {
            swatch.border_1().border_color(color_to_hsla(border))
        };

        swatch = swatch.on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
            this.state.specimens.select("color-picker", i);
            cx.notify();
        }));

        swatches = swatches.child(swatch);
    }

    let mut selected_badge = BadgeSpec::new().with_variant(BadgeVariant::Accent);
    selected_badge.content = Some(colors[selected].to_string());

    div().flex().flex_col().gap(px(6.0))
        .child(swatches)
        .child(PugBadge::new(selected_badge, theme))
}
