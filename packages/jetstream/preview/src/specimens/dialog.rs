//! Dialog specimen — dialogs with title, description, content, width variants, and actions.

use crate::compat::js_button;
use crate::compat::js_dialog;
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlSize, DialogKind, DialogSpec, DialogWidth,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div()
        .flex_col()
        .gap(24.0)
        // With title and description
        .child(group(
            "With title and description",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Edit Profile")
                    .with_description("Make changes to your profile here."),
                theme,
                vec![
                    div()
                        .flex_col()
                        .gap(8.0)
                        .child(
                            label("Name: Jane Doe")
                                .text_color(text_primary)
                                .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                        )
                        .child(
                            label("Email: jane@example.com")
                                .text_color(secondary)
                                .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                        ),
                ],
                None,
            ),
        ))
        // Title only
        .child(group(
            "Title only",
            secondary,
            js_dialog(
                &DialogSpec::new().with_title("Confirm Action"),
                theme,
                vec![
                    label("Are you sure you want to proceed?")
                        .text_color(text_primary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                None,
            ),
        ))
        // Empty content
        .child(group(
            "Empty content",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Empty Dialog")
                    .with_description("This dialog has no additional content."),
                theme,
                vec![],
                None,
            ),
        ))
        // With close button
        .child(group(
            "With close button",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Closeable Dialog")
                    .with_description("This dialog has a close button in the header.")
                    .with_show_close_button(true),
                theme,
                vec![
                    label("Content goes here.")
                        .text_color(text_primary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                None,
            ),
        ))
        // With actions slot — real Buttons in the footer row
        .child(group(
            "With actions",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("New project")
                    .with_description("Set up a new project workspace.")
                    .with_show_close_button(true),
                theme,
                vec![
                    label("Configure the project before continuing.")
                        .text_color(secondary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                Some(
                    div()
                        .flex_row()
                        .justify_end()
                        .gap(8.0)
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Cancel"),
                            theme,
                        ))
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Create project"),
                            theme,
                        )),
                ),
            ),
        ))
        // Alert role — destructive confirmation with danger-tone Delete button
        .child(group(
            "Alert role",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_role(DialogKind::AlertDialog)
                    .with_title("Delete item?")
                    .with_description(
                        "This will permanently remove the item and all associated data.",
                    ),
                theme,
                vec![
                    label("This action cannot be undone.")
                        .text_color(secondary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                Some(
                    div()
                        .flex_row()
                        .justify_end()
                        .gap(8.0)
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Cancel"),
                            theme,
                        ))
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_tone(ButtonTone::Danger)
                                .with_label("Delete"),
                            theme,
                        )),
                ),
            ),
        ))
        // Width: Sm
        .child(group(
            "Width: Sm",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Small Dialog")
                    .with_width(DialogWidth::Sm),
                theme,
                vec![
                    label("24rem wide.")
                        .text_color(text_primary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                None,
            ),
        ))
        // Width: Lg
        .child(group(
            "Width: Lg",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Large Dialog")
                    .with_width(DialogWidth::Lg),
                theme,
                vec![
                    label("48rem wide.")
                        .text_color(text_primary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                None,
            ),
        ))
        // Width: Xl
        .child(group(
            "Width: Xl",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Extra-large Dialog")
                    .with_width(DialogWidth::Xl),
                theme,
                vec![
                    label("64rem wide.")
                        .text_color(text_primary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                None,
            ),
        ))
        // Width: Full
        .child(group(
            "Width: Full",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Full-width Dialog")
                    .with_width(DialogWidth::Full),
                theme,
                vec![
                    label("Fills the available overlay width.")
                        .text_color(text_primary)
                        .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                ],
                None,
            ),
        ))
        // Scrollable body — long content with footer actions
        .child(group(
            "Scrollable body",
            secondary,
            js_dialog(
                &DialogSpec::new()
                    .with_title("Activity log")
                    .with_description("Recent activity across all projects.")
                    .with_show_close_button(true),
                theme,
                scroll_log_rows(text_primary, secondary),
                Some(
                    div()
                        .flex_row()
                        .justify_end()
                        .gap(8.0)
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Close"),
                            theme,
                        ))
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Export log"),
                            theme,
                        )),
                ),
            ),
        ))
        // Bare mode
        .child(group(
            "Bare mode",
            secondary,
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
                        .child(
                            label("Fully custom content")
                                .text_color(text_primary)
                                .text_size(rem_to_px(size_font_rem(ControlSize::Md))),
                        ),
                ],
                None,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}

/// Build a long, vertically-scrolling list of timestamped log rows for the
/// scrollable-body dialog specimen. Mirrors the Svelte 20-row activity log.
fn scroll_log_rows(text_primary: ColorValue, text_secondary: ColorValue) -> Vec<El> {
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let time_font = rem_to_px(size_font_rem(ControlSize::Xs));
    let messages = [
        "User signed in",
        "Project created",
        "File uploaded",
        "Settings updated",
        "Comment added",
        "Build completed",
        "Deploy started",
        "Review requested",
    ];
    let mut rows = div().flex_col().gap(4.0);
    for i in 0..20u32 {
        let hour = (9 + i / 3).min(23);
        let minute = (i * 17) % 60;
        rows = rows.child(
            div()
                .flex_row()
                .gap(12.0)
                .child(
                    label(format!("{:02}:{:02}", hour, minute))
                        .text_color(text_secondary)
                        .text_size(time_font),
                )
                .child(
                    label(messages[(i as usize) % messages.len()])
                        .text_color(text_primary)
                        .text_size(body_font),
                ),
        );
    }
    vec![rows]
}
