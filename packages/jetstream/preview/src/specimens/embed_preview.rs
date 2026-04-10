//! EmbedPreview specimen — rich preview of detected embeds.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::embed_preview::js_embed_preview;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::{EmbedPreviewSpec, ParsedEmbed};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("YouTube embed", secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new().with_parsed(
                    ParsedEmbed::new("youtube", "dQw4w9WgXcQ")
                        .with_original_url("https://youtube.com/watch?v=dQw4w9WgXcQ"),
                ),
                theme,
            )
        ))
        .child(group("Loading", secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new().with_loading(true),
                theme,
            )
        ))
        .child(group("Empty", secondary,
            js_embed_preview(&EmbedPreviewSpec::new(), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
