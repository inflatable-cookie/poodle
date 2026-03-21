//! Card specimen — cards with different variants and layouts.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::card::js_card;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::{CardLayout, CardSpec, CardVariant};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    div().flex_col().gap(24.0)
        // Default variant
        .child(group("Default", secondary,
            js_card(&CardSpec::new(), theme, vec![
                label("Card title").text_color(text_primary).text_size(14.0),
                label("Card body content goes here.").text_color(secondary).text_size(13.0),
            ])
        ))
        // Outlined variant
        .child(group("Outlined", secondary,
            js_card(&CardSpec::new().with_variant(CardVariant::Outlined), theme, vec![
                label("Outlined card").text_color(text_primary).text_size(14.0),
                label("With visible border.").text_color(secondary).text_size(13.0),
            ])
        ))
        // Horizontal layout
        .child(group("Horizontal layout", secondary,
            js_card(&CardSpec::new().with_layout(CardLayout::Horizontal), theme, vec![
                div().w(48.0).h(48.0).rounded(8.0).bg(resolve_color(theme, "semantic.color.accent.base")),
                div().flex_col().gap(4.0)
                    .child(label("Horizontal card").text_color(text_primary).text_size(14.0))
                    .child(label("Content beside media.").text_color(secondary).text_size(13.0)),
            ])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
