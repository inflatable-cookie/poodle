//! Drawer specimen — slide-out drawer panels with content.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::drawer::{js_drawer, js_drawer_with_actions};
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, DrawerEdge, DrawerSpec};
use poodle_jetstream_components::button::js_button;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        // With title and content
        .child(group("With title and content", secondary,
            js_drawer(
                &DrawerSpec::new().with_title("Drawer Title"),
                theme,
                Some(
                    div().flex_col().gap(8.0)
                        .child(label("Drawer body content goes here.").text_color(text_primary).text_size(body_font))
                        .child(label("Additional details below.").text_color(secondary).text_size(body_font))
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
                    label("Settings content area.").text_color(text_primary).text_size(body_font)
                ),
            )
        ))
        // Right edge with footer actions (Cancel / Save)
        .child(group("Right edge with actions", secondary,
            js_drawer_with_actions(
                &DrawerSpec::new()
                    .with_title("Settings")
                    .with_description("Configure your preferences."),
                theme,
                Some(
                    label("Body content area.").text_color(text_primary).text_size(body_font)
                ),
                Some(
                    div().flex_row().gap(8.0)
                        .child(js_button(
                            &ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Cancel"),
                            theme,
                        ))
                        .child(js_button(
                            &ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save"),
                            theme,
                        ))
                ),
            )
        ))
        // Left edge (navigation), no actions
        .child(group("Left edge", secondary,
            js_drawer(
                &DrawerSpec::new()
                    .with_edge(DrawerEdge::Left)
                    .with_title("Navigation"),
                theme,
                Some(
                    label("Navigation links here.").text_color(text_primary).text_size(body_font)
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
