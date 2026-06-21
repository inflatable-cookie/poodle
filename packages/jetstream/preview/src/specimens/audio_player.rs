//! AudioPlayer specimen — transport, volume, speed, sizes, densities.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::audio_player::js_audio_player;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{AudioPlayerSpec, ControlDensity, ControlSize, SemanticControlSizeRole};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let src = "podcast-episode.mp3";

    div().flex_col().gap(24.0)
        // Transport: idle/paused vs playing (seek fill is proportional to current_time).
        .child(group("Transport", secondary,
            div().flex_col().gap(12.0)
                // Idle / paused at start.
                .child(player(theme,
                    AudioPlayerSpec::new(src).with_duration(1842.0).with_current_time(0.0)))
                // Playing, mid-progress.
                .child(player(theme,
                    AudioPlayerSpec::new(src)
                        .with_playing(true)
                        .with_duration(1842.0)
                        .with_current_time(723.0)))
        ))
        // Volume: muted vs reduced volume (volume slider fill proportional).
        .child(group("Volume", secondary,
            div().flex_col().gap(12.0)
                .child(player(theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(1842.0)
                        .with_current_time(960.0)
                        .with_muted(true)))
                .child(player(theme,
                    AudioPlayerSpec::new(src)
                        .with_duration(1842.0)
                        .with_current_time(960.0)
                        .with_volume(0.4)))
        ))
        // Speed control: speed-active, distinct selected rates.
        .child(group("Speed control", secondary,
            div().flex_col().gap(12.0)
                .child(player(theme,
                    AudioPlayerSpec::new("interview.mp3")
                        .with_duration(3600.0)
                        .with_current_time(1200.0)
                        .with_show_speed_control(true)
                        .with_rate(1.0)))
                .child(player(theme,
                    AudioPlayerSpec::new("interview.mp3")
                        .with_playing(true)
                        .with_duration(3600.0)
                        .with_current_time(1200.0)
                        .with_show_speed_control(true)
                        .with_rate(1.5)))
        ))
        // Sizes — button/icon/time/volume scale per size variant.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(player(theme, sized(src).with_size(ControlSize::Xs)))
                .child(player(theme, sized(src).with_size(ControlSize::Sm)))
                .child(player(theme, sized(src).with_size(ControlSize::Md)))
                .child(player(theme, sized(src).with_size(ControlSize::Lg)))
                .child(player(theme, sized(src).with_size(ControlSize::Xl)))
        ))
        // Densities — spacing only; component height unchanged.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(player(theme, sized(src).with_density(ControlDensity::Compact)))
                .child(player(theme, sized(src).with_density(ControlDensity::Default)))
                .child(player(theme, sized(src).with_density(ControlDensity::Comfortable)))
        ))
        // Semantic size role — inherited scale via role.
        .child(group("Semantic role", secondary,
            player(theme,
                sized(src)
                    .with_playing(true)
                    .with_size_role(SemanticControlSizeRole::Prominent))
        ))
}

/// A mid-progress spec with the speed control on, used by size/density groups.
fn sized(src: &str) -> AudioPlayerSpec {
    AudioPlayerSpec::new(src)
        .with_duration(1842.0)
        .with_current_time(600.0)
        .with_show_speed_control(true)
}

fn player(theme: &JetstreamThemeProvider, spec: AudioPlayerSpec) -> JsEl {
    div().w(400.0).child(js_audio_player(&spec, theme))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
