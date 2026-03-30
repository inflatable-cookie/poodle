//! VideoPlayer specimen — with source, paused.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::video_player::js_video_player;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::VideoPlayerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Paused (initial state)
        .child(group("Paused", secondary,
            div().w(480.0)
                .child(js_video_player(
                    &VideoPlayerSpec::new("demo-video.mp4")
                        .with_duration(120.0)
                        .with_current_time(0.0),
                    theme,
                ))
        ))
        // Playing, mid-way
        .child(group("Playing", secondary,
            div().w(480.0)
                .child(js_video_player(
                    &VideoPlayerSpec::new("demo-video.mp4")
                        .with_playing(true)
                        .with_duration(120.0)
                        .with_current_time(45.0),
                    theme,
                ))
        ))
        // With poster
        .child(group("With poster", secondary,
            div().w(480.0)
                .child(js_video_player(
                    &VideoPlayerSpec::new("tutorial.mp4")
                        .with_poster("thumbnail.jpg")
                        .with_duration(600.0),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
