use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::TimeZoneSelectSpec;
use pug_gpui_components::TimeZoneSelect;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let is_open = state.specimens.is_on("tz-select-open");

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            TimeZoneSelect::from_spec(
                TimeZoneSelectSpec::new()
                    .with_placeholder("Select time zone…")
                    .with_open(is_open),
                theme,
            )
            .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                this.state.specimens.toggle("tz-select-open");
                cx.notify();
            }))
        )
        // --- With default value ---
        .child(section_label("WITH DEFAULT VALUE", text_secondary))
        .child(
            TimeZoneSelect::from_spec(
                TimeZoneSelectSpec::new()
                    .with_value("America/New_York"),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            TimeZoneSelect::from_spec(
                TimeZoneSelectSpec::new()
                    .with_value("Europe/London")
                    .with_disabled(true),
                theme,
            )
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
