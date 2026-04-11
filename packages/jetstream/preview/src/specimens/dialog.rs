//! Dialog specimen — dialogs with title, description, and content.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::dialog::js_dialog;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::DialogSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        // With title and description
        .child(group("With title and description", secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Edit Profile")
                    .with_description("Make changes to your profile here."),
                theme,
                Some(
                    div().flex_col().gap(8.0)
                        .child(label("Name: Jane Doe").text_color(text_primary).text_size(13.0))
                        .child(label("Email: jane@example.com").text_color(secondary).text_size(13.0))
                ),
            )
        ))
        // Title only
        .child(group("Title only", secondary,
            js_dialog(
                &DialogSpec::new().with_title("Confirm Action"),
                theme,
                Some(
                    label("Are you sure you want to proceed?").text_color(text_primary).text_size(13.0)
                ),
            )
        ))
        // Empty content
        .child(group("Empty content", secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Empty Dialog")
                    .with_description("This dialog has no additional content."),
                theme,
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
