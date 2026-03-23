use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{
    MediaPreviewSpec, MediaThumbnailSpec, AspectRatio, MediaKind, MediaState,
    AudioPlayerSpec, VideoPlayerSpec, MediaPickerSpec,
};
use pug_gpui_components::{
    MediaPreview, MediaThumbnail, AudioPlayer, VideoPlayer,
    MediaPicker, MediaPickerItem,
};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // ── MediaPreview ──

        // --- Image preview ---
        .child(section_label("MEDIA PREVIEW: IMAGE PREVIEW", text_secondary))
        .child(
            MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Image, "Hero banner")
                    .with_description("Main landing page banner image for the product launch.")
                    .with_metadata(vec![
                        "1920 x 1080".into(),
                        "245 KB".into(),
                        "PNG".into(),
                    ]),
                theme,
            )
        )
        // --- Video preview ---
        .child(section_label("MEDIA PREVIEW: VIDEO PREVIEW", text_secondary))
        .child(
            MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Video, "Onboarding walkthrough")
                    .with_metadata(vec![
                        "3:42".into(),
                        "48 MB".into(),
                    ]),
                theme,
            )
        )
        // --- Error state ---
        .child(section_label("MEDIA PREVIEW: ERROR STATE", text_secondary))
        .child(
            MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
                    .with_state(MediaState::Error),
                theme,
            )
        )

        // ── MediaThumbnail ──

        // --- Image thumbnails ---
        .child(section_label("MEDIA THUMBNAIL: IMAGE THUMBNAILS", text_secondary))
        .child(
            div().flex().gap(px(8.0)).flex_wrap()
                .child(
                    MediaThumbnail::from_spec(
                        MediaThumbnailSpec::new(MediaKind::Image)
                            .with_title("Photo 1")
                            .with_badge_label("New")
                            .with_aspect_ratio(AspectRatio::Square),
                        theme,
                    )
                )
                .child(
                    MediaThumbnail::from_spec(
                        MediaThumbnailSpec::new(MediaKind::Image)
                            .with_title("Photo 2")
                            .with_meta("2.4 MB")
                            .with_aspect_ratio(AspectRatio::Square),
                        theme,
                    )
                )
                .child(
                    MediaThumbnail::from_spec(
                        MediaThumbnailSpec::new(MediaKind::Video)
                            .with_title("Clip")
                            .with_badge_label("HD")
                            .with_meta("1:24")
                            .with_aspect_ratio(AspectRatio::Square),
                        theme,
                    )
                )
        )
        // --- Compact presentation ---
        .child(section_label("MEDIA THUMBNAIL: COMPACT PRESENTATION", text_secondary))
        .child(
            div().flex().gap(px(8.0)).flex_wrap()
                .child(
                    MediaThumbnail::from_spec(
                        MediaThumbnailSpec::new(MediaKind::Document)
                            .with_title("Report.pdf")
                            .with_aspect_ratio(AspectRatio::Landscape),
                        theme,
                    )
                )
                .child(
                    MediaThumbnail::from_spec(
                        MediaThumbnailSpec::new(MediaKind::Audio)
                            .with_title("Interview.mp3")
                            .with_aspect_ratio(AspectRatio::Landscape),
                        theme,
                    )
                )
        )
        // --- Loading state ---
        .child(section_label("MEDIA THUMBNAIL: LOADING STATE", text_secondary))
        .child(
            MediaThumbnail::from_spec(
                MediaThumbnailSpec::new(MediaKind::Image)
                    .with_state(MediaState::Loading)
                    .with_aspect_ratio(AspectRatio::Square),
                theme,
            )
        )

        // ── AudioPlayer ──

        // --- Basic audio player ---
        .child(section_label("AUDIO PLAYER: BASIC", text_secondary))
        .child(
            AudioPlayer::from_spec(
                AudioPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3")
                    .with_duration(2.0),
                theme,
            )
        )

        // --- Audio with speed control ---
        .child(section_label("AUDIO PLAYER: WITH SPEED CONTROL", text_secondary))
        .child(
            AudioPlayer::from_spec(
                AudioPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3")
                    .with_duration(2.0)
                    .with_show_speed_control(true),
                theme,
            )
        )

        // ── VideoPlayer ──

        // --- Basic video player ---
        .child(section_label("VIDEO PLAYER: BASIC", text_secondary))
        .child(
            div().max_w(px(480.0)).child(
                VideoPlayer::from_spec(
                    VideoPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4")
                        .with_duration(6.0),
                    theme,
                )
            )
        )

        // --- Video with custom aspect ratio ---
        .child(section_label("VIDEO PLAYER: LANDSCAPE ASPECT RATIO", text_secondary))
        .child(
            div().max_w(px(400.0)).child(
                VideoPlayer::from_spec(
                    VideoPlayerSpec::new("https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4")
                        .with_aspect_ratio(AspectRatio::Landscape)
                        .with_duration(6.0),
                    theme,
                )
            )
        )

        // ── MediaPicker ──

        // --- Media picker (open) ---
        .child(section_label("MEDIA PICKER (BROWSE TAB)", text_secondary))
        .child(
            MediaPicker::from_spec(
                MediaPickerSpec::new("Select media")
                    .with_open(true),
                theme,
            )
            .with_thumbnails(vec![
                MediaPickerItem { id: "img-1".to_string(), label: "Banner image".to_string(), is_selected: false },
                MediaPickerItem { id: "img-2".to_string(), label: "Profile photo".to_string(), is_selected: true },
                MediaPickerItem { id: "img-3".to_string(), label: "Icon set".to_string(), is_selected: false },
                MediaPickerItem { id: "doc-1".to_string(), label: "Readme.pdf".to_string(), is_selected: false },
                MediaPickerItem { id: "vid-1".to_string(), label: "Demo video".to_string(), is_selected: false },
                MediaPickerItem { id: "img-4".to_string(), label: "Screenshot".to_string(), is_selected: false },
            ])
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
