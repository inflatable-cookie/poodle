//! FormDialog specimen — modal dialog wrapping a form.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::field::js_field;
use poodle_jetstream_components::form_actions::js_form_actions;
use poodle_jetstream_components::form_dialog::js_form_dialog;
use poodle_jetstream_components::text_input::js_text_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ButtonSpec, ButtonVariant, FieldSpec, FormActionsSpec, FormDialogSpec, TextInputSpec,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Real Field (label + control) wrapping a real TextInput — no fakes (CLAUDE.md).
    let field = |id: &str, lbl: &str, placeholder: &str| -> JsEl {
        let input = js_text_input(
            &TextInputSpec::new().with_id(id).with_placeholder(placeholder),
            theme,
        );
        js_field(&FieldSpec::new(id, lbl), theme, Some(input))
    };

    div().flex_col().gap(32.0)
        // Default state — rendered inline for the specimen (not truly overlaid)
        .child(group("Default (submit + cancel actions)", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Create project"),
                theme,
                vec![field("fd-project", "Project name", "Untitled project"), field("fd-desc", "Description", "What is this for?")],
                None,
            )
        ))
        .child(group("With subtitle and error", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Edit track")
                    .with_subtitle("Update track metadata before re-publishing.")
                    .with_error("Title is required."),
                theme,
                vec![field("fd-title", "Title", "Track title"), field("fd-artist", "Artist", "Artist name")],
                None,
            )
        ))
        .child(group("Submitting state (disabled)", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Save changes")
                    .with_submitting(true),
                theme,
                vec![field("fd-name", "Name", "Name")],
                None,
            )
        ))
        .child(group("Success confirmation", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Upload complete")
                    .with_success("Your file was uploaded successfully."),
                theme,
                vec![field("fd-filename", "Filename", "file.wav")],
                None,
            )
        ))
        .child(group("No default actions (custom slot)", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Custom actions")
                    .with_show_default_actions(false),
                theme,
                vec![field("fd-value", "Value", "Enter a value")],
                Some(
                    js_form_actions(
                        &FormActionsSpec::new().with_top_separation(false),
                        theme,
                        vec![
                            js_button(
                                &ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_label("Cancel"),
                                theme,
                            ),
                            js_button(
                                &ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save changes"),
                                theme,
                            ),
                        ],
                    )
                ),
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
