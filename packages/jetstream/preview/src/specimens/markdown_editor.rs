//! MarkdownEditor specimen — markdown authoring with live preview.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::markdown_editor::js_markdown_editor;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::MarkdownEditorSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let split_spec = MarkdownEditorSpec::new()
        .with_value("# Notes\n\nSide-by-side **editing** with `code` and a [link](url).")
        .with_mode("split");
    let split_status = format!("Mode: split | {} chars", split_spec.char_count());

    div().flex_col().gap(24.0)
        .child(group("Edit mode", secondary,
            js_markdown_editor(
                &MarkdownEditorSpec::new()
                    .with_value("# Hello\n\nThis is **bold** and _italic_ text.")
                    .with_mode("edit"),
                theme,
            )
        ))
        .child(group_with_status("Split mode", secondary, &split_status,
            js_markdown_editor(&split_spec, theme)
        ))
        .child(group("Preview mode (tools disabled)", secondary,
            js_markdown_editor(
                &MarkdownEditorSpec::new()
                    .with_value("# Preview\n\nRendered output; toolbar tools are disabled here.")
                    .with_mode("preview"),
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
        .child(group("Disabled", secondary,
            js_markdown_editor(
                &MarkdownEditorSpec::new()
                    .with_value("Read-only content")
                    .with_disabled(true),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}

fn group_with_status(title: &str, text_secondary: glam::Vec4, status: &str, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
        .child(label(status).text_color(text_secondary).text_size(11.0))
}
