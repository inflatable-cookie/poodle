//! SplitButton specimen — primary action paired with a dropdown chevron toggle.
//!
//! Full contract-state coverage: variant × tone matrix, dropdown menu open
//! (items + separator), loading, disabled, and the size grid. Every example
//! is a real `js_split_button` resolving all visuals from tokens.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::split_button::js_split_button;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ButtonTone, ButtonVariant, ControlSize, SplitButtonSpec, SplitMenuItem};

fn save_items() -> Vec<SplitMenuItem> {
    vec![
        SplitMenuItem::action("save-draft", "Save as draft"),
        SplitMenuItem::action("save-template", "Save as template"),
        SplitMenuItem::Separator,
        SplitMenuItem::action("discard", "Discard changes"),
    ]
}

fn export_items() -> Vec<SplitMenuItem> {
    vec![
        SplitMenuItem::action("csv", "Export as CSV"),
        SplitMenuItem::action("json", "Export as JSON"),
        SplitMenuItem::action("pdf", "Export as PDF"),
    ]
}

fn delete_items() -> Vec<SplitMenuItem> {
    vec![
        SplitMenuItem::action("delete-selected", "Delete selected"),
        SplitMenuItem::action("delete-all", "Delete all"),
    ]
}

/// One split button: variant + tone + label + menu items.
fn split(
    theme: &JetstreamThemeProvider,
    variant: ButtonVariant,
    tone: ButtonTone,
    label: &str,
    items: Vec<SplitMenuItem>,
) -> JsEl {
    js_split_button(
        &SplitButtonSpec::new()
            .with_variant(variant)
            .with_tone(tone)
            .with_label(label)
            .with_items(items),
        theme,
    )
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // ── Primary variant ──
        .child(group("Primary variant", secondary,
            split(theme, ButtonVariant::Primary, ButtonTone::Default, "Save", save_items())
        ))
        // ── Secondary variant ──
        .child(group("Secondary variant", secondary,
            split(theme, ButtonVariant::Secondary, ButtonTone::Default, "Export", export_items())
        ))
        // ── Ghost variant ──
        .child(group("Ghost variant", secondary,
            split(theme, ButtonVariant::Ghost, ButtonTone::Default, "Share", export_items())
        ))
        // ── Variant × tone matrix ──
        .child(group("Variant × tone (default / danger / success)", secondary,
            div().flex_col().gap(8.0)
                .child(div().flex_row().gap(8.0).items_center()
                    .child(split(theme, ButtonVariant::Primary, ButtonTone::Default, "Primary", save_items()))
                    .child(split(theme, ButtonVariant::Primary, ButtonTone::Danger, "Danger", delete_items()))
                    .child(split(theme, ButtonVariant::Primary, ButtonTone::Success, "Success", save_items())))
                .child(div().flex_row().gap(8.0).items_center()
                    .child(split(theme, ButtonVariant::Secondary, ButtonTone::Default, "Secondary", save_items()))
                    .child(split(theme, ButtonVariant::Secondary, ButtonTone::Danger, "Danger", delete_items()))
                    .child(split(theme, ButtonVariant::Secondary, ButtonTone::Success, "Success", save_items())))
                .child(div().flex_row().gap(8.0).items_center()
                    .child(split(theme, ButtonVariant::Ghost, ButtonTone::Default, "Ghost", save_items()))
                    .child(split(theme, ButtonVariant::Ghost, ButtonTone::Danger, "Danger", delete_items()))
                    .child(split(theme, ButtonVariant::Ghost, ButtonTone::Success, "Success", save_items())))
        ))
        // ── Danger tone ──
        .child(group("Danger tone", secondary,
            split(theme, ButtonVariant::Secondary, ButtonTone::Danger, "Delete", delete_items())
        ))
        // ── Dropdown menu open ──
        .child(group("Dropdown menu open (items + separator)", secondary,
            js_split_button(
                &SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Save")
                    .with_items(save_items())
                    .with_open(true),
                theme,
            )
        ))
        // ── Loading state ──
        .child(group("Loading state (spinner in primary half)", secondary,
            js_split_button(
                &SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Saving…")
                    .with_items(save_items())
                    .with_loading(true),
                theme,
            )
        ))
        // ── Disabled ──
        .child(group("Disabled", secondary,
            js_split_button(
                &SplitButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Save")
                    .with_items(save_items())
                    .with_disabled(true),
                theme,
            )
        ))
        // ── Sizes (xs–xl) ──
        .child(group("Sizes (xs–xl)", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(size_btn(theme, ControlSize::Xs))
                .child(size_btn(theme, ControlSize::Sm))
                .child(size_btn(theme, ControlSize::Md))
                .child(size_btn(theme, ControlSize::Lg))
                .child(size_btn(theme, ControlSize::Xl))
        ))
}

fn size_btn(theme: &JetstreamThemeProvider, size: ControlSize) -> JsEl {
    js_split_button(
        &SplitButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_size(size)
            .with_label("Action")
            .with_items(export_items()),
        theme,
    )
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
