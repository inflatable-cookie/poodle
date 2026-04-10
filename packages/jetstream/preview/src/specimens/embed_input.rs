//! EmbedInput specimen — URL or embed code input.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::embed_input::js_embed_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::EmbedInputSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("With URL", secondary,
            js_embed_input(
                &EmbedInputSpec::new()
                    .with_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                    .with_detected_parse(),
                theme,
            )
        ))
        .child(group("Empty", secondary,
            js_embed_input(
                &EmbedInputSpec::new()
                    .with_placeholder("Paste a URL or embed code..."),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
