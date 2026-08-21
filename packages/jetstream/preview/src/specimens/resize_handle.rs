//! ResizeHandle specimen — draggable resize divider between panes.
//!
//! Mirrors the GPUI specimen's four groups: horizontal split (vertical handle),
//! vertical split (horizontal handle), and the disabled variant of each. The
//! handle sits between real token-styled panes so its grip + orientation read
//! in context. Every handle is a real `js_resize_handle`.

use crate::compat::js_resize_handle;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{Orientation, ResizeHandleSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let panel = resolve_color(theme, "color.background.panel");
    let border = resolve_color(theme, "color.border.subtle");

    let pane = move |text: &str| -> El {
        div()
            .flex_1()
            .flex_row()
            .items_center()
            .justify_center()
            .bg(tint(panel, 0.5))
            .child(label(text).text_color(secondary).text_size(13.0))
    };

    // Horizontal split: panes side-by-side, vertical handle drags left/right.
    let h_split = |disabled: bool| -> El {
        div()
            .flex_row()
            .h(96.0)
            .border_1()
            .border_color(border)
            .rounded(6.0)
            .overflow_hidden()
            .child(pane("Left"))
            .child(js_resize_handle(
                &ResizeHandleSpec::new(if disabled {
                    "resize-handle:disabled-horizontal"
                } else {
                    "resize-handle:horizontal"
                })
                .with_orientation(Orientation::Horizontal)
                .with_disabled(disabled)
                .with_aria_label(if disabled {
                    "Disabled resize"
                } else {
                    "Resize horizontal"
                }),
                theme,
            ))
            .child(pane("Right"))
    };

    // Vertical split: panes stacked, horizontal handle drags up/down.
    let v_split = |disabled: bool| -> El {
        div()
            .flex_col()
            .h(160.0)
            .border_1()
            .border_color(border)
            .rounded(6.0)
            .overflow_hidden()
            .child(pane("Top"))
            .child(js_resize_handle(
                &ResizeHandleSpec::new(if disabled {
                    "resize-handle:disabled-vertical"
                } else {
                    "resize-handle:vertical"
                })
                .with_orientation(Orientation::Vertical)
                .with_disabled(disabled)
                .with_aria_label(if disabled {
                    "Disabled resize vertical"
                } else {
                    "Resize vertical"
                }),
                theme,
            ))
            .child(pane("Bottom"))
    };

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Horizontal split (vertical handle — drag left/right)",
            secondary,
            h_split(false),
        ))
        .child(group(
            "Vertical split (horizontal handle — drag up/down)",
            secondary,
            v_split(false),
        ))
        .child(group(
            "Disabled (horizontal split)",
            secondary,
            h_split(true),
        ))
        .child(group("Disabled (vertical split)", secondary, v_split(true)))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
