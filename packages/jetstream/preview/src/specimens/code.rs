//! Code specimen — code blocks with sample content.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::code::js_code;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::CodeSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Inline code
        .child(group("Code block", secondary,
            div().w(300.0)
                .child(js_code(&CodeSpec::new().with_content("fn main() {\n    println!(\"Hello\");\n}"), theme))
        ))
        // With language
        .child(group("With language hint", secondary,
            div().w(300.0)
                .child(js_code(&CodeSpec::new().with_content("const x = 42;").with_language("javascript"), theme))
        ))
        // With line numbers
        .child(group("With line numbers", secondary,
            div().w(300.0)
                .child(js_code(&CodeSpec::new().with_content("let a = 1;\nlet b = 2;\nlet c = a + b;").with_show_line_numbers(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
