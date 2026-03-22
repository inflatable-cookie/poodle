//! ScrollShell specimen — scrollable container with child content.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::scroll_shell::js_scroll_shell;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::ScrollShellSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    let children = vec![
        label("Scrollable content line 1").text_color(text_primary).text_size(13.0),
        label("Scrollable content line 2").text_color(text_primary).text_size(13.0),
        label("Scrollable content line 3").text_color(text_primary).text_size(13.0),
    ];

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            div().h(120.0)
                .child(js_scroll_shell(&ScrollShellSpec::new(), theme, children))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
