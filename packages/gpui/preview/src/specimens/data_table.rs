use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{StatusIndicatorSpec, StatusTone};
use pug_gpui_components::PugStatusIndicator;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let border = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let accent = theme.resolve_color("semantic.color.accent.base");
    let selected = state.specimens.selected("table-row");

    let mut table = div().flex().flex_col();
    table = table.child(
        div().flex().px(px(8.0)).py(px(6.0)).border_b_1().border_color(color_to_hsla(border))
            .child(div().flex_1().text_xs().text_color(color_to_hsla(text_secondary)).child("Name"))
            .child(div().w(px(80.0)).text_xs().text_color(color_to_hsla(text_secondary)).child("Status"))
            .child(div().w(px(60.0)).text_xs().text_color(color_to_hsla(text_secondary)).child("Actions"))
    );

    let rows = [
        ("Project Alpha", "Active", StatusTone::Success),
        ("Project Beta", "Draft", StatusTone::Warning),
        ("Project Gamma", "Archived", StatusTone::Neutral),
    ];

    for (i, (name, status_label, tone)) in rows.iter().enumerate() {
        let is_selected = selected == i;
        let mut status_spec = StatusIndicatorSpec::new().with_status(*tone);
        status_spec.label = Some(status_label.to_string());

        let row = div()
            .id(SharedString::from(format!("table-row-{}", i)))
            .flex().px(px(8.0)).py(px(8.0))
            .border_b_1().border_color(color_to_hsla(border).opacity(0.5))
            .cursor_pointer()
            .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)))
            .when(is_selected, |el| el.bg(color_to_hsla(accent).opacity(0.08)))
            .child(div().flex_1().text_sm().child(name.to_string()))
            .child(div().w(px(80.0)).child(PugStatusIndicator::new(status_spec, theme)))
            .child(div().w(px(60.0)).text_xs().text_color(color_to_hsla(text_secondary)).child("Edit"))
            .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.select("table-row", i);
                cx.notify();
            }));
        table = table.child(row);
    }
    table
}
