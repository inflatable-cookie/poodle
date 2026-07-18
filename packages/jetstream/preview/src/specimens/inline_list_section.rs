//! InlineListSection specimen — compact card-contained related-list shell.
//!
//! Mirrors `packages/gpui/preview/src/specimens/inline_list_section_specimen.rs`:
//! framed section with count pill + header action, header actions without a count,
//! empty state, and an unframed (no-card) variant. Rows compose real `js_text` +
//! `js_pill`; header actions are real `js_icon_button`s — no hand-rolled boxes.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::icon_button::js_icon_button;
use poodle_jetstream_components::inline_list_section::js_inline_list_section;
use poodle_jetstream_components::pill::js_pill;
use poodle_jetstream_components::text::js_text;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ButtonVariant, IconButtonSpec, InlineListSectionSpec, PillSpec, PillTone, TextSpec, TextWeight,
};

fn row(name: &str, status: &str, theme: &JetstreamThemeProvider) -> JsEl {
    let tone = if status == "Ready" {
        PillTone::Success
    } else {
        PillTone::Neutral
    };
    div()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(resolve_px(theme, "space.inline.md"))
        .flex_grow()
        .min_w_0()
        .child(js_text(
            &TextSpec::new(name).with_weight(TextWeight::Medium),
            theme,
        ))
        .child(js_pill(
            &PillSpec::new().with_label(status).with_tone(tone),
            theme,
        ))
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(20.0)
        // -- Framed — count pill and header action --
        .child(group("Framed — count pill and header action", secondary,
            js_inline_list_section(
                &InlineListSectionSpec::new("Versions").with_count("3"),
                theme,
                vec![
                    row("Version 3", "Ready", theme),
                    row("Version 2", "Archived", theme),
                    row("Version 1", "Archived", theme),
                ],
                Some(js_icon_button(
                    &IconButtonSpec::new()
                        .with_icon("plus")
                        .with_aria_label("Add version")
                        .with_variant(ButtonVariant::Secondary),
                    theme,
                )),
            ),
        ))
        // -- Header actions (no count) --
        .child(group("Header actions (no count)", secondary,
            js_inline_list_section(
                &InlineListSectionSpec::new("Usages"),
                theme,
                vec![
                    row("checkout-flow", "Ready", theme),
                    row("billing-portal", "Archived", theme),
                ],
                Some(js_icon_button(
                    &IconButtonSpec::new()
                        .with_icon("external-link")
                        .with_aria_label("Open all usages")
                        .with_variant(ButtonVariant::Ghost),
                    theme,
                )),
            ),
        ))
        // -- Empty state --
        .child(group("Empty state", secondary,
            js_inline_list_section(
                &InlineListSectionSpec::new("Aliases").with_empty_message("No aliases yet."),
                theme,
                vec![],
                None,
            ),
        ))
        // -- Unframed (no card) --
        .child(group("Unframed (no card)", secondary,
            js_inline_list_section(
                &InlineListSectionSpec::new("References")
                    .with_count("2")
                    .with_framed(false),
                theme,
                vec![
                    row("Version 3", "Ready", theme),
                    row("Version 2", "Archived", theme),
                ],
                None,
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
