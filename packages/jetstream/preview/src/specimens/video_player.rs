//! VideoPlayer specimen — initial, playing, muted, fullscreen, aspect ratios, sizes.
//!
//! Captions and a title overlay are not represented: `js_video_player` takes only
//! `(spec, theme)` and `VideoPlayerSpec` exposes no captions/title field, so they
//! can't be rendered honestly here. (GPUI shows captions via its own
//! `VideoPlayer::with_captions` component builder, which has no Jetstream analogue.)

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::video_player::js_video_player;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    AspectRatio, ControlDensity, ControlSize, SemanticControlSizeRole, VideoPlayerSpec,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let src = "demo-video.mp4";

    div().flex_col().gap(24.0)
        // Initial: paused at start → big play overlay, controls visible.
        .child(group("Paused at start", secondary,
            player(theme, 480.0,
                VideoPlayerSpec::new(src)
                    .with_poster("thumbnail.jpg")
                    .with_duration(120.0)
                    .with_current_time(0.0))
        ))
        // Playing, mid-progress → controls bar, seek fill proportional.
        .child(group("Playing", secondary,
            player(theme, 480.0,
                VideoPlayerSpec::new(src)
                    .with_playing(true)
                    .with_duration(120.0)
                    .with_current_time(45.0)
                    .with_volume(0.75))
        ))
        // Muted — volume 0 drives the muted (volume-x) glyph.
        .child(group("Muted", secondary,
            player(theme, 480.0,
                VideoPlayerSpec::new(src)
                    .with_playing(true)
                    .with_duration(120.0)
                    .with_current_time(70.0)
                    .with_volume(0.0))
        ))
        // Fullscreen — fullscreen button shows exit-fullscreen glyph.
        .child(group("Fullscreen", secondary,
            player(theme, 480.0,
                VideoPlayerSpec::new(src)
                    .with_duration(120.0)
                    .with_current_time(95.0)
                    .with_fullscreen(true))
        ))
        // Aspect ratios — container shape from aspect_ratio prop.
        .child(group("Aspect ratios", secondary,
            div().flex_col().gap(12.0)
                .child(player(theme, 480.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Video)
                        .with_duration(120.0)
                        .with_current_time(30.0)))
                .child(player(theme, 400.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Landscape)
                        .with_duration(120.0)
                        .with_current_time(30.0)))
                .child(player(theme, 260.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Portrait)
                        .with_duration(120.0)
                        .with_current_time(30.0)))
                .child(player(theme, 320.0,
                    VideoPlayerSpec::new(src)
                        .with_aspect_ratio(AspectRatio::Square)
                        .with_duration(120.0)
                        .with_current_time(30.0)))
        ))
        // Sizes — control button/icon/volume/time scale per size variant.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(player(theme, 420.0, sized(src).with_size(ControlSize::Xs)))
                .child(player(theme, 420.0, sized(src).with_size(ControlSize::Sm)))
                .child(player(theme, 420.0, sized(src).with_size(ControlSize::Lg)))
                .child(player(theme, 420.0, sized(src).with_size(ControlSize::Xl)))
        ))
        // Densities — controls-bar spacing only; height unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(player(theme, 420.0, sized(src).with_density(ControlDensity::Compact)))
                .child(player(theme, 420.0, sized(src).with_density(ControlDensity::Comfortable)))
        ))
        // Semantic size role — inherited scale via role.
        .child(group("Semantic role", secondary,
            player(theme, 520.0,
                sized(src).with_size_role(SemanticControlSizeRole::Prominent))
        ))
}

/// A playing, mid-progress spec used by the size/density/role groups.
fn sized(src: &str) -> VideoPlayerSpec {
    VideoPlayerSpec::new(src)
        .with_playing(true)
        .with_duration(120.0)
        .with_current_time(45.0)
}

fn player(theme: &JetstreamThemeProvider, width: f32, spec: VideoPlayerSpec) -> JsEl {
    div().w(width).child(js_video_player(&spec, theme))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
