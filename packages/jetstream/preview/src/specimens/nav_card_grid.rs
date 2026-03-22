//! NavCardGrid specimen — responsive grid of navigation cards.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::nav_card::js_nav_card;
use pug_jetstream_components::nav_card_grid::js_nav_card_grid;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::{NavCardGridSpec, NavCardSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let cards = vec![
        js_nav_card(
            &NavCardSpec::new()
                .with_title("Primitives")
                .with_description("Core building blocks"),
            theme,
        ),
        js_nav_card(
            &NavCardSpec::new()
                .with_title("Composites")
                .with_description("Higher-order components"),
            theme,
        ),
        js_nav_card(
            &NavCardSpec::new()
                .with_title("Tokens")
                .with_description("Design token reference"),
            theme,
        ),
        js_nav_card(
            &NavCardSpec::new()
                .with_title("Demo")
                .with_description("Interactive demos"),
            theme,
        ),
    ];

    div().flex_col().gap(24.0)
        .child(group("2-column grid", secondary,
            js_nav_card_grid(&NavCardGridSpec::new().with_columns(2), theme, cards)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
