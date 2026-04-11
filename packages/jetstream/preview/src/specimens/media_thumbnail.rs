//! MediaThumbnail specimen — compact media item thumbnail.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::media_thumbnail::js_media_thumbnail;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{MediaKind, MediaThumbnailSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Image thumbnail", secondary,
            js_media_thumbnail(
                &MediaThumbnailSpec::new(MediaKind::Image).with_title("Photo.jpg"),
                theme,
            )
        ))
        .child(group("Video thumbnail", secondary,
            js_media_thumbnail(
                &MediaThumbnailSpec::new(MediaKind::Video).with_title("Clip.mp4"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
