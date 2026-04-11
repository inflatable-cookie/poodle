//! Callout specimen — status callouts with different tones.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::callout::js_callout;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{CallOutSpec, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Info", secondary,
            js_callout(&CallOutSpec::new()
                .with_tone(StatusTone::Info)
                .with_title("Information")
                .with_content("This is an informational callout."), theme)
        ))
        .child(group("Success", secondary,
            js_callout(&CallOutSpec::new()
                .with_tone(StatusTone::Success)
                .with_title("Success")
                .with_content("Operation completed successfully."), theme)
        ))
        .child(group("Warning", secondary,
            js_callout(&CallOutSpec::new()
                .with_tone(StatusTone::Warning)
                .with_title("Warning")
                .with_content("Please review before continuing."), theme)
        ))
        .child(group("Danger", secondary,
            js_callout(&CallOutSpec::new()
                .with_tone(StatusTone::Danger)
                .with_title("Error")
                .with_content("Something went wrong."), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
