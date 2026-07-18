//! ListGrid specimen — responsive tile layout vs compact stack.
//!
//! Mirrors `packages/gpui/preview/src/specimens/list_grid.rs`: default wrap grid,
//! header-actions row, compact single-column, a grid of real `ListCard` tiles, and
//! an empty grid (header + token-resolved empty `Text`). All children are real
//! components (`js_list_card` / `js_button` / `js_icon_button` / `js_text`) —
//! no hand-rolled boxes.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::icon_button::js_icon_button;
use poodle_jetstream_components::list_card::js_list_card;
use poodle_jetstream_components::list_grid::js_list_grid;
use poodle_jetstream_components::text::js_text;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ButtonSpec, ButtonVariant, IconButtonSpec, LeadingShape, ListCardSpec, ListGridSpec,
    ListGridVariant, TextSpec, TextTone,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let tile = |title: &str, subtitle: &str| {
        js_list_card(
            &ListCardSpec::new().with_title(title).with_subtitle(subtitle),
            theme,
        )
    };

    let card_tile = |title: &str, subtitle: &str| {
        js_list_card(
            &ListCardSpec::new()
                .with_title(title)
                .with_subtitle(subtitle)
                .with_leading_shape(LeadingShape::RoundedSquare)
                .with_interactive(true),
            theme,
        )
    };

    // -- Default: wrap grid with min tile width --
    let default_grid = js_list_grid(
        &ListGridSpec::new()
            .with_variant(ListGridVariant::Default)
            .with_min_item_width_em(14.0),
        theme,
        None,
        vec![
            tile("Alpha", "Auto-fill columns from min width."),
            tile("Bravo", "Resize the preview to see wrapping."),
            tile("Charlie", "Third tile."),
        ],
    );

    // -- With header actions (Export button + refresh IconButton) --
    let actions_row = div()
        .flex_row()
        .items_center()
        .gap(resolve_px(theme, "space.inline.sm"))
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label("Export"),
            theme,
        ))
        .child(js_icon_button(
            &IconButtonSpec::new()
                .with_icon("refresh-cw")
                .with_aria_label("Refresh list")
                .with_variant(ButtonVariant::Secondary),
            theme,
        ));

    let with_actions = js_list_grid(
        &ListGridSpec::new().with_min_item_width_em(16.0),
        theme,
        Some(actions_row),
        vec![
            tile("Project A", "With header actions row."),
            tile("Project B", "Second tile."),
        ],
    );

    // -- Compact single-column stack --
    let compact_grid = js_list_grid(
        &ListGridSpec::new().with_variant(ListGridVariant::Compact),
        theme,
        None,
        vec![
            tile("One", "Compact single-column stack."),
            tile("Two", "Tighter default gap."),
        ],
    );

    // -- Grid of real ListCard tiles --
    let with_cards = js_list_grid(
        &ListGridSpec::new().with_min_item_width_em(16.0),
        theme,
        None,
        vec![
            card_tile("Design system", "Tokens, primitives, composites"),
            card_tile("Media library", "Images, video, and audio"),
            card_tile("Documentation", "Contracts and guides"),
            card_tile("Analytics", "Usage and adoption metrics"),
        ],
    );

    // -- Empty: header actions row, no items, token-resolved empty Text --
    let empty_header = div()
        .flex_row()
        .items_center()
        .gap(resolve_px(theme, "space.inline.sm"))
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label("New item"),
            theme,
        ));

    let empty_grid = js_list_grid(
        &ListGridSpec::new().with_min_item_width_em(16.0),
        theme,
        Some(empty_header),
        vec![div()
            .flex_row()
            .items_center()
            .justify_center()
            .pt(resolve_px(theme, "space.stack.lg"))
            .pb(resolve_px(theme, "space.stack.lg"))
            .child(js_text(
                &TextSpec::new("No items yet \u{2014} add one to populate the grid.")
                    .with_tone(TextTone::Secondary),
                theme,
            ))],
    );

    div().flex_col().gap(24.0)
        .child(group("Default — wrap grid (min 14em)", secondary, default_grid))
        .child(group("With header actions", secondary, with_actions))
        .child(group("Compact — single column", secondary, compact_grid))
        .child(group("With ListCards (min 16em)", secondary, with_cards))
        .child(group("Empty — header with no items", secondary, empty_grid))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
