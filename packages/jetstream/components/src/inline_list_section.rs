//! JsInlineListSection — compact titled list section backed by InlineListSectionSpec.
//!
//! Contract: `docs/contracts/components/inline-list-section.md`
//! Reference: GPUI `composites/inline_list_section.rs`.
//!
//! Uppercase title + optional count pill + optional action header, then either an
//! empty message or the item list, optionally wrapped in a Card. Values resolve
//! from tokens / contract exact-rem constants.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CardSpec, InlineListSectionSpec};

use crate::card::js_card;
use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// Build an inline-list-section from its spec + item rows + an optional action.
pub fn js_inline_list_section(
    spec: &InlineListSectionSpec,
    theme: &JetstreamThemeProvider,
    items: Vec<JsEl>,
    action: Option<JsEl>,
) -> JsEl {
    // Colors (contract Token Usage tables).
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let border = resolve_color(theme, "color.border.default");
    let elevated = resolve_color(theme, "color.background.elevated");
    let surface = resolve_color(theme, "color.background.surface");
    // Item chrome: color-mix(in srgb, surface 93%, text-primary).
    let row_bg = color_mix(surface, text_primary, 0.93);

    // Typography (contract Token Usage tables).
    let label_size = resolve_px(theme, "typography.label.size");
    let body_size = resolve_px(theme, "typography.body.size");

    // Spacing (token + contract-exact rem).
    let root_gap = resolve_px(theme, "space.stack.md");
    let items_gap = resolve_px(theme, "space.stack.sm");

    // Item-row radius: calc(radius.surface − 0.1875rem).
    let surface_radius = resolve_radius(theme, "radius.surface");
    let item_radius = surface_radius - rem_to_px(0.1875);

    // Title cluster: uppercase title + optional count pill.
    let mut title_cluster = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.5))
        .min_w_0()
        .child(
            ui_element::label(spec.title.to_uppercase())
                .text_size(label_size)
                .text_weight(600)
                .letter_spacing_em(0.05) // contract Title: letter-spacing 0.05em
                .text_color(text_secondary),
        );
    if let Some(count) = &spec.count {
        title_cluster = title_cluster.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .justify_center()
                .min_w(rem_to_px(1.875))
                .h(rem_to_px(1.375))
                .px(rem_to_px(0.5))
                .rounded(999.0)
                .border(rem_to_px(0.0625))
                .border_color(border)
                .bg(elevated)
                .child(
                    ui_element::label(count.as_str())
                        .text_size(label_size)
                        .text_weight(600)
                        .text_color(text_secondary),
                ),
        );
    }

    let mut header = ui_element::div()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(rem_to_px(0.75))
        .child(title_cluster);
    if let Some(action) = action {
        header = header.child(action);
    }

    let mut body = ui_element::div().flex_col().gap(root_gap).child(header);

    if items.is_empty() {
        if let Some(message) = &spec.empty_message {
            body = body.child(
                ui_element::label(message.as_str())
                    .text_size(body_size)
                    .text_color(text_secondary),
            );
        }
    } else {
        let mut list = ui_element::div().flex_col().gap(items_gap);
        for item in items {
            list = list.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(rem_to_px(0.75))
                    .min_w_0()
                    .px(rem_to_px(0.625))
                    .py(rem_to_px(0.5))
                    .rounded(item_radius)
                    .bg(row_bg)
                    .child(item),
            );
        }
        body = body.child(list);
    }

    if spec.framed {
        js_card(&CardSpec::new(), theme, vec![body])
    } else {
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_uppercase_title() {
        let el = js_inline_list_section(
            &InlineListSectionSpec::new("Recent"),
            &theme(),
            vec![],
            None,
        );
        let tree = probe(&el, 320.0, 200.0);
        assert!(tree.has_text("RECENT"), "uppercased title missing: {:?}", tree.texts());
    }

    #[test]
    fn empty_shows_message() {
        let el = js_inline_list_section(
            &InlineListSectionSpec::new("Files"),
            &theme(),
            vec![],
            None,
        );
        let tree = probe(&el, 320.0, 200.0);
        assert!(tree.has_text("No items yet."), "empty message missing: {:?}", tree.texts());
    }

    #[test]
    fn renders_items_and_count() {
        let el = js_inline_list_section(
            &InlineListSectionSpec::new("Tags").with_count("2"),
            &theme(),
            vec![ui_element::label("alpha"), ui_element::label("beta")],
            None,
        );
        let tree = probe(&el, 320.0, 200.0);
        assert!(tree.has_text("alpha") && tree.has_text("beta"), "items missing");
        assert!(tree.has_text("2"), "count pill missing");
    }
}
