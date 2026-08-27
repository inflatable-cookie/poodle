use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DurationInput, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DurationInputSpec, EyebrowSpec};

fn duration_total_label(hours: u32, minutes: u32, seconds: u32, show_seconds: bool) -> String {
    if show_seconds {
        format!("Total: {hours}h {minutes}m {seconds}s")
    } else {
        format!("Total: {hours}h {minutes}m")
    }
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let stored = |key: &str, fallback: (u32, u32, u32)| {
        state
            .specimens
            .durations
            .get(key)
            .copied()
            .unwrap_or(fallback)
    };
    macro_rules! live_duration {
        ($builder:expr, $key:literal) => {{
            let queue = std::sync::Arc::clone(&state.node_events);
            $builder.on_change(std::sync::Arc::new(
                move |hours: u32, minutes: u32, seconds: u32, _total: u64| {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetDuration {
                        key: $key.to_string(),
                        hours,
                        minutes,
                        seconds,
                    });
                },
            ))
        }};
    }

    let (full_h, full_m, full_s) = stored("duration-full", (1, 30, 0));
    let (hm_h, hm_m, hm_s) = stored("duration-hm", (0, 45, 0));
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(384.0))
        // --- Hours, minutes, seconds ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Hours, minutes, seconds"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(live_duration!(
                            DurationInput::from_spec(
                                DurationInputSpec::new().with_segments(full_h, full_m, full_s),
                                theme,
                            )
                            .with_id("duration-full"),
                            "duration-full"
                        ))
                        .child(
                            div()
                                .text_xs()
                                .text_color(color_to_hsla(text_secondary))
                                .child(duration_total_label(full_h, full_m, full_s, true)),
                        ),
                ),
        )
        // --- Hours and minutes only ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Hours and minutes only"),
                    theme,
                ))
                .child(live_duration!(
                    DurationInput::from_spec(
                        DurationInputSpec::new()
                            .with_segments(hm_h, hm_m, hm_s)
                            .with_show_seconds(false),
                        theme,
                    )
                    .with_id("duration-hm"),
                    "duration-hm"
                )),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(
                    DurationInput::from_spec(
                        DurationInputSpec::new()
                            .with_segments(2, 15, 30)
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("duration-disabled"),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "duration-input",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                DurationInput::from_spec(DurationInputSpec::new().with_segments(1, 0, 0), theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                DurationInput::from_spec(DurationInputSpec::new().with_segments(1, 0, 0), theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
