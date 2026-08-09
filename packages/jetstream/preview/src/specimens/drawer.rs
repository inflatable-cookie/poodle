//! Drawer specimen — slide-out drawer panels with content.

use crate::compat::{js_drawer, js_drawer_with_actions};
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use crate::compat::js_button;
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, DrawerEdge, DrawerSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    div()
        .flex_col()
        .gap(24.0)
        // With title and content
        .child(group(
            "With title and content",
            secondary,
            js_drawer(
                &DrawerSpec::new().with_open(true).with_title("Drawer Title"),
                theme,
                Some(
                    div()
                        .flex_col()
                        .gap(8.0)
                        .child(
                            label("Drawer body content goes here.")
                                .text_color(text_primary)
                                .text_size(body_font),
                        )
                        .child(
                            label("Additional details below.")
                                .text_color(secondary)
                                .text_size(body_font),
                        ),
                ),
            ),
        ))
        // With title and description
        .child(group(
            "With description",
            secondary,
            js_drawer(
                &DrawerSpec::new()
                    .with_title("Settings")
                    .with_description("Adjust application settings."),
                theme,
                Some(
                    label("Settings content area.")
                        .text_color(text_primary)
                        .text_size(body_font),
                ),
            ),
        ))
        // Right edge with footer actions (Cancel / Save)
        .child(group(
            "Right edge with actions",
            secondary,
            js_drawer_with_actions(
                &DrawerSpec::new()
                    .with_title("Settings")
                    .with_description("Configure your preferences."),
                theme,
                Some(
                    label("Body content area.")
                        .text_color(text_primary)
                        .text_size(body_font),
                ),
                Some(
                    div()
                        .flex_row()
                        .gap(8.0)
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Cancel"),
                            theme,
                        ))
                        .child(js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Save"),
                            theme,
                        )),
                ),
            ),
        ))
        // Left edge (navigation), no actions
        .child(group(
            "Left edge",
            secondary,
            js_drawer(
                &DrawerSpec::new()
                    .with_edge(DrawerEdge::Left)
                    .with_title("Navigation"),
                theme,
                Some(
                    label("Navigation links here.")
                        .text_color(text_primary)
                        .text_size(body_font),
                ),
            ),
        ))
        // Top edge (full-width banner), dismiss action
        .child(group(
            "Top edge",
            secondary,
            js_drawer_with_actions(
                &DrawerSpec::new()
                    .with_edge(DrawerEdge::Top)
                    .with_title("Notifications")
                    .with_description("Recent activity slides down from the top edge."),
                theme,
                Some(
                    label("Top-anchored drawers span the full width.")
                        .text_color(text_primary)
                        .text_size(body_font),
                ),
                Some(
                    div().flex_row().justify_end().gap(8.0).child(js_button(
                        &ButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Dismiss"),
                        theme,
                    )),
                ),
            ),
        ))
        // Bottom edge (bottom sheet) with Cancel / Apply actions
        .child(group(
            "Bottom edge",
            secondary,
            js_drawer_with_actions(
                &DrawerSpec::new()
                    .with_edge(DrawerEdge::Bottom)
                    .with_title("Quick actions")
                    .with_description("A bottom sheet anchored to the lower edge."),
                theme,
                Some(
                    label("Bottom-anchored drawers rise from the lower edge.")
                        .text_color(text_primary)
                        .text_size(body_font),
                ),
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
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Apply"),
                            theme,
                        )),
                ),
            ),
        ))
        // Empty content
        .child(group(
            "Empty content",
            secondary,
            js_drawer(&DrawerSpec::new().with_title("Empty Drawer"), theme, None),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
