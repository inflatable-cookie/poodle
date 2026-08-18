use crate::app_state::AppState;
use crate::node_compat::{AudioPlayer, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::AudioPlayerSpec;
use poodle_specs::{EyebrowSpec, SemanticControlSizeRole};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let src = "https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3";
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // Transport states: idle/paused vs playing, plus mid-progress seek fill.
        .child(group(
            theme,
            "Transport",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                // Idle / paused at start — play icon, 0:00.
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(0.0),
                ))
                // Playing, mid-progress — pause icon + seek fill proportional to current_time.
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(72.0)
                        .with_playing(true),
                )),
        ))
        // Volume: muted vs reduced volume (volume slider fill is proportional).
        .child(group(
            theme,
            "Volume",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                // Muted — muted icon, volume reads 0.
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(96.0)
                        .with_muted(true),
                ))
                // Reduced volume — partial volume fill.
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(96.0)
                        .with_volume(0.4),
                )),
        ))
        // Speed control: speed-active, distinct selected rates.
        .child(group(
            theme,
            "Speed control",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(48.0)
                        .with_show_speed_control(true)
                        .with_rate(1.0),
                ))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(48.0)
                        .with_playing(true)
                        .with_show_speed_control(true)
                        .with_rate(1.5),
                )),
        ))
        // Semantic size role — inherited scale via role rather than explicit size.
        .child(group(
            theme,
            "Semantic role",
            player(
                theme,
                AudioPlayerSpec::new(src)
                    .with_duration(184.0)
                    .with_current_time(60.0)
                    .with_playing(true)
                    .with_show_speed_control(true)
                    .with_size_role(SemanticControlSizeRole::Prominent),
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "audio-player",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                player(
                    theme,
                    AudioPlayerSpec::new("/media/sample.mp3")
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_size(size),
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                player(
                    theme,
                    AudioPlayerSpec::new("/media/sample.mp3")
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_density(density),
                )
                .into_any_element()
            }),
    )
}

fn player(theme: &GpuiThemeProvider, spec: AudioPlayerSpec) -> Div {
    div()
        .w(px(420.0))
        .child(AudioPlayer::from_spec(spec, theme))
}

fn group(theme: &GpuiThemeProvider, title: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}
