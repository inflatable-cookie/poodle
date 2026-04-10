//! NavCard specimen — navigation cards with title and description.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::nav_card::js_nav_card;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::NavCardSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Navigation cards", secondary,
            div().flex_col().gap(8.0)
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Primitives")
                        .with_description("Core building blocks of the design system"),
                    theme,
                ))
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Composites")
                        .with_description("Higher-order components built from primitives")
                        .with_badge("12"),
                    theme,
                ))
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Disabled")
                        .with_description("This card is disabled")
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
