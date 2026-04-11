use gpui::*;
use poodle_components::{MediaThumbnailSpec, AspectRatio, MediaKind, MediaState};
use poodle_components::EyebrowSpec;
use poodle_gpui_components::{MediaThumbnail, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Image thumbnails"), theme))
            .child(div().flex().gap(px(8.0)).flex_wrap()
                .child(MediaThumbnail::from_spec(MediaThumbnailSpec::new(MediaKind::Image).with_title("Photo 1").with_badge_label("New").with_aspect_ratio(AspectRatio::Square), theme))
                .child(MediaThumbnail::from_spec(MediaThumbnailSpec::new(MediaKind::Image).with_title("Photo 2").with_meta("2.4 MB").with_aspect_ratio(AspectRatio::Square), theme))
                .child(MediaThumbnail::from_spec(MediaThumbnailSpec::new(MediaKind::Video).with_title("Clip").with_badge_label("HD").with_meta("1:24").with_aspect_ratio(AspectRatio::Square), theme))))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Compact presentation"), theme))
            .child(div().flex().gap(px(8.0)).flex_wrap()
                .child(MediaThumbnail::from_spec(MediaThumbnailSpec::new(MediaKind::Document).with_title("Report.pdf").with_aspect_ratio(AspectRatio::Landscape), theme))
                .child(MediaThumbnail::from_spec(MediaThumbnailSpec::new(MediaKind::Audio).with_title("Interview.mp3").with_aspect_ratio(AspectRatio::Landscape), theme))))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading state"), theme))
            .child(MediaThumbnail::from_spec(
                MediaThumbnailSpec::new(MediaKind::Image)
                    .with_state(MediaState::Loading)
                    .with_aspect_ratio(AspectRatio::Square)
                    .with_state_title("Loading preview")
                    .with_state_message("Preview data is still being prepared."),
                theme,
            )))
}
