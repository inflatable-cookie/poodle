use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::ZonedDateTimePickerSpec;
use pug_gpui_components::ZonedDateTimePicker;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let is_open = state.specimens.is_on("zoned-dtp-open");

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            ZonedDateTimePicker::from_spec(
                ZonedDateTimePickerSpec::new()
                    .with_value("2026-03-23T14:30:00")
                    .with_time_zone("America/New_York")
                    .with_open(is_open),
                theme,
            )
            .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                this.state.specimens.toggle("zoned-dtp-open");
                cx.notify();
            }))
        )
        // --- With different time zone ---
        .child(section_label("EUROPE/LONDON", text_secondary))
        .child(
            ZonedDateTimePicker::from_spec(
                ZonedDateTimePickerSpec::new()
                    .with_value("2026-03-23T19:30:00")
                    .with_time_zone("Europe/London"),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            ZonedDateTimePicker::from_spec(
                ZonedDateTimePickerSpec::new()
                    .with_value("2026-01-01T00:00:00")
                    .with_time_zone("UTC")
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
