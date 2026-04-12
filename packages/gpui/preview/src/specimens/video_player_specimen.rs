use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, VideoPlayer};
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    AspectRatio, ControlDensity, ControlSize, SemanticControlSizeRole, VideoPlayerSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let src = "https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4";
    let poster = "https://interactive-examples.mdn.mozilla.net/media/examples/friday.mp4";

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
                .child(div().max_w(px(480.0)).child(VideoPlayer::from_spec(
                    VideoPlayerSpec::new(src).with_duration(6.0),
                    theme,
                ))),
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
                        .child(
                            div().max_w(px(480.0)).child(VideoPlayer::from_spec(
                                VideoPlayerSpec::new(src)
                                    .with_duration(42.0)
                                    .with_current_time(13.5)
                                    .with_playing(true)
                                    .with_volume(0.75),
                                theme,
                            )),
                        )
                        .child(
                            div().max_w(px(480.0)).child(VideoPlayer::from_spec(
                                VideoPlayerSpec::new(src)
                                    .with_duration(42.0)
                                    .with_current_time(38.0)
                                    .with_fullscreen(true)
                                    .with_poster(poster),
                                theme,
                            )),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Presentation variants"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(
                            div().max_w(px(400.0)).child(VideoPlayer::from_spec(
                                VideoPlayerSpec::new(src)
                                    .with_aspect_ratio(AspectRatio::Landscape)
                                    .with_duration(6.0)
                                    .with_size(ControlSize::Sm)
                                    .with_density(ControlDensity::Compact),
                                theme,
                            )),
                        )
                        .child(
                            div().max_w(px(520.0)).child(VideoPlayer::from_spec(
                                VideoPlayerSpec::new(src)
                                    .with_duration(6.0)
                                    .with_size_role(SemanticControlSizeRole::Prominent),
                                theme,
                            )),
                        ),
                ),
        )
}
