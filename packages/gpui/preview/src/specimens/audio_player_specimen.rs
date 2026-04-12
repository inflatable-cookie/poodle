use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{AudioPlayer, Eyebrow};
use poodle_specs::AudioPlayerSpec;
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let src = "https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3";

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic"),
                    theme,
                ))
                .child(AudioPlayer::from_spec(
                    AudioPlayerSpec::new(src).with_duration(2.0),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Playback states"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(AudioPlayer::from_spec(
                            AudioPlayerSpec::new(src)
                                .with_duration(24.0)
                                .with_current_time(8.5)
                                .with_playing(true)
                                .with_show_speed_control(true),
                            theme,
                        ))
                        .child(AudioPlayer::from_spec(
                            AudioPlayerSpec::new(src)
                                .with_duration(24.0)
                                .with_current_time(18.0)
                                .with_volume(0.35)
                                .with_muted(true),
                            theme,
                        )),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(AudioPlayer::from_spec(
                            AudioPlayerSpec::new(src)
                                .with_duration(24.0)
                                .with_current_time(6.0)
                                .with_size(ControlSize::Sm)
                                .with_density(ControlDensity::Compact),
                            theme,
                        ))
                        .child(AudioPlayer::from_spec(
                            AudioPlayerSpec::new(src)
                                .with_duration(24.0)
                                .with_current_time(12.0)
                                .with_playing(true)
                                .with_show_speed_control(true)
                                .with_size_role(SemanticControlSizeRole::Prominent)
                                .with_density(ControlDensity::Compact),
                            theme,
                        )),
                ),
        )
}
