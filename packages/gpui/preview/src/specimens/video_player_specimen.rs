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
        // Initial: paused at start → big play button overlay, controls visible.
        .child(group(
            theme,
            "Paused at start",
            player(
                theme,
                480.0,
                VideoPlayerSpec::new(src)
                    .with_poster(poster)
                    .with_duration(64.0)
                    .with_current_time(0.0),
            ),
        ))
        // Playing, mid-progress → no big play, controls bar with seek fill.
        .child(group(
            theme,
            "Playing",
            player(
                theme,
                480.0,
                VideoPlayerSpec::new(src)
                    .with_duration(64.0)
                    .with_current_time(28.0)
                    .with_playing(true)
                    .with_volume(0.75),
            ),
        ))
        // Muted — volume 0 drives the muted (volume-x) glyph.
        .child(group(
            theme,
            "Muted",
            player(
                theme,
                480.0,
                VideoPlayerSpec::new(src)
                    .with_duration(64.0)
                    .with_current_time(40.0)
                    .with_playing(true)
                    .with_volume(0.0),
            ),
        ))
        // Captions track present (GPUI VideoPlayer::with_captions builder).
        .child(group(
            theme,
            "With captions",
            div().max_w(px(480.0)).child(
                VideoPlayer::from_spec(
                    VideoPlayerSpec::new(src)
                        .with_duration(64.0)
                        .with_current_time(40.0)
                        .with_playing(true),
                    theme,
                )
                .with_captions(true),
            ),
        ))
        // Fullscreen — fullscreen button shows exit-fullscreen icon.
        .child(group(
            theme,
            "Fullscreen",
            player(
                theme,
                480.0,
                VideoPlayerSpec::new(src)
                    .with_duration(64.0)
                    .with_current_time(52.0)
                    .with_fullscreen(true),
            ),
        ))
        // Aspect ratios — container shape driven by aspect_ratio prop.
        .child(group(
            theme,
            "Aspect ratios",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(player(
                    theme,
                    480.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Video)
                        .with_duration(64.0)
                        .with_current_time(20.0),
                ))
                .child(player(
                    theme,
                    400.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Landscape)
                        .with_duration(64.0)
                        .with_current_time(20.0),
                ))
                .child(player(
                    theme,
                    260.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Portrait)
                        .with_duration(64.0)
                        .with_current_time(20.0),
                ))
                .child(player(
                    theme,
                    320.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Square)
                        .with_duration(64.0)
                        .with_current_time(20.0),
                )),
        ))
        // Sizes — control button/icon/volume/time scale per size variant.
        .child(group(
            theme,
            "Sizes",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new(src)
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_size(ControlSize::Xs),
                ))
                .child(player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new(src)
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_size(ControlSize::Sm),
                ))
                .child(player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new(src)
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_size(ControlSize::Lg),
                ))
                .child(player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new(src)
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_size(ControlSize::Xl),
                )),
        ))
        // Densities — controls-bar spacing only; component height unchanged.
        .child(group(
            theme,
            "Densities",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new(src)
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_density(ControlDensity::Compact),
                ))
                .child(player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new(src)
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_density(ControlDensity::Comfortable),
                )),
        ))
        // Semantic size role — inherited scale via role.
        .child(group(
            theme,
            "Semantic role",
            player(
                theme,
                520.0,
                VideoPlayerSpec::new(src)
                    .with_duration(64.0)
                    .with_current_time(28.0)
                    .with_playing(true)
                    .with_size_role(SemanticControlSizeRole::Prominent),
            ),
        ))
}

fn player(theme: &GpuiThemeProvider, max_w: f32, spec: VideoPlayerSpec) -> Div {
    div()
        .max_w(px(max_w))
        .child(VideoPlayer::from_spec(spec, theme))
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
