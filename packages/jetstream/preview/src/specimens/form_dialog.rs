//! FormDialog specimen — modal dialog wrapping a form.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::form_actions::js_form_actions;
use poodle_jetstream_components::form_dialog::js_form_dialog;
use poodle_jetstream_components::presentation::rem_to_px;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ButtonSpec, ButtonVariant, FormActionsSpec, FormDialogSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");

    let field = |lbl: &str| -> JsEl {
        div().flex_col().gap(rem_to_px(0.25))
            .child(label(lbl).text_color(secondary).text_size(11.0))
            .child(
                div()
                    .h(rem_to_px(2.0))
                    .border_1().border_color(border)
                    .rounded(rem_to_px(0.25))
                    .px(rem_to_px(0.5))
            )
    };

    div().flex_col().gap(32.0)
        // Default state — rendered inline for the specimen (not truly overlaid)
        .child(group("Default (submit + cancel actions)", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Create project"),
                theme,
                vec![field("Project name"), field("Description")],
                None,
            )
        ))
        .child(group("With subtitle and error", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Edit track")
                    .with_subtitle("Update track metadata before re-publishing.")
                    .with_error("Title is required."),
                theme,
                vec![field("Title"), field("Artist")],
                None,
            )
        ))
        .child(group("Submitting state (disabled)", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Save changes")
                    .with_submitting(true),
                theme,
                vec![field("Name")],
                None,
            )
        ))
        .child(group("Success confirmation", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Upload complete")
                    .with_success("Your file was uploaded successfully."),
                theme,
                vec![field("Filename")],
                None,
            )
        ))
        .child(group("No default actions (custom slot)", secondary,
            js_form_dialog(
                &FormDialogSpec::new("Custom actions")
                    .with_show_default_actions(false),
                theme,
                vec![field("Value")],
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
