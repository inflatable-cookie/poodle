use gpui::*;
use poodle_composites::AudioPlayerSpec;
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{AudioPlayer, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic"), theme))
            .child(AudioPlayer::from_spec(AudioPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3").with_duration(2.0), theme)))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With speed control"), theme))
            .child(AudioPlayer::from_spec(AudioPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3").with_duration(2.0).with_show_speed_control(true), theme)))
}
