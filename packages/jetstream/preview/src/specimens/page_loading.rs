//! PageLoading specimen — full-viewport loading overlay.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::page_loading::js_page_loading;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::PageLoadingSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Indeterminate", secondary,
            div().h(200.0).child(
                js_page_loading(
                    &PageLoadingSpec::new()
                        .with_message("Loading your workspace..."),
                    theme,
                )
            )
        ))
        .child(group("Determinate (45%)", secondary,
            div().h(200.0).child(
                js_page_loading(
                    &PageLoadingSpec::new()
                        .with_message("Importing assets...")
                        .with_value(45.0)
                        .with_can_cancel(true),
                    theme,
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
