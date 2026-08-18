use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, VideoPlayer};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EyebrowSpec;
use poodle_specs::{AspectRatio, SemanticControlSizeRole, VideoPlayerSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let src = "https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4";
    let poster = "https://interactive-examples.mdn.mozilla.net/media/examples/friday.mp4";
    let examples = div()
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
        // Captions track present. Contract §2 renders it as a `<track>` child
        // of the media element, so it carries no visible chrome — the old GPUI
        // tier's extra "subtitles" control button was a tier-local invention
        // with no contract or Svelte counterpart and does not survive the move.
        .child(group(
            theme,
            "With captions",
            player(
                theme,
                480.0,
                VideoPlayerSpec::new(src)
                    .with_duration(64.0)
                    .with_current_time(40.0)
                    .with_playing(true)
                    .with_captions_src("https://example.invalid/flower.vtt")
                    .with_show_captions(true),
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
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "video-player",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new("/media/sample.mp4")
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_size(size),
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                player(
                    theme,
                    420.0,
                    VideoPlayerSpec::new("/media/sample.mp4")
                        .with_duration(64.0)
                        .with_current_time(28.0)
                        .with_playing(true)
                        .with_density(density),
                )
                .into_any_element()
            }),
    )
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
