//! Dialog specimen — dialogs with title, description, content, width variants, and actions.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::dialog::js_dialog;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, DialogSpec, DialogWidth};

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
                vec![
                    div().flex_col().gap(8.0)
                        .child(label("Name: Jane Doe").text_color(text_primary).text_size(rem_to_px(size_font_rem(ControlSize::Md))))
                        .child(label("Email: jane@example.com").text_color(secondary).text_size(rem_to_px(size_font_rem(ControlSize::Md))))
                ],
                None,
            )
        ))
        // Title only
        .child(group("Title only", secondary,
            js_dialog(
                &DialogSpec::new().with_title("Confirm Action"),
                theme,
                vec![
                    label("Are you sure you want to proceed?").text_color(text_primary).text_size(rem_to_px(size_font_rem(ControlSize::Md)))
                ],
                None,
            )
        ))
        // Empty content
        .child(group("Empty content", secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Empty Dialog")
                    .with_description("This dialog has no additional content."),
                theme,
                vec![],
                None,
            )
        ))
        // With close button
        .child(group("With close button", secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Closeable Dialog")
                    .with_description("This dialog has a close button in the header.")
                    .with_show_close_button(true),
                theme,
                vec![
                    label("Content goes here.").text_color(text_primary).text_size(rem_to_px(size_font_rem(ControlSize::Md)))
                ],
                None,
            )
        ))
        // With actions slot
        .child(group("With actions", secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Confirm Delete")
                    .with_description("This action cannot be undone."),
                theme,
                vec![
                    label("The item will be permanently removed.").text_color(secondary).text_size(rem_to_px(size_font_rem(ControlSize::Md)))
                ],
                Some(
                    div().flex_row().justify_end().gap(8.0)
                        .child(label("Cancel").text_color(secondary).text_size(rem_to_px(size_font_rem(ControlSize::Md))))
                        .child(label("Delete").text_color(text_primary).text_size(rem_to_px(size_font_rem(ControlSize::Md))))
                ),
            )
        ))
        // Width: Sm
        .child(group("Width: Sm", secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Small Dialog")
                    .with_width(DialogWidth::Sm),
                theme,
                vec![label("24rem wide.").text_color(text_primary).text_size(rem_to_px(size_font_rem(ControlSize::Md)))],
                None,
            )
        ))
        // Width: Lg
        .child(group("Width: Lg", secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Large Dialog")
                    .with_width(DialogWidth::Lg),
                theme,
                vec![label("48rem wide.").text_color(text_primary).text_size(rem_to_px(size_font_rem(ControlSize::Md)))],
                None,
            )
        ))
        // Bare mode
        .child(group("Bare mode", secondary,
            js_dialog(
                &DialogSpec::new().with_bare(true),
                theme,
                vec![
                    div()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .pt(rem_to_px(1.5))
                        .pb(rem_to_px(1.5))
                        .child(label("Fully custom content").text_color(text_primary).text_size(rem_to_px(size_font_rem(ControlSize::Md))))
                ],
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
