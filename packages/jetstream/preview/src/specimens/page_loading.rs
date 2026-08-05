//! PageLoading specimen — inline + overlay presentation, indeterminate,
//! determinate, cancel control, and the size ladder.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_page_loading;

use poodle_specs::{ControlSize, PageLoadingPresentation, PageLoadingSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    let sizes: [(ControlSize, &str); 5] = [
        (ControlSize::Xs, "xs"),
        (ControlSize::Sm, "sm"),
        (ControlSize::Md, "md"),
        (ControlSize::Lg, "lg"),
        (ControlSize::Xl, "xl"),
    ];
    let mut size_ladder = div().flex_col().gap(16.0);
    for (size, name) in sizes {
        size_ladder = size_ladder.child(group(
            name,
            secondary,
            div().h(200.0).child(js_page_loading(
                &PageLoadingSpec::new()
                    .with_message("Loading data...")
                    .with_size(size),
                theme,
            )),
        ));
    }

    div()
        .flex_col()
        .gap(24.0)
        // Inline presentation — in-flow centered spinner + message, no scrim.
        .child(group(
            "Inline",
            secondary,
            div().h(220.0).child(js_page_loading(
                &PageLoadingSpec::new()
                    .with_presentation(PageLoadingPresentation::Inline)
                    .with_message("Loading section content..."),
                theme,
            )),
        ))
        // Indeterminate overlay — ring spinner + message, no progress bar.
        .child(group(
            "Indeterminate",
            secondary,
            div().h(200.0).child(js_page_loading(
                &PageLoadingSpec::new().with_message("Loading your workspace..."),
                theme,
            )),
        ))
        // Determinate overlay — spinner + progress bar + percentage message.
        .child(group(
            "Determinate (45%)",
            secondary,
            div().h(200.0).child(js_page_loading(
                &PageLoadingSpec::new()
                    .with_message("Importing assets... 45%")
                    .with_value(45.0)
                    .with_max(100.0),
                theme,
            )),
        ))
        // With cancel — spinner + message + bordered cancel control.
        .child(group(
            "With cancel button",
            secondary,
            div().h(200.0).child(js_page_loading(
                &PageLoadingSpec::new()
                    .with_message("Processing request...")
                    .with_can_cancel(true),
                theme,
            )),
        ))
        // Size ladder — xs → xl (size drives message font size).
        .child(group("Sizes", secondary, size_ladder))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
