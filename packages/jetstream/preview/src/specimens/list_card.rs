//! ListCard specimen — card items for list views.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::list_card::js_list_card;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::ListCardSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("List cards", secondary,
            div().flex_col().gap(4.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Button component")
                        .with_subtitle("Core primitive"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Dialog component")
                        .with_subtitle("Overlay primitive")
                        .with_meta("Updated 2h ago"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Disabled card")
                        .with_disabled(true),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
