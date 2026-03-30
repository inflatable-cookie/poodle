//! BlockEditor specimen — with blocks, empty, disabled.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::block_editor::js_block_editor;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::BlockEditorSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // With blocks
        .child(group("With blocks", secondary,
            div().w(500.0)
                .child(js_block_editor(
                    &BlockEditorSpec::new().with_block_count(4),
                    theme,
                ))
        ))
        // Empty
        .child(group("Empty", secondary,
            div().w(500.0)
                .child(js_block_editor(&BlockEditorSpec::new(), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(500.0)
                .child(js_block_editor(
                    &BlockEditorSpec::new().with_block_count(2).with_disabled(true),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
