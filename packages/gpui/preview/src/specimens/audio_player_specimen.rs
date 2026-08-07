use crate::node_compat::{AudioPlayer, Eyebrow};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::AudioPlayerSpec;
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let src = "https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3";

    div()
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
        // Sizes — every size variant scales button/icon/time/volume.
        .child(group(
            theme,
            "Sizes",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_size(ControlSize::Xs),
                ))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_size(ControlSize::Sm),
                ))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_size(ControlSize::Md),
                ))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_size(ControlSize::Lg),
                ))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_size(ControlSize::Xl),
                )),
        ))
        // Densities — spacing only; height stays constant.
        .child(group(
            theme,
            "Densities",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_density(ControlDensity::Compact),
                ))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_density(ControlDensity::Default),
                ))
                .child(player(
                    theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(184.0)
                        .with_current_time(60.0)
                        .with_show_speed_control(true)
                        .with_density(ControlDensity::Comfortable),
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
