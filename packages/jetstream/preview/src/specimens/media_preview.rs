//! MediaPreview specimen — expanded media item preview.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::media_preview::js_media_preview;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{MediaKind, MediaPreviewSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Image preview", secondary,
            js_media_preview(
                &MediaPreviewSpec::new(MediaKind::Image, "Photo.jpg"),
                theme,
            )
        ))
        .child(group("Audio with description", secondary,
            js_media_preview(
                &MediaPreviewSpec::new(MediaKind::Audio, "Stem waveform.wav")
                    .with_description("High-quality audio stem"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
