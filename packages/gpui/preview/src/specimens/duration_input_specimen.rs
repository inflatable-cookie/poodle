use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DurationInput, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DurationInputSpec, EyebrowSpec, ValidationState};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    // Segment edits are host state, like every other value on this page: the
    // component reports hours/minutes/seconds after carry and the host stores
    // the formatted result.
    let stored = |key: &str, fallback: &str| {
        state
            .specimens
            .text
            .get(key)
            .cloned()
            .unwrap_or_else(|| fallback.to_string())
    };
    macro_rules! live_duration {
        ($builder:expr, $key:literal, $seconds:expr) => {{
            let queue = std::sync::Arc::clone(&state.node_events);
            let show_seconds = $seconds;
            $builder.on_change(std::sync::Arc::new(
                move |h: u32, m: u32, sec: u32, _total: u32| {
                    let value = if show_seconds {
                        format!("{h:02}:{m:02}:{sec:02}")
                    } else {
                        format!("{h:02}:{m:02}")
                    };
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: $key.to_string(),
                        value,
                    });
                },
            ))
        }};
    }

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(384.0))
        // --- Full (H:M:S) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Full (H:M:S)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(live_duration!(
                            DurationInput::from_spec(
                                DurationInputSpec::new()
                                    .with_value(stored("duration-full", "01:30:00"))
                                    .with_show_seconds(true),
                                theme,
                            )
                            .with_id("duration-full"),
                            "duration-full",
                            true
                        ))
                        .child(
                            div()
                                .text_xs()
                                .text_color(color_to_hsla(text_secondary))
                                .child("Value: 01:30:00 (5400 seconds)"),
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
                            .with_value(stored("duration-hm", "02:45"))
                            .with_show_seconds(false),
                        theme,
                    )
                    .with_id("duration-hm"),
                    "duration-hm",
                    false
                )),
        )
        // --- Empty / zero (default zero display) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty / zero"),
                    theme,
                ))
                .child(
                    DurationInput::from_spec(
                        DurationInputSpec::new().with_show_seconds(true),
                        theme,
                    )
                    .with_id("duration-empty"),
                ),
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
                            .with_value("00:30:00")
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("duration-disabled"),
                ),
        )
        // --- Invalid (out-of-bounds danger border) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Invalid (out of bounds)"),
                    theme,
                ))
                .child(
                    DurationInput::from_spec(
                        DurationInputSpec::new()
                            .with_value("00:00:05")
                            .with_show_seconds(true)
                            .with_min_total_seconds(60)
                            .with_validation_state(ValidationState::Invalid),
                        theme,
                    )
                    .with_id("duration-invalid"),
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
                DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
