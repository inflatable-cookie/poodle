//! Drawer specimen — slide-out drawer panels with content.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::drawer::js_drawer;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::DrawerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        // With title and content
        .child(group("With title and content", secondary,
            js_drawer(
                &DrawerSpec::new().with_title("Drawer Title"),
                theme,
                Some(
                    div().flex_col().gap(8.0)
                        .child(label("Drawer body content goes here.").text_color(text_primary).text_size(13.0))
                        .child(label("Additional details below.").text_color(secondary).text_size(13.0))
                ),
            )
        ))
        // With title and description
        .child(group("With description", secondary,
            js_drawer(
                &DrawerSpec::new()
                    .with_title("Settings")
                    .with_description("Adjust application settings."),
                theme,
                Some(
                    label("Settings content area.").text_color(text_primary).text_size(13.0)
                ),
            )
        ))
        // Empty content
        .child(group("Empty content", secondary,
            js_drawer(
                &DrawerSpec::new().with_title("Empty Drawer"),
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
