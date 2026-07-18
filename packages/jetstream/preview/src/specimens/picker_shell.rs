//! PickerShell specimen — generic searchable picker container.
//!
//! Mirrors `packages/gpui/preview/src/specimens/picker_shell_specimen.rs`: inline
//! ready (search + result rows + result count), loading, no-results, multiple
//! selection, popover variant (search + body + footer), modal variant, error, and
//! empty. Search fields are real `js_text_input`, body rows are real `js_text`,
//! footers are the real `js_form_actions` primitive with `js_button` actions, and
//! the selection summary is the real `js_selection_summary` — no hand-rolled boxes.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::form_actions::js_form_actions;
use poodle_jetstream_components::picker_shell::js_picker_shell;
use poodle_jetstream_components::selection_summary::js_selection_summary;
use poodle_jetstream_components::text::js_text;
use poodle_jetstream_components::text_input::js_text_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    BrowseState, ButtonSpec, ButtonVariant, FormActionsSpec, PickerShellSpec, PickerVariant,
    RemediationAction, SelectionMode, SelectionSummaryItem, SelectionSummarySpec, TextInputSpec,
    TextSpec, TextTone,
};

/// One candidate row: primary label + optional secondary meta (real `js_text`).
fn result_row(label_text: &str, meta: &str, theme: &JetstreamThemeProvider) -> JsEl {
    let mut row = div()
        .flex_row()
        .items_center()
        .justify_between()
        .px(resolve_px(theme, "space.inline.md"))
        .py(resolve_px(theme, "space.stack.sm"))
        .child(js_text(&TextSpec::new(label_text), theme));
    if !meta.is_empty() {
        row = row.child(js_text(
            &TextSpec::new(meta).with_tone(TextTone::Secondary),
            theme,
        ));
    }
    row
}

/// Confirm/cancel footer built from the real `js_form_actions` primitive.
fn picker_footer(theme: &JetstreamThemeProvider) -> JsEl {
    js_form_actions(
        &FormActionsSpec::new(),
        theme,
        vec![
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Cancel"),
                theme,
            ),
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Add"),
                theme,
            ),
        ],
    )
}

fn search_field(placeholder: &str, theme: &JetstreamThemeProvider) -> JsEl {
    js_text_input(&TextInputSpec::new().with_placeholder(placeholder), theme)
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // --- Inline variant (ready): search + result rows + result count ---
        .child(group("Inline variant (ready)", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Select a component")
                    .with_description("Browse and select from available components.")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::Ready)
                    .with_result_count(5),
                theme,
                Some(search_field("Search components...", theme)),
                None,
                Some(
                    div().flex_col()
                        .child(result_row("Button", "Primitive", theme))
                        .child(result_row("Checkbox", "Primitive", theme))
                        .child(result_row("Select", "Primitive", theme))
                        .child(result_row("Dialog", "Overlay", theme))
                        .child(result_row("Table", "Composite", theme)),
                ),
                None,
                None,
            ),
        ))
        // --- Loading ---
        .child(group("Loading", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Select an item")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::Loading)
                    .with_state_message("Fetching the latest results."),
                theme,
                None,
                None,
                None,
                None,
                None,
            ),
        ))
        // --- No results ---
        .child(group("No results", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Select an item")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::NoResults)
                    .with_query("xyzzy")
                    .with_state_title("No matches")
                    .with_state_message("Try a different search term."),
                theme,
                None,
                None,
                None,
                None,
                None,
            ),
        ))
        // --- Multiple selection: selection summary + selected rows ---
        .child(group("Multiple selection", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Select components")
                    .with_variant(PickerVariant::Inline)
                    .with_selection_mode(SelectionMode::Multiple)
                    .with_state(BrowseState::Ready)
                    .with_result_count(3)
                    .with_selected_count(2),
                theme,
                Some(search_field("Search components...", theme)),
                Some(js_selection_summary(
                    &SelectionSummarySpec::new(vec![
                        SelectionSummaryItem::new("button", "Button"),
                        SelectionSummaryItem::new("dialog", "Dialog"),
                    ])
                    .with_clear_action(RemediationAction::new("clear", "Clear")),
                    theme,
                )),
                Some(
                    div().flex_col()
                        .child(result_row("Button", "Selected", theme))
                        .child(result_row("Dialog", "Selected", theme))
                        .child(result_row("Table", "", theme)),
                ),
                None,
                None,
            ),
        ))
        // --- Popover variant: search + body + footer ---
        .child(group("Popover variant", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Add component")
                    .with_description("Pick a component to insert.")
                    .with_variant(PickerVariant::Popover)
                    .with_state(BrowseState::Ready)
                    .with_result_count(3)
                    .with_selected_count(1),
                theme,
                Some(search_field("Search components...", theme)),
                None,
                Some(
                    div().flex_col()
                        .child(result_row("Button", "Primitive", theme))
                        .child(result_row("Checkbox", "Primitive", theme))
                        .child(result_row("Select", "Primitive", theme)),
                ),
                None,
                Some(picker_footer(theme)),
            ),
        ))
        // --- Modal variant: search + body + footer ---
        .child(group("Modal variant", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Select a relation")
                    .with_description("Choose one or more related records.")
                    .with_variant(PickerVariant::Modal)
                    .with_state(BrowseState::Ready)
                    .with_result_count(3)
                    .with_selected_count(2),
                theme,
                Some(search_field("Search records...", theme)),
                None,
                Some(
                    div().flex_col()
                        .child(result_row("Acme Corp", "Selected", theme))
                        .child(result_row("Globex", "Selected", theme))
                        .child(result_row("Initech", "", theme)),
                ),
                None,
                Some(picker_footer(theme)),
            ),
        ))
        // --- Error ---
        .child(group("Error", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Search media")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::Error)
                    .with_state_message("The request failed. Try again in a moment."),
                theme,
                None,
                None,
                None,
                None,
                None,
            ),
        ))
        // --- Empty ---
        .child(group("Empty", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Choose item")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::Empty),
                theme,
                None,
                None,
                None,
                None,
                None,
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
