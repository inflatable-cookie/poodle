use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, VideoPlayer};
use poodle_specs::EyebrowSpec;
use poodle_specs::{AspectRatio, VideoPlayerSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic"), theme))
            .child(div().max_w(px(480.0)).child(
                VideoPlayer::from_spec(VideoPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4").with_duration(6.0), theme))))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Landscape aspect ratio"), theme))
            .child(div().max_w(px(400.0)).child(
                VideoPlayer::from_spec(VideoPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4").with_aspect_ratio(AspectRatio::Landscape).with_duration(6.0), theme))))
}
