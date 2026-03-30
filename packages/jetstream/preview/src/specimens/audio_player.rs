//! AudioPlayer specimen — playing, paused, with speed control.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::audio_player::js_audio_player;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::AudioPlayerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Paused
        .child(group("Paused", secondary,
            div().w(400.0)
                .child(js_audio_player(
                    &AudioPlayerSpec::new("podcast-episode.mp3")
                        .with_duration(1842.0)
                        .with_current_time(0.0),
                    theme,
                ))
        ))
        // Playing, mid-track
        .child(group("Playing", secondary,
            div().w(400.0)
                .child(js_audio_player(
                    &AudioPlayerSpec::new("podcast-episode.mp3")
                        .with_playing(true)
                        .with_duration(1842.0)
                        .with_current_time(723.0),
                    theme,
                ))
        ))
        // With speed control
        .child(group("With speed control", secondary,
            div().w(400.0)
                .child(js_audio_player(
                    &AudioPlayerSpec::new("interview.mp3")
                        .with_playing(true)
                        .with_duration(3600.0)
                        .with_current_time(1200.0)
                        .with_show_speed_control(true),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
