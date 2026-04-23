//! FormShell specimen — form container with sections and submission.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::form_shell::js_form_shell;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, FormSectionSpec, FormShellSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    let sections = vec![
        FormSectionSpec::new("details", "Project Details", vec![
            String::from("name"),
            String::from("description"),
        ]),
        FormSectionSpec::new("settings", "Settings", vec![
            String::from("visibility"),
        ]),
    ];

    let actions_slot = div().flex_row().gap(8.0).justify_end()
        .child(
            button("Cancel")
                .text_color(text_primary).text_size(body_font)
        )
        .child(
            button("Save")
                .text_color(text_primary).text_size(body_font)
        );

    div().flex_col().gap(24.0)
        .child(group("Basic form", secondary,
            js_form_shell(
                &FormShellSpec::new("project-form")
                    .with_title("New Project")
                    .with_description("Create a new project workspace.")
                    .with_sections(sections),
                theme,
                vec![None, None],
                Some(actions_slot),
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
