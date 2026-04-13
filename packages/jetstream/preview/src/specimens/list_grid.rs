//! ListGrid specimen — wrapped tile layout vs compact stack.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::list_card::js_list_card;
use poodle_jetstream_components::list_grid::js_list_grid;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ListCardSpec, ListGridSpec, ListGridVariant};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let tile = |title: &str| {
        js_list_card(
            &ListCardSpec::new()
                .with_title(title)
                .with_subtitle("Sample row"),
            theme,
        )
    };

    let default_grid = js_list_grid(
        &ListGridSpec::new().with_variant(ListGridVariant::Default),
        theme,
        Some(
            label("Optional header")
                .text_color(secondary)
                .text_size(11.0),
        ),
        vec![tile("Alpha"), tile("Beta"), tile("Gamma")],
    );

    let compact_grid = js_list_grid(
        &ListGridSpec::new().with_variant(ListGridVariant::Compact),
        theme,
        None,
        vec![tile("One"), tile("Two")],
    );

    div().flex_col().gap(24.0)
        .child(group("Default (wrap + min tile width)", secondary, default_grid))
        .child(group("Compact (vertical stack)", secondary, compact_grid))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
