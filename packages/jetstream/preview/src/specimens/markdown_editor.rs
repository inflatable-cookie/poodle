//! MarkdownEditor specimen — markdown authoring with live preview.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::markdown_editor::js_markdown_editor;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::MarkdownEditorSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Edit mode", secondary,
            js_markdown_editor(
                &MarkdownEditorSpec::new()
                    .with_value("# Hello\n\nThis is **bold** and _italic_ text.")
                    .with_mode("edit"),
                theme,
            )
        ))
        .child(group("Split mode", secondary,
            js_markdown_editor(
                &MarkdownEditorSpec::new()
                    .with_value("## Preview\n\nSide-by-side editing.")
                    .with_mode("split"),
                theme,
            )
        ))
        .child(group("Empty placeholder", secondary,
            js_markdown_editor(
                &MarkdownEditorSpec::new()
                    .with_placeholder("Write your content here..."),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
