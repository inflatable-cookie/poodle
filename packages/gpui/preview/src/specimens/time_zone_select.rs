use gpui::*;
use poodle_primitives::{ControlDensity, ControlSize, TimeZoneSelectSpec, EyebrowSpec};
use poodle_gpui_components::{TimeZoneSelect, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let is_open = state.specimens.is_on("tz-select-open");

    div().flex().flex_col().gap(px(24.0)).max_w(px(320.0))
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
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let sizes: &[ControlSize] = &[
                        ControlSize::Xs,
                        ControlSize::Sm,
                        ControlSize::Md,
                        ControlSize::Lg,
                        ControlSize::Xl,
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &size in sizes {
                        col = col.child(
                            TimeZoneSelect::from_spec(
                                TimeZoneSelectSpec::new()
                                    .with_value("America/New_York"),
                                theme,
                            )
                            .size(size)
                        );
                    }
                    col
                })
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child({
                    let densities: &[ControlDensity] = &[
                        ControlDensity::Compact,
                        ControlDensity::Default,
                        ControlDensity::Comfortable,
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &density in densities {
                        col = col.child(
                            TimeZoneSelect::from_spec(
                                TimeZoneSelectSpec::new()
                                    .with_value("America/New_York"),
                                theme,
                            )
                            .with_density(density)
                        );
                    }
                    col
                })
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
}
