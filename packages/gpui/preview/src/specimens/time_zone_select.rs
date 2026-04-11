use gpui::*;
use poodle_components::{TimeZoneSelectSpec, EyebrowSpec};
use poodle_gpui_components::{TimeZoneSelect, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let is_open = state.specimens.is_on("tz-select-open");

    let examples = div().flex().flex_col().gap(px(24.0)).max_w(px(320.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    TimeZoneSelect::from_spec(
                        TimeZoneSelectSpec::new()
                            .with_placeholder("Select time zone\u{2026}")
                            .with_open(is_open),
                        theme,
                    )
                    .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                        this.state.specimens.toggle("tz-select-open");
                        cx.notify();
                    }))
                )
        )
        // --- With pre-selected zone ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With pre-selected zone"), theme))
                .child(
                    TimeZoneSelect::from_spec(
                        TimeZoneSelectSpec::new()
                            .with_value("America/New_York"),
                        theme,
                    )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    TimeZoneSelect::from_spec(
                        TimeZoneSelectSpec::new()
                            .with_value("Europe/London")
                            .with_disabled(true),
                        theme,
                    )
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "time-zone-select",
        examples,
        |size, theme: &GpuiThemeProvider| {
            TimeZoneSelect::from_spec(
                TimeZoneSelectSpec::new().with_value("America/New_York"),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            TimeZoneSelect::from_spec(
                TimeZoneSelectSpec::new().with_value("America/New_York"),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
